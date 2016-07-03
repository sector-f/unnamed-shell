extern crate copperline;

use copperline::{Copperline, Encoding};
use std::process::exit;

fn parse(line: &str) {
    match line {
        "exit" => exit(0),
        _ => {},
    }
}

fn main() {
    let mut copperline = Copperline::new();
    loop {
        let line_result = copperline.read_line("> ", Encoding::Utf8);
        if let Ok(s) = line_result {
            parse(&s);
        }
    }
}
