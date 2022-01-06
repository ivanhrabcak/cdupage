use std::{collections::HashMap};

use crate::{Cache, HTTPClient, Response};

pub struct InMemoryCache<T> {
    pub(crate) cache: HashMap<String, T>
}

impl<T> Cache<T> for InMemoryCache<T> {
    fn store(&mut self, key: &String, val: T) {
        self.cache.insert(key.clone(), val);
    }

    fn get(&self, key: &String) -> Option<&T> {
        self.cache.get(key)
    }
}

pub struct ReqwestHTTPClient {
    client: reqwest::blocking::Client
}



impl HTTPClient for ReqwestHTTPClient {
    fn new() -> Self {
        Self { client: reqwest::blocking::Client::new() }
    }

    fn get(&mut self, url: String, 
                      headers: HashMap<String, String>, 
                      cache: &mut dyn Cache<String>) -> Result<Response, String> {
        let mut request = (&self.client).get(url.clone());

        for (k, v) in headers {
            request = request.header(k, v);
        }

        match request.send() {
            Ok(r) => {
                let mut headers: HashMap<String, String> = HashMap::new();
                for (k, v) in r.headers() {
                    let v = match v.to_str() {
                        Ok(v) => v,
                        Err(e) => return Err(e.to_string())
                    };
                    headers.insert(k.to_string(), v.to_string());
                }
                let cache_key = url.clone();

                let response_code = r.status().as_u16();
                let response_text = r.text();
                if let Ok(text) = response_text {
                    cache.store(&cache_key, text);
                    
                    Ok(Response { cache_key, headers, response_code})
                }
                else {
                    Err("Failed to get response text".to_string())
                }
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    fn post(&mut self, url: String, 
                       headers: HashMap<String, String>, 
                       post_data: String, 
                       cache: &mut dyn Cache<String>) -> Result<Response, String> {
        let mut request = (&self.client).post(url.clone()).body(post_data);

        for (k, v) in headers {
            request = request.header(k, v);
        }
        
        match request.send() {
            Ok(r) => {
                let mut headers: HashMap<String, String> = HashMap::new();
                for (k, v) in r.headers() {
                    let v = match v.to_str() {
                        Ok(v) => v,
                        Err(e) => return Err(e.to_string())
                    };
                    headers.insert(k.to_string(), v.to_string());
                }
                let cache_key = url.clone();

                let response_code = r.status().as_u16();
                let response_text = r.text();
                if let Ok(text) = response_text {
                    cache.store(&cache_key, text);

                    Ok(Response { cache_key, headers, response_code })
                }
                else {
                    Err("Failed to get response text".to_string())
                }
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }
}