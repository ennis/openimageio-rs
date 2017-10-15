extern crate openimageio_sys;
extern crate libc;

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
    WriteError(String),
}

impl error::Error for Error
{
    fn description(&self) -> &str {
        match *self {
            Error::OpenError(_) => { "error opening image" },
            Error::WriteError(_) => { "error writing image data" },
            _ => { "unknown error" },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match *self {
            Error::OpenError(ref msg) => { write!(f, "Error opening image: {}", msg) },
            Error::WriteError(ref msg) => { write!(f, "Error writing image data: {}", msg) },
            _ => { write!(f, "Unknown error.") },
        }
    }
}

unsafe fn cstring_to_owned(cstr: *const c_char) -> String {
    // assume utf8 input
    let msg = CStr::from_ptr(cstr).to_str().unwrap().to_owned();
    openimageio_sys::COIIO_delete_cstring(cstr);
    msg
}

fn get_last_error() -> String {
    unsafe {
        cstring_to_owned(openimageio_sys::COIIO_geterror())
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
            Err(Error::OpenError( get_last_error()))
        } else {
            Ok(ImageInput {
                ptr: imginput
            })
        }
    }

    fn get_last_error(&self) -> String {
        unsafe {
            cstring_to_owned(openimageio_sys::COIIO_ImageInput_geterror(self.ptr))
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
    ptr: *mut openimageio_sys::ImageOutput,
    path: CString
}

pub struct SubimageOutput<'a> {
    parent: &'a mut ImageOutput
}

impl ImageOutput {
    fn get_last_error(&self) -> String {
        unsafe {
            cstring_to_owned(openimageio_sys::COIIO_ImageOutput_geterror(self.ptr))
        }
    }

    // Open or create an imageoutput
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ImageOutput, Error> {
        let path =  CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let plugin_search_path = CString::new("").unwrap();
        let ptr = unsafe {
            openimageio_sys::COIIO_ImageOutput_create(path.as_ptr(), plugin_search_path.as_ptr())
        };
        if ptr.is_null() {
            Err(Error::OpenError(get_last_error()))
        } else {
            Ok(ImageOutput {
                ptr,
                path
            })
        }
    }

    // Open a subimage
    // May fail if spec is not supported
    // Mut-borrows the subimage so that you can't open more than one at a time
    pub fn open_subimage<'a>(&'a mut self, spec: &ImageSpec) -> Result<SubimageOutput<'a>,Error> {
        let open_result = unsafe {
            openimageio_sys::COIIO_ImageOutput_open(self.ptr, self.path.as_ptr(), spec.ptr, ImageOutput_OpenMode::AppendSubimage)
        };

        if !open_result {
            Err(Error::OpenError(self.get_last_error()))
        } else {
            Ok(SubimageOutput {
                parent: self
            })
        }
    }

    // Open subimages
    // For formats that do not support appending subimages
    // Should return an iterator or something, or pass a closure, maybe
    pub fn open_subimages<'a>(&'a mut self, specs: &[ImageSpec]) -> ! {
        unimplemented!()
    }

    fn close_internal(&mut self) {
        unsafe {
            openimageio_sys::COIIO_ImageOutput_close(self.ptr);
        }
    }
}

impl Drop for ImageOutput {
    fn drop(&mut self) {
        self.close_internal();
        unsafe {
            openimageio_sys::COIIO_ImageOutput_destroy(self.ptr);
        }
    }
}

impl<'a> SubimageOutput<'a> {
    pub fn write_image(&mut self) -> Result<(),Error> {
        unimplemented!()
    }

    pub unsafe fn write_image_raw(&mut self, typedesc: &TypeDesc, pixels: &[u8]) -> Result<(),Error> {
        let write_result =
            openimageio_sys::COIIO_ImageOutput_write_image(self.parent.ptr, typedesc, -1 as libc::ptrdiff_t, -1 as libc::ptrdiff_t, -1 as libc::ptrdiff_t);
        if !write_result {
            Err(Error::WriteError(self.parent.get_last_error()))
        }
        else {
            Ok(())
        }
    }


    // finish writing to this subimage (and release the borrow)
    pub fn close(self) {

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
