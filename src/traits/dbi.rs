use crate::{
    edupage::EdupageError, edupage::Edupage, types::dbi::*, types::person::*,
    types::timeline::*, types::timetable::Timetable as EduTimetable,
    types::RingingTime,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};


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


impl DBI for Edupage {
    fn get_teachers(
        &self,
    ) -> Result<Vec<crate::types::person::Teacher>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.teachers.clone())
    }

    fn get_teacher_by_id(
        &self,
        id: i64,
    ) -> Result<Option<crate::types::person::Teacher>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        for teacher in data.dbi.teachers.clone() {
            if teacher.id.is_some() && teacher.id.unwrap() == id {
                return Ok(Some(teacher));
            }
        }

        Ok(None)
    }

    fn get_students(
        &self,
    ) -> Result<Vec<crate::types::person::Student>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.students.clone())
    }

    fn get_student_by_id(
        &self,
        id: i64,
    ) -> Result<Option<crate::types::person::Student>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        for student in data.dbi.students.clone() {
            if student.id.is_some() && student.id.unwrap() == id {
                return Ok(Some(student));
            }
        }

        Ok(None)
    }

    fn get_subjects(
        &self,
    ) -> Result<Vec<crate::types::dbi::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.subjects.clone())
    }

    fn get_subject_by_id(
        &self,
        id: i64,
    ) -> Result<Option<crate::types::dbi::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        for subject in data.dbi.subjects.clone() {
            if subject.id.is_some() && subject.id.unwrap() == id {
                return Ok(Some(subject));
            }
        }

        Ok(None)
    }

    fn get_classrooms(
        &self,
    ) -> Result<Vec<crate::types::dbi::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.classrooms.clone())
    }

    fn get_classroom_by_id(
        &self,
        id: i64,
    ) -> Result<Option<crate::types::dbi::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        for classroom in data.dbi.classrooms.clone() {
            if classroom.id.is_some() && classroom.id.unwrap() == id {
                return Ok(Some(classroom));
            }
        }

        Ok(None)
    }
}