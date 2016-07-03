use std::io::{stderr, Write};
use std::process::Command;

pub fn run_external_command(line: &[&str]) -> i32 {
    let command = &line[0];
    let arguments = &line[1..];
    match Command::new(command).args(arguments).spawn() {
        Ok(mut child) => {
            return child.wait().unwrap().code().unwrap()
        },
        Err(e) => {
            let _ = writeln!(stderr(), "Error: {}", e);
            return -1
        },
    }
    0
}
