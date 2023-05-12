//! Sync logic

use std::{
    env::current_dir,
    fs::metadata,
    path::{Path, PathBuf},
};
mod parse;
use crate::colors;

/// Initialize sync
pub fn init() {
    let entries = parse::get_entries();

    for entry in entries {
        // 2 entries being valid is guaranteed
        let (src, dest) = (Path::new(&entry.0), Path::new(&entry.1));

        if !src.exists() {
            eprintln!(
                "{}[!] The following path doesn't exist: {}{}{}",
                colors::RED_FG,
                colors::WHITE_FG,
                entry.0,
                colors::RESET
            );
            continue;
        }

        symlink(src, dest);
    }
}

fn symlink(src: &Path, dest: &Path) {
    #[cfg(unix)]
    {
        // TODO: Handle errors properly
        std::os::unix::fs::symlink(src, dest).unwrap();
    }

    #[cfg(windows)]
    {
        // TODO: Handle possible errors correctly
        let src_md = metadata(src).unwrap();
        let cur_dir = current_dir().unwrap();
        let src = if src.is_relative() {
            cur_dir.join(src).canonicalize().unwrap()
        } else {
            PathBuf::from(src)
        };
        let dest = if dest.is_relative() {
            cur_dir.join(dest).canonicalize().unwrap()
        } else {
            PathBuf::from(dest)
        };

        // TODO: Handle possible errors correctly
        match src_md.is_file() {
            true => std::os::windows::fs::symlink_file(src, dest).unwrap(),
            _ => std::os::windows::fs::symlink_dir(src, dest).unwrap(),
        }
    }

    // WARNING: Re-trying to link already linked files/folders WILL cause a panic!
    // TODO: Solve it

    println!(
        "{}[+] Symbolic link creation successfully: {:?} <-> {:?}{}",
        colors::GREEN_FG,
        src,
        dest,
        colors::RESET
    );
}
