use std::collections::HashMap;

use crate::{Cache, HTTPClient};

#[derive(Clone)]
pub struct Edupage<T, C> where T: HTTPClient, C: Cache<String> {
    cache: C,
    client: T,
    is_logged_in: bool,
    session_id: Option<String>
}

#[derive(Debug)]
pub enum EdupageError {
    InvalidCredentials(()),
    HTTPError(String),
    InvalidResponse(()),
    ParseError(String)
}

impl<T, C> Edupage<T, C> where T: HTTPClient, C: Cache<String> {
    pub fn new(cache: C, client: T) -> Self {
        Self { cache, client, is_logged_in: false, session_id: None }
    }

    pub fn login(&mut self, subdomain: &String, username: &String, password: &String) -> Result<(), EdupageError> {
        let url = format!("https://{}.edupage.org/login/index.php", subdomain);

        let result = self.client.get(url, HashMap::new(), &mut self.cache);
        if result.is_err() {
            return Err(EdupageError::HTTPError(result.unwrap_err().to_string()));
        }

        let result = result.unwrap();
        let response_text = self.cache.get(&result.cache_key);
        
        if response_text.is_none() {
            return Err(EdupageError::InvalidResponse(()));
        }

        let response_text = response_text.unwrap();

        if !response_text.contains("csrfauth") {
            return Err(EdupageError::InvalidResponse(()));
        }

        let csrf_token = match response_text.split("name=\"csrfauth\" value=\"").nth(1) {
            Some(x) => x,
            None => return Err(EdupageError::ParseError("Failed to parse csrf token.".to_string()))
        }.split("\"").nth(0).unwrap();

        let post_data = format!("
        {{
            \"username\": \"{}\",
            \"password\": \"{}\",
            \"csrfauth\": \"{}\"
        }}
        ", username, password, csrf_token);

        let url = format!("https://{}.edupage.org/login/edubarLogin.php", subdomain);
        let result = self.client.post(url, HashMap::new(), post_data, &mut self.cache);

        if result.is_err() {
            return Err(EdupageError::HTTPError(result.unwrap_err().to_string()));
        }

        let result = result.unwrap();

        let response_text = self.cache.get(&result.cache_key);
        if response_text.is_none() {
            return Err(EdupageError::InvalidResponse(()))
        }

        let response_text = response_text.unwrap();
        println!("{}", response_text);
        Ok(())
    }
}