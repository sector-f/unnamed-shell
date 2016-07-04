pub fn get_prompt(prev_exit_val: i32) -> String {
    match prev_exit_val {
        0 => format!("> "),
        _ => format!("({})> ", prev_exit_val),
    }
}
