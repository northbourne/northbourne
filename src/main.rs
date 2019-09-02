#[macro_use]
extern crate clap;
extern crate config;

use std::{fs, io, process};
use git2::{Repository, Error};

use std::collections::HashMap;
use std::num::ParseIntError;

///
/// Northbourne
///
fn main() {
    match run() {
        Ok(0) => process::exit(1),
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
        _ => {}
    }
}

#[cfg(feature = "yaml")]
fn run() -> Result<u64, ParseIntError> {
    use clap::App;

    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Config
    match matches.value_of("config") {
        Some(config_path) => {
            let mut settings = config::Config::default();

            settings.merge(config::File::with_name("conf/default.yml")).unwrap();
            settings.merge(config::File::with_name(config_path)).unwrap();

            // print!("{:?}", settings.try_into::<HashMap<String, String>>().unwrap());

            clone_repo(settings.get_str("repo").unwrap().as_str(), "/tmp/repo2");
        },
        _ => {

        }
    }

    // Repo
    match matches.value_of("repo") {
        Some(repo) => {
            println!("Value for repo: {}", repo);
        },
        _ => {

        }
    }

    Ok(1)
}


fn clone_repo(repo_url: &str, temp_directory: &str) -> Repository {
    println!("Cloning repo {}", repo_url);

    let repo: Repository = match clone(repo_url, temp_directory) {
        Ok(repo) => {
            println!("Cloned repo {}", repo_url);
            repo
        },
        Err(e) => {
            panic!("Error in cloning repo: {}", e)
        }
    };

    match repo.is_empty() {
        Ok(_) => repo,
        Err(_e) => {
            panic!("Repo is empty")
        },
    }
}

fn clone(url: &str, temp_directory: &str) -> Result<Repository, Error>
{
    // Remove all of the previous repo
    match fs::read_dir(temp_directory) {
        Ok(dir) => {
            match dir.count() {
                0 => {},
                _ => {
                    match fs::remove_dir_all(temp_directory) {
                        Err(_e) => {
                            panic!("Directory could not be emptied");
                        }
                        _ => {}
                    }
                }
            }
        },
        _ => {

        }
    }

    // Clone the repo into the
    Repository::clone(url, temp_directory)
}
