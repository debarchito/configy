//! ANSI color escape codes.
//! NOTE: This is known to not work in Windows Command Prompt.
//! Users are advised to make use of Ansicon (https://github.com/adoxa/ansicon).

pub const RED_FG: &str = "\x1b[31m";
pub const GREEN_FG: &str = "\x1b[32m";
pub const BLUE_FG: &str = "\x1b[34m";
pub const WHITE_FG: &str = "\x1b[37m";
pub const RESET: &str = "\x1b[0m";
