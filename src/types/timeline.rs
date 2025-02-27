use chrono::NaiveDateTime;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::deserializers::*;
use crate::macro_aliases::*;

use super::person::UserID;

#[cfg(feature = "node-types")]
use ts_rs::TS;

#[derive(
    Serde!, Serialize, IntoPrimitive, TryFromPrimitive, PartialEq, Copy,
)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
#[repr(usize)]
pub enum TimelineItemType {
    News = 0,
    Message = 1,
    HDailyPlan = 2,
    StudentAbsent = 3,
    Confirmation = 4,
    HClearPlans = 5,
    HFinances = 6,
    HLunchMenu = 7,
    HClearISICData = 8,
    Substitution = 9,
    HClearCache = 10,
    Event = 11,
    HHomework = 12,
    Grade = 13,
    HSubstitution = 14,
    HGrades = 15,
    Homework = 16,
    HClearDBI = 17,
    Unknown = 18,
    TestAssignment = 19,
}

#[repr(C)]
#[derive(Serde!, Serialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct TimelineItem {
    #[cfg_attr(not(feature = "node-types"), serde(rename = "user"))]
    pub user: UserID,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "cas_pridania", with = "javascript_date_format_option")
    )]
    #[cfg_attr(feature = "node-types", ts(as = "Option<NaiveDateTime>"))]
    pub time_added: Option<NaiveDateTime>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "cas_pridania_btc", with = "javascript_date_format_option")
    )]
    pub time_added_btc: Option<NaiveDateTime>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "cas_udalosti", with = "javascript_date_format_option")
    )]
    pub time_of_event: Option<NaiveDateTime>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "data"))]
    pub additional_data: String,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(
            rename = "pocet_reakcii",
            deserialize_with = "deserialize_number_from_string"
        )
    )]
    pub reactions_n: i64,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "target_user"))]
    pub target_user: Option<UserID>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "typ", with = "timeline_item_type")
    )]
    pub item_type: TimelineItemType,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(
            rename = "timelineid",
            deserialize_with = "deserialize_number_from_string"
        )
    )]
    pub timeline_id: i64,

    #[serde(with = "javascript_date_format_option")]
    pub timestamp: Option<NaiveDateTime>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "reakcia_na", with = "string_i64_option")
    )]
    pub reaction_to: Option<i64>,

    pub text: String,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "user_meno"))]
    pub user_name: String,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "vlastnik"))]
    pub owner: String,
}
