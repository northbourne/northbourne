use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};

use config::Config;

use crate::error::Error;
use crate::error::Error::Generic;
use crate::pm::PackageManagerInterface;

#[derive(Debug)]
pub struct Homebrew {
    name: String,
    root_cmd: String,
    install_cmd: String,
    settings_repo: String,
    remove_cmd: String,
    check_cmd: String
}

impl Homebrew {
    pub fn new(settings: &Config) -> Homebrew {
        Homebrew {
            name: String::from("Homebrew"),
            root_cmd: String::from("brew"),
            install_cmd: String::from("install"),
            remove_cmd: String::from("remove"),
            check_cmd: String::from("list"),
            settings_repo: settings.get_str("repo_url").unwrap(),
        }
    }
}

impl PackageManagerInterface for Homebrew {
    fn check(&self, package_name: &str) -> Result<bool, Error> {
        let program = Command::new(self.root_cmd.as_str())
            .arg(self.check_cmd.as_str()).output().unwrap();

        match String::from_utf8(program.stdout).unwrap()
            .lines()
            .find(|string| {
                string == &package_name
            }) {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    fn install(&self, package_name: &str) -> Result<bool, Error> {
        let mut program = Command::new(self.root_cmd.as_str())
            .arg(self.install_cmd.as_str())
            .arg(package_name)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(ref mut stdout) = program.stdout {
            for line in BufReader::new(stdout).lines() {
                debug!("{:}", line.unwrap());
            }
        }

        if let Some(ref mut stderr) = program.stderr {
            for line in BufReader::new(stderr).lines() {
                warn!("{:}", line.unwrap());
            }
        }

        Ok(true)
    }

    fn uninstall(&self, package_name: &str) -> Result<bool, Error> {
        let mut program = Command::new(self.root_cmd.as_str())
            .arg(self.remove_cmd.as_str())
            .arg(package_name)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(ref mut stdout) = program.stdout {
            for line in BufReader::new(stdout).lines() {
                debug!("{:}", line.unwrap());
            }
        }

        if let Some(ref mut stderr) = program.stderr {
            for line in BufReader::new(stderr).lines() {
                warn!("{:}", line.unwrap());
            }
        }

        Ok(true)
    }

    fn remove(&self, package_name: &str) -> Result<bool, Error> {
        Err(Generic)
    }
}