use std::error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Generic = 0,
    Permission = 1,
    NotFound = 2,
    Unimplimented = 4,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Generic => "Generic Error was found, exiting",
            Error::Permission => "Permissions error was found, exiting",
            Error::NotFound => "Not found exception was found thrown, exiting",
            Error::Unimplimented => "wip"
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}