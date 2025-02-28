use std::{fs::File, io::Write};

use crate::types::UserData;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[repr(C)]
#[derive(Clone)]
pub struct Edupage {
    pub(crate) is_logged_in: bool,
    pub(crate) client: reqwest::blocking::Client,
    pub(crate) data: Option<UserData>,
    pub(crate) gsec_hash: Option<String>,
    pub subdomain: Option<String>,
}
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdupageError {
    InvalidCredentials,
    HTTPError(String),
    InvalidResponse,
    ParseError(String),
    SerializationError(String),
    NotLoggedIn,
    MissingData,
    Other(String),
}
#[repr(C)]
pub enum RequestType {
    GET,
    POST,
}

impl Default for Edupage {
    fn default() -> Self {
        Self::new()
    }
}

impl Edupage {
    pub(crate) fn build_client() -> reqwest::blocking::Client {
        Client::builder()
            .connection_verbose(true)
            .cookie_store(true)
            .build()
            .unwrap()
    }

    pub fn new() -> Self {
        let client = Self::build_client();

        Self {
            is_logged_in: false,
            data: None,
            client,
            gsec_hash: None,
            subdomain: None,
        }
    }
    /// This method can be used for making authenticated requests to edupage.
    ///
    /// Example usage (sending a message manually):
    /// ```no_run
    /// use std::collections::HashMap;
    /// use cdupage::{edupage::{RequestType, Edupage}, traits::Login};
    ///
    /// let mut edupage = Edupage::new();
    ///
    /// edupage.login("subdomain", "username", "password").unwrap();
    ///
    /// let url = format!("https://{}.edupage.org/timeline/?akcia=createItem", edupage.subdomain.clone().unwrap());
    /// let request_data = "{\"selectedUser\": \"Student12345\", \"text\": \"Hello World!\", \"attachements\": {}, \"receipt\": 0, \"typ\": \"sprava\"}";
    /// let response = edupage.request(
    ///     url,
    ///     RequestType::POST,
    ///     Some(HashMap::from([("Content-Type".to_string(), "application/json".to_string())])),
    ///     Some(request_data.to_string())
    /// ).unwrap();
    ///
    /// println!("{:?}", response);
    ///
    /// ```
    pub  fn request(
        &self,
        url: String,
        request_type: RequestType,
        headers: Option<HashMap<String, String>>,
        post_data: Option<String>,
    ) -> Result<Response, String> {
        let header_map = if let Some(hd) = headers {
            let mut header_map: HeaderMap = HeaderMap::new();

            for (k, v) in hd {
                header_map.insert(
                    HeaderName::from_bytes(k.as_bytes()).unwrap(),
                    HeaderValue::from_str(&v).unwrap(),
                );
            }

            header_map
        } else {
            HeaderMap::new()
        };

        let response = match request_type {
            RequestType::GET => self.client.get(url).headers(header_map),
            RequestType::POST => {
                let mut request_builder = self.client.post(url).headers(header_map);

                if let Some(pd) = post_data {
                    request_builder = request_builder.body(pd);
                }

                request_builder
            }
        }
        .send();

        match response {
            Ok(x) => Ok(x),
            Err(e) => Err(e.to_string()),
        }
    }

    pub(crate) fn parse_login_data(&mut self, html: String) -> Result<(), String> {
        let json = match html.split("userhome(").nth(1) {
            Some(x) => x,
            None => return Err("Bad data!".to_string()),
        }
        .rsplitn(3, ");")
        .nth(2)
        .unwrap()
        .replace("\t", "")
        .replace("\n", "")
        .replace("\r", "");

        File::create("dump.json")
            .unwrap()
            .write_all(json.as_bytes())
            .unwrap();

        self.data = Some(match serde_json::from_str(&json) {
            Ok(x) => x,
            Err(e) => {
                return Err(e.to_string());
            }
        });

        self.gsec_hash = match html.split("ASC.gsechash=\"").nth(1) {
            None => return Err("No gsechash in response!".to_string()),
            Some(v) => Some(v.split("\"").nth(0).unwrap().to_string()),
        };

        Ok(())
    }

    pub fn logged_in(&self) -> bool {
        self.is_logged_in
    }
}
