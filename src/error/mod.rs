use std::error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    Generic
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Generic => write!(f, "Generic Error")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            Error::Parse(ref e) => Some(e),
        }
    }
}
