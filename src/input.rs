use crate::{
    cstring_to_owned, error::get_last_error, typedesc::ImageData, Error, ImageBuffer, ImageSpec,
    TypeDesc,
};
use openimageio_sys as sys;
use openimageio_sys::AsStringRef;
use std::{ffi::c_void, mem, ops::Range, path::Path, ptr};

/// Image file opened for input.
///
/// Use [ImageInput::open] to open an image file.
///
/// Images may contain multiple _subimages_ (e.g. the faces of a cube map)
/// and/or _mip maps_. You must select which subimage to read from with the
/// [ImageInput::subimage_0] or [ImageInput::subimage] methods, and use
/// the returned [SubimageInput] object to read image data.
/// These methods exclusively borrow the `ImageInput` object, so it's impossible to read multiple
/// subimages at once.
pub struct ImageInput {
    ptr: *mut sys::OIIO_ImageInput,
}

impl ImageInput {
    /// Opens the image file at the specified path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ImageInput, Error> {
        let path = path.as_ref().to_str().unwrap();
        let ptr = unsafe { sys::OIIO_ImageInput_open(path.as_stringref(), ptr::null()) };
        if ptr.is_null() {
            Err(Error::OpenError(get_last_error()))
        } else {
            let input = ImageInput { ptr };
            //input.seek_subimage_mipmap(0, 0)?;
            Ok(input)
        }
    }

    /// Returns the metadata of this image.
    pub fn spec(&self) -> &ImageSpec {
        unsafe { &*(sys::OIIO_ImageInput_spec(self.ptr) as *const ImageSpec) }
    }

    /// Returns the width of this image.
    ///
    /// Equivalent to `spec().width()`.
    pub fn width(&self) -> u32 {
        self.spec().width()
    }

    /// Returns the width of this image.
    ///
    /// Equivalent to `spec().height()`.
    pub fn height(&self) -> u32 {
        self.spec().height()
    }

    /// Returns the width of this image.
    ///
    /// Equivalent to `spec().depth()`.
    pub fn depth(&self) -> u32 {
        self.spec().depth()
    }

    /// Returns the range of all channels of this image.
    /// Shorthand for `self.spec().all_channels()`.
    pub fn all_channels(&self) -> Range<usize> {
        self.spec().all_channels()
    }

    /// Returns the range of channel indices corresponding to the RGBA channels (in this order).
    /// Shorthand for `self.spec().rgba_channels()`.
    pub fn rgba_channels(&self) -> Result<Range<usize>, Error> {
        self.spec().rgba_channels()
    }

    /// Returns the range of channels corresponding to the given channel names.
    ///
    /// The specified channels must exist, must be in the same order as specified,
    /// and must be contiguous. Otherwise, an error is returned.
    ///
    /// Shorthand for `self.spec().channels_by_name(channel_names)`.
    pub fn channels_by_name(&self, channel_names: &[&str]) -> Result<Range<usize>, Error> {
        self.spec().channels_by_name(channel_names)
    }

    pub fn read<T: ImageData>(
        &self,
        subimage: usize,
        miplevel: usize,
        channels: Range<usize>,
    ) -> Result<ImageBuffer<T>, Error> {
        let spec = self.spec();
        let n = (spec.width() * spec.height() * spec.depth()) as usize * self.spec().num_channels();

        let mut data: Vec<T> = Vec::with_capacity(n);

        unsafe {
            self.read_unchecked(
                subimage,
                miplevel,
                channels.clone(),
                T::DESC,
                data.as_mut_ptr() as *mut u8,
                channels.len() * mem::size_of::<T>(),
            )?;
            data.set_len(n);

            Ok(ImageBuffer {
                width: self.width() as usize,
                height: self.height() as usize,
                depth: self.depth() as usize,
                data,
                num_channels: channels.len(),
            })
        }
    }

    pub fn read_into<T: ImageData>(
        &self,
        subimage: usize,
        miplevel: usize,
        channels: Range<usize>,
        out: &mut [T],
    ) -> Result<(), Error> {
        if self.spec().width() as usize
            * self.spec().height() as usize
            * self.spec().depth() as usize
            > out.len()
        {
            return Err(Error::BufferTooSmall);
        }

        unsafe {
            self.read_unchecked(
                subimage,
                miplevel,
                channels.clone(),
                T::DESC,
                out.as_mut_ptr() as *mut u8,
                channels.len() * mem::size_of::<T>(),
            )
        }
    }

    /// xstride: number of elements (T) to skip between pixels in a row
    pub unsafe fn read_unchecked(
        &self,
        subimage: usize,
        miplevel: usize,
        channels: Range<usize>,
        typedesc: TypeDesc,
        out: *mut u8,
        xstride: usize,
    ) -> Result<(), Error> {
        let mut success = true;

        success &= sys::OIIO_ImageInput_read_image(
            self.ptr,
            subimage as i32,
            miplevel as i32,
            channels.start as i32,
            channels.end as i32,
            typedesc.0,
            out as *mut c_void,
            xstride as isize,
            sys::OIIO_AutoStride,
            sys::OIIO_AutoStride,
        );

        if success {
            Ok(())
        } else {
            Err(Error::ReadError(self.get_last_error()))
        }
    }

    pub fn read_tiles_into<T: ImageData>(
        &self,
        subimage: usize,
        miplevel: usize,
        x: Range<u32>,
        y: Range<u32>,
        z: Range<u32>,
        channels: Range<usize>,
        out: &mut [T],
    ) -> Result<(), Error> {
        let tile_size = {
            let spec = self.spec();
            spec.tile_width() as usize * spec.tile_height() as usize * spec.num_channels()
        };
        if tile_size == 0 {
            // scanline image
            return Err(Error::InvalidForImageType);
        }
        if x.len() * y.len() * z.len() * channels.len() > out.len() {
            return Err(Error::BufferTooSmall);
        }

        let mut success = true;
        success &= unsafe {
            openimageio_sys::OIIO_ImageInput_read_tiles(
                self.ptr,
                subimage as i32,
                miplevel as i32,
                x.start as i32,
                x.end as i32,
                y.start as i32,
                y.end as i32,
                z.start as i32,
                z.end as i32,
                channels.start as i32,
                channels.end as i32,
                T::DESC.0,
                out.as_mut_ptr() as *mut c_void,
                sys::OIIO_AutoStride,
                sys::OIIO_AutoStride,
                sys::OIIO_AutoStride,
            )
        };
        if success {
            Ok(())
        } else {
            Err(Error::ReadError(self.get_last_error()))
        }
    }

    pub fn read_scanlines_into<T: ImageData>(
        &self,
        subimage: usize,
        miplevel: usize,
        y: Range<u32>,
        z: u32,
        channels: Range<usize>,
        out: &mut [T],
    ) -> Result<(), Error> {
        if self.spec().tile_width() != 0 {
            // tile image
            return Err(Error::InvalidForImageType);
        }
        let scanline_size = self.spec().width() * channels.len() as u32;
        if scanline_size > out.len() as u32 {
            return Err(Error::BufferTooSmall);
        }
        let mut success = true;
        success &= unsafe {
            openimageio_sys::OIIO_ImageInput_read_scanlines(
                self.ptr,
                subimage as i32,
                miplevel as i32,
                y.start as i32,
                y.end as i32,
                z as i32,
                channels.start as i32,
                channels.end as i32,
                T::DESC.0,
                out.as_mut_ptr() as *mut c_void,
                sys::OIIO_AutoStride,
                sys::OIIO_AutoStride,
            )
        };
        if success {
            Ok(())
        } else {
            Err(Error::ReadError(self.get_last_error()))
        }
    }

    fn get_last_error(&self) -> String {
        unsafe { cstring_to_owned(sys::OIIO_ImageInput_geterror(self.ptr)) }
    }
}

impl Drop for ImageInput {
    fn drop(&mut self) {
        unsafe {
            sys::OIIO_ImageInput_delete(self.ptr);
        }
    }
}
