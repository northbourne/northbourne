#[macro_use] extern crate clap;
extern crate config;
#[macro_use]
extern crate log;

use std::{mem, process};
use std::fs;
use std::process::Output;
use std::str;

use simplelog::*;

use repo::Repo;

use crate::pm::PackageManagerInterface;
use crate::repo::GitRepo;

mod pm;
mod repo;
mod error;
mod cmd;

fn main() {
    match program() {
        Ok(0) => process::exit(0),
        Ok(_) => process::exit(1),
        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1);
        }
    }
}

#[cfg(feature = "yaml")]
pub fn program() -> Result<u64, &'static str> {
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

    // Log Level
    let log_level = match settings.get_str("log_level").unwrap().as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => { LevelFilter::Off }
    };

    let simple_logger = TermLogger::init(log_level, Config::default(), TerminalMode::Mixed);

    // Repo
    match matches.value_of("repo_url") {
        Some(repo) => {
            settings.set("repo_url", repo);
        }
        _ => {}
    }

    if settings.get_str("repo_url").unwrap() == "" {
        // return Err("No code specified")
    }


    let mut repo: GitRepo = Repo::new();
    repo.set_repo_url(settings.get_str("repo_url").unwrap());
    repo.set_repo_directory(settings.get_str("repo_directory").unwrap());

    repo.sync();


    repo::clone(
        settings.get_str("repo").unwrap().as_str(),
        settings.get_str("tmp_repo").unwrap().as_str(),
    );

    let pm = pm::homebrew::Homebrew::new(&settings);

    packages.iter().for_each(|package| -> () {
        match pm.check(package) {
            Ok(true) => {},
            Ok(false) => {
                pm.install(package);
            },
            Err(_) => {},
        };
    });

    Ok(1)
}
