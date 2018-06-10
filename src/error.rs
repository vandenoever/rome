//! The error and result types for this crate.

use nom;
use std::fmt;
use std::io;
use std::result;
use std::string;

/// The result type for this crate.
pub type Result<T> = result::Result<T, Error>;

/// The error type for this crate.
#[derive(Debug)]
pub enum Error {
    /// This error was caused by an IO error.
    IOError(io::Error),
    /// This error was caused by the nom parser.
    NomError(String),
    /// A custom error from `&'static str`.
    Custom(&'static str),
    /// A custom error from `String`.
    String(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IOError(ref e) => e.fmt(f),
            Error::NomError(ref e) => e.fmt(f),
            Error::Custom(s) => f.write_str(s),
            Error::String(ref s) => f.write_str(s.as_str()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Error {
        Error::IOError(io_error)
    }
}
impl<T: ::std::fmt::Debug> From<nom::Context<T>> for Error {
    fn from(error: nom::Context<T>) -> Error {
        Error::NomError(format!("{:?}", error))
    }
}
impl From<String> for Error {
    fn from(error: String) -> Error {
        Error::String(error)
    }
}
impl From<string::FromUtf8Error> for Error {
    fn from(_: string::FromUtf8Error) -> Error {
        Error::Custom("Error decoding invalid UTF-8")
    }
}
