use std::path::PathBuf;

use crate::{
    edupage::{Edupage, EdupageError},
    types::{CloudFile, EdupageCloudResponse},
};
use reqwest::blocking::multipart::Form;

pub trait Cloud {
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
    extern "C" fn upload(&self, file: PathBuf) -> Result<CloudFile, EdupageError>;
}

impl Cloud for Edupage {
    #[unsafe(no_mangle)]
    extern "C" fn upload(&self, path: PathBuf) -> Result<CloudFile, EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        // the user is logged in, so subdomain cannot be `None`
        let url = format!(
            "https://{}.edupage.org/timeline/?akcia=uploadAtt",
            self.subdomain.clone().unwrap()
        );

        let path_str = path
            .to_str()
            .ok_or(EdupageError::Other("Invalid path!".to_string()))?;

        let form = Form::new()
            .file("att", path_str)
            .map_err(|err| EdupageError::Other(err.to_string()))?;

        // we have to send a multipart file so we cannot use the built-in request method
        let response = self
            .client
            .post(url)
            .multipart(form)
            .send()
            .map_err(|err| EdupageError::HTTPError(err.to_string()))?;

        let cloud_response = response
            .json::<EdupageCloudResponse>()
            .map_err(|err| EdupageError::SerializationError(err.to_string()))?;

        match cloud_response.status {
            crate::types::EdupageCloudResponseStatus::Ok => {
                // we can safely unwrap as the response is Ok
                Ok(cloud_response.response.unwrap())
            }
            crate::types::EdupageCloudResponseStatus::Other(status_type) => Err(
                EdupageError::Other(format!("Edupage returned a {status_type} status")),
            ),
        }
    }
}

impl CloudFile {
    #[unsafe(no_mangle)]
    pub extern "C" fn get_url(&self, edupage: Edupage) -> Result<String, EdupageError> {
        if !edupage.logged_in() {
            Err(EdupageError::NotLoggedIn)
        } else {
            Ok(format!(
                "https://{}.edupage.org{}",
                edupage.subdomain.unwrap(),
                self.file
            ))
        }
    }
}
