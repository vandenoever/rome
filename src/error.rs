use std::io;
use std::result;
use std::error;
pub type Result<T> = result::Result<T, Error>;

#[derive (Debug)]
pub enum Error {
    IOError(io::Error),
    Custom(&'static str),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Error {
        Error::IOError(io_error)
    }
}
