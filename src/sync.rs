//! Sync logic.

use std::{
    fs::{remove_dir_all, remove_file},
    path::{Path, PathBuf},
};
mod parse;
use crate::colors::{BLUE_FG, GREEN_FG, RED_FG, RESET, WHITE_FG};
use std::process::exit;

/// Initialize sync.
pub fn init(force: bool) {
    let entries = parse::get_entries();

    for entry in entries {
        // 2 entries being valid is guaranteed.
        for value in entry.1 {
            let (src, dest) = (Path::new(&entry.0), Path::new(&value));

            if !src.exists() {
                eprintln!(
                    "{}[?] The following path doesn't exist: {}{}{}",
                    BLUE_FG, WHITE_FG, entry.0, RESET,
                );
                continue;
            }

            symlink(src, dest, force);
        }
    }
}

/// Symlink the src and dest.
fn symlink(src: &Path, dest: &Path, force: bool) {
    #[cfg(unix)]
    {
        if clean(&PathBuf::from(dest), force) {
            return;
        }

        std::os::unix::fs::symlink(src, dest).unwrap_or_else(|err| {
            eprintln!(
                "{}[!] Failed to create symbolic link: {:?} <==> {:?}\n==> {}{}",
                RED_FG, src, dest, err, RESET
            );
            exit(1);
        });
    }

    #[cfg(windows)]
    {
        let cur_dir = std::env::current_dir().unwrap_or_else(|err| {
            eprintln!(
                "{}[!] Failed while trying to find the current folder\n==> {}{}",
                RED_FG, err, RESET
            );
            exit(1);
        });
        // Make sure relative paths are parsed correctly on Windows.
        let src = match src.is_relative() {
            true => cur_dir.join(src),
            _ => PathBuf::from(src),
        };
        let dest = match dest.is_relative() {
            true => cur_dir.join(dest),
            _ => PathBuf::from(dest),
        };

        if clean(&dest, force) {
            return;
        }

        match src.is_file() {
            true => std::os::windows::fs::symlink_file(&src, &dest).unwrap_or_else(|err| {
                eprintln!(
                    "{}[!] Failed to create symbolic link: {:?} <==> {:?}\n==> {}{}",
                    RED_FG, src, dest, err, RESET
                );
                exit(1);
            }),
            _ => std::os::windows::fs::symlink_dir(&src, &dest).unwrap_or_else(|err| {
                eprintln!(
                    "{}[!] Failed to create symbolic link: {:?} <==> {:?}\n==> {}{}",
                    RED_FG, src, dest, err, RESET
                );
                exit(1);
            }),
        }
    }

    println!(
        "{}[+] Created symbolic link successfully: {:?} <==> {:?}{}",
        GREEN_FG, src, dest, RESET,
    );
}

/// Cleans the destination path by removing the file or directory if it exists.
fn clean(dest: &PathBuf, force: bool) -> bool {
    if !dest.exists() {
        return false;
    }

    if !force {
        eprintln!(
            "{}[?] The following path already exists: {}{:?}\n{}==> Use \"forcesync\" instead of \"sync\" to overwrite{}",
                BLUE_FG, WHITE_FG, dest, BLUE_FG, RESET
            );
        return true;
    }

    match dest.is_file() {
        true => {
            remove_file(dest).unwrap_or_else(|err| {
                eprintln!(
                    "{}[!] Failed to remove file: {}{:?}\n{}==> {}{}",
                    RED_FG, WHITE_FG, dest, RED_FG, err, RESET
                );
                exit(1);
            });
            false
        }
        _ => {
            remove_dir_all(dest).unwrap_or_else(|err| {
                eprintln!(
                    "{}[!] Failed to remove folder: {}{:?}\n{}==> {}{}",
                    RED_FG, WHITE_FG, dest, RED_FG, err, RESET
                );
                exit(1);
            });
            false
        }
    }
}
