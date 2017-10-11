extern crate openimageio_sys;

use std::ffi::{CStr,CString};
use std::path::Path;
use std::ptr;
use std::error;
use std::fmt;
use std::os::raw::*;

pub use openimageio_sys::{TypeDesc, AggregateKind, VecSemantics};

#[derive(Clone,Debug)]
pub enum Error {
    OpenError(String),
}

impl error::Error for Error
{
    fn description(&self) -> &str {
        match *self {
            Error::OpenError(_) => { "error opening image" },
            _ => { "unknown error" },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match *self {
            Error::OpenError(ref msg) => { write!(f, "Error opening image: {}", msg) },
            _ => { write!(f, "Unknown error.") },
        }
    }
}

fn get_last_error() -> String {
    const ERROR_BUFSIZE: usize = 1024;
    let mut buf = [0 as u8; ERROR_BUFSIZE];
    unsafe {
        openimageio_sys::COIIO_geterror(buf.as_mut_ptr() as *mut c_char, ERROR_BUFSIZE as c_int);
        // assume utf8 input
        String::from_utf8_lossy(&buf).into_owned()
    }
}

pub struct ImageInput
{
    ptr: *mut openimageio_sys::ImageInput
}

impl ImageInput {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ImageInput,Error> {
        let path_cstr = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let imginput = unsafe {
            openimageio_sys::COIIO_ImageInput_open(path_cstr.as_ptr(), ptr::null())
        };
        if imginput == ptr::null_mut() {
            Err(Error::OpenError(get_last_error()))
        } else {
            Ok(ImageInput {
                ptr: imginput
            })
        }
    }
}

impl Drop for ImageInput {
    fn drop(&mut self) {
        unsafe {
            openimageio_sys::COIIO_ImageInput_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn open_image() {
        let img = super::ImageInput::open("../test_images/tonberry.jpg");
        assert!(img.is_ok());
    }

    #[test]
    fn open_image_exr() {
        let img = super::ImageInput::open("../test_images/output0013.exr");
        assert!(img.is_ok());
    }

    #[test]
    fn open_image_psd() {
        let img = super::ImageInput::open("../test_images/cup.psd");
        assert!(img.is_ok());
    }

    #[test]
    fn open_image_tif() {
        let img = super::ImageInput::open("../test_images/cup.tif");
        assert!(img.is_ok());
    }

    #[test]
    fn open_nonexistent_image() {
        let img = super::ImageInput::open("../test_images/nonexistent.png");
        if let Err(ref e) = img {
            println!("{}", e);
        }
        assert!(img.is_err());
    }

}
