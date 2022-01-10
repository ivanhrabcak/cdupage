use node_bindgen::core::TryIntoJs;

use crate::{edupage::EdupageError, edupage_types::Teacher};
use crate::edupage_deserializers::get_string_representation;

use super::edupage_types::UserID;



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