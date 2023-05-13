use std::env::args;
pub mod colors;
mod sync;

const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        help();
        return;
    };

    // args.len() being >= 2 is guaranteed, so direct access is safe
    match args[1].as_str() {
        "help" | "h" => help(),
        "version" | "v" => println!("configy {}", VERSION),
        "sync" | "s" => sync::init(false),
        "forcesync" | "fsync" | "fs" => sync::init(true),
        sub_cmd @ _ => eprintln!(
            "{}[!] Not a valid subcommand: {}{}{}",
            colors::RED_FG,
            colors::WHITE_FG,
            sub_cmd,
            colors::RESET
        ),
    }
}

/// Prints the help message with usage instructions and available subcommands.
fn help() {
    println!(
        "configy {VERSION}
Debarchito Nath <dev.debarchito.nath@protonmail.com>
A simple (zero-dependency) file and folder syncing (local) utility

USAGE:
    configy [SUBCOMMAND]

SUBCOMMANDS:
    help, h                 Print this message
    version, v              Version info
    sync, s                 Read from \".configy\" and sync
    forcesync, fsync, fs    Read from \".configy\" and force sync it (overwrite allowed)"
    );
}
