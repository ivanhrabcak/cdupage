use crate::edupage::EdupageError;
use serde::{Deserialize, Serialize};

use common_macros::hash_map;

use crate::{
    edupage::{Edupage, RequestType},
};



pub trait Login {
    fn login(
        &mut self,
        subdomain: &str,
        username: &str,
        password: &str,
    ) -> Result<(), EdupageError>;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
    pub csrfauth: String,
}

impl LoginCredentials {
    pub fn new(username: String, password: String, csrfauth: String) -> Self {
        Self {
            username,
            password,
            csrfauth,
        }
    }
}

impl Login for Edupage {
    fn login(
        &mut self,
        subdomain: &str,
        username: &str,
        password: &str,
    ) -> Result<(), EdupageError> {
        let url = format!("https://{}.edupage.org/login/index.php", subdomain);

        let result = self.request(url, RequestType::GET, None, None);

        if let Err(error) = result {
            return Err(EdupageError::HTTPError(error.to_string()));
        }

        let result = result.unwrap();
        let response_text = match result.text() {
            Ok(x) => x,
            Err(e) => return Err(EdupageError::HTTPError(e.to_string())),
        };

        if !response_text.contains("csrfauth") {
            return Err(EdupageError::InvalidResponse);
        }

        let csrf_token = match response_text.split("name=\"csrfauth\" value=\"").nth(1) {
            Some(x) => x,
            None => {
                return Err(EdupageError::ParseError(
                    "Failed to parse csrf token.".to_string(),
                ))
            }
        }
        .split("\"")
        .nth(0)
        .unwrap();

        let login_credentials = LoginCredentials::new(
            username.to_string(),
            password.to_string(),
            csrf_token.to_string(),
        );

        let post_data = match serde_urlencoded::to_string(&login_credentials) {
            Ok(x) => x,
            Err(e) => return Err(EdupageError::SerializationError(e.to_string())),
        };

        let url = format!("https://{}.edupage.org/login/edubarLogin.php", subdomain);

        let result = self.request(
            url,
            RequestType::POST,
            Some(hash_map! {
                "Content-Type".to_string() => "application/x-www-form-urlencoded".to_string()
            }),
            Some(post_data),
        );

        if let Err(error) = result {
            return Err(EdupageError::HTTPError(error.to_string()));
        }

        let result = result.unwrap();

        if result.url().as_str().contains("bad=1") {
            return Err(EdupageError::InvalidCredentials);
        }

        let response_text = match result.text() {
            Ok(x) => x,
            Err(e) => return Err(EdupageError::HTTPError(e.to_string())),
        };

        match self.parse_login_data(response_text.to_string()) {
            Ok(_) => {
                self.is_logged_in = true;
                self.subdomain = Some(subdomain.to_string());
                Ok(())
            }
            Err(e) => Err(EdupageError::ParseError(e.to_string())),
        }
    }
}
