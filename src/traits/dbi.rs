use crate::{edupage::Edupage, edupage::EdupageError, types::dbi::*, types::person::*};

pub trait DBI {
    extern "C" fn get_teachers(&self) -> Result<Vec<Teacher>, EdupageError>;
    extern "C" fn get_teacher_by_id(&self, id: i64) -> Result<Option<Teacher>, EdupageError>;

    extern "C" fn get_students(&self) -> Result<Vec<Student>, EdupageError>;
    extern "C" fn get_student_by_id(&self, id: i64) -> Result<Option<Student>, EdupageError>;

    extern "C" fn get_subjects(&self) -> Result<Vec<DBIBase>, EdupageError>;
    extern "C" fn get_subject_by_id(&self, id: i64) -> Result<Option<DBIBase>, EdupageError>;

    extern "C" fn get_classrooms(&self) -> Result<Vec<DBIBase>, EdupageError>;
    extern "C" fn get_classroom_by_id(&self, id: i64) -> Result<Option<DBIBase>, EdupageError>;
}

impl DBI for Edupage {
    #[no_mangle]
    extern "C" fn get_teachers(
        &self,
    ) -> Result<Vec<crate::types::person::Teacher>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.teachers.clone())
    }
    #[no_mangle]
    extern "C" fn get_teacher_by_id(
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
    #[no_mangle]
    extern "C" fn get_students(
        &self,
    ) -> Result<Vec<crate::types::person::Student>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.students.clone())
    }
    #[no_mangle]
    extern "C" fn get_student_by_id(
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
    #[no_mangle]
    extern "C" fn get_subjects(
        &self,
    ) -> Result<Vec<crate::types::dbi::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.subjects.clone())
    }
    #[no_mangle]
    extern "C" fn get_subject_by_id(
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
    #[no_mangle]
    extern "C" fn get_classrooms(
        &self,
    ) -> Result<Vec<crate::types::dbi::DBIBase>, crate::edupage::EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        Ok(data.dbi.classrooms.clone())
    }
    #[no_mangle]
    extern "C" fn get_classroom_by_id(
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
