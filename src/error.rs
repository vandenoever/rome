use std::io;
use std::result;
use url;
pub type Result<T> = result::Result<T, Error>;

#[derive (Debug)]
pub enum Error {
    IOError(io::Error),
    UrlError(url::ParseError),
    Custom(&'static str),
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
