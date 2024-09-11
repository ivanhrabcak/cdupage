use chrono::{Local, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Deserializer};

pub mod year_month_day_optional {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

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
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;

        if s.is_none() {
            return Ok(None);
        }
        let s = s.unwrap();
        match NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom) {
            Ok(x) => Ok(Some(x)),
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
