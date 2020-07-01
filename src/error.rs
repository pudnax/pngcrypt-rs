use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error<'static>>;

#[derive(Debug)]
pub enum Error<'a> {
    Custom(&'a str),
    Io(io::Error),
    Fmt(std::fmt::Error),
    ParseInt(std::num::ParseIntError),
    Utf8Err(std::str::Utf8Error),
    FromUtf8Error(std::string::FromUtf8Error),
    FromSlice(std::array::TryFromSliceError),
}

impl<'a> From<std::array::TryFromSliceError> for Error<'a> {
    fn from(e: std::array::TryFromSliceError) -> Self {
        Self::FromSlice(e)
    }
}

impl<'a> From<std::fmt::Error> for Error<'a> {
    fn from(e: std::fmt::Error) -> Self {
        Self::Fmt(e)
    }
}

impl<'a> From<Error<'a>> for std::fmt::Error {
    fn from(_: Error) -> Self {
        std::fmt::Error
    }
}

impl<'a> From<io::Error> for Error<'a> {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl<'a> From<std::num::ParseIntError> for Error<'a> {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl<'a> From<std::str::Utf8Error> for Error<'a> {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8Err(e)
    }
}

impl<'a> From<std::string::FromUtf8Error> for Error<'a> {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(e)
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FromSlice(e) => write!(f, "{}", e),
            Self::Custom(s) => write!(f, "{}", s),
            Self::Io(e) => write!(f, "{}", e),
            Self::ParseInt(e) => write!(f, "{}", e),
            Self::Fmt(e) => write!(f, "{}", e),
            Self::Utf8Err(e) => write!(f, "{}", e),
            Self::FromUtf8Error(e) => write!(f, "{}", e),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}
