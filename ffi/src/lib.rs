use std::ffi::c_char;
use cdupage::edupage::Edupage;
use ffi_sys::*;
#[repr(transparent)]
pub struct Cdupage {
	ep: Edupage
}
impl Cdupage {
	#[unsafe(no_mangle)]
	extern "C" fn new() -> Cdupage {
		Cdupage { 
			ep: Edupage::new()
		}
	}
	extern "C" fn request(&self, url: vector) {  }
}