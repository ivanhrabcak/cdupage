//! FFI binds to the [`cdupage`] crate.
//! Automatic header binds generation will be moved here
use abi_stable::std_types::{RHashMap, ROption, RStr, RString};
use cdupage::{
    edupage::{Edupage, RequestType},
    traits::Login,
};
use std::convert::Into;
//use ffi_sys::*;

#[repr(C)]
pub enum CequestType {
    GET,
    POST,
}
#[allow(clippy::from_over_into)]
impl Into<RequestType> for CequestType {
    fn into(self) -> RequestType {
        match self {
            CequestType::GET => RequestType::GET,
            CequestType::POST => RequestType::POST,
        }
    }
}
struct CduLogin {
    sub: String,
    username: String,
    password: String,
}
#[repr(C)]
#[derive(Clone)]
pub struct Cdupage(Edupage);
#[allow(clippy::unnecessary_operation)]
impl Cdupage {
    /// Logs in to EduPage
    pub extern "C" fn login(&mut self, sub: RStr, user: RStr, pass: RStr) {
        self.0.login(sub.into(), user.into(), pass.into()).unwrap();
        CduLogin { sub: sub.into(), username: user.into(), password: pass.into(), };
    }
    /// Request data from given EduPage server
    pub extern "C" fn request(
        &self,
        url: RString,
        request_type: CequestType,
        headers: ROption<RHashMap<String, String>>,
        post_data: ROption<RString>,
    ) {
        self.0
            .request(
                url.into(),
                request_type.into(),
                Some(headers.unwrap().into()),
                Some(post_data.unwrap().into()),
            )
            .unwrap();
    }
    pub fn logged_in(&self) -> bool {
        self.0.logged_in()
    }
}
