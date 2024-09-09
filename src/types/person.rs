
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::deserializers::*;
use crate::macro_aliases::*;

use super::dbi::Gender;

#[cfg(feature = "node-types")]
use ts_rs::TS;

#[derive(Serde!, Serialize)]
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

#[derive(Serde!, Serialize)]
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

#[derive(Serde!, Serialize)]
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

#[derive(Serde!, Serialize)]
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

#[derive(Copy, Clone, Debug)]
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
