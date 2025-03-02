use serde::{Deserialize, Deserializer, Serialize};

use crate::types::UserID;

pub fn get_string_representation(item_type: &UserID) -> String {
    match item_type {
        UserID::Teacher(id) => format!("Ucitel{}", id),
        UserID::Student(id) => format!("Student{}", id),
        UserID::Parent(id) => format!("Rodic{}", id),
        UserID::Class(id) => format!("Trieda{}", id),
        UserID::Plan(id) => format!("Plan{}", id),
        UserID::CustomPlan(id) => format!("CustPlan{}", id),
        UserID::StudentClass(id) => format!("StudTrieda{}", id),
        UserID::OnlyStudent(id) => format!("StudentOnly{}", id),
        UserID::StudentPlan(id) => format!("StudPlan{}", id),
        UserID::OnlyAllStudents => "StudentOnly*".to_string(),
        UserID::AllStudents => "Student*".to_string(),
        UserID::AllTeachers => "Ucitel*".to_string(),
        UserID::Everyone => "*".to_string(),
    }
}

fn parse_userid(s: &str) -> Option<UserID> {
    // we first try to parse the simple ones
    let user_type: Option<UserID> = match s {
        "*" => Some(UserID::Everyone),
        "Student*" => Some(UserID::AllStudents),
        "Ucitel*" => Some(UserID::AllTeachers),
        "StudentOnly*" => Some(UserID::OnlyAllStudents),
        _ => None,
    };

    if let Some(user_type) = user_type {
        return Some(user_type);
    }

    let mut id: String = String::new();
    let mut user_type: String = String::new();

    for char in s.chars() {
        if char.is_alphabetic() {
            user_type += &char.to_string();
        } else {
            id += &char.to_string();
        }
    }

    let id: i64 = id.parse().unwrap(); // should always be a number
    let user_type: &str = &user_type;

    Some(match user_type {
        "Ucitel" => UserID::Teacher(id),
        "Student" => UserID::Student(id),
        "Rodic" => UserID::Parent(id),
        "Trieda" => UserID::Class(id),
        "Plan" => UserID::Plan(id),
        "CustPlan" => UserID::CustomPlan(id),
        "StudTrieda" => UserID::StudentClass(id),
        "StudentOnly" => UserID::OnlyStudent(id),
        "StudPlan" => UserID::StudentPlan(id),
        _ => return None,
    })
}

impl Serialize for UserID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string_representation = get_string_representation(self);
        serializer.serialize_str(&string_representation)
    }
}

impl<'de> Deserialize<'de> for UserID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = &String::deserialize(deserializer)?;

        let user_id = parse_userid(s);
        if user_id.is_none() {
            return Err(serde::de::Error::custom("Unexpected user type".to_string()));
        }

        Ok(user_id.unwrap())
    }
}
