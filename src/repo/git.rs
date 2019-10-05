use std::fmt::{Debug, Formatter};
use std::{fs, io};
use std::io::Write;
use git2::{Error as GitError, ErrorCode, ErrorCode::NotFound, Remote, Repository as GitRepository, AutotagOption, FetchOptions, RemoteCallbacks};
use log::{error, info, warn};
use crate::error::{Error, Error::*};
use crate::repo::Repo;


pub type Result<T> = std::result::Result<T, Error>;

pub struct GitRepo {
    pub repo_url: String,
    pub repo_directory: String
}

impl GitRepo {
    pub fn is_available(&self) -> bool {
        true
    }

    fn check_remote_is_the_same(remote: &Remote, origin: &str) -> bool {
        remote.url().unwrap() == origin
    }


    pub fn set_repo_url(&mut self, repo_url: String) -> &Self {
        self.repo_url = repo_url;
        self
    }

    pub fn set_repo_directory(&mut self, repo_directory: String) -> &Self {
        self.repo_directory = repo_directory;
        self
    }

}

impl Repo<GitRepository> for GitRepo {
    fn new() -> Self {
        GitRepo {
            repo_url: String::from("Test"),
            repo_directory: String::from("")
        }
    }

    fn discover(&self) -> Result<GitRepository> {
        // Remove all of the previous repo
        info!("---- North Repository {:} ----", self.repo_url);

        match GitRepository::discover(&self.repo_directory) {
            Ok(repo) => {
                // GitRepo::check_remote_is_the_same();
                Ok (repo)
            },
            Err(ref e) if e.code() == ErrorCode::NotFound => {
                Err(Error::NotFound)
            },
            Err(e) => {
                println!("{:?}", e);
                panic!();
            }
        }

        // Clone the repo into the repo_directory
    }

    fn sync(&self) -> Result<GitRepository> {
        // Fetch
        let repo = GitRepository::open(self.repo_directory.as_str()).unwrap();
        let mut origin_remote = repo.find_remote("origin").unwrap();
        origin_remote.fetch(&["master"], None, None).or_else(|e| {
            return Err(e);
        });

        // Reset
        let oid = repo.refname_to_id("refs/remotes/origin/master").unwrap();
        let object = repo.find_object(oid, None).unwrap();
        repo.reset(&object, git2::ResetType::Hard, None).unwrap();

        Ok(GitRepository::open(&self.repo_directory).unwrap())
    }


    fn clone(&self) -> Result<GitRepository> {
        info!("Cloning into directory {}", &self.repo_directory);

        match GitRepository::clone(self.repo_url.as_str(), &self.repo_directory).map_err(|_| ErrorCode::Directory) {
            Ok(repo) => {
                info!("Successfully cloned repo {:?}", self.repo_url);
                Ok(repo)
            },
            Err(error_code) => {
                error!("Error cloning repo {:?}", error_code);
                Err(Generic)
            }
        }
    }
}