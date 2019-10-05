use std::fmt::{Debug, Formatter};
use std::{fs, io};
use std::io::Write;
use git2::{Error as GitError, ErrorCode, ErrorCode::NotFound, Remote, Repository, AutotagOption, FetchOptions, RemoteCallbacks};
use log::{error, info, warn};
use crate::error::{Error, Error::*};

pub type Result<T> = std::result::Result<T, Error>;

pub trait Repo <T> {
    // Static method signature; `Self` refers to the implementor type.
    fn new() -> Self;

    // Discover new repository
    fn discover(&self) -> Result<T>;

    // Update changes
    fn sync(&self) -> Result<T>;

    // Create new folder
    fn clone(&self) -> Result<T>;
}

pub mod git;