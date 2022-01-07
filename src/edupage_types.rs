use chrono::{Utc, DateTime, NaiveDate};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::edupage_deserializers::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimelineItem {
    #[serde(rename = "user", with = "user_id")]
    pub user: UserID,

    #[serde(rename = "cas_pridania", with = "javascript_date_format")]
    pub time_added: DateTime<Utc>,

    #[serde(rename = "cas_pridania_btc", with = "javascript_date_format")]
    pub time_added_btc: DateTime<Utc>,

    #[serde(rename = "cas_udalosti", with = "javascript_date_format_option")]
    pub time_of_event: Option<DateTime<Utc>>,

    #[serde(rename = "data")]
    pub additional_data: String,

    #[serde(rename = "pocet_reakcii", deserialize_with = "deserialize_number_from_string")]
    pub reactions_n: i64,

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

    pub text: String,

    #[serde(rename = "user_meno")]
    pub user_name: String,
    
    #[serde(rename = "vlastnik")]
    pub owner: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DBI {
    #[serde(with = "dbi_item")]
    pub teachers: Vec<Teacher>,

    #[serde(with = "dbi_item")]
    pub classes: Vec<Class>,

    #[serde(with = "dbi_item")]
    pub subjects: Vec<DBIBase>,

    #[serde(with = "dbi_item")]
    pub classrooms: Vec<DBIBase>,

    #[serde(with = "dbi_item")]
    pub students: Vec<Student>,

    #[serde(with = "dbi_item")]
    pub parents: Vec<Parents>,

    #[serde(rename = "jeZUS")]
    pub is_art_school: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub items: Vec<TimelineItem>,
    pub dbi: DBI,

    #[serde(rename = "meninyDnes")]
    pub nameday_today: String,

    #[serde(rename = "meninyZajtra")]
    pub nameday_tomorrow: String,

    #[serde(rename = "userid", with = "user_id")]
    pub user_id: UserID
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
    pub date_to: Option<NaiveDate>
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
    pub classroom_id: Option<i64>
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

    #[serde(rename = "numberinclass", deserialize_with = "deserialize_number_from_string")]
    pub number_in_class: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parents {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    #[serde(rename = "firstname")]
    pub first_name: String,

    #[serde(rename = "lastname")]
    pub last_name: String,

    #[serde(with = "gender_option")]
    pub gender: Option<Gender>
}


// only the base properties a lot dbi entries have in common
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DBIBase {
    #[serde(with = "string_i64_option")]
    pub id: Option<i64>,

    pub name: String,
    pub short: String
}