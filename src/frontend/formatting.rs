use colored::*;

pub fn colorize_info(msg: &str) -> String {
    format!("{}", msg.green())
}

pub fn colorize_error(msg: &str) -> String {
    format!("{}", msg.red())
}
