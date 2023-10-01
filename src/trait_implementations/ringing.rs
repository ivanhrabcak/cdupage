use chrono::{Local, NaiveDateTime};
use crate::edupage::Edupage;
use crate::edupage_traits::{NextDayPart, Ringing};
use crate::edupage_types::RingingTime;

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

    fn get_next_lesson_time(&self, time: NaiveDateTime) -> Option<(chrono::NaiveDateTime, NextDayPart)> {
        let day_of_week = Local::now().date().format("%a").to_string();
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

        return None;
    }
}