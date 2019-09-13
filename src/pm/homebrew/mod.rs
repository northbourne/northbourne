use crate::pm::PackageManagerInterface;
use crate::error::Error;
use crate::error::Error::Generic;
use config::Config;

#[derive(Debug)]
pub struct Homebrew {
    name: String,
    root_cmd: String,
    install_cmd: String,
    settings_repo: String,
}

impl Homebrew {
    pub fn new(settings: &Config) -> Homebrew {
        Homebrew {
            name: String::from("Homebrew"),
            root_cmd: String::from("brew"),
            install_cmd: String::from("install"),
            settings_repo: settings.get_str("repo_url").unwrap()
        }
    }
}

impl PackageManagerInterface for Homebrew {
    fn install(&self) -> Result<bool, Error> {
        Err(Generic)
    }

    fn uninstall(&self) -> Result<bool, Error> {
        Err(Generic)
    }

    fn remove(&self) -> Result<bool, Error> {
        Err(Generic)
    }
}