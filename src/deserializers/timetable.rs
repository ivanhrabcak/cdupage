pub mod plan_item_type_option {
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::types::timetable::PlanItemType;

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
