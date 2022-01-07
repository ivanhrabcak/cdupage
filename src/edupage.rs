use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;

use crate::edupage_types::UserData;

#[derive(Clone)]
pub struct Edupage {
    pub(crate) is_logged_in: bool,
    pub(crate) client: reqwest::blocking::Client,
    pub(crate) data: Option<UserData>
}

#[derive(Debug)]
pub enum EdupageError {
    InvalidCredentials,
    HTTPError(String),
    InvalidResponse,
    ParseError(String),
    SerializationError(String),
    NotLoggedIn
}

impl Edupage {
    pub fn new() -> Self {
        let client = Client::builder()
            .connection_verbose(true)
            .cookie_store(true).build().unwrap();

        Self { is_logged_in: false, data: None, client }
    }

    pub(crate) fn parse_login_data(&mut self, html: String) -> Result<(), String> {
        let json = html.split("$j(document).ready(function() {").nth(1).unwrap()
            .split(");").nth(0).unwrap()
            .replace("\t", "")
            .split("userhome(").nth(1).unwrap()
            .replace("\n", "")
            .replace("\r", "");

        self.data = Some(match serde_json::from_str(&json) {
            Ok(x) => x,
            Err(e) => {
                return Err(e.to_string());
            }
        });

        Ok(())
    }

    pub fn logged_in(&self) -> bool {
        self.is_logged_in
    }
}