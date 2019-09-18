use std::process::{Command, Output};

use crate::error::Error;

pub mod homebrew;

pub enum PackageManagerType {
    Yum,
    Apt,
    Homebrew
}

pub trait PackageManagerInterface {
    fn check(&self, package_name: &str) -> Result<bool, Error>;
    fn install(&self, package_name: &str) -> Result<bool, Error>;
    fn uninstall(&self, package_name: &str) -> Result<bool, Error>;
    fn remove(&self, package_name: &str) -> Result<bool, Error>;
}