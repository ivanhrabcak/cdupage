use std::collections::HashMap;

#[cfg_attr(any(unix, windows), path = "reqwest.rs")]
pub mod implementation;

trait Cache<T> {
    fn store(&mut self, key: &'static str, val: T);
    fn get(&self, key: &'static str) -> Option<&T>;
}

struct Response {
    headers: HashMap<String, String>,
    cache_key: &'static str
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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
