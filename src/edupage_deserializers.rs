use num_enum::TryFromPrimitiveError;

use crate::edupage_types::TimelineItemType;

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

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

// pub mod user_id {
//     use serde::{self, Deserialize, Serializer, Deserializer};

//     use crate::edupage_types::UserID;

//     pub fn serialize<S>(
//         item_type: &UserID,
//         serializer: S,
//     ) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {

//         let string_representation = match item_type {
//             UserID::Teacher(id) => format!("Ucitel{}", id),
//             UserID::Student(id) => format!("Student{}", id),
//             UserID::Parent(id) => format!("Rodic{}", id),
//             UserID::Class(id) => format!("Trieda{}", id),
//             UserID::Plan(id) => format!("Plan{}", id),
//             UserID::CustomPlan(id) => format!("CustPlan{}", id),
//             UserID::StudentClass(id) => format!("StudTrieda{}", id),
//             UserID::OnlyStudent(id) => format!("StudentOnly{}", id),
//             UserID::StudentPlan(id) => format!("StudPlan{}", id),
//             UserID::AllStudents => "Student*".to_string(),
//             UserID::AllTeachers => "Ucitel*".to_string(),
//             UserID::Everyone => "*".to_string(),
//         };

//         serializer.serialize_str(&string_representation)
//     }

//     pub fn deserialize<'de, D>(
//         deserializer: D,
//     ) -> Result<UserID, D::Error>
//     where
//         D: Deserializer<'de> 
//     {
//         let s: &str = &String::deserialize(deserializer)?;

//         // we first try to parse the simple ones
//         let user_type: Option<UserID> = match s {
//             "*" => Some(UserID::Everyone),
//             "Student*" => Some(UserID::AllStudents),
//             "Ucitel*" => Some(UserID::AllTeachers),
//             _ => None
//         };

//         if let Some(user_type) = user_type {
//             return Ok(user_type)
//         }

//         // let (user_type, i128) = scan!(user_type)

//     }
// }

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