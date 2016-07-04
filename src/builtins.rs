pub fn builtin_exit(line: &[String], last_val: i32) -> i32 {
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

pub fn builtin_cd(line: &[String]) -> i32 {
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
            if let Err(e) = env::set_current_dir(&line[0]) {
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

pub fn builtin_true() -> i32 {
    return 0
}

pub fn builtin_false() -> i32 {
    use rand::{Rng, thread_rng};
    let mut rng = thread_rng();
    return rng.gen_range(1,256) as i32
}

pub fn builtin_help(line: &[String]) -> i32 {
    use std::io::{stderr, Write};

    let returnval = match line.len() {
        0 => {
            println!(
"Run help [command] for more information.

The following builtin commands are available:
* help
* exit
* cd
* true
* false");
        0
        },
        1 => {
            match &*line[0] {
                "help" => help_help(),
                "exit" => help_exit(),
                "cd" => help_cd(),
                "true" => help_true(),
                "false" => help_false(),
                _ => {
                    let _ = writeln!(stderr(), "help: no help info found for ''{0}''. Try ''man {0}'' or ''info {0}''", &line[0]);
                    1
                },
            }
        }
        _ => {
            let _ = writeln!(stderr(), "help: wrong number of arguments given");
            1
        },
    };

    returnval
}

fn help_help() -> i32 {
    println!("help: help [command]");
    println!("  Displays help information about COMMAND.");
    0
}

fn help_exit() -> i32 {
    println!("exit: exit [n]");
    println!("  Exits the shell.");
    println!("  N is used as the shell's return value.");
    println!("  If N is not specified, the return value of the last command is used.");
    0
}

fn help_cd() -> i32 {
    println!("cd: cd [dir]");
    println!("  Changes the working directory to DIR.");
    println!("  If no DIR is specified, the HOME environment variable is used.");
    0
}

fn help_true() -> i32 {
    println!("true: true");
    println!("  Returns 0.");
    0
}

fn help_false() -> i32 {
    println!("false: false");
    println!("  Returns a random value between 1 and 255, inclusive.");
    0
}
