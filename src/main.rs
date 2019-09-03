#[macro_use]
extern crate clap;
extern crate config;

use std::process;

mod run;

fn main() {
    match run::program() {
        Ok(0) => process::exit(1),
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1);
        }
    }
}