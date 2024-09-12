use crate::edupage::{Edupage, EdupageError};
use serde_json::{json, Value};
pub trait ECloudFile {
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
    /// If you are willing to upload some files, you will probably have to increase the request timeout.
    fn download_file(&self) -> Result<Value, EdupageError>;
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