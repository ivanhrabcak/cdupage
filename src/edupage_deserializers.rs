use num_enum::TryFromPrimitiveError;

use crate::edupage_types::{TimelineItemType, UserID};

pub const TIMELINE_ITEM_TYPE_NAMES: [&'static str; 18] = 
            ["news", "sprava", "h_dailyplan", "student_absent", "confirmation", 
                "h_clearplany", "h_financie", "h_stravamenu", "h_clearisicdata", 
                "substitution", "h_clearcache", "event", "h_homework", "znamka", 
                "h_substitution", "h_znamky", "homework", "h_cleardbi"];

impl TryFrom<&str> for TimelineItemType {
    type Error = TryFromPrimitiveError<TimelineItemType>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let integer_value = match TIMELINE_ITEM_TYPE_NAMES.binary_search(&value) {
            Ok(v) => v,
            Err(_) => Self::Unknown as usize
        };

        integer_value.try_into()
    }
}

impl TimelineItemType {
    pub fn as_str(&self) -> &'static str {
        let integer_value = *self as usize;
        
        Self::key_name_for_n(integer_value)
    }

    pub fn key_name_for_n(n: usize) -> &'static str {
        TIMELINE_ITEM_TYPE_NAMES[n]
    }
}


pub mod timeline_item_type {
    use serde::{self, Deserialize, Serializer, Deserializer};

    use crate::edupage_types::TimelineItemType;

    use super::TIMELINE_ITEM_TYPE_NAMES;

    pub fn serialize<S>(
        item_type: &TimelineItemType,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let integer_value = *item_type as usize;

        serializer.serialize_str(TIMELINE_ITEM_TYPE_NAMES[integer_value])
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<TimelineItemType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = &String::deserialize(deserializer)?;
        
        let item_type: TimelineItemType = match s.try_into() {
            Ok(x) => x,
            Err(_) => return Err(serde::de::Error::custom("Failed to deserialize TimelineItemType"))
        };

        Ok(item_type)
    }
}


pub mod gender {
    use serde::{self, Deserialize, Serializer, Deserializer};

    use crate::edupage_types::Gender;

    pub fn serialize<S>(
        gender: &Gender,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match gender {
            &Gender::Male => "M",
            &Gender::Female => "F"
        })
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Gender, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = &String::deserialize(deserializer)?.to_lowercase();
        
        match s {
            "m" => Ok(Gender::Male),
            "f" => Ok(Gender::Female),
            _ => 
                Err(serde::de::Error::custom(format!("Failed to deserialize gender: {}", s)))
        }
    }
}


pub mod gender_option {
    use std::borrow::Cow;

    use serde::{self, Deserialize, Serializer, Deserializer};

    use crate::edupage_types::Gender;

    pub fn serialize<S>(
        gender: &Option<Gender>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if gender.is_none() {
            return serializer.serialize_none();
        }
        let gender = gender.unwrap();
        serializer.serialize_str(match gender {
            Gender::Male => "M",
            Gender::Female => "F"
        })
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Gender>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &Option<Cow<str>> = &Deserialize::deserialize(deserializer)?;

        if s.is_none() {
            return Ok(None);
        }

        let string = match s {
                Some(val) => val,
                None => unreachable!(),
            }.as_ref().to_lowercase();
        
        if string.is_empty() {
            return Ok(None);
        }

        if string.eq("f") {
            Ok(Some(Gender::Female))
        }
        else if string.eq("m") {
            Ok(Some(Gender::Male))
        }
        else {
            Err(serde::de::Error::custom(format!("Failed to deserialize gender: {}", string)))
        }
    }
}

fn get_string_representation(item_type: &UserID) -> String {
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
        _ => None
    };

    if let Some(user_type) = user_type {
        return Some(user_type)
    }

    let mut id: String = String::new();
    let mut user_type: String = String::new();

    for char in s.chars() {

        if char.is_alphabetic() {
            user_type += &char.to_string();
        }
        else {
            id += &char.to_string();
        }
    }

    let id: i128 = id.parse().unwrap(); // should always be a number
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
        _ => return None
    })
}

pub mod user_id_option {
    use serde::{self, Deserialize, Serializer, Deserializer};

    use crate::edupage_types::UserID;

    use super::{get_string_representation, parse_userid};

    pub fn serialize<S>(
        item_type: &Option<UserID>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if item_type.is_none() {
            return serializer.serialize_none();
        }

        let item_type = item_type.unwrap();
        let string_representation = get_string_representation(&item_type);
        serializer.serialize_str(&string_representation)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<UserID>, D::Error>
    where
        D: Deserializer<'de> 
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;
        
        if s.is_none() {
            return Ok(None);
        }

        let s = &s.unwrap();

        Ok(parse_userid(s))
    }
}

pub mod user_id {
    use serde::{self, Deserialize, Serializer, Deserializer};

    use crate::edupage_types::UserID;

    use super::{get_string_representation, parse_userid};

    pub fn serialize<S>(
        item_type: &UserID,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_representation = get_string_representation(item_type);
        serializer.serialize_str(&string_representation)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<UserID, D::Error>
    where
        D: Deserializer<'de> 
    {
        let s: &str = &String::deserialize(deserializer)?;

        let user_id = parse_userid(s);
        if user_id.is_none() {
            return Err(serde::de::Error::custom(format!("Unexpected user type")));
        }

        return Ok(user_id.unwrap())
    }
}

pub mod string_i64_option {
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        id: &Option<i64>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if id.is_none() {
            serializer.serialize_none()
        }
        else {
            serializer.serialize_i64(id.unwrap())
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de> 
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;

        if s.is_none() {
            return Ok(None);
        }

        let s = &s.unwrap();
        if s.is_empty() {
            return Ok(None);
        }

        match s.parse() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Ok(None)
        }
    }
}

pub mod javascript_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod teachers {
    use serde::{self, Serializer, Deserializer, Serialize, ser, Deserialize};
    use serde_json::{Value, Map};

    use crate::edupage_types::Teacher;

    pub fn serialize<S>(
        teacher: &Vec<Teacher>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let j = serde_json::to_string(teacher).map_err(ser::Error::custom)?;
        j.serialize(serializer)  
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Vec<Teacher>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ts: Map<String, Value> = Map::deserialize(deserializer)?;

        
        let mut teachers: Vec<Teacher> = Vec::new();
        for teacher in ts.values() {
            let t: Teacher = serde_json::from_value(teacher.clone()).unwrap();
            teachers.push(t);
        }

        Ok(teachers)
    }
}


pub mod year_month_day_optional {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(
        date: &Option<NaiveDate>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if date.is_none() {
            return serializer.serialize_none();
        }
        let date = date.unwrap();

        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;

        if s.is_none() {
            return Ok(None);
        }
        let s = s.unwrap();
        
        if s.is_empty() {
            Ok(None)
        }
        else {
            match NaiveDate::parse_from_str(&s, FORMAT) {
                Ok(x) => Ok(Some(x)),
                Err(e) => Err(serde::de::Error::custom(e.to_string()))
            }
        }
        
    }
}

pub mod javascript_date_format_option {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &Option<DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if date.is_none() {
            return serializer.serialize_none();
        }

        let date = date.unwrap();
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;

        if s.is_none() {
            return Ok(None);
        }
        let s = s.unwrap();
        match Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom) {
            Ok(x) => Ok(Some(x)),
            Err(e) => Err(e)
        }
    }
}