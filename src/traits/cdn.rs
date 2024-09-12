use crate::edupage::{Edupage, EdupageError};
use reqwest::blocking::Request;
use serde_json::{json, Value};
use std::{collections::HashMap, io::Read};
pub(crate) trait ECloudFile {
    /// Upload file to EduPage.
    fn upload(self, body: &Value, domain: &str) -> Result<(), EdupageError>;
}
pub trait CDN {
    /// Upload file to EduPage cloud.
    ///
    /// The file will be hosted forever (and for free) on EduPage's servers. The file is tied to
    /// your user account, but anybody with a link can view it.
    ///
    /// **Warning!** EduPage limits file size to 50 MB and the file can have only some extensions.
    /// You can find all supported file extensions on this
    /// [Edupage help site](https://help.edupage.org/?p=u1/u113/u132/u362/u467).
    ///
    /// If you are willing to upload some files, you will probably have to increase the request
    /// timeout.
    /// # Example:
    /// ```rust
    /// let file = File::open("image.jpg").unwrap();
    /// let result = cloud.upload_file(file);
    /// ```
    fn upload_file(&self) -> Result<(), EdupageError>;
}

impl ECloudFile for Edupage {
    fn upload(self, body: &serde_json::Value, domain: &str) -> Result<(), EdupageError> {
        let _ = Edupage::new()
            .client
            .post(domain)
            .body(json!(body).to_string())
            .build();
        Ok(())
    }
}

impl CDN for Edupage {
    fn upload_file(&self) -> Result<(), EdupageError> {
        let request_url = format!(
            "https://{}.edupage.org/timeline/?akcia=uploadAtt",
            self.subdomain.as_ref().unwrap()
        );
        let mut s = String::new();
        self.request(
            request_url.clone(),
            crate::edupage::RequestType::GET,
            Some(HashMap::from([(
                "Content-Type".to_string(),
                "application/binary".to_string(),
            )])),
            None,
        )
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
        match serde_json::from_str::<Value>(&s) {
            Ok(response_json) => {
                if response_json.get("status") != Some(&Value::String("ok".to_string())) {
                    panic!("Edupage didn't return positive value")
                }

                let metadata = response_json.get("data");
                Edupage::new().upload(metadata.unwrap(), request_url.as_str())
            }
            Err(_) => panic!("Failed to decode json response"),
        }
    }
}
