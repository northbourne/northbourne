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

#[macro_use]
extern crate rust_embed;

extern crate config;

extern crate yaml_rust;

mod error;
mod repo;
mod pm;
mod transaction;
mod action;
mod tree;

use tree::cartography::Cartography;

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
use config::{Config, File as ConfigFile};
use std::convert::TryInto;
use git2::Repository;
use yaml_rust::{YamlLoader, YamlEmitter};
use crate::action::{Actionable, Action};
use std::io::Read;
use yaml_rust::yaml::Array;
use crate::pm::homebrew::Homebrew;
use crate::pm::PackageManagerInterface;
use simplelog::{TermLogError, TermLogger};
use crate::error::Error::Generic;
use std::fs::File;
use std::borrow::Borrow;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(RustEmbed)]
#[folder = "conf/"]
struct DefaultConfig;

struct GodObject {
    repo: Repository,
    cartography_current: Cartography,
    cartography_incoming: Cartography,
    action_list: Vec<Action>
}
///
/// Program Wrapper
///
fn main() {
    match program() {
        Ok(()) => process::exit(0),
        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1);
        }
    }
}

fn start_logging() -> Result<()> {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed)
        .map_err(|logger_error| -> crate::error::Error {
            Generic
        })
}

fn load_config_from_embed(settings: &mut Config, default_file: &str) -> Result<Config> {
    let default_config = DefaultConfig::get(default_file).unwrap();

    info!("{:?}", default_config.as_ref());

    settings.merge(ConfigFile::from_str(std::str::from_utf8(default_config.as_ref()).unwrap(), config::FileFormat::Yaml))
        .map_err(|logger_error| -> crate::error::Error {
            Generic
        });

    Ok(settings.clone())
}

//
#[cfg(feature = "yaml")]
fn program() -> Result<()> {
    // Initialise Logging
    let simple_logger = simplelog::TermLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default(), simplelog::TerminalMode::Mixed);

    info!("Starting Northbourne");

    // Load up
    let mut settings = load_config_from_embed(&mut config::Config::default(), "default.yml").expect("Could not unwrap config file");
    info!("{:?}", settings.get_str("config"));


    // CLI
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
    println!("{:?}", matches.value_of("repo_url"));
    match matches.value_of("repo_url") {
        Some("") => return Err(Error::Generic),
        Some(repo) => {
            settings.set("repo_url", repo);
        }
        _ => return Err(Error::Generic)
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
// Ensure Method

fn ensure(settings: &Config) -> Result<()> {
    let mut repo_handler = GitRepo::new();

    repo_handler.set_repo_url(settings.get_str("repo_url").unwrap());
    repo_handler.set_repo_directory(settings.get_str("repo_directory").unwrap());

    repo_handler.discover()
        .and_then(|repository| -> Result<GodObject> {
            info!("Found Existing Repository: {:?}", repository.path());
            
            Ok(GodObject { 
                repo: repository,
                cartography_current: Cartography {},
                cartography_incoming: Cartography {},
                action_list: vec![]
            })
        })
        .and_then(|mut god: GodObject| -> Result<Repository> {
            info!("Fetching updates from Repository");



            // Sync
            repo_handler.sync().and_then(|repository| -> Result<Repository>{
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
                repo_handler.clone().and_then(|repository| -> Result<Repository> {
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
