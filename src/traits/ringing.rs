use crate::{edupage::Edupage, types::RingingTime};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum NextDayPart {
    LESSON,
    BREAK,
}

pub trait Ringing {
    extern "C" fn get_ringing_times(&self) -> Vec<RingingTime>;
    extern "C" fn get_next_lesson_time(
        &self,
        time: NaiveDateTime,
    ) -> Option<(NaiveDateTime, NextDayPart)>;
}

impl RingingTime {
    pub extern "C" fn new(name: i64, start_time: NaiveDateTime, end_time: NaiveDateTime) -> Self {
        Self {
            name,
            start_time,
            end_time,
        }
    }
}

impl Ringing for Edupage {
    /// Get the start and end times for lessons. The lessons are in-order.
    extern "C" fn get_ringing_times(&self) -> Vec<RingingTime> {
        match &self.data {
            Some(x) => x.ringing_times.clone(),
            None => Vec::new(),
        }
    }

    /// Returns `None` if the specified date is on a weekend.
    ///
    /// If parameter `time` is a time during a lesson, `NextDayPart::BREAK` is reported as the next lesson.   
    extern "C" fn get_next_lesson_time(
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
