extern crate copperline;
extern crate unnamed_shell;

use copperline::{Copperline, Encoding};
use std::io::{stderr, Write};
use unnamed_shell::parse::parse;
use unnamed_shell::prompt::get_prompt;

fn main() {
    let mut exit_val: i32 = 0;
    let mut copperline = Copperline::new();

    loop {
        let prompt = get_prompt(exit_val);

        let line_result = copperline.read_line(&prompt, Encoding::Utf8);
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
