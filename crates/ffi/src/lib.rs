//! FFI binds to the [`cdupage`] crate.
//! Automatic header binds generation will be moved here
use abi_stable::std_types::{RHashMap, ROption, RStr, RString};
use cdupage::{
    edupage::{Edupage, RequestType},
    traits::Login,
};
use std::{collections::HashMap, convert::Into};
//use ffi_sys::*;
#[repr(C)]
pub enum CequestType {
    GET,
    POST,
}
#[derive(Clone)]
#[repr(C)]
struct CduDetails {
    sub: RString,
    user: RString,
    pass: RString,
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
#[repr(transparent)]
#[derive(Clone)]
pub struct Cdupage(CduDetails);
impl CduDetails {
    fn write_info(sub: &str, user: &str, pass: &str) -> CduDetails {
        Self {
            sub: sub.into(),
            user: user.into(),
            pass: pass.into(),
        }
    }
}
#[allow(clippy::unnecessary_operation)]
impl Cdupage {
    /// Logs in to EduPage
    #[unsafe(no_mangle)]
    pub extern "C" fn login(&mut self, sub: RStr, user: RStr, pass: RStr) -> Cdupage {
        Edupage::new()
            .login(sub.into(), user.into(), pass.into())
            .unwrap_or_default();
        Cdupage(CduDetails::write_info(sub.into(), user.into(), pass.into()))
    }
    /// Request data from given EduPage server.
    /// The data must be JSON encoded in a string literal
    #[unsafe(no_mangle)]
    pub extern "C" fn request(
        &self,
        url: RString,
        request_type: CequestType,
        headers: ROption<RHashMap<RString, RString>>,
        post_data: ROption<RString>,
    ) -> RString {
        let mut headers_map: Option<HashMap<String, String>> = None;
        if let ROption::RSome(hdrs) = headers {
            let mut m = HashMap::new();
            for pair in hdrs.into_iter() {
                m.insert(pair.0.into(), pair.1.into());
            }
            headers_map = Some(m);
        }

        let post_data_str: Option<String> = match post_data {
            ROption::RSome(s) => Some(s.into()),
            ROption::RNone => None,
        };
        let mut ep = Edupage::new();
        ep.login(
            &self.0.sub.clone(),
            &self.0.user.clone(),
            &self.0.pass.clone(),
        )
        .unwrap_or_default();
        let res = ep
            .request(url.into(), request_type.into(), headers_map, post_data_str)
            .unwrap()
            .text()
            .unwrap();

        res.into()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn logged_in(&self) -> bool {
        let mut ep = Edupage::new();
        ep.login(&self.0.sub, &self.0.user, &self.0.pass).unwrap();
        ep.logged_in()
    }
}
