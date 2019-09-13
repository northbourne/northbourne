pub mod homebrew;

use std::process::{Command, Output};
use crate::error::Error;

pub enum PackageManagerType {
    Yum,
    Apt,
    Homebrew
}

const HOME : PackageManagerType = PackageManagerType::Homebrew;

pub trait PackageManagerInterface {
    fn install(&self) -> Result<bool, Error>;
    fn uninstall(&self) -> Result<bool, Error>;
    fn remove(&self) -> Result<bool, Error>;
}

#[derive(Debug)]
pub struct PackageManager<T: PackageManagerInterface> {
    pub provider: T
}