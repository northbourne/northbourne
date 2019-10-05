//! # Northbourne
//! Testing Example Documentation
//!
//! ```rust
//! Testing
//! ```

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate config;
extern crate yaml_rust;


mod error;
mod repo;
mod pm;
mod transaction;
mod action;

use crate::error::Error;
use crate::repo::Repo;
use crate::repo::git::GitRepo;
use std::{mem, process};
use std::fs;
use std::process::Output;
use std::str;
use std::thread;
use std::time::Duration;
use clap::App;
use clokwerk::{Scheduler, TimeUnits};
use clokwerk::Interval::*;
use config::Config;
use std::convert::TryInto;
use git2::Repository;
use yaml_rust::{YamlLoader, YamlEmitter};
use crate::action::{Actionable, Action};
use std::fs::File;
use std::io::Read;
use yaml_rust::yaml::Array;
use crate::pm::homebrew::Homebrew;
use crate::pm::PackageManagerInterface;

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    match program() {
        Ok(()) => process::exit(0),
        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1);
        }
    }
}

#[cfg(feature = "yaml")]
fn program() -> Result<()> {
    // Initialise Logging
    let simple_logger = simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed);

    info!("Starting Northbourne");

    // Load up
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("conf/default.yml")).expect("Could not open ");

    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // -c Configuration File
    match matches.value_of("config") {
        Some(config_path) => {
            info!("Loading Configuration: {}", config_path);
            settings
                .merge(config::File::with_name(config_path))
                .unwrap();
        }
        _ => {}
    }

    // Log Level
    let log_level = match settings.get_str("log_level").unwrap().as_str() {
        "debug" => simplelog::LevelFilter::Debug,
        "info" => simplelog::LevelFilter::Info,
        "warn" => simplelog::LevelFilter::Warn,
        "error" => simplelog::LevelFilter::Error,
        _ => { simplelog::LevelFilter::Off }
    };

    let simple_logger = simplelog::TermLogger::init(log_level, simplelog::Config::default(), simplelog::TerminalMode::Mixed);

    // ////////////// -------- ///////////////

    // Repo
    match matches.value_of("repo_url") {
        Some(repo) => {
            settings.set("repo_url", repo);
        }
        _ => {}
    }

    if settings.get_str("repo_url").unwrap() == "" {
        return Err(Error::Generic);
    }

    let every: u32 = settings
        .get_int("every")      .expect("Could not find default value for \"every\".")
        .try_into()                 .expect("Could not convert \"every\" value to u32");
    let mut scheduler = Scheduler::new();
    scheduler.every(every.seconds()).run(move || {
        ensure(&mut settings).unwrap()
    });

    loop {
        scheduler.run_pending();
    }

    Ok(())
}

fn ensure(settings: &Config) -> Result<()> {
    let mut repo: GitRepo = GitRepo::new();

    repo.set_repo_url(settings.get_str("repo_url").unwrap());
    repo.set_repo_directory(settings.get_str("repo_directory").unwrap());

    repo.discover()
        .and_then(|repository| -> Result<Repository> {
            info!("Found Existing Repository: {:?}", repository.path());
            info!("Fetching updates from Repository");

            // Sync
            repo.sync().and_then(|repository| -> Result<Repository>{
                info!("Successfully pulled down new changes");
                Ok(repository)
            }).or_else(|e| {
                error!("{:}", e);
                error!("Couldn't pull down new changes");
                Err(e)
            })
        })
        .or_else(|e| match e {
            Error::NotFound => {
                repo.clone().and_then(|repository| -> Result<Repository> {
                    Ok(repository)
                })
            },
            _ => {
                Err(Error::Generic)
            }
        })
        .and_then(|repository| -> Result<Action> {
            let mut file = File::open("/tmp/northbourne/repo/north.yml").expect("Unable to open file");
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("Unable to read file");


            let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
            let doc = &docs[0]; // select the first document
            let packages = doc["global"]["packages"].as_vec();

            match packages {
                None => {},
                Some(packages) => {
                    for package in packages {
                        (Homebrew::new(settings)).check(package.as_str().unwrap()).and_then(|is_installed| -> Result<bool> {
                            match is_installed {
                                true => {
                                    info!("{} is already installed, skipping.", package.as_str().unwrap());
                                    Ok(true)
                                },
                                false => {
                                    info!("Installing {}", package.as_str().unwrap());
                                    (Homebrew::new(settings)).install(package.as_str().unwrap())
                                }
                            }
                        });

                    }

                },
            }
            Ok(Action{})
        });



    // repo.clone();

    Ok(())
}
