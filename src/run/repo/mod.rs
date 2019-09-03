use git2::{ErrorCode, Repository};
use std::fs;

pub struct GitRepo { url: &'static str }

pub trait Repo {
    // Static method signature; `Self` refers to the implementor type.
    fn new(path: &'static str) -> Self;
}

impl GitRepo {
    pub fn is_available(&self) -> bool {
        true
    }

    pub fn clone(temp_directory: &str) -> Result<Repository, ErrorCode> {
        // Remove all of the previous repo
        match fs::read_dir(temp_directory) {
            Ok(dir) => match dir.count() {
                0 => {}
                _ => match fs::remove_dir_all(temp_directory) {
                    Err(_e) => {
                        return Err(ErrorCode::Directory);
                    }
                    _ => {}
                },
            },
            Err(_e) => {
                return Err(ErrorCode::Directory);
            }
        }

        // Clone the repo into the directory
        Repository::clone(url, temp_directory).map_err(|_| ErrorCode::Directory)
    }
}

impl Repo for GitRepo {
    fn new(path: &'static str) -> GitRepo {
        GitRepo { url: path }
    }
}