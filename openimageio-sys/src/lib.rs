//#![allow(clippy::all)]
#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    missing_copy_implementations,
    non_upper_case_globals
)]

use std::{os::raw::c_char, path::Path, str::Utf8Error};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//--------------------------------------------------------------------------------------------------
pub trait AsStringRef {
    fn as_stringref(&self) -> OIIO_StringRef;
}

impl AsStringRef for Path {
    fn as_stringref(&self) -> OIIO_StringRef {
        let path_str = self
            .as_os_str()
            .to_str()
            .expect("non-utf8 characters in path");
        OIIO_StringRef {
            len: path_str.len(),
            ptr: path_str.as_ptr() as *const c_char,
        }
    }
}

impl AsStringRef for str {
    fn as_stringref(&self) -> OIIO_StringRef {
        OIIO_StringRef {
            len: self.len(),
            ptr: self.as_ptr() as *const c_char,
        }
    }
}

impl OIIO_StringRef {
    // warning: unbounded lifetime, extra unsafe
    pub unsafe fn try_into_str<'a>(self) -> Result<&'a str, Utf8Error> {
        let slice = std::slice::from_raw_parts(self.ptr as *const u8, self.len);
        std::str::from_utf8(slice)
    }
}
