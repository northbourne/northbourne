pub mod homebrew;

use std::process::{Command, Output};
use crate::error::GenericError;

pub enum PackageManagerType {
    Yum,
    Apt,
    Homebrew
}

pub trait PackageManagerInterface {
    fn install(&self) -> Result<(), GenericError>;
    fn uninstall(&self) -> Result<(), GenericError>;
    fn remove(&self) -> Result<(), GenericError>;
}

#[derive(Debug)]
pub struct PackageManager<T: PackageManagerInterface> {
    provider: T
}