pub fn run_external_command(line: &[String]) -> i32 {
    use std::io::{ErrorKind, stderr, Write};
    use std::process::Command;

    let command = &line[0];
    let arguments = &line[1..];

    return match Command::new(command).args(arguments).spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => {
                    match status.code() {
                        Some(val) => val,
                        None => {
                            let _ = writeln!(stderr(), "Error: could not determine exit value");
                            -1
                        },
                    }
                },
                Err(e) => {
                    let _ = writeln!(stderr(), "Error: {}", e);
                    -1
                },
            }
        },
        Err(e) => {
            let _ = writeln!(stderr(), "Error: {}", e);
            match e.kind() {
                ErrorKind::PermissionDenied => 126,
                ErrorKind::NotFound => 127,
                _ => -1,
            }
        },
    };
}
