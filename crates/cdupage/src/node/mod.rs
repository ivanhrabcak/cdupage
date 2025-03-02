use chrono::{TimeZone, Utc};
use node_bindgen::{
    core::{NjError, TryIntoJs, val::JsObject},
    derive::node_bindgen,
};
use num_enum::TryFromPrimitive;
use serde::Serialize;
use serde_json::Value;
// use tslink::tslink;
use crate::{
    edupage::{Edupage as IEdupage, EdupageError},
    traits::{DBI, Login, Ringing, Timeline, Timetable},
    types::{
        DBIBase, RingingTime, Student, Teacher, TimelineItem, TimelineItemType,
        Timetable as EduTimetable,
    },
};

struct SerdeIntoJs<T: Serialize>(T);

impl<T: Serialize> SerdeIntoJs<T> {
    fn value_to_js(
        js_env: &node_bindgen::core::val::JsEnv,
        value: Value,
    ) -> Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> {
        match value {
            Value::Null => js_env.get_null(),
            Value::Bool(v) => js_env.create_boolean(v),
            Value::Number(i) => {
                if i.is_f64() {
                    js_env.create_double(i.as_f64().unwrap())
                } else if i.is_i64() {
                    js_env.create_int64(i.as_i64().unwrap())
                } else {
                    js_env.create_bigint_uint64(i.as_u64().unwrap())
                }
            }
            Value::String(s) => js_env.create_string_utf8(&s),
            Value::Array(v) => {
                let values = v.iter()
                    .map(|k| Self::value_to_js(js_env, k.clone()))
                    .collect::<Vec<Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError>>>();

                let result = js_env.create_array_with_len(values.len())?;
                for (i, val) in values.into_iter().enumerate() {
                    js_env.set_element(result, val?, i)?;
                }

                Ok(result)
            }
            Value::Object(o) => {
                let mut object = JsObject::new(*js_env, js_env.create_object()?);

                for (k, v) in o.into_iter() {
                    object.set_property(&k, Self::value_to_js(js_env, v)?)?
                }

                Ok(object.napi_value())
            }
        }
    }
}

impl<T: Serialize> TryIntoJs for SerdeIntoJs<T> {
    fn try_to_js(
        self,
        js_env: &node_bindgen::core::val::JsEnv,
    ) -> Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> {
        let serialized_instance: Value =
            serde_json::to_value(self.0).map_err(|e| NjError::Other(e.to_string()))?;

        Self::value_to_js(js_env, serialized_instance)
    }
}

impl TryIntoJs for IEdupage {
    fn try_to_js(
        self,
        js_env: &node_bindgen::core::val::JsEnv,
    ) -> Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> {
        let mut instance = JsObject::new(*js_env, js_env.create_object()?);

        instance.set_property("isLoggedIn", js_env.create_boolean(self.is_logged_in)?)?;

        if let Some(data) = self.data {
            instance.set_property("data", SerdeIntoJs(data).try_to_js(js_env)?)?;
        } else {
            instance.set_property("data", js_env.get_null()?)?;
        }

        if let Some(gsec_hash) = self.gsec_hash {
            instance.set_property("gsecHash", js_env.create_string_utf8(&gsec_hash)?)?;
        } else {
            instance.set_property("gsecHash", js_env.get_null()?)?;
        }

        if let Some(subdomain) = self.subdomain {
            instance.set_property("subdomain", js_env.create_string_utf8(&subdomain)?)?;
        } else {
            instance.set_property("subdomain", js_env.get_null()?)?;
        }

        instance.try_to_js(js_env)
    }
}

#[node_bindgen]
struct Edupage(IEdupage);

#[node_bindgen]
impl Edupage {
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self(IEdupage::new())
    }

    #[node_bindgen]
    pub fn login(
        &mut self,
        subdomain: String,
        username: String,
        password: String,
    ) -> Result<(), SerdeIntoJs<EdupageError>> {
        self.0
            .login(&subdomain, &username, &password)
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_teachers(&self) -> Result<Vec<SerdeIntoJs<Teacher>>, SerdeIntoJs<EdupageError>> {
        self.0
            .get_teachers()
            .map(|r| r.into_iter().map(|k| SerdeIntoJs(k)).collect())
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_teacher_by_id(
        &self,
        id: i64,
    ) -> Result<Option<SerdeIntoJs<Teacher>>, SerdeIntoJs<EdupageError>> {
        self.0
            .get_teacher_by_id(id)
            .map(|r| match r {
                Some(v) => Some(SerdeIntoJs(v)),
                None => None,
            })
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_students(&self) -> Result<Vec<SerdeIntoJs<Student>>, SerdeIntoJs<EdupageError>> {
        self.0
            .get_students()
            .map(|r| r.into_iter().map(|k| SerdeIntoJs(k)).collect())
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_student_by_id(
        &self,
        id: i64,
    ) -> Result<Option<SerdeIntoJs<Student>>, SerdeIntoJs<EdupageError>> {
        self.0
            .get_student_by_id(id)
            .map(|r| match r {
                Some(v) => Some(SerdeIntoJs(v)),
                None => None,
            })
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_subjects(&self) -> Result<Vec<SerdeIntoJs<DBIBase>>, SerdeIntoJs<EdupageError>> {
        self.0
            .get_subjects()
            .map(|r| r.into_iter().map(|k| SerdeIntoJs(k)).collect())
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_subject_by_id(
        &self,
        id: i64,
    ) -> Result<Option<SerdeIntoJs<DBIBase>>, SerdeIntoJs<EdupageError>> {
        self.0
            .get_subject_by_id(id)
            .map(|r| match r {
                Some(v) => Some(SerdeIntoJs(v)),
                None => None,
            })
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_classrooms(&self) -> Result<Vec<SerdeIntoJs<DBIBase>>, SerdeIntoJs<EdupageError>> {
        self.0
            .get_classrooms()
            .map(|r| r.into_iter().map(|k| SerdeIntoJs(k)).collect())
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_classroom_by_id(
        &self,
        id: i64,
    ) -> Result<Option<SerdeIntoJs<DBIBase>>, SerdeIntoJs<EdupageError>> {
        self.0
            .get_classroom_by_id(id)
            .map(|r| match r {
                Some(v) => Some(SerdeIntoJs(v)),
                None => None,
            })
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn filter_timeline_by_item_type(
        &self,
        item_type: i32,
    ) -> Result<Vec<SerdeIntoJs<TimelineItem>>, SerdeIntoJs<EdupageError>> {
        let item_type = match TimelineItemType::try_from_primitive(item_type as usize) {
            Ok(v) => v,
            Err(_) => {
                return Err(SerdeIntoJs(EdupageError::ParseError(
                    "Invalid item type!".to_string(),
                )));
            }
        };

        self.0
            .filter_timeline_by_item_type(item_type)
            .map(|r| r.into_iter().map(|k| SerdeIntoJs(k)).collect())
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn filter_by_item_types(
        &self,
        item_types: Vec<i32>,
    ) -> Result<Vec<SerdeIntoJs<TimelineItem>>, SerdeIntoJs<EdupageError>> {
        let mut typed_item_types = Vec::with_capacity(item_types.len());

        for item_type in item_types {
            typed_item_types.push(
                match TimelineItemType::try_from_primitive(item_type as usize) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(SerdeIntoJs(EdupageError::ParseError(
                            "Invalid item type!".to_string(),
                        )));
                    }
                },
            )
        }

        self.0
            .filter_timeline_by_item_types(typed_item_types)
            .map(|r| r.into_iter().map(|k| SerdeIntoJs(k)).collect())
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_timetable(
        &self,
        date_timestamp: i64,
    ) -> Result<SerdeIntoJs<EduTimetable>, SerdeIntoJs<EdupageError>> {
        let datetime = Utc.timestamp_opt(date_timestamp, 0).unwrap().naive_local();

        self.0
            .get_timetable(datetime.date())
            .map(|r| SerdeIntoJs(r))
            .map_err(|e| SerdeIntoJs(e))
    }

    #[node_bindgen]
    pub fn get_ringing_times(&self) -> Vec<SerdeIntoJs<RingingTime>> {
        self.0
            .get_ringing_times()
            .into_iter()
            .map(|k| SerdeIntoJs(k))
            .collect()
    }
}
