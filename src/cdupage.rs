//! FFI-safe wrappers around the [`Edupage`] struct.
//! Do note that you must enable forward declarations.
//! ```
//! Cdupage cp;
//! cp.new();
//! cp.login();
//! cp.request(...);
//! ```
#![cfg(feature = "c_any_other_lang")]

use crate::{
    edupage::{Edupage, RequestType},
    traits::Login,
};
use std::ffi::{CString, c_char};
/// The struct that has it all
#[repr(C)]
#[derive(Default)]
pub struct Cdupage {
    cdupage: Edupage,
}

#[repr(C)]
pub enum CequestType {
    GET,
    POST,
}
impl Into<RequestType> for CequestType {
    fn into(self) -> RequestType {
        match self {
            CequestType::GET => RequestType::GET,
            CequestType::POST => RequestType::POST,
        }
    }
}
impl Cdupage {
    /// Initializes Cdupage
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn new() -> Self {
        Self::default()
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ep(&self) -> *const Edupage {
        &self.cdupage
    }
    /// Logs in to EduPage
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn login(
        &mut self,
        sub: *const c_char,
        user: *const c_char,
        pass: *const c_char,
    ) {
        let sub_str = unsafe { CString::from_raw(sub as *mut c_char) };
        let user_str = unsafe { CString::from_raw(user as *mut c_char) };
        let pass_str = unsafe { CString::from_raw(pass as *mut c_char) };
        let sub_str = sub_str.to_str().map_err(|_| ());
        let user_str = user_str.to_str().map_err(|_| ());
        let pass_str = pass_str.to_str().map_err(|_| ());
        if !self
            .cdupage
            .login(sub_str.unwrap(), user_str.unwrap(), pass_str.unwrap())
            .is_ok()
        {
            panic!("Failed to log in to EduPage")
        }
    }
    /// Request data from [`Edupage`]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn request(
        &mut self,
        url: *const c_char,
        request_type: CequestType,
        headers_ptr: *const c_char,
        post_data_ptr: *const c_char,
    ) -> *const c_char {
        let url = unsafe { CString::from_raw(url as *mut c_char) };
        let url = url.to_str().unwrap();

        let headers_str = unsafe { CString::from_raw(headers_ptr as *mut c_char) };
        let headers_str = headers_str.to_str().unwrap();

        let mut headers = std::collections::HashMap::new();
        for header in headers_str.split('\n') {
            if let Some((key, value)) = header.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let post_data = unsafe { CString::from_raw(post_data_ptr as *mut c_char) };
        let post_data = post_data.to_str().map(|s| s.to_string()).ok();

        let resp = self
            .cdupage
            .request(
                url.to_string(),
                request_type.into(),
                Some(headers),
                post_data,
            )
            .unwrap_or(panic!("Failed to log in to EduPage"));

        let c_string = CString::new(resp.text().unwrap());
        c_string
            .unwrap_or_else(|e| panic!("Nothing was returned {e}"))
            .into_raw()
    }
}
