use crate::pm::PackageManagerInterface;

#[derive(Debug)]
pub struct Homebrew<'a> {
    name: &'a str,
    root_cmd: &'a str,
    install_cmd: &'a str,
}

impl Homebrew {
    pub fn new() -> &Homebrew {
        &Homebrew {
            name: "Homebrew",
            root_cmd: "brew",
            install_cmd: "install"
        }
    }
}

impl PackageManagerInterface for Homebrew {
    fn install(&self) -> Result<_, _> {

    }

    fn uninstall(&self) -> Result<_, _> {

    }

    fn remove(&self) -> Result<_, _> {

    }
}