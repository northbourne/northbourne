#[macro_use]
extern crate clap;

use std::{fs, io};
use git2::{Repository, Error};

///
/// Northbourne
///
#[cfg(feature = "yaml")]
fn main() {
    use clap::App;
//
//    let yaml = load_yaml!("../cli.yml");
//    let matches = App::from_yaml(yaml).get_matches();
//
//    println!("{:?}", matches);
//
//    let config = matches.value_of("config").unwrap_or("default.conf");
//    println!("Value for config: {}", config);
//
//    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//    // required we could have used an 'if let' to conditionally get the value)
//    println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//
//    // Vary the output based on how many times the user used the "verbose" flag
//    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//    match matches.occurrences_of("v") {
//        0 => println!("No verbose info"),
//        1 => println!("Some verbose info"),
//        2 => println!("Tons of verbose info"),
//        3 | _ => println!("Don't be crazy"),
//    }
//
//    // You can handle information about subcommands by requesting their matches by name
//    // (as below), requesting just the name used, or both at the same time
//    if let Some(matches) = matches.subcommand_matches("test") {
//        if matches.is_present("debug") {
//            println!("Printing debug info...");
//        } else {
//            println!("Printing normally...");
//        }
//    }

    let repo_url =  "https://github.com/sifex/northbourne-test-repo";

    println!("Cloning repo {}", repo_url);

    let repo: Repository = match clone_dir(repo_url, "/tmp/repo") {
        Err(_e) => {
            panic!("Error in cloning repo")
        },
        Ok(repo) => {
            println!("Cloned repo {}", repo_url);
            repo
        }
    };

    match repo.is_empty() {
        Ok(_) => {},
        Err(_e) => {
            panic!("Repo is empty")
        },
    }
}


fn clone_dir(url: &str, temp_directory: &str) -> Result<Repository, Error>
{
    // Remove all of the previous repo
    match fs::remove_dir_all(temp_directory) {
        Err(_e) => {
            panic!("Directory could not be emptied")
        }
        _ => {}
    }

    // Clone the repo into the
    Repository::clone(url, temp_directory)
}
