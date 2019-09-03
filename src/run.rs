use git2::{ErrorCode, Repository};
use std::{fs, io};

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::num::ParseIntError;

#[cfg(feature = "yaml")]
pub fn program() -> Result<u64, Error> {
    use clap::App;

    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("conf/default.yml"))
        .unwrap();

    // Config
    match matches.value_of("config") {
        Some(config_path) => {
            settings
                .merge(config::File::with_name(config_path))
                .unwrap();
        }
        _ => {}
    }

    // Repo
    match matches.value_of("repo") {
        Some(repo) => {
            settings.set("repo", repo);
        }
        _ => {}
    }

    if settings.get_str("repo").unwrap() == "" {
        return Err(Error("No repository specified", ErrorCode::NotFound))
    }

    clone(
        settings.get_str("repo").unwrap().as_str(),
        settings.get_str("tmp_repo").unwrap().as_str(),
    );

    Ok(1)
}

fn clone(url: &str, temp_directory: &str) -> Result<Repository, ErrorCode> {
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