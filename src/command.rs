pub fn run_external_command(line: &[String]) -> i32 {
    use std::io::{ErrorKind, stderr, Write};
    use std::process::Command;

    let command = &line[0];
    let arguments = &line[1..];
    match Command::new(command).args(arguments).spawn() {
        Ok(mut child) => {
            return child.wait().unwrap().code().unwrap()
        },
        Err(e) => {
            let _ = writeln!(stderr(), "Error: {}", e);
            match e.kind() {
                ErrorKind::PermissionDenied => {
                    return 126
                },
                ErrorKind::NotFound => {
                    return 127
                },
                _ => {
                    return -1
                },
            }
        },
    }
    0
}
