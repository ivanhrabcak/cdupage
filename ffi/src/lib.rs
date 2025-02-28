use std::ffi::c_char;
use cdupage::edupage::{Edupage, RequestType};
use ffi_sys::*;
#[repr(C)]
pub struct Cdupage {
	ep: Edupage
}
#[repr(C)]
pub struct CequestType(RequestType);
impl Cdupage {
	#[unsafe(no_mangle)]
	extern "C" fn new() -> Cdupage {
		Cdupage {
			ep: Edupage::new()
		}
	}
	extern "C" fn request(&self, url: &[c_char], request_type: CequestType, ) {  }
}