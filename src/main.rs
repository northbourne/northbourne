#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate config;
extern crate simplelog;

mod pm;
mod repo;
mod error;

use std::process;
use std::{fs};
use std::str;
use repo::Repo;
use std::process::Output;
use pm::PackageManagerType::Homebrew as HB;
use pm::homebrew::Homebrew;
use crate::pm::PackageManager;
use crate::repo::GitRepo;
use simplelog::*;
use std::fmt::{Debug, Formatter, Error, Write};


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
//    use clap::App;
//
//    let yaml = load_yaml!("../cli.yml");
//    let matches = App::from_yaml(yaml).get_matches();
//
//    let mut settings = config::Config::default();
//
//    settings
//        .merge(config::File::with_name("conf/default.yml"))
//        .unwrap();
//
//    // Config
//    match matches.value_of("config") {
//        Some(config_path) => {
//            settings
//                .merge(config::File::with_name(config_path))
//                .unwrap();
//        }
//        _ => {}
//    }
//
//    // Log Level
//    let log_level = match settings.get_str("log_level").unwrap().as_str() {
//        "debug" => LevelFilter::Debug,
//        "info" => LevelFilter::Info,
//        "warn" => LevelFilter::Warn,
//        "error" => LevelFilter::Error,
//        _ => { LevelFilter::Off }
//    };
//
//    let simple_logger = SimpleLogger::init(log_level, Config::default());
//
//    // Repo
//    match matches.value_of("repo_url") {
//        Some(repo) => {
//            settings.set("repo_url", repo);
//        }
//        _ => {}
//    }
//
//    if settings.get_str("repo_url").unwrap() == "" {
//        // return Err("No code specified")
//    }
//
//
//    // ------------
//    let pm = PackageManager {
//        provider: pm::homebrew::Homebrew::new(&settings)
//    };
//
//    let mut repo = GitRepo::new();
//    repo.set_repo_url(settings.get_str("repo_url").unwrap());
//    repo.set_repo_directory(settings.get_str("repo_directory").unwrap());
//
//    repo.sync();
//
//
//
//    repo::clone(
//        settings.get_str("repo").unwrap().as_str(),
//        settings.get_str("tmp_repo").unwrap().as_str(),
//    );

    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    // Instantiate a `Point`
    let point: Point = Point { x: 2.0, y: 2.0 };

    

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: 0.0, y: 0.0 },
        p2: point,
    };

    // #[allow(unsafe)]
    impl Rectangle {
        pub fn area(&self) -> f32 {
            let x_length: f32 = (self.p1.x - self.p2.x).abs();
            let y_length: f32 = (self.p1.y - self.p2.y).abs();
            x_length * y_length
        }
    }

    println!("{:}", _rectangle.area());

    Ok(1)
}
