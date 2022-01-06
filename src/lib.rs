#![feature(assert_matches)]
use std::collections::HashMap;

#[cfg_attr(any(unix, windows), path = "reqwest.rs")]
pub mod implementation;

trait Cache<T> {
    fn store(&mut self, key: &'static str, val: T);
    fn get(&self, key: &'static str) -> Option<&T>;
}

#[derive(Debug)]
struct Response {
    headers: HashMap<String, String>,
    cache_key: &'static str,
    response_code: u16
}

trait HTTPClient {
    fn new() -> Self;
    fn get(&mut self, url: &'static str, 
                      headers: HashMap<String, String>, 
                      cache: &mut dyn Cache<String>) -> Result<Response, String>;
    fn post(&mut self, url: &'static str, 
                       headers: HashMap<String, String>, 
                       post_data: String, 
                       cache: &mut dyn Cache<String>) -> Result<Response, String>;
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(any(unix, windows))]
    fn get_request() {
        use std::{collections::HashMap, assert_matches::assert_matches};

        use crate::{implementation::{InMemoryCache, ReqwestHTTPClient}, HTTPClient, Cache};

        let mut in_memory_cache: InMemoryCache<String> = InMemoryCache { cache: HashMap::new() };
        let mut http_client: ReqwestHTTPClient = ReqwestHTTPClient::new();

        let result = http_client.get("https://www.google.com", HashMap::new(), &mut in_memory_cache);
        assert_matches!(result, Ok(_));

        let result = result.unwrap();
        assert_matches!(in_memory_cache.get(result.cache_key), Some(_));

        assert_eq!(result.response_code, 200);
    }

    #[test]
    #[cfg(any(unix, windows))]
    fn post_request() {
        use std::{collections::HashMap, assert_matches::{assert_matches}};

        use crate::{implementation::{InMemoryCache, ReqwestHTTPClient}, HTTPClient, Cache};

        let mut in_memory_cache: InMemoryCache<String> = InMemoryCache { cache: HashMap::new() };
        let mut http_client: ReqwestHTTPClient = ReqwestHTTPClient::new();

        let result = http_client.post("https://www.google.com", HashMap::new(), "".to_string(), &mut in_memory_cache);
        assert_matches!(result, Ok(_));

        let result = result.unwrap();
        assert_matches!(in_memory_cache.get(result.cache_key), Some(_));

        assert_ne!(result.response_code, 200);
    }

    #[test]
    #[cfg(any(unix, windows))]
    fn in_memory_cache_test() {
        use std::collections::HashMap;

        use crate::{implementation::InMemoryCache, Cache};

        let mut in_memory_cache: InMemoryCache<String> = InMemoryCache { cache: HashMap::new() };
        in_memory_cache.store("key", "val".to_string());

        assert_eq!(in_memory_cache.get("key"), Some(&"val".to_string()))
    }
}
