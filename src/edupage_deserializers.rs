use chrono::{Local, NaiveDateTime, NaiveTime};
use num_enum::TryFromPrimitiveError;
use serde::{de::DeserializeOwned, ser, Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

use crate::edupage_types::{DBIBase, TimelineItemType, UserID};

pub const TIMELINE_ITEM_TYPE_NAMES: [&'static str; 19] = [
    "news",
    "sprava",
    "h_dailyplan",
    "student_absent",
    "confirmation",
    "h_clearplany",
    "h_financie",
    "h_stravamenu",
    "h_clearisicdata",
    "substitution",
    "h_clearcache",
    "event",
    "h_homework",
    "znamka",
    "h_substitution",
    "h_znamky",
    "homework",
    "h_cleardbi",
    "testpridelenie",
];

impl TryFrom<&str> for TimelineItemType {
    type Error = TryFromPrimitiveError<TimelineItemType>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for (i, timeline_item_type) in TIMELINE_ITEM_TYPE_NAMES.iter().enumerate() {
            if *timeline_item_type == value {
                return i.try_into();
            }
        }

        Ok(TimelineItemType::Unknown)
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
    use serde::{self, Deserialize, Deserializer, Serializer};

    use crate::edupage_types::TimelineItemType;

    use super::TIMELINE_ITEM_TYPE_NAMES;

    pub fn serialize<S>(item_type: &TimelineItemType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let integer_value = *item_type as usize;

        serializer.serialize_str(TIMELINE_ITEM_TYPE_NAMES[integer_value])
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TimelineItemType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = &String::deserialize(deserializer)?;

        let item_type: TimelineItemType = match s.try_into() {
            Ok(x) => x,
            Err(_) => {
                return Err(serde::de::Error::custom(
                    "Failed to deserialize TimelineItemType",
                ))
            }
        };

        Ok(item_type)
    }
}

pub mod gender {
    use serde::{self, Deserialize, Deserializer, Serializer};

    use crate::edupage_types::Gender;

    pub fn serialize<S>(gender: &Gender, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match gender {
            &Gender::Male => "M",
            &Gender::Female => "F",
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Gender, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = &String::deserialize(deserializer)?.to_lowercase();

        match s {
            "m" => Ok(Gender::Male),
            "f" => Ok(Gender::Female),
            _ => Err(serde::de::Error::custom(format!(
                "Failed to deserialize gender: {}",
                s
            ))),
        }
    }
}

pub mod gender_option {
    use std::borrow::Cow;

    use serde::{self, Deserialize, Deserializer, Serializer};

    use crate::edupage_types::Gender;

    pub fn serialize<S>(gender: &Option<Gender>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if gender.is_none() {
            return serializer.serialize_none();
        }
        let gender = gender.unwrap();
        serializer.serialize_str(match gender {
            Gender::Male => "M",
            Gender::Female => "F",
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Gender>, D::Error>
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
        }
        .as_ref()
        .to_lowercase();

        if string.is_empty() {
            return Ok(None);
        }

        if string.eq("f") {
            Ok(Some(Gender::Female))
        } else if string.eq("m") {
            Ok(Some(Gender::Male))
        } else {
            Err(serde::de::Error::custom(format!(
                "Failed to deserialize gender: {}",
                string
            )))
        }
    }
}

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
            return Err(serde::de::Error::custom(format!("Unexpected user type")));
        }

        return Ok(user_id.unwrap());
    }
}

pub mod string_i64_option {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(id: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if id.is_none() {
            serializer.serialize_none()
        } else {
            serializer.serialize_i64(id.unwrap())
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = match Deserialize::deserialize(deserializer) {
            Ok(x) => x,
            Err(_) => return Ok(None),
        };

        if s.is_none() {
            return Ok(None);
        }

        let s = &s.unwrap();
        if s.is_empty() {
            return Ok(None);
        }

        match s.parse() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Ok(None),
        }
    }
}
impl Serialize for DBIBase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let j = serde_json::to_string(self).map_err(ser::Error::custom)?;
        j.serialize(serializer)
    }
}

pub fn deserialize_dbi_base<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MapOrVec<T> {
        Map(Map<String, Value>),
        Vec(Vec<T>),
    }

    let ts: Map<String, Value> = match MapOrVec::<T>::deserialize(deserializer)? {
        MapOrVec::Map(m) => m,
        MapOrVec::Vec(_) => return Ok(Vec::new()),
    };

    let mut output: Vec<T> = Vec::new();
    for v in ts.values() {
        let t: T = serde_json::from_value(v.clone()).unwrap();
        output.push(t);
    }

    Ok(output)
}

pub mod year_month_day_optional {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = match Deserialize::deserialize(deserializer) {
            Ok(x) => x,
            Err(_) => return Ok(None),
        };

        if s.is_none() {
            return Ok(None);
        }
        let s = s.unwrap();

        if s.is_empty() {
            Ok(None)
        } else {
            match NaiveDate::parse_from_str(&s, FORMAT) {
                Ok(x) => Ok(Some(x)),
                Err(e) => Err(serde::de::Error::custom(e.to_string())),
            }
        }
    }
}

pub mod javascript_date_format_option {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;

        if s.is_none() {
            return Ok(None);
        }
        let s = s.unwrap();
        match DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom) {
            Ok(x) => Ok(Some(x.into())),
            Err(e) => Err(e),
        }
    }
}

pub fn deserialize_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = &String::deserialize(deserializer)?;

    let time: Vec<&str> = s.split(":").collect();

    let hours: u32 = match time[0].parse() {
        Ok(x) => x,
        Err(_) => return Err(serde::de::Error::custom("Failed to parse hours")),
    };
    let minutes: u32 = match time[1].parse() {
        Ok(x) => x,
        Err(_) => return Err(serde::de::Error::custom("Failed to parse minutes")),
    };

    Ok(NaiveDateTime::new(
        Local::now().date_naive(),
        NaiveTime::from_hms_opt(hours, minutes, 0).unwrap(),
    ))
}

pub mod string_i64_vec_option {
    use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(vec: &Option<Vec<i64>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if vec.is_none() {
            return serializer.serialize_none();
        }

        let vec = vec.clone().unwrap();

        let mut seq = serializer.serialize_seq(Some(vec.len()))?;
        for item in vec {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<i64>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<Vec<String>> = Deserialize::deserialize(deserializer)?;

        if let None = s {
            return Ok(None);
        }

        let seq = s.unwrap();
        if seq.len() == 0 {
            return Ok(Some(Vec::new()));
        }

        let mut result = Vec::with_capacity(seq.len());
        for item in seq {
            let parsed_item = match item.parse::<i64>() {
                Ok(v) => v,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };

            result.push(parsed_item);
        }

        Ok(Some(result))
    }
}

pub mod plan_item_type_option {
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::edupage_types::PlanItemType;

    pub fn serialize<S>(item: &Option<PlanItemType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if item.is_none() {
            return serializer.serialize_none();
        }

        let item = item.unwrap();

        match item {
            PlanItemType::Period => serializer.serialize_str("period"),
            PlanItemType::Lesson => serializer.serialize_str("lesson"),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PlanItemType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: &str = match Deserialize::deserialize(deserializer) {
            Ok(x) => x,
            Err(_) => return Ok(None),
        };

        Ok(match value {
            "period" => Some(PlanItemType::Period),
            "lesson" => Some(PlanItemType::Lesson),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Unknown plan item type {value}"
                )))
            }
        })
    }
}

pub mod hh_mm_naivedatetime_option {
    use chrono::{NaiveDateTime, NaiveTime, Timelike, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(item: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if item.is_none() {
            return serializer.serialize_none();
        }

        let item = item.unwrap();

        serializer.serialize_str(&format!("{}:{}", item.hour(), item.minute()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: &str = match Deserialize::deserialize(deserializer) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        let mut parts = value.split(":");

        let hour = match parts.next() {
            Some(h) => h.parse::<u32>().unwrap(),
            None => {
                return Err(serde::de::Error::custom(format!(
                    "Failed to deserialize hour from {value}"
                )))
            }
        };

        let minute = match parts.next() {
            Some(h) => h.parse::<u32>().unwrap(),
            None => {
                return Err(serde::de::Error::custom(format!(
                    "Failed to deserialize minute from {value}"
                )))
            }
        };

        let now = Utc::now().naive_local();
        let time = match NaiveTime::from_hms_opt(hour, minute, 0) {
            Some(x) => x,
            None => {
                return Err(serde::de::Error::custom(format!(
                    "Failed to create NaiveTime from {value}"
                )))
            }
        };

        Ok(Some(NaiveDateTime::new(now.date(), time)))
    }
}

pub fn none<T>() -> Option<T> {
    None
}
