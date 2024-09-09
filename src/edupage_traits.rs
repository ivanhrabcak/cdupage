use crate::{edupage::EdupageError, edupage_types::Timetable as EduTimetable, edupage_types::*};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

pub trait Login {
    fn login(
        &mut self,
        subdomain: &String,
        username: &String,
        password: &String,
    ) -> Result<(), EdupageError>;
}

pub trait DBI {
    fn get_teachers(&self) -> Result<Vec<Teacher>, EdupageError>;
    fn get_teacher_by_id(&self, id: i64) -> Result<Option<Teacher>, EdupageError>;

    fn get_students(&self) -> Result<Vec<Student>, EdupageError>;
    fn get_student_by_id(&self, id: i64) -> Result<Option<Student>, EdupageError>;

    fn get_subjects(&self) -> Result<Vec<DBIBase>, EdupageError>;
    fn get_subject_by_id(&self, id: i64) -> Result<Option<DBIBase>, EdupageError>;

    fn get_classrooms(&self) -> Result<Vec<DBIBase>, EdupageError>;
    fn get_classroom_by_id(&self, id: i64) -> Result<Option<DBIBase>, EdupageError>;
}

pub trait Timeline {
    fn filter_timeline_by_item_type(
        &self,
        item_type: TimelineItemType,
    ) -> Result<Vec<TimelineItem>, EdupageError>;
    fn filter_timeline_by_item_types(
        &self,
        item_types: Vec<TimelineItemType>,
    ) -> Result<Vec<TimelineItem>, EdupageError>;
}

pub trait Timetable {
    fn get_timetable(&self, date: NaiveDate) -> Result<EduTimetable, EdupageError>;
}

pub trait Substitution {
    fn get_substitution_html(
        &self,
        date: &NaiveDate,
        subdomain: &String,
    ) -> Result<String, EdupageError>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum NextDayPart {
    LESSON,
    BREAK,
}

pub trait Ringing {
    fn get_ringing_times(&self) -> Vec<RingingTime>;
    fn get_next_lesson_time(&self, time: NaiveDateTime) -> Option<(NaiveDateTime, NextDayPart)>;
}
