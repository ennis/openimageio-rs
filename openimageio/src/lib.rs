extern crate openimageio_sys;

use std::ffi::{CStr,CString};
use std::path::Path;
use std::ptr;
use std::error;
use std::fmt;
use std::os::raw::*;

pub use openimageio_sys::{TypeDesc, AggregateKind, VecSemantics};
use openimageio_sys::ImageOutput_OpenMode;
pub type OpenMode = ImageOutput_OpenMode;

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

// Options:
// - safe wrapper over raw ImageSpec, all properties are functions
//      - function call overhead
//      - cloning is expensive (C heap allocation)
// - struct converted on-the-fly at creation time
//      - additional space rust-side
//      - easier to handle
//      - each query must allocate an imagespec on the heap before calling ffi
// - go with opaque
// - imagespec builder?
pub struct ImageSpec
{
    ptr: *mut openimageio_sys::ImageSpec
}

impl ImageSpec {
    pub fn new_2d(typedesc: &TypeDesc, xres: u32, yres: u32, num_channels: u32) -> ImageSpec {
        let ptr = unsafe {
            openimageio_sys::COIIO_ImageSpec_new_2D(xres as c_int,yres as c_int, num_channels as c_int, typedesc as *const TypeDesc)
        };

        ImageSpec {
            ptr
        }
    }
}

impl Drop for ImageSpec {
    fn drop(&mut self) {
        unsafe {
            openimageio_sys::COIIO_ImageSpec_delete(self.ptr);
        }
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
        if imginput.is_null() {
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

pub struct ImageOutput {
    ptr: *mut openimageio_sys::ImageOutput
}

pub struct SubimageOutput<'a> {
    parent: &'a mut ImageOutput
}

impl ImageOutput {

    // Open or create an imageoutput
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ImageOutput, Error> {
       unimplemented!()
    }

    // Open a subimage
    // May fail if spec is not supported
    // Mut-borrows the subimage so that you can't open more than one at a time
    pub fn open_subimage<'a>(&'a mut self, spec: &ImageSpec) -> Result<SubimageOutput<'a>,Error> {
        unimplemented!()
    }

    // Open subimages
    // For formats that do not support appending subimages
    // Should return an iterator or something, or pass a closure, maybe
    pub fn open_subimages<'a>(&'a mut self, specs: &[ImageSpec]) -> ! {
        unimplemented!()
    }
}

impl<'a> SubimageOutput<'a> {
    pub fn write_image(&self) -> Result<(),Error> {
        unimplemented!()
    }

    // TODO write scanline

    // finish writing to this subimage (and release the borrow)
    pub fn close(self) {
        unimplemented!()
    }
}

// use-case
// open file with specs, write one image, close
// open file with specs, write multiple images, close
// open file, append image, close
// An imageoutput must be ready to write
//
// Open existing image for appending or modification
//   open(Existing), append_subimage(spec) -> SubimageWriter, modify_subimage(spec)
// OR create(path), open(spec) -> SubimageWriter,
// Create new (empty) image, append images
//   create(path), open(spec, Create)

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
