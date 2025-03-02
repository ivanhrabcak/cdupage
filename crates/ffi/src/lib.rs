//! FFI binds to the [`cdupage`] crate.
//! Automatic header binds generation will be moved here
use std::collections;
use abi_stable::StableAbi;
use abi_stable::reexports::SelfOps;
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
#[repr(C)]
#[derive(Clone)]
pub struct CdupageLogin(Edupage);

impl CdupageLogin {
    /// Logs in to EduPage
    /// Will store the values
    extern "C" fn login(&mut self, sub: RStr, user: RStr, pass: RStr) {
        self.0.login(sub.into(), user.into(), pass.into()).unwrap()
    }
    extern "C" fn request(
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
}
