use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::deserializers::*;
use crate::macro_aliases::*;

use super::dbi::DBIBase;
use super::person::Teacher;

#[cfg(feature = "node-types")]
use ts_rs::TS;

#[repr(C)]
#[derive(Serde!, Serialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct Timetable {
    pub lessons: Vec<Lesson>,
}

#[repr(C)]
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

#[repr(C)]
#[derive(Serde!, Serialize)]
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

#[repr(C)]
#[derive(Serde!, Serialize)]
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

#[repr(C)]
#[derive(Serialize, Serde!)]
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

#[repr(C)]
#[derive(Serde!, Serialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct PlanItemHeaderPart {
    pub item: Option<PlanItemHeaderItem>,
}

#[repr(C)]
#[derive(Serde!, Serialize)]
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

#[derive(Serde!, Serialize)]
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
