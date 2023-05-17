use crate::{Error, TypeDesc};
use std::mem::MaybeUninit;
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
    ptr,
};

pub unsafe trait AttributeType {
    const TYPEDESC: TypeDesc;
    unsafe fn get(get: impl Fn(*mut c_void) -> Result<(), Error>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut v = MaybeUninit::uninit();
        get(v.as_mut_ptr() as *mut c_void)?;
        Ok(v.assume_init())
    }

    unsafe fn set(&self, set: impl Fn(*const c_void) -> Result<(), Error>) -> Result<(), Error> {
        set(self as *const Self as *const c_void)
    }
}

unsafe impl AttributeType for i32 {
    const TYPEDESC: TypeDesc = TypeDesc::INT;
}

unsafe impl AttributeType for f32 {
    const TYPEDESC: TypeDesc = TypeDesc::FLOAT;
}

unsafe impl AttributeType for f64 {
    const TYPEDESC: TypeDesc = TypeDesc::DOUBLE;
}

unsafe impl<'a> AttributeType for &'a str {
    const TYPEDESC: TypeDesc = TypeDesc::STRING;

    /// WARNING: lifetime of returned &str is unbounded
    unsafe fn get(get: impl Fn(*mut c_void) -> Result<(), Error>) -> Result<Self, Error> {
        let mut v: *const c_char = ptr::null();
        get(&mut v as *mut *const c_char as *mut c_void)?;
        Ok(CStr::from_ptr(v).to_str().expect("invalid UTF-8"))
    }

    unsafe fn set(&self, set: impl Fn(*const c_void) -> Result<(), Error>) -> Result<(), Error> {
        let cstring = CString::new(*self).unwrap();
        let ptr = cstring.as_ptr();
        set(&ptr as *const *const c_char as *const c_void)
    }
}
