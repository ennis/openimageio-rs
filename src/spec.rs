//! Types describing the format and content of an image file.
use crate::{Error, TypeDesc};
use openimageio_sys as sys;
use openimageio_sys::AsStringRef;
use std::{
    ffi::CStr,
    ops::{Bound, Deref, Range, RangeBounds},
    os::raw::c_int,
};

/// Describes one color channel of an image.
#[derive(Copy, Clone, Debug)]
pub struct Channel<'a> {
    /// Format of the channel data.
    pub format: TypeDesc,
    /// Name of the channel.
    pub name: &'a str,
}

impl<'a> Channel<'a> {
    /// Converts this object into a [ChannelDesc] which contains the same information
    /// but does not borrow.
    pub fn to_channel_desc(&self) -> ChannelDesc {
        ChannelDesc {
            format: self.format,
            name: self.name.to_string(),
        }
    }
}

/// Version of [Channel] that owns its contents.
#[derive(Clone, Debug)]
pub struct ChannelDesc {
    /// Format of the channel data.
    pub format: TypeDesc,
    /// Name of the channel.
    pub name: String,
}

/// Image specification: contains metadata about an image.
pub struct ImageSpec(pub(crate) sys::OIIO_ImageSpec); // ImageSpec is zero-sized

/// Represents a rectangular window in some coordinate space.
pub struct Window {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl ImageSpec {
    /// Returns the _data window_ of the image, containing:
    /// - the origin `(x, y, z)` of the pixel data of the image.
    /// - the size `(width, height, depth)` of the data of this image.
    ///
    /// (OpenImageIO:) A depth greater than 1 indicates a 3D "volumetric" image.
    ///
    /// `x,y,z` default to (0,0,0), but setting them differently may indicate
    /// that this image is offset from the usual origin.
    ///
    /// Pixel data are defined over pixel coordinates \[x ... x+width-1\] horizontally,
    /// \[y ... y+height-1\] vertically, and \[z ... z+depth-1\] in depth.
    pub fn data_window(&self) -> Window {
        Window {
            x: self.x(),
            y: self.y(),
            z: self.z(),
            width: self.width(),
            height: self.height(),
            depth: self.depth(),
        }
    }

    /// Equivalent to `self.data_window().x`.
    pub fn x(&self) -> i32 {
        unsafe { sys::OIIO_ImageSpec_x(&self.0) }
    }

    /// Equivalent to `self.data_window().y`.
    pub fn y(&self) -> i32 {
        unsafe { sys::OIIO_ImageSpec_y(&self.0) }
    }

    /// Equivalent to `self.data_window().z`.
    pub fn z(&self) -> i32 {
        unsafe { sys::OIIO_ImageSpec_z(&self.0) }
    }

    /// Returns the size of the image data `(width, height, depth)`.
    ///
    /// Equivalent to `(self.width(), self.height(), self.depth())`
    pub fn size(&self) -> (u32, u32, u32) {
        (self.width(), self.height(), self.depth())
    }

    /// Returns the width of this image.
    ///
    /// Equivalent to `self.data_window().width`.
    pub fn width(&self) -> u32 {
        unsafe { sys::OIIO_ImageSpec_width(&self.0) as u32 }
    }

    /// Returns the height of this image.
    ///
    /// Equivalent to `self.data_window().height`.
    pub fn height(&self) -> u32 {
        unsafe { sys::OIIO_ImageSpec_height(&self.0) as u32 }
    }

    /// Returns the depth of this image.
    ///
    /// Equivalent to `self.data_window().depth`.
    pub fn depth(&self) -> u32 {
        unsafe { sys::OIIO_ImageSpec_depth(&self.0) as u32 }
    }

    /// Returns the 2D size `(width,height)` of the image data, or `None` if this is not a 2D image
    /// (i.e. `depth != 1`)
    pub fn width_height(&self) -> Option<(u32, u32)> {
        if self.depth() == 1 {
            Some((self.width(), self.height()))
        } else {
            None
        }
    }

    /// Returns the "full" or "display" window of the image.
    ///
    /// (OpenImageIO) Having the full display window different from the pixel data window can be helpful in
    /// cases where you want to indicate that your image is a crop window of a larger image (if
    /// the pixel data window is a subset of the full display window), or that the pixels include
    /// overscan (if the pixel data is a superset of the full display window), or may simply indicate
    /// how different non-overlapping images piece together.
    pub fn display_window(&self) -> Window {
        Window {
            x: self.display_x(),
            y: self.display_y(),
            z: self.display_z(),
            width: self.display_width(),
            height: self.display_height(),
            depth: self.display_depth(),
        }
    }

    /// Equivalent to `self.display_window().x`.
    pub fn display_x(&self) -> i32 {
        unsafe { sys::OIIO_ImageSpec_full_x(&self.0) }
    }

    /// Equivalent to `self.display_window().y`.
    pub fn display_y(&self) -> i32 {
        unsafe { sys::OIIO_ImageSpec_full_y(&self.0) }
    }

    /// Equivalent to `self.display_window().z`.
    pub fn display_z(&self) -> i32 {
        unsafe { sys::OIIO_ImageSpec_full_z(&self.0) }
    }

    /// Returns the origin of the display window.
    ///
    /// Equivalent to `(self.display_x(),self.display_y(),self.display_z())`.
    pub fn display_origin(&self) -> (i32, i32, i32) {
        (self.display_x(), self.display_y(), self.display_z())
    }

    /// Equivalent to `self.display_window().width`.
    pub fn display_width(&self) -> u32 {
        unsafe { sys::OIIO_ImageSpec_full_width(&self.0) as u32 }
    }

    /// Equivalent to `self.display_window().height`.
    pub fn display_height(&self) -> u32 {
        unsafe { sys::OIIO_ImageSpec_full_height(&self.0) as u32 }
    }

    /// Equivalent to `self.display_window().depth`.
    pub fn display_depth(&self) -> u32 {
        unsafe { sys::OIIO_ImageSpec_full_depth(&self.0) as u32 }
    }

    /// Returns the size of the display window.
    ///
    /// Equivalent to `(self.display_width(),self.display_height(),self.display_depth())`.
    pub fn display_size(&self) -> (u32, u32, u32) {
        (
            self.display_width(),
            self.display_height(),
            self.display_depth(),
        )
    }

    /// (OpenImageIO:) The number of channels (color values) present in each pixel of the image.
    ///
    /// For example, an RGB image has 3 channels.
    pub fn num_channels(&self) -> usize {
        unsafe { sys::OIIO_ImageSpec_nchannels(&self.0) as usize }
    }

    /// Returns an iterator over the descriptions of the channels of the image.
    pub fn channels<'a>(&'a self) -> impl Iterator<Item = Channel> + 'a {
        let nch = self.num_channels();
        (0..nch).map(move |i| self.channel_by_index(i).unwrap())
    }

    /// Returns the description of the channel at index `index`.
    ///
    /// Returns [Error::InvalidChannelIndex] if no such channel exists.
    pub fn channel_by_index(&self, index: usize) -> Result<Channel, Error> {
        let nch = self.num_channels();
        if index >= nch {
            return Err(Error::InvalidChannelIndex);
        }
        let i = index as i32;

        let name = unsafe {
            CStr::from_ptr(sys::OIIO_ImageSpec_channelname(&self.0, i))
                .to_str()
                .unwrap()
        };

        let format = unsafe { TypeDesc(sys::OIIO_ImageSpec_channelformat(&self.0, i)) };

        Ok(Channel {
            format,
            name,
            //pixel_bytes,
        })
    }

    /// Returns the index and description of the channel identified by `name`, if it exists.
    pub fn channel_by_name(&self, name: &str) -> Result<(usize, Channel), Error> {
        self.channels()
            .enumerate()
            .find(move |(_, ch)| ch.name == name)
            .ok_or(Error::ChannelNotFound)
    }

    /// Returns the range of channel indices corresponding to the range bounds given in `index_range`.
    ///
    /// Returns [Error::InvalidParameter] if the end of the range is before the start, or if one
    /// bound exceeds the number of channels.
    pub fn channel_range(
        &self,
        index_range: impl RangeBounds<usize>,
    ) -> Result<Range<usize>, Error> {
        let nch = self.num_channels();

        let start = match index_range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match index_range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => nch,
        };

        if start >= end {
            return Err(Error::InvalidParameter);
        }
        if start >= nch {
            return Err(Error::ChannelIndexOutOfBounds);
        }
        if end > nch {
            return Err(Error::ChannelIndexOutOfBounds);
        }
        Ok(start..end)
    }

    /// Returns the range of channels corresponding to the given channel names.
    ///
    /// The specified channels must exist, must be in the same order as specified,
    /// and must be contiguous.
    /// Otherwise, an error is returned.
    pub fn channels_by_name(&self, channel_names: &[&str]) -> Result<Range<usize>, Error> {
        if channel_names.is_empty() {
            return Err(Error::InvalidParameter);
        }

        let mut range = None;

        for name in channel_names.iter() {
            let (next, _) = self.channel_by_name(name)?;
            if let Some((_begin, ref mut end)) = range {
                if *end != next {
                    return Err(Error::NoncontiguousChannels);
                } else {
                    *end = next + 1;
                }
            } else {
                range = Some((next, next + 1));
            }
        }

        if let Some((begin, end)) = range {
            Ok(begin..end)
        } else {
            Err(Error::ChannelNotFound)
        }
    }

    /// Equivalent to `self.channel_range(..)`
    pub fn all_channels(&self) -> Range<usize> {
        0..self.num_channels()
    }

    /// Returns the range of channel indices corresponding to the RGB channels (in this order).
    pub fn rgb_channels(&self) -> Result<Range<usize>, Error> {
        let nch = self.num_channels();
        if nch < 3 {
            return Err(Error::ChannelIndexOutOfBounds);
        }
        Ok(0..3)
    }

    /// Returns the range of channel indices corresponding to the RGBA channels (in this order).
    pub fn rgba_channels(&self) -> Result<Range<usize>, Error> {
        self.channels_by_name(&["R", "G", "B", "A"])
    }

    /// Returns the index of the alpha channel (the channel named "A").
    pub fn alpha_channel(&self) -> Result<usize, Error> {
        Ok(self.channels_by_name(&["A"])?.start)
    }

   /* /// Finds every channel whose name match the specified regular expression.
    pub fn find_channels<'a>(&'a self, re: &str) -> impl Iterator<Item = (usize, Channel)> + 'a {
        let re = regex::Regex::new(re).expect("invalid regular expression");
        self.channels()
            .enumerate()
            .filter(move |(_, ch)| re.is_match(ch.name))
    }*/

    /// Calculates coordinate ranges from the specified bounds.
    ///
    /// Example:
    /// ```rust
    /// // with width,height,depth = 1024
    /// assert_eq!(spec.calculate_bounds(..,..,..) == (0..1024,0..1024,0..1024));
    /// assert_eq!(spec.calculate_bounds(512..,512..,..) == (512..1024,512..1024,512..1024));
    /// ```
    pub fn calculate_bounds(
        &self,
        xs: impl RangeBounds<i32>,
        ys: impl RangeBounds<i32>,
        zs: impl RangeBounds<i32>,
    ) -> (Range<i32>, Range<i32>, Range<i32>) {
        let (width, height, depth) = self.size();
        let (xmax, ymax, zmax) = (width as i32, height as i32, depth as i32);
        // X
        let xstart = match xs.start_bound() {
            Bound::Included(&xstart) => xstart,
            Bound::Excluded(&xstart) => xstart + 1,
            Bound::Unbounded => 0,
        };
        let xend = match xs.end_bound() {
            Bound::Included(&xend) => xend + 1,
            Bound::Excluded(&xend) => xend,
            Bound::Unbounded => xmax,
        };
        // Y
        let ystart = match ys.start_bound() {
            Bound::Included(&ystart) => ystart,
            Bound::Excluded(&ystart) => ystart + 1,
            Bound::Unbounded => 0,
        };
        let yend = match ys.end_bound() {
            Bound::Included(&yend) => yend + 1,
            Bound::Excluded(&yend) => yend,
            Bound::Unbounded => ymax,
        };
        // Z
        let zstart = match zs.start_bound() {
            Bound::Included(&zstart) => zstart,
            Bound::Excluded(&zstart) => zstart + 1,
            Bound::Unbounded => 0,
        };
        let zend = match zs.end_bound() {
            Bound::Included(&zend) => zend + 1,
            Bound::Excluded(&zend) => zend,
            Bound::Unbounded => zmax,
        };

        (xstart..xend, ystart..yend, zstart..zend)
    }
}

/// Version of [ImageSpec] that owns its data.
pub struct ImageSpecOwned(pub(crate) *mut sys::OIIO_ImageSpec);

impl Clone for ImageSpecOwned {
    fn clone(&self) -> Self {
        unsafe { ImageSpecOwned(sys::OIIO_ImageSpec_clone(self.0)) }
    }
}

impl ImageSpecOwned {
    /// Creates the metadata of a zero-sized image with unknown format.
    pub fn new() -> ImageSpecOwned {
        let ptr = unsafe { sys::OIIO_ImageSpec_new(TypeDesc::UNKNOWN.0) };
        ImageSpecOwned(ptr)
    }

    /// Creates the metadata of a 2D image with the specified format, resolution, and channels.
    ///
    /// All channels share the same format.
    pub fn new_2d(format: TypeDesc, xres: u32, yres: u32, channels: &[&str]) -> ImageSpecOwned {
        let channels = channels
            .iter()
            .map(|s| s.as_stringref())
            .collect::<Vec<_>>();

        let formatptr = &format.0;

        let ptr = unsafe {
            sys::OIIO_ImageSpec_new_2d(
                xres as c_int,
                yres as c_int,
                channels.len() as c_int,
                false,
                formatptr,
                channels.as_ptr(),
            )
        };

        ImageSpecOwned(ptr)
    }
}

impl Drop for ImageSpecOwned {
    fn drop(&mut self) {
        unsafe {
            sys::OIIO_ImageSpec_delete(self.0);
        }
    }
}

impl Deref for ImageSpecOwned {
    type Target = ImageSpec;

    fn deref(&self) -> &ImageSpec {
        unsafe { &*(self.0 as *const ImageSpec) }
    }
}

