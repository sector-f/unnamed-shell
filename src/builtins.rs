pub fn builtin_exit(line: &[&str], last_val: i32) -> i32 {
    use std::process::exit;

    if line.len() > 0 {
        match line[0].parse::<i32>() {
            Ok(i) => {
                exit(i);
            },
            Err(_) => {
                exit(last_val);
            },
        }
    } else {
        exit(last_val);
    }
}

pub fn builtin_cd(line: &[&str]) -> i32 {
    use std::io::{stderr, Write};
    use std::env;

    match line.len() {
        0 => {
            match env::home_dir() {
                Some(path) => {
                    if let Err(e) = env::set_current_dir(path) {
                        let _ = writeln!(stderr(), "cd: {}", e);
                        return -1
                    }
                },
                None => {
                    let _ = writeln!(stderr(), "cd: could not determine home directory",);
                    return -1
                }
            }
        },
        1 => {
            if let Err(e) = env::set_current_dir(line[0]) {
                let _ = writeln!(stderr(), "cd: {}", e);
                return -1
            }
        },
        _ => {
            let _ = writeln!(stderr(), "cd: too many arguments");
            return -1
        }
    }
    0
}
