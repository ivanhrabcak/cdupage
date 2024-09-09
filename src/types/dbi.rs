
use serde::{Deserialize, Serialize};

use crate::deserializers::*;
use crate::macro_aliases::*;

use super::person::Class;
use super::person::Parent;
use super::person::Student;
use super::person::Teacher;

#[cfg(feature = "node-types")]
use ts_rs::TS;

#[derive(Copy, Serde!, Serialize)]
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

#[derive(Serde!, Serialize)]
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

// only the base properties a lot dbi entries have in common
#[derive(Deserialize, Clone, Debug)]
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
