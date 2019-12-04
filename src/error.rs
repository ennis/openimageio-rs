use crate::cstring_to_owned;
use std::{error, fmt};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    OpenError(String),
    WriteError(String),
    ReadError(String),
    SubimageNotFound,
    ChannelIndexOutOfBounds,
    ChannelNotFound,
    InvalidChannelIndex,
    InvalidAttributeNameOrType,
    NoncontiguousChannels,
    InvalidParameter,
    BufferTooSmall,
    InvalidForImageType,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::OpenError(ref msg) => write!(f, "error opening image: {}", msg),
            Error::WriteError(ref msg) => write!(f, "error writing image data: {}", msg),
            Error::ReadError(ref msg) => write!(f, "error reading image data: {}", msg),
            Error::SubimageNotFound => write!(f, "non-existent subimage"),
            Error::ChannelNotFound => write!(f, "non-existent channel"),
            Error::InvalidParameter => write!(f, "invalid parameter"),
            Error::ChannelIndexOutOfBounds => write!(f, "channel index out of bounds"),
            Error::NoncontiguousChannels => write!(
                f,
                "selected channels are non-contiguous and cannot be read all at once"
            ),
            Error::InvalidAttributeNameOrType => {
                write!(f, "non-existent attribute or incorrect type for attribute")
            }
            Error::InvalidChannelIndex => write!(f, "non-existent channel index"),
            Error::BufferTooSmall => write!(f, "buffer was too small"),
            Error::InvalidForImageType=> write!(f, "image type did not support operation"),
            //_ => write!(f, "Unknown error."),
        }
    }
}

pub fn get_last_error() -> String {
    unsafe { cstring_to_owned(openimageio_sys::OIIO_geterror()) }
}
