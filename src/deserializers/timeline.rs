use num_enum::TryFromPrimitiveError;

use crate::types::timeline::TimelineItemType;

extern "C" const TIMELINE_ITEM_TYPE_NAMES: [&str; 19] = [
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

    use crate::types::timeline::TimelineItemType;

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
