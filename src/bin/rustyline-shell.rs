extern crate rustyline;
extern crate unnamed_shell;

use unnamed_shell::parse::parse;
use unnamed_shell::prompt::get_prompt;

use std::io::{stderr, Write};
use rustyline::Editor;
use rustyline::completion;

fn main() {
    let completer = completion::FilenameCompleter::default();
    let mut editor = Editor::default();
    editor.set_completer(Some(&completer));

    let mut exit_val: i32 = 0;

    loop {
        let prompt = get_prompt(exit_val);

        let line_result = editor.readline(&prompt);
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
