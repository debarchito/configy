#![macro_use]

//! `ANSI` color escape code(s) and macro(s) to make colored messages easy to write. <br>
//! `NOTE:` This is known to not work in `Windows Command Prompt`.
//! Users are advised to make use of `Ansicon` (https://github.com/adoxa/ansicon).

#[macro_export]
/// A helper macro to make colored messages easy to write.
macro_rules! msg {
  ($($arg:tt)*) => {{
    let msg = format!($($arg)*);
    let msg = msg
      .replace("<r>", "\x1b[31m")    // Red
      .replace("<g>", "\x1b[32m")    // Green
      .replace("<b>", "\x1b[34m")    // Blue
      .replace("<w>", "\x1b[37m")    // White
      .replace("</rs>", "\x1b[0m");  // Reset
    eprintln!("{}", msg);
  }};
}

#[macro_export]
/// `msg!()` but also exits the process with code 1.
macro_rules! msg_exit {
  ($($arg:tt)*) => {{
     msg!($($arg)*);
     std::process::exit(1);
  }};
}
