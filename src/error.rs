use std::io;
use std::result;
use std::fmt;
use url;
use nom;
pub type Result<T> = result::Result<T, Error>;

#[derive (Debug)]
pub enum Error {
    IOError(io::Error),
    UrlError(url::ParseError),
    NomError(nom::ErrorKind),
    Custom(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IOError(ref e) => e.fmt(f),
            Error::UrlError(ref e) => e.fmt(f),
            Error::NomError(ref e) => e.fmt(f),
            Error::Custom(s) => f.write_str(s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Error {
        Error::IOError(io_error)
    }
}
impl From<url::ParseError> for Error {
    fn from(url_error: url::ParseError) -> Error {
        Error::UrlError(url_error)
    }
}
impl From<nom::ErrorKind> for Error {
    fn from(error: nom::ErrorKind) -> Error {
        Error::NomError(error)
    }
}
