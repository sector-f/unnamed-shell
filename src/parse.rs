use command;
use builtins::*;

pub fn parse(line: &str, last_val: i32) -> i32 {
    let line = parse_to_arguments(line);
    let mut exit_val = 0;

    match line[0] {
        "cd" => {
            exit_val = builtin_cd(&line[1..]);
        }
        "exit" => {
            exit_val = builtin_exit(&line[1..], last_val);
        },
        _ => {
            exit_val = command::run_external_command(&line);
        },
    }

    exit_val
}

fn parse_to_arguments(line: &str) -> Vec<&str> {
    line.trim().split_whitespace().collect::<Vec<&str>>()
}
