//! Image output
use crate::{cstring_to_owned, error::get_last_error, typedesc::ImageData, Error, ImageSpec};
use openimageio_sys as sys;
use openimageio_sys::AsStringRef;
use std::{ffi::c_void, io, path::Path};

///
pub struct ImageOutput {
    ptr: *mut sys::OIIO_ImageOutput,
    path: String,
}

pub struct SingleImageOutput<'a>(&'a mut ImageOutput);
pub struct MultiImageOutput<'a>(SingleImageOutput<'a>);

impl ImageOutput {
    fn get_last_error(&self) -> String {
        unsafe { cstring_to_owned(sys::OIIO_ImageOutput_geterror(self.ptr)) }
    }

    /// Creates an imageoutput
    pub fn create<P: AsRef<Path>>(path: P) -> Result<ImageOutput, Error> {
        let path = path.as_ref().to_str().unwrap();
        let plugin_search_path = "";
        let ptr = unsafe { sys::OIIO_ImageOutput_create(path.as_stringref(), plugin_search_path.as_stringref()) };

        if ptr.is_null() {
            return Err(Error::OpenError(get_last_error()));
        }

        Ok(ImageOutput {
            ptr,
            path: path.to_string(),
        })
    }

    /// Opens an imageoutput
    pub fn open(&mut self, spec: &ImageSpec) -> Result<SingleImageOutput, Error> {
        // init headers
        let specs = &[spec];
        let success = unsafe {
            sys::OIIO_ImageOutput_open2(
                self.ptr,
                self.path.as_stringref(),
                1,
                specs.as_ptr() as *const *const sys::OIIO_ImageSpec,
            )
        };

        if !success {
            return Err(Error::OpenError(self.get_last_error()));
        }

        Ok(SingleImageOutput(self))
    }

    /// Opens an imageoutput
    pub fn open_multi(&mut self, subimages: &[&ImageSpec]) -> Result<MultiImageOutput, Error> {
        // init headers
        let success = unsafe {
            sys::OIIO_ImageOutput_open2(
                self.ptr,
                self.path.as_stringref(),
                subimages.len() as i32,
                subimages.as_ptr() as *const *const sys::OIIO_ImageSpec,
            )
        };

        if !success {
            return Err(Error::OpenError(self.get_last_error()));
        }

        Ok(MultiImageOutput(SingleImageOutput(self)))
    }

    fn close_internal(&mut self) {
        unsafe {
            sys::OIIO_ImageOutput_close(self.ptr);
        }
    }
}

impl Drop for ImageOutput {
    fn drop(&mut self) {
        self.close_internal();
        unsafe {
            sys::OIIO_ImageOutput_delete(self.ptr);
        }
    }
}

impl<'a> SingleImageOutput<'a> {
    pub fn spec(&self) -> &ImageSpec {
        unsafe { &*(sys::OIIO_ImageOutput_spec(self.0.ptr) as *const ImageSpec) }
    }

    pub fn write_image<T: ImageData>(&mut self, pixels: &[T]) -> Result<(), Error> {
        let spec = self.spec();
        let nch = spec.num_channels();
        let _n = (spec.width() * spec.height() * spec.depth()) as usize * nch;

        let write_result = unsafe {
            sys::OIIO_ImageOutput_write_image(
                self.0.ptr,
                T::DESC.0,
                pixels.as_ptr() as *const c_void,
                sys::OIIO_AutoStride,
                sys::OIIO_AutoStride,
                sys::OIIO_AutoStride,
            )
        };
        if !write_result {
            Err(Error::WriteError(self.0.get_last_error()))
        } else {
            Ok(())
        }
    }

    pub fn write_scanline<T: ImageData>(&mut self, y: i32, z: i32, pixels: &[T]) -> Result<(), Error> {
        let spec = self.spec();
        let nch = spec.num_channels();
        let _n = (spec.width() * spec.height() * spec.depth()) as usize * nch;

        let write_result = unsafe {
            sys::OIIO_ImageOutput_write_scanline(
                self.0.ptr,
                y,
                z,
                T::DESC.0,
                pixels.as_ptr() as *const c_void,
                sys::OIIO_AutoStride,
            )
        };
        if !write_result {
            Err(Error::WriteError(self.0.get_last_error()))
        } else {
            Ok(())
        }
    }

    // finish writing to this subimage (and release the borrow)
    pub fn close(self) {}
}
