use crate::{edupage_traits::DBI, edupage::{Edupage, EdupageError}};

impl DBI for Edupage {
    fn get_teachers(&self) -> Result<Vec<crate::edupage_types::Teacher>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn)
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.teachers.clone())
    }

    fn get_teacher_by_id(&self, id: i64) -> Result<Option<crate::edupage_types::Teacher>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn)
        }

        let data = self.data.as_ref().unwrap();

        for teacher in data.dbi.teachers.clone() {
            if teacher.id.is_some() && teacher.id.unwrap() == id {
                return Ok(Some(teacher));
            }
        }

        return Ok(None);
    }

    fn get_students(&self) -> Result<Vec<crate::edupage_types::Student>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn)
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.students.clone())
    }

    fn get_student_by_id(&self, id: i64) -> Result<Option<crate::edupage_types::Student>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn)
        }

        let data = self.data.as_ref().unwrap();

        for student in data.dbi.students.clone() {
            if student.id.is_some() && student.id.unwrap() == id {
                return Ok(Some(student));
            }
        }

        Ok(None)
    }

    fn get_subjects(&self) -> Result<Vec<crate::edupage_types::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn)
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.subjects.clone())
    }

    fn get_subject_by_id(&self, id: i64) -> Result<Option<crate::edupage_types::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn)
        }

        let data = self.data.as_ref().unwrap();

        for subject in data.dbi.subjects.clone() {
            if subject.id.is_some() && subject.id.unwrap() == id {
                return Ok(Some(subject));
            }
        }

        Ok(None)
    }

    fn get_classrooms(&self) -> Result<Vec<crate::edupage_types::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn)
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.classrooms.clone())
    }

    fn get_classroom_by_id(&self, id: i64) -> Result<Option<crate::edupage_types::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn)
        }

        let data = self.data.as_ref().unwrap();

        for classroom in data.dbi.classrooms.clone() {
            if classroom.id.is_some() && classroom.id.unwrap() == id {
                return Ok(Some(classroom))
            }
        }

        Ok(None)
    }
}