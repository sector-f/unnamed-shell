extern crate readline;
extern crate unnamed_shell;

use std::io::{stderr, Write};
use unnamed_shell::parse::parse;

fn main() {
    let mut exit_val: i32 = 0;

    loop {
        let line_result = readline::readline(&format!("{}> ", exit_val));
        match line_result {
            Ok(s) => {
                exit_val = parse(&s, exit_val);
            },
            Err(e) => {
                let _ = writeln!(stderr(), "Error: {}", e);
            },
        }
    }
}
