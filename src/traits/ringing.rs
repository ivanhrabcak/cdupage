use crate::{
    edupage::Edupage, types::RingingTime,
};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum NextDayPart {
    LESSON,
    BREAK,
}

pub trait Ringing {
    fn get_ringing_times(&self) -> Vec<RingingTime>;
    fn get_next_lesson_time(&self, time: NaiveDateTime) -> Option<(NaiveDateTime, NextDayPart)>;
}

impl RingingTime {
    pub fn new(name: i64, start_time: NaiveDateTime, end_time: NaiveDateTime) -> Self {
        Self {
            name,
            start_time,
            end_time,
        }
    }
}

impl Ringing for Edupage {
    fn get_ringing_times(&self) -> Vec<RingingTime> {
        match &self.data {
            Some(x) => x.ringing_times.clone(),
            None => Vec::new(),
        }
    }

    fn get_next_lesson_time(
        &self,
        time: NaiveDateTime,
    ) -> Option<(chrono::NaiveDateTime, NextDayPart)> {
        let day_of_week = Local::now().date_naive().format("%a").to_string();
        if day_of_week == "Sun" || day_of_week == "Sat" {
            return None;
        }

        for lesson in self.get_ringing_times() {
            if time < lesson.start_time {
                return Some((lesson.start_time, NextDayPart::LESSON));
            } else if time < lesson.end_time {
                return Some((lesson.end_time, NextDayPart::BREAK));
            }
        }

        None
    }
}

