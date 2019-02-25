use crate::typedesc::ImageData;
use std::{mem, slice};

/// Memory buffer containing image data.
///
/// The image data is stored in a `Vec`, which you can extract with [into_vec].
pub struct ImageBuffer<T: ImageData> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) depth: usize,
    pub(crate) num_channels: usize,
    //pub(crate) channels: Vec<ChannelDesc>,
    pub(crate) data: Vec<T>,
}

impl<T: ImageData> ImageBuffer<T> {
    /// Returns the width of this image.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of this image.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the depth of this image.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the image data reinterpreted as a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * mem::size_of::<T>(),
            )
        }
    }

    /// Returns the number of channels of this image.
    pub fn num_channels(&self) -> usize {
        self.num_channels
        //self.channels.len()
    }

    /*/// Returns the descriptions of all channels of this image.
    pub fn channels(&self) -> &[ChannelDesc] {
        &self.channels
    }*/

    /// Returns the image data.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Consumes this object and returns the `Vec` containing the image data.
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}
