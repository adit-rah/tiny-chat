use crate::frontend::formatting::{colorize_info, colorize_error};

pub fn info(msg: &str) -> String {
    colorize_info(msg)
}

pub fn error(msg: &str) -> String {
    colorize_error(msg)
}
