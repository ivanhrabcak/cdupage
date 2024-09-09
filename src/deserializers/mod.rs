use chrono::{Local, NaiveDateTime, NaiveTime};
use num_enum::TryFromPrimitiveError;
use serde::{de::DeserializeOwned, ser, Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

use crate::types::{dbi::DBIBase, timeline::TimelineItemType, UserID};

pub mod timeline;
pub mod dbi;
pub mod person;
pub mod timetable;
pub mod date;

pub use timeline::*;
pub use dbi::*;
pub use person::*;
pub use timetable::*;
pub use date::*;

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

        if s.is_none() {
            return Ok(None);
        }

        let seq = s.unwrap();
        if seq.is_empty() {
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

pub fn none<T>() -> Option<T> {
    None
}
