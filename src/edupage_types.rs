use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::edupage_deserializers::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(
    Serialize, Deserialize, Debug, IntoPrimitive, TryFromPrimitive, PartialEq, Clone, Copy,
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
pub struct TimelineItem {
    #[serde(rename = "user")]
    pub user: UserID,

    #[serde(rename = "cas_pridania", with = "javascript_date_format_option")]
    pub time_added: Option<DateTime<Utc>>,

    #[serde(rename = "cas_pridania_btc", with = "javascript_date_format_option")]
    pub time_added_btc: Option<DateTime<Utc>>,

    #[serde(rename = "cas_udalosti", with = "javascript_date_format_option")]
    pub time_of_event: Option<DateTime<Utc>>,

    #[serde(rename = "data")]
    pub additional_data: String,

    #[serde(
        rename = "pocet_reakcii",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub reactions_n: i64,

    #[serde(rename = "target_user")]
    pub target_user: Option<UserID>,

    #[serde(rename = "typ", with = "timeline_item_type")]
    pub item_type: TimelineItemType,

    #[serde(
        rename = "timelineid",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub timeline_id: i64,

    #[serde(with = "javascript_date_format_option")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(rename = "reakcia_na", with = "string_i64_option")]
    pub reaction_to: Option<i64>,

    pub text: String,

    #[serde(rename = "user_meno")]
    pub user_name: String,

    #[serde(rename = "vlastnik")]
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    #[serde(rename = "jeZUS")]
    pub is_art_school: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub items: Vec<TimelineItem>,
    pub dbi: DBI,

    #[serde(rename = "meninyDnes")]
    pub nameday_today: String,

    #[serde(rename = "meninyZajtra")]
    pub nameday_tomorrow: String,

    #[serde(rename = "userid")]
    pub user_id: UserID,

    #[serde(rename = "zvonenia")]
    pub ringing_times: Vec<RingingTime>,

    pub dp: DP
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Teacher {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    #[serde(rename = "firstname")]
    pub first_name: String,

    #[serde(rename = "lastname")]
    pub last_name: String,

    pub short: String,

    #[serde(with = "gender_option")]
    pub gender: Option<Gender>,

    #[serde(rename = "classroomid", with = "string_i64_option")]
    pub classroom_id: Option<i64>,

    #[serde(rename = "isOut")]
    pub is_out: bool,

    #[serde(rename = "datefrom", with = "year_month_day_optional")]
    pub date_from: Option<NaiveDate>,

    #[serde(rename = "dateto", with = "year_month_day_optional")]
    pub date_to: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    pub name: String,
    pub short: String,

    #[serde(with = "string_i64_option")]
    pub grade: Option<i64>,

    #[serde(rename = "teacherid", with = "string_i64_option")]
    pub first_teacher_id: Option<i64>,

    #[serde(rename = "teacher2id", with = "string_i64_option")]
    pub second_teacher_id: Option<i64>,

    #[serde(rename = "classroomid", with = "string_i64_option")]
    pub classroom_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    #[serde(rename = "classid", with = "string_i64_option")]
    pub class_id: Option<i64>,

    #[serde(rename = "firstname")]
    pub first_name: String,

    #[serde(rename = "lastname")]
    pub last_name: String,

    #[serde(rename = "parent1id", with = "string_i64_option")]
    pub first_parent_id: Option<i64>,

    #[serde(rename = "parent2id", with = "string_i64_option")]
    pub second_parent_id: Option<i64>,

    #[serde(rename = "parent3id", with = "string_i64_option")]
    pub third_parent_id: Option<i64>, // what the fuck

    #[serde(with = "gender_option")]
    pub gender: Option<Gender>,

    #[serde(rename = "datefrom", with = "year_month_day_optional")]
    pub date_from: Option<NaiveDate>,

    #[serde(rename = "dateto", with = "year_month_day_optional")]
    pub date_to: Option<NaiveDate>,

    #[serde(rename = "numberinclass", with = "string_i64_option")]
    pub number_in_class: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parent {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    #[serde(rename = "firstname")]
    pub first_name: String,

    #[serde(rename = "lastname")]
    pub last_name: String,

    #[serde(with = "gender_option")]
    pub gender: Option<Gender>,
}

// only the base properties a lot dbi entries have in common
#[derive(Deserialize, Debug, Clone)]
pub struct DBIBase {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    pub name: String,
    pub short: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lesson {
    pub teachers: Vec<Teacher>,
    pub classrooms: Vec<DBIBase>,
    pub start_of_lesson: NaiveDateTime,
    pub end_of_lesson: NaiveDateTime,
    pub online_lesson_link: Option<String>,
    pub subject_id: i64,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DP {
    pub dates: HashMap<String, Plan>,

    #[serde(rename = "year")]
    pub school_year: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plan {
    #[serde(rename = "tt_day")]
    pub day: i32,

    #[serde(rename = "plan")]
    pub plan_items: Vec<PlanItem>,

    #[serde(rename = "tt_week")]
    pub week: i32
}

#[derive(Debug, Clone, Copy)]
pub enum PlanItemType {
    Period, Lesson
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanItemHeaderPart {
    pub item: PlanItemHeaderItem
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanItemHeaderItem {
    #[serde(rename = "subjectid", with = "string_i64_option")]
    pub subject_id: Option<i64>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanItem {
    #[serde(rename = "classids", with = "string_i64_vec_option", default = "none")]
    pub class_ids: Option<Vec<i64>>,

    #[serde(with = "year_month_day_optional", default = "none")]
    pub date: Option<NaiveDate>,

    #[serde(with = "plan_item_type_option", default = "none")]
    pub item_type: Option<PlanItemType>,

    pub header: Vec<PlanItemHeaderPart>,

    #[serde(rename = "subjectid", with = "string_i64_option", default = "none")]
    pub subject_id: Option<i64>,
    
    #[serde(rename = "teacherids", with = "string_i64_vec_option", default = "none")]
    pub teacher_ids: Option<Vec<i64>>,

    #[serde(rename = "classroomids", with = "string_i64_vec_option", default = "none")]
    pub classroom_ids: Option<Vec<i64>>,

    #[serde(rename = "starttime", with = "hh_mm_naivedatetime_option", default = "none")]
    pub start_time: Option<NaiveDateTime>,

    #[serde(rename = "endtime", with = "hh_mm_naivedatetime_option", default = "none")]
    pub end_time: Option<NaiveDateTime>,

    #[serde(rename = "ol_url")]
    pub online_link: Option<String>
    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timetable {
    pub lessons: Vec<Lesson>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RingingTime {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub name: i64,

    #[serde(rename = "starttime", deserialize_with = "deserialize_time")]
    pub start_time: NaiveDateTime,

    #[serde(rename = "endtime", deserialize_with = "deserialize_time")]
    pub end_time: NaiveDateTime,
}