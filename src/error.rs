use std::io;
use std::result;
use std::fmt;
use std::string;
use nom;
pub type Result<T> = result::Result<T, Error>;

#[derive (Debug)]
pub enum Error {
    IOError(io::Error),
    NomError(nom::ErrorKind),
    Custom(&'static str),
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
impl From<nom::ErrorKind> for Error {
    fn from(error: nom::ErrorKind) -> Error {
        Error::NomError(error)
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
