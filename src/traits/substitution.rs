use std::collections::HashMap;

use crate::edupage::RequestType::POST;
use crate::edupage::{Edupage, EdupageError};
use chrono::NaiveDate;
use common_macros::hash_map;
use serde_json::Value;

pub trait Substitution {
    /// Checks for substitution
    fn get_substitution_html(
        &self,
        date: &NaiveDate,
        subdomain: &str,
    ) -> Result<String, EdupageError>;
}

impl Substitution for Edupage {
    fn get_substitution_html(
        &self,
        date: &NaiveDate,
        subdomain: &str,
    ) -> Result<String, EdupageError> {
        let url = format!(
            "https://{}.edupage.org/substitution/server/viewer.js?__func=getSubstViewerDayDataHtml",
            subdomain
        );

        let headers = hash_map! {
            "Content-Type".to_string() => "application/json".to_string()
        };

        // no point in trying to serialize, too complex structure
        let post_data = format!(
            "{{\"__args\":[null,{{\"date\":\"{}\",\"mode\":\"classes\"}}],\"__gsh\":\"{}\"}}",
            date.format("%Y-%m-%d"),
            self.gsec_hash.as_ref().unwrap()
        );

        let response = match self.request(url, POST, Some(headers), Some(post_data)) {
            Ok(x) => x,
            Err(e) => return Err(EdupageError::HTTPError(e)),
        };

        Ok(match response.json::<HashMap<String, Value>>() {
            Ok(x) => {
                let r = x.get("r");
                if r.is_none() {
                    return Err(EdupageError::SerializationError(
                        "Failed to traverse json".to_string(),
                    ));
                }

                r.unwrap().as_str().unwrap().to_string()
            }
            Err(e) => return Err(EdupageError::SerializationError(e.to_string())),
        })
    }
}
