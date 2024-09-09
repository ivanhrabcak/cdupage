use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

pub use dbi::DBI;
pub use person::UserID;
pub use timeline::TimelineItem;
pub use timetable::DP;

use crate::edupage_deserializers::*;
use crate::macro_aliases::*;

pub mod dbi;
pub mod person;
pub mod timeline;
pub mod timetable;

#[cfg(feature = "node-types")]
use ts_rs::TS;

#[derive(Serde!, Serialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct UserData {
    pub items: Vec<TimelineItem>,
    pub dbi: DBI,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "meninyDnes"))]
    pub nameday_today: String,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "meninyZajtra"))]
    pub nameday_tomorrow: String,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "userid"))]
    pub user_id: UserID,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "zvonenia"))]
    pub ringing_times: Vec<RingingTime>,

    pub dp: DP,
}

#[derive(Serde!, Serialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct RingingTime {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub name: i64,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "starttime", deserialize_with = "deserialize_time")
    )]
    pub start_time: NaiveDateTime,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "endtime", deserialize_with = "deserialize_time")
    )]
    pub end_time: NaiveDateTime,
}
