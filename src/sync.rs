//! Sync logic

use std::{
    env::current_dir,
    fs::{remove_dir_all, remove_file},
    path::{Path, PathBuf},
};
mod parse;
use crate::colors::{BLUE_FG, GREEN_FG, RED_FG, RESET, WHITE_FG};
use std::process::exit;

/// Initialize sync
pub fn init(force: bool) {
    let entries = parse::get_entries();

    for entry in entries {
        // 2 entries being valid is guaranteed
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

/// Symlink the src and dest
fn symlink(src: &Path, dest: &Path, force: bool) {
    #[cfg(unix)]
    {
        if try_clean(&PathBuf::from(dest), force) { return; }

        std::os::unix::fs::symlink(&src, &dest).unwrap_or_else(|err| {
            eprintln!(
                "{}[!] Failed to create symbolic link: {:?} <==> {:?}\n==> {}{}",
                RED_FG, src, dest, err, RESET
            );
            exit(1);
        });
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::{symlink_dir, symlink_file};

        let cur_dir = current_dir().unwrap_or_else(|err| {
            eprintln!(
                "{}[!] Failed while trying to find the current folder\n==> {}{}",
                RED_FG, err, RESET
            );
            exit(1);
        });
        let src = if src.is_relative() {
            cur_dir.join(src)
        } else {
            PathBuf::from(src)
        };
        let dest = if dest.is_relative() {
            cur_dir.join(dest)
        } else {
            PathBuf::from(dest)
        };

        if try_clean(&dest, force) { return; }

        match src.is_file() {
            true => symlink_file(&src, &dest).unwrap_or_else(|err| {
                eprintln!(
                    "{}[!] Failed to create symbolic link: {:?} <==> {:?}\n==> {}{}",
                    RED_FG, src, dest, err, RESET
                );
                exit(1);
            }),
            _ => symlink_dir(&src, &dest).unwrap_or_else(|err| {
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
fn try_clean(dest: &PathBuf, force: bool) -> bool {
    if dest.exists() {
        if force {
            match dest.is_file() {
                true => remove_file(&dest).unwrap_or_else(|err| {
                    eprintln!(
                        "{}[!] Failed to remove file: {}{:?}\n{}==> {}{}",
                        RED_FG, WHITE_FG, dest, RED_FG, err, RESET
                    );
                    exit(1);
                }),
                _ => remove_dir_all(&dest).unwrap_or_else(|err| {
                    eprintln!(
                        "{}[!] Failed to remove folder: {}{:?}\n{}==> {}{}",
                        RED_FG, WHITE_FG, dest, RED_FG, err, RESET
                    );
                    exit(1);
                }),
            };
            return false;
        } else {
            eprintln!(
                "{}[?] The following path already exists: {}{:?}\n{}==> Use \"forcesync\" instead of \"sync\" to overwrite{}",
                BLUE_FG, WHITE_FG, dest, BLUE_FG, RESET
            );
            return true;
        }
    }
    false
}
