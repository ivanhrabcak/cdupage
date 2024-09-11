use serde::{de::DeserializeOwned, ser, Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

use crate::types::dbi::DBIBase;

pub mod gender {
    use serde::{self, Deserialize, Deserializer, Serializer};

    use crate::types::dbi::Gender;

    pub fn serialize<S>(gender: &Gender, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *gender {
            Gender::Male => "M",
            Gender::Female => "F",
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

    use crate::types::dbi::Gender;

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
    #[allow(dead_code)]
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
