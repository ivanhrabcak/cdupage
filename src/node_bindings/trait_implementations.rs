use std::any::Any;
use std::collections::HashMap;

use node_bindgen::core::{TryIntoJs, NjError};
use node_bindgen::core::val::JsObject;
use node_bindgen::sys::napi_value__;
use serde::Serialize;
use serde_json::Value;

use crate::edupage_types::TimelineItem;
use crate::{edupage::EdupageError, edupage_types::Teacher};
use crate::edupage_deserializers::get_string_representation;


fn value_to_js(x: &Value, js_env: &node_bindgen::core::val::JsEnv) -> Result<*mut napi_value__, NjError> {
    match x {
        Value::Null => Option::<String>::try_to_js(None, js_env),
        Value::Bool(x) => x.try_to_js(js_env),
        Value::Number(x) => x.as_i64().unwrap().try_to_js(js_env), // every number is i64
        Value::String(x) => x.clone().try_to_js(js_env),
        Value::Array(x) => {
            if x.len() == 0 {
                Vec::<String>::new().try_to_js(js_env)
            }
            else {
                match x.get(0).unwrap() {
                    Value::Array(_) => return Err(NjError::Other("Cannot serialize 2d array".to_string())),
                    _ => {
                        let mut v = Vec::new();
                        for val in x {
                            v.push(value_to_js(val, js_env));
                        }

                        v.try_to_js(js_env)
                    }
                }
            }
        },
        Value::Object(x) => value_to_js(&serde_json::Value::Object(x.clone()), js_env),
    }
}

fn get_js_object<T: Serialize>(x: T, js_env: &node_bindgen::core::val::JsEnv) -> Result<JsObject, ()> {
    let map: Value = match serde_json::to_value(x) {
        Ok(x) => x,
        Err(_) => return Err(())
    };

    if !map.is_object() {
        return Err(());
    }

    let mut json = match JsObject::create(js_env) {
        Ok(x) => x,
        Err(_) => return Err(())
    };

    let map = map.as_object().unwrap();
    for (key, v) in map {
        let value = match value_to_js(v, js_env) {
            Ok(x) => x,
            Err(_) => return Err(())
        };

        match json.set_property(key, value) {
            Ok(_) => (),
            Err(_) => return Err(())
        };
    }

    Ok(json)
}


impl TryIntoJs for EdupageError {
    fn try_to_js(self, js_env: &node_bindgen::core::val::JsEnv) -> Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> {
        js_env.create_error(match self {
            EdupageError::InvalidCredentials => "Invalid credentials.",
            EdupageError::HTTPError(_) => "HTTP error.",
            EdupageError::InvalidResponse => "Got invalid response from Edupage's servers.",
            EdupageError::ParseError(_) => "Failed to parse Edupage's response.",
            EdupageError::SerializationError(_) => "Serialization error.",
            EdupageError::NotLoggedIn => "You have to be logged in to use this method",
        })
    }
}

impl TryIntoJs for TimelineItem {
    fn try_to_js(self, js_env: &node_bindgen::core::val::JsEnv) -> Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> {
        let json = match get_js_object(self, js_env) {
            Ok(x) => x,
            Err(_) => return Err(NjError::Other("Failed to serialize!".to_string())),
        };

        json.try_to_js(js_env)
    }
}

impl TryIntoJs for Teacher {
    fn try_to_js(self, js_env: &node_bindgen::core::val::JsEnv) -> Result<node_bindgen::sys::napi_value, NjError> {
        let json = match get_js_object(self, js_env) {
            Ok(x) => x,
            Err(_) => return Err(NjError::Other("Failed to serialize!".to_string())),
        };

        json.try_to_js(js_env)
    }
}