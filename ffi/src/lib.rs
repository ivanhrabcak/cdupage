use std::ffi::c_char;
use cdupage::edupage::{Edupage, RequestType};
use ffi_sys::*;
#[repr(C)]
pub enum CequestType {
	GET,
	POST
}
impl Into<RequestType> for CequestType {
	fn into(self) -> RequestType {
		match self {
			CequestType::GET => RequestType::GET,
			CequestType::POST => RequestType::POST
		}
	}
}
extern "C" fn request(url: &[c_char], request_type: CequestType, headers: &[(String, String)], post_data: Option<&[c_char]>) {
	self.ep.request(url, request_type, headers, Some(post_data.unwrap()));
}