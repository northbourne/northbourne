use std::io;
use std::process::{Command, Output};

pub struct Run {}

struct RunResult {
    stdout: String,
    stderr: String,
    status_code: String,
}

const BASH: &str = "bash";

impl Run {
//    pub fn command(command: &str) -> io::Result<Output> {
//        Command::new(command)
//    }
}