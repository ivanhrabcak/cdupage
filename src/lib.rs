#![feature(assert_matches)]
use std::collections::HashMap;

#[cfg_attr(any(unix, windows), path = "reqwest.rs")]
pub mod implementation;

pub mod edupage;

pub trait Cache<T> {
    fn store(&mut self, key: &String, val: T);
    fn get(&self, key: &String) -> Option<&T>;
}

#[derive(Debug)]
pub struct Response {
    pub headers: HashMap<String, String>,
    pub cache_key: String,
    pub response_code: u16
}

pub trait HTTPClient {
    fn new() -> Self;
    fn get(&mut self, url: String, 
                      headers: HashMap<String, String>, 
                      cache: &mut dyn Cache<String>) -> Result<Response, String>;
    fn post(&mut self, url: String, 
                       headers: HashMap<String, String>, 
                       post_data: String, 
                       cache: &mut dyn Cache<String>) -> Result<Response, String>;
}

#[cfg(test)]
mod tests {
    fn get_env_var(name: &'static str) -> Option<String> {
        use std::env;

        match env::var(name) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }

    #[test]
    #[cfg(any(unix, windows))]
    fn get_request() {
        use std::{collections::HashMap, assert_matches::assert_matches};

        use crate::{implementation::{InMemoryCache, ReqwestHTTPClient}, HTTPClient, Cache};

        let mut in_memory_cache: InMemoryCache<String> = InMemoryCache { cache: HashMap::new() };
        let mut http_client: ReqwestHTTPClient = ReqwestHTTPClient::new();

        let result = http_client.get("https://www.google.com".to_string(), HashMap::new(), &mut in_memory_cache);
        assert_matches!(result, Ok(_));

        let result = result.unwrap();
        assert_matches!(in_memory_cache.get(&result.cache_key), Some(_));

        assert_eq!(result.response_code, 200);
    }

    #[test]
    #[cfg(any(unix, windows))]
    fn post_request() {
        use std::{collections::HashMap, assert_matches::{assert_matches}};

        use crate::{implementation::{InMemoryCache, ReqwestHTTPClient}, HTTPClient, Cache};

        let mut in_memory_cache: InMemoryCache<String> = InMemoryCache { cache: HashMap::new() };
        let mut http_client: ReqwestHTTPClient = ReqwestHTTPClient::new();

        let result = http_client.post("https://www.google.com".to_string(), HashMap::new(), "".to_string(), &mut in_memory_cache);
        assert_matches!(result, Ok(_));

        let result = result.unwrap();
        assert_matches!(in_memory_cache.get(&result.cache_key), Some(_));

        assert_ne!(result.response_code, 200);
    }

    #[test]
    #[cfg(any(unix, windows))]
    fn in_memory_cache_test() {
        use std::collections::HashMap;

        use crate::{implementation::InMemoryCache, Cache};

        let mut in_memory_cache: InMemoryCache<String> = InMemoryCache { cache: HashMap::new() };
        in_memory_cache.store(&"key".to_string(), "val".to_string());

        assert_eq!(in_memory_cache.get(&"key".to_string()), Some(&"val".to_string()))
    }

    #[test]
    #[cfg(any(unix, windows))]
    fn login_test() -> Result<(), crate::edupage::EdupageError> {
        dotenv::dotenv().ok();

        use std::collections::HashMap;

        use crate::{implementation::{InMemoryCache, ReqwestHTTPClient}, HTTPClient, edupage::Edupage};

        let in_memory_cache: InMemoryCache<String> = InMemoryCache { cache: HashMap::new() };
        let http_client: ReqwestHTTPClient = ReqwestHTTPClient::new();

        let subdomain = get_env_var("SUBDOMAIN");
        let username = get_env_var("USERNAME");
        let password = get_env_var("PASSWORD");

        if vec![&subdomain, &username, &password].contains(&&None) {
            debug_assert_ne!(subdomain, None);
            debug_assert_ne!(username, None);
            debug_assert_ne!(password, None);
        }

        let mut edupage = Edupage::new(in_memory_cache, http_client);
        
        let subdmain = subdomain.unwrap();
        let username = username.unwrap();
        let password = password.unwrap();

        edupage.login(&subdmain, &username, &password)
    }
}
