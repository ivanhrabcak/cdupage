use chrono::{Utc, DateTime};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::edupage_deserializers::*;

#[derive(Serialize, Deserialize, Debug, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(usize)]
pub enum TimelineItemType {
    News = 0,
    Message = 1,
    HDailyPlan = 2,
    StudentAbset = 3,
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
    Unknown = 18
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum UserID {
    Teacher(i128),
    Student(i128),
    Parent(i128),
    Class(i128),
    Plan(i128),
    CustomPlan(i128),
    StudentClass(i128),
    StudentPlan(i128),
    OnlyStudent(i128),
    AllStudents,
    OnlyAllStudents,
    AllTeachers,
    Everyone
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TimelineItem {
    #[serde(rename = "user", with = "user_id")]
    pub user: UserID,

    #[serde(rename = "target_user", with = "user_id_option")]
    pub target_user: Option<UserID>,

    #[serde(rename = "typ", with = "timeline_item_type")]
    pub item_type: TimelineItemType,

    #[serde(rename = "timelineid", deserialize_with = "deserialize_number_from_string")]
    pub timeline_id: i64,

    #[serde(with = "javascript_date_format")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "reakcia_na", with = "string_i64_option")]
    pub reaction_to: Option<i64>,


}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub items: Vec<TimelineItem>,
}

