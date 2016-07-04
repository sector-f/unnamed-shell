extern crate shellexpand;

use command;
use builtins::*;

use std::env::VarError;
use std::io::{stderr, Write};
use self::shellexpand::LookupError;

pub fn parse(line: String, last_val: i32) -> i32 {
    let line_result = parse_to_arguments(line);
    let mut exit_val = 0;

    let line = match line_result {
        Ok(s) => s,
        Err(_) => { return -1 },
    };

    if line.is_empty() {
        return 0
    }

    exit_val = match &*line[0] {
        "help" => builtin_help(&line[1..]),
        "true" => builtin_true(),
        "false" => builtin_false(),
        "cd" => builtin_cd(&line[1..]),
        "exit" => builtin_exit(&line[1..], last_val),
        _ => command::run_external_command(&line),
    };

    exit_val
}

fn parse_to_arguments(line: String) -> Result<Vec<String>, LookupError<VarError>> {
    let line_vec = line.trim().split_whitespace().map(String::from).collect::<Vec<_>>();
    let mut return_vec = Vec::new();

    for item in line_vec {
        let expansion_result = shellexpand::full(&item);
        match expansion_result {
            Ok(line) => {
                return_vec.push(line.into_owned());
            },
            Err(e) => {
                let _ = writeln!(stderr(), "Error: {}", e);
                return Err(e)
            },
        }
    }
    Ok(return_vec)
}
