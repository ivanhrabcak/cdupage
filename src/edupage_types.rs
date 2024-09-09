use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::edupage_deserializers::*;

#[cfg(feature = "node-types")]
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub enum Gender {
    Male,
    Female,
}

#[derive(
    Serialize, Deserialize, Debug, IntoPrimitive, TryFromPrimitive, PartialEq, Clone, Copy,
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

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub enum UserID {
    Teacher(i64),
    Student(i64),
    Parent(i64),
    Class(i64),
    Plan(i64),
    CustomPlan(i64),
    StudentClass(i64),
    StudentPlan(i64),
    OnlyStudent(i64),
    AllStudents,
    OnlyAllStudents,
    AllTeachers,
    Everyone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[cfg_attr(feature = "node-types", ts(as = "Option<DateTime<Utc>>"))]
    pub time_added: Option<DateTime<Utc>>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "cas_pridania_btc", with = "javascript_date_format_option")
    )]
    pub time_added_btc: Option<DateTime<Utc>>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "cas_udalosti", with = "javascript_date_format_option")
    )]
    pub time_of_event: Option<DateTime<Utc>>,

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
    pub timestamp: Option<DateTime<Utc>>,

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct DBI {
    #[serde(deserialize_with = "deserialize_dbi_base")]
    pub teachers: Vec<Teacher>,

    #[serde(deserialize_with = "deserialize_dbi_base")]
    pub classes: Vec<Class>,

    #[serde(deserialize_with = "deserialize_dbi_base")]
    pub subjects: Vec<DBIBase>,

    #[serde(deserialize_with = "deserialize_dbi_base")]
    pub classrooms: Vec<DBIBase>,

    #[serde(deserialize_with = "deserialize_dbi_base")]
    pub students: Vec<Student>,

    #[serde(deserialize_with = "deserialize_dbi_base")]
    pub parents: Vec<Parent>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "jeZUS"))]
    pub is_art_school: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct Teacher {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "firstname"))]
    pub first_name: String,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "lastname"))]
    pub last_name: String,

    pub short: String,

    #[serde(with = "gender_option")]
    pub gender: Option<Gender>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "classroomid", with = "string_i64_option")
    )]
    pub classroom_id: Option<i64>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "isOut"))]
    pub is_out: bool,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "datefrom", with = "year_month_day_optional")
    )]
    pub date_from: Option<NaiveDate>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "dateto", with = "year_month_day_optional")
    )]
    pub date_to: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct Class {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    pub name: String,
    pub short: String,

    #[serde(with = "string_i64_option")]
    pub grade: Option<i64>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "teacherid", with = "string_i64_option")
    )]
    pub first_teacher_id: Option<i64>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "teacher2id", with = "string_i64_option")
    )]
    pub second_teacher_id: Option<i64>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "classroomid", with = "string_i64_option")
    )]
    pub classroom_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct Student {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "classid", with = "string_i64_option")
    )]
    pub class_id: Option<i64>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "firstname"))]
    pub first_name: String,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "lastname"))]
    pub last_name: String,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "parent1id", with = "string_i64_option")
    )]
    pub first_parent_id: Option<i64>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "parent2id", with = "string_i64_option")
    )]
    pub second_parent_id: Option<i64>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "parent3id", with = "string_i64_option")
    )]
    pub third_parent_id: Option<i64>, // what the fuck

    #[serde(with = "gender_option")]
    pub gender: Option<Gender>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "datefrom", with = "year_month_day_optional")
    )]
    pub date_from: Option<NaiveDate>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "dateto", with = "year_month_day_optional")
    )]
    pub date_to: Option<NaiveDate>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "numberinclass", with = "string_i64_option")
    )]
    pub number_in_class: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct Parent {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "firstname"))]
    pub first_name: String,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "lastname"))]
    pub last_name: String,

    #[serde(with = "gender_option")]
    pub gender: Option<Gender>,
}

// only the base properties a lot dbi entries have in common
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct DBIBase {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    pub name: String,
    pub short: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct Lesson {
    pub teachers: Vec<Teacher>,
    pub classrooms: Vec<DBIBase>,
    pub start_of_lesson: NaiveDateTime,
    pub end_of_lesson: NaiveDateTime,
    pub online_lesson_link: Option<String>,
    pub subject_id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct DP {
    pub dates: HashMap<String, Plan>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "year"))]
    pub school_year: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct Plan {
    #[cfg_attr(not(feature = "node-types"), serde(rename = "tt_day"))]
    pub day: i32,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "plan"))]
    pub plan_items: Vec<PlanItem>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "tt_week"))]
    pub week: i32,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub enum PlanItemType {
    Period,
    Lesson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct PlanItemHeaderPart {
    pub item: PlanItemHeaderItem,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct PlanItemHeaderItem {
    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "subjectid", with = "string_i64_option")
    )]
    pub subject_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct PlanItem {
    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "classids", with = "string_i64_vec_option", default = "none")
    )]
    pub class_ids: Option<Vec<i64>>,

    #[serde(with = "year_month_day_optional", default = "none")]
    pub date: Option<NaiveDate>,

    #[serde(with = "plan_item_type_option", default = "none")]
    pub item_type: Option<PlanItemType>,

    pub header: Vec<PlanItemHeaderPart>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(rename = "subjectid", with = "string_i64_option", default = "none")
    )]
    pub subject_id: Option<i64>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(
            rename = "teacherids",
            with = "string_i64_vec_option",
            default = "none"
        )
    )]
    pub teacher_ids: Option<Vec<i64>>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(
            rename = "classroomids",
            with = "string_i64_vec_option",
            default = "none"
        )
    )]
    pub classroom_ids: Option<Vec<i64>>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(
            rename = "starttime",
            with = "hh_mm_naivedatetime_option",
            default = "none"
        )
    )]
    pub start_time: Option<NaiveDateTime>,

    #[cfg_attr(
        not(feature = "node-types"),
        serde(
            rename = "endtime",
            with = "hh_mm_naivedatetime_option",
            default = "none"
        )
    )]
    pub end_time: Option<NaiveDateTime>,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "ol_url"))]
    pub online_link: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct Timetable {
    pub lessons: Vec<Lesson>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
