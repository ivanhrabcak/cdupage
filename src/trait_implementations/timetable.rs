use std::any::Any;
use std::collections::HashMap;
use std::fmt::format;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use common_macros::hash_map;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use crate::edupage::{Edupage, EdupageError, RequestType};
use crate::edupage::RequestType::POST;
use crate::edupage_types::{Lesson, Timetable as EduTimetable};

impl EduTimetable {

}

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

        let gsec_request_url = format!("https://{}.edupage.org/dashboard/eb.php", &edupage.subdomain.unwrap());

        let gsec_hash = match edupage.request(gsec_request_url, RequestType::GET, None, None) {
            Ok(h) => h.text().unwrap().split("gsechash=").nth(1).unwrap().split("\"").nth(1).unwrap()
            Err(_) => return Err(EdupageError::InvalidResponse)
        }

        let request_url = format!(
            "https://{}.edupage.org/dashboard/server/onlinelesson.js?__func=getOnlineLessonOpenUrl",
            edupage.subdomain.unwrap()
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
            &self.online_lesson_link.unwrap(),
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


        let json_result = match response {
            Ok(r) => r.json::<HashMap<String, dyn Any>>(),
            Err(e) => return Err(EdupageError::HTTPError(e.to_string()))
        };

        return match json_result {
            Ok(r) => if r["reload"].is_some() { Err(EdupageError::InvalidResponse) } else { Ok(()) },
            Err(e) => Err(EdupageError::ParseError(e.to_string()))
        }
    }
}