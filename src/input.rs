use crate::{
    cstring_to_owned, error::get_last_error, typedesc::ImageData, Error, ImageBuffer, ImageSpec,
    ImageSpecOwned,
};
use openimageio_sys as sys;
use openimageio_sys::AsStringRef;
use std::{
    ffi::c_void,
    mem,
    ops::{Range, RangeBounds},
    path::Path,
    ptr,
};

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

    /// Selects a subimage
    pub fn subimage(&mut self, subimage: usize) -> Result<SubimageMipmapInput, Error> {
        self.subimage_mipmap(subimage, 0)
    }

    /// Selects a subimage and mip level.
    pub fn subimage_mipmap(
        &mut self,
        subimage: usize,
        miplevel: usize,
    ) -> Result<SubimageMipmapInput, Error> {
        let spec = self.seek_subimage_mipmap(subimage, miplevel)?;
        Ok(SubimageMipmapInput {
            img: self,
            spec,
            miplevel,
            subimage,
        })
    }

    fn seek_subimage_mipmap(
        &mut self,
        subimage: usize,
        miplevel: usize,
    ) -> Result<ImageSpecOwned, Error> {
        let spec = ImageSpecOwned::new();

        let exists = unsafe {
            sys::OIIO_ImageInput_seek_subimage_miplevel(
                self.ptr,
                subimage as i32,
                miplevel as i32,
                spec.0,
            )
        };

        if exists {
            Ok(spec)
        } else {
            Err(Error::SubimageNotFound)
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

    fn with_channels(&mut self, channels: Range<usize>) -> ImageChannelsInput {
        let spec = self
            .seek_subimage_mipmap(0, 0)
            .expect("failed to seek to main image");
        ImageChannelsInput {
            img: self,
            spec,
            subimage: 0,
            miplevel: 0,
            channels,
        }
    }

    /// Selects channels.
    pub fn channels_by_name(
        &mut self,
        channel_names: &[&str],
    ) -> Result<ImageChannelsInput, Error> {
        Ok(self.with_channels(self.spec().channels_by_name(channel_names)?))
    }

    /// Selects channels.
    pub fn channels(
        &mut self,
        channels: impl RangeBounds<usize>,
    ) -> Result<ImageChannelsInput, Error> {
        Ok(self.with_channels(self.spec().channel_range(channels)?))
    }

    /// Selects channels.
    pub fn all_channels(&mut self) -> ImageChannelsInput {
        self.with_channels(self.spec().all_channels())
    }

    /// Selects channels.
    pub fn rgba_channels(&mut self) -> Result<ImageChannelsInput, Error> {
        Ok(self.with_channels(self.spec().rgba_channels()?))
    }

    /// Selects channels.
    pub fn alpha_channel(&mut self) -> Result<ImageChannelsInput, Error> {
        let ch = self.spec().alpha_channel()?;
        Ok(self.with_channels(ch..(ch + 1)))
    }

    /// Shorthand to read all the channels of the top mip level of the first subimage
    /// into an [ImageBuffer].
    pub fn read<I: ImageData>(&mut self) -> Result<ImageBuffer<I>, Error> {
        self.all_channels().read()
    }

    pub fn read_into<T: ImageData>(&mut self, out: &mut [T]) -> Result<(), Error> {
        self.all_channels().read_into(out)
    }

    fn get_last_error(&self) -> String {
        unsafe { cstring_to_owned(sys::OIIO_ImageInput_geterror(self.ptr)) }
    }
}

pub struct SubimageMipmapInput<'a> {
    img: &'a mut ImageInput,
    spec: ImageSpecOwned,
    subimage: usize,
    miplevel: usize,
}

impl<'a> SubimageMipmapInput<'a> {
    /// Returns the metadata of this subimage.
    pub fn spec(&self) -> &ImageSpec {
        &self.spec
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

    fn with_channels(self, channels: Range<usize>) -> ImageChannelsInput<'a> {
        ImageChannelsInput {
            img: self.img,
            spec: self.spec,
            subimage: self.subimage,
            miplevel: self.miplevel,
            channels,
        }
    }

    /// Selects channels.
    pub fn channels_by_name(self, channel_names: &[&str]) -> Result<ImageChannelsInput<'a>, Error> {
        let channels = self.spec.channels_by_name(channel_names)?;
        Ok(self.with_channels(channels))
    }

    /// Selects channels.
    pub fn channels(
        self,
        channels: impl RangeBounds<usize>,
    ) -> Result<ImageChannelsInput<'a>, Error> {
        let channels = self.spec.channel_range(channels)?;
        Ok(self.with_channels(channels))
    }

    /// Selects channels.
    pub fn all_channels(self) -> ImageChannelsInput<'a> {
        let channels = self.spec.all_channels();
        self.with_channels(channels)
    }

    /// Selects channels.
    pub fn rgba_channels(self) -> Result<ImageChannelsInput<'a>, Error> {
        let channels = self.spec.rgba_channels()?;
        Ok(self.with_channels(channels))
    }

    /// Selects channels.
    pub fn channel_alpha(self) -> Result<ImageChannelsInput<'a>, Error> {
        let ch = self.spec.alpha_channel()?;
        Ok(self.with_channels(ch..(ch + 1)))
    }

    /// Shorthand to read all the channels into an [ImageBuffer].
    pub fn read<I: ImageData>(self) -> Result<ImageBuffer<I>, Error> {
        self.all_channels().read()
    }
    pub fn read_into<T: ImageData>(self, out: &mut [T]) -> Result<(), Error> {
        self.all_channels().read_into(out)
    }
}

/// A subimage, mip level and a set of channels selected from a parent image.
///
/// This type has methods to actually read the image data.
pub struct ImageChannelsInput<'a> {
    img: &'a mut ImageInput,
    spec: ImageSpecOwned,
    subimage: usize,
    miplevel: usize,
    channels: Range<usize>,
}

impl<'a> ImageChannelsInput<'a> {
    /// Returns the metadata of this subimage.
    pub fn spec(&self) -> &ImageSpec {
        &self.spec
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

    /// Returns the index of this subimage in the parent [ImageInput].
    pub fn subimage_index(&self) -> usize {
        self.subimage
    }

    /// Returns the mip level of this subimage.
    ///
    /// 0 is the topmost and largest level. It is also the value returned if the image has no mipmap.
    ///
    /// A subimage is uniquely identified by its subimage index and mip level.
    pub fn mip_level(&self) -> usize {
        self.miplevel
    }

    /// Reads channels of the image to an [ImageBuffer].
    pub fn read<T: ImageData>(&self) -> Result<ImageBuffer<T>, Error> {
        let spec = self.spec();
        let n = (spec.width() * spec.height() * spec.depth()) as usize * self.channels.len();

        let mut data: Vec<T> = Vec::with_capacity(n);

        unsafe {
            self.read_unchecked(data.as_mut_ptr())?;
            data.set_len(n);

            Ok(ImageBuffer {
                width: self.width() as usize,
                height: self.height() as usize,
                depth: self.depth() as usize,
                data,
                num_channels: self.channels.len(),
            })
        }
    }

    /// Reads channels into an existing buffer.
    pub fn read_into<T: ImageData>(&self, out: &mut [T]) -> Result<(), Error> {
        let spec = self.spec();
        let n = (spec.width() * spec.height() * spec.depth()) as usize * self.channels.len();
        if out.len() < n {
            return Err(Error::BufferTooSmall);
        }

        unsafe { self.read_unchecked(out.as_mut_ptr()) }
    }

    unsafe fn read_unchecked<T: ImageData>(&self, out: *mut T) -> Result<(), Error> {
        let mut success = true;

        success &= sys::OIIO_ImageInput_read_image_format2(
            self.img.ptr,
            self.channels.start as i32,
            self.channels.end as i32,
            T::DESC.0,
            out as *mut c_void,
            (self.channels.len() * mem::size_of::<T>()) as isize,
            sys::OIIO_AutoStride,
            sys::OIIO_AutoStride,
            ptr::null_mut(),
        );

        if success {
            Ok(())
        } else {
            Err(Error::ReadError(self.img.get_last_error()))
        }
    }

    /*unsafe fn read_unchecked_channel_ranges<T: ImageData>(&self, out: *mut T) -> Result<(), Error> {
        let mut success = true;
        let mut ich = 0;
        for r in self.channels.ranges.iter() {
            success &= sys::OIIO_ImageInput_read_image_format2(
                self.img.ptr,
                r.start as i32,
                r.end as i32,
                T::DESC.0,
                out.offset(ich) as *mut c_void,
                (self.channels.count * mem::size_of::<T>()) as isize,
                sys::OIIO_AutoStride,
                sys::OIIO_AutoStride,
                ptr::null_mut(),
            );

            ich += r.len() as isize;
        }

        if success {
            Ok(())
        } else {
            Err(Error::ReadError(self.img.get_last_error()))
        }
    }*/
}

impl Drop for ImageInput {
    fn drop(&mut self) {
        unsafe {
            sys::OIIO_ImageInput_delete(self.ptr);
        }
    }
}
