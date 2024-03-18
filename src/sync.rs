//! Sync logic.

use std::fs::{remove_dir_all, remove_file};
use std::os::unix::fs::symlink as unix_symlink;
use std::path::Path;
mod parse;

/// Initialize sync.
pub fn init(force: bool) {
  let entries = parse::get_entries();

  for entry in entries {
    for value in entry.1 {
      let (src, dest) = (Path::new(&entry.0), Path::new(&value));

      if !src.exists() {
        msg!(
          "<b>[?] The following path doesn't exist: <w>{}</rs>",
          entry.0
        );
        continue;
      }

      symlink(src, dest, force);
    }
  }
}

/// Symlink the src and dest.
fn symlink(src: &Path, dest: &Path, force: bool) {
  if clean(dest, force) {
    return;
  }

  if let Err(err) = unix_symlink(src, dest) {
    msg_exit!(
      "<r>[!] Failed to create symbolic link: <w>{src:?}<r> <==> <w>{dest:?}\n==></rs> {err}"
    );
  };

  msg!("<g>[+] Created symbolic link successfully: <w>{src:?}<g> <==> <w>{dest:?}</rs>");
}

/// Cleans the destination path by removing the file or directory if it exists.
fn clean(dest: &Path, force: bool) -> bool {
  if !dest.exists() {
    return false;
  }

  if !force {
    msg!("<b>[?] The following path already exists: <w>{dest:?}\n<b>==> Use \"forcesync\" instead of \"sync\" to overwrite</rs>");
    return true;
  }

  if dest.is_file() {
    match remove_file(dest) {
      Ok(_) => return false,
      Err(err) => msg_exit!("<r>[!] Failed to remove file: <w>{dest:?}\n<r>==> {err}</rs>"),
    }
  }

  match remove_dir_all(dest) {
    Ok(_) => false,
    Err(err) => msg_exit!("<r>[!] Failed to remove directory: <w>{dest:?}\n<r>==> {err}</rs>"),
  }
}
