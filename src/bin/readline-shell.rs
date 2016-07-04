extern crate readline;
extern crate unnamed_shell;

use std::io::{stderr, Write};
use unnamed_shell::parse::parse;
use unnamed_shell::prompt::get_prompt;

fn main() {
    let mut exit_val: i32 = 0;

    loop {
        let prompt = get_prompt(exit_val);
        let line_result = readline::readline(&prompt);
        match line_result {
            Ok(s) => {
                exit_val = parse(s, exit_val);
            },
            Err(e) => {
                let _ = writeln!(stderr(), "Error: {}", e);
            },
        }
    }
}
