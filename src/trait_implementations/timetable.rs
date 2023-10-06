use std::any::Any;
use std::collections::HashMap;
use std::fmt::format;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use common_macros::hash_map;
use reqwest::Error;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use crate::edupage::{Edupage, EdupageError, RequestType};
use crate::edupage::RequestType::POST;
use crate::edupage_traits::Timetable;
use crate::edupage_types::{Lesson, Timetable as EduTimetable};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OnlineLessonErrorLoginResponse {
    pub reload: bool
}

impl Lesson {
    pub fn is_online_lesson(&self) -> bool {
        return self.online_lesson_link.is_some()
    }

    pub fn sign_into_lesson(&self, edupage: &Edupage) -> Result<(), EdupageError> {
        if !edupage.logged_in() {
            return Err(EdupageError::NotLoggedIn)
        }

        let gsec_request_url = format!("https://{}.edupage.org/dashboard/eb.php", edupage.subdomain.as_ref().unwrap());

        let gsec_hash_response = match edupage.request(gsec_request_url, RequestType::GET, None, None) {
            Ok(h) => h,
            Err(_) => return Err(EdupageError::InvalidResponse)
        };

        let gsec_hash_response_text = gsec_hash_response.text().unwrap();
        let gsec_hash = gsec_hash_response_text.split("gsechash=").nth(1).unwrap().split("\"").nth(1).unwrap();

        let request_url = format!(
            "https://{}.edupage.org/dashboard/server/onlinelesson.js?__func=getOnlineLessonOpenUrl",
            edupage.subdomain.clone().unwrap()
        );

        let today = Utc::now().naive_local();
        let post_data = format!("{{[\
            null,\
            {{\
                \"click\": true,
                \"date\": \"{}\",
                \"ol_url\": \"{}\",
                \"subjectid\": \"{}\",
            }}
            ], \"__gsh\": \"{}\"}}",
            today.format("%Y-%m-%d"),
            &self.online_lesson_link.clone().unwrap(),
            &self.subject_id,
            gsec_hash
        );

        println!("{post_data}");

        let response = edupage.request(
            request_url,
            POST,
            Some(hash_map!(
                "Content-Type".to_string() => "application/json".to_string()
            )),
            Some(post_data)
        );


        let json_result: Result<HashMap<String, serde_json::Value>, Error> = match response {
            Ok(r) => r.json::<HashMap<String, serde_json::Value>>(),
            Err(e) => return Err(EdupageError::HTTPError(e.to_string()))
        };

        return match json_result {
            Ok(r) => if !r["reload"].is_null() { Err(EdupageError::InvalidResponse) } else { Ok(()) },
            Err(e) => Err(EdupageError::ParseError(e.to_string()))
        }
    }
}


pub struct TimetableIntoIterator {
    index: usize,
    timetable: EduTimetable
}

impl IntoIterator for EduTimetable {
    type Item = Lesson;
    type IntoIter = TimetableIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TimetableIntoIterator {
            index: 0,
            timetable: self
        }
    }
}

impl Iterator for TimetableIntoIterator {
    type Item = Lesson;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.timetable.lessons.get(self.index).cloned();
        self.index += 1;

        return result;
    }
}

impl EduTimetable {
    pub fn get_lesson_at_time(&self, time: NaiveDateTime) -> Option<Lesson> {
        for lesson in self.clone().into_iter() {
            if time >= lesson.start_of_lesson && time <= lesson.end_of_lesson {
                return Some(lesson.clone())
            }
        }

        None
    }

    pub fn get_next_lesson_at_time(&self, time: NaiveDateTime) -> Option<Lesson> {
        for lesson in self.clone().into_iter() {
            if time < lesson.start_of_lesson {
                return Some(lesson)
            }
        }

        None
    }

    pub fn get_next_online_lesson_at_time(&self, time: NaiveDateTime) -> Option<Lesson> {
        for lesson in self.clone().into_iter() {
            if time < lesson.start_of_lesson && lesson.is_online_lesson() {
                return Some(lesson)
            }
        }

        None
    }

    pub fn get_first_lesson(&self) -> Option<Lesson> {
        if self.lessons.len() > 0 {
            return Some(self.lessons[0].clone())
        }
        
        None
    }

    pub fn get_last_lesson(&self) -> Option<Lesson> {    
        return self.lessons.last().cloned()
    }


}