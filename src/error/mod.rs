use std::error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Generic
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Generic => write!(f, "Generic Error")
        }
    }
}