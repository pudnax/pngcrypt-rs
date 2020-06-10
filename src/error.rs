use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Io(io::Error),
    Fmt(std::fmt::Error),
    ParseInt(std::num::ParseIntError),
    Utf8Err(usize),
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self::Fmt(e)
    }
}

impl From<Error> for std::fmt::Error {
    fn from(_: Error) -> Self {
        std::fmt::Error
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8Err(e.valid_up_to())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "{}", s),
            Self::Io(e) => write!(f, "{}", e),
            Self::ParseInt(e) => write!(f, "{}", e),
            Self::Fmt(e) => write!(f, "{}", e),
            Self::Utf8Err(num) => write!(f, "Invalid byte was encountered on index: {}", num),
        }
    }
}

impl std::error::Error for Error {}
