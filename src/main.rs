use std::env::args;
pub mod colors;
mod list;
pub mod sync;

const VERSION: &str = "0.1.0";

fn main() {
  let args: Vec<String> = args().collect();

  if args.len() == 1 {
    help();
    return;
  };

  match args[1].as_str() {
    "help" | "h" => help(),
    "version" | "v" => println!("configy {}", VERSION),
    "sync" | "s" => sync::init(false),
    "forcesync" | "fsync" | "fs" => sync::init(true),
    "list" | "l" => list::list_links(),
    sub_cmd => msg!("<r>[!] Not a valid subcommand: <w>{sub_cmd}</rs>"),
  }
}

/// Prints the help message with usage instructions and available subcommands.
fn help() {
  println!(
    "configy {VERSION}
Debarchito Nath <dev.debarchito.nath@protonmail.com>
A simple (zero-dependency) file and directory syncing utility

USAGE:
    configy [SUBCOMMAND]

SUBCOMMANDS:
    help, h                 Print this message
    version, v              Version info
    sync, s                 Read from \".configy\" and sync
    forcesync, fsync, fs    Read from \".configy\" and force sync it (overwrite allowed)
    list, l                 List all links available in \".configy\""
  );
}
