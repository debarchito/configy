//! Sync logic.

use std::fs::{remove_dir_all, remove_file};
use std::os::unix::fs::symlink as unix_symlink;
use std::path::PathBuf;
mod parse;
mod resolve;

/// Initialize sync.
pub fn init(force: bool) {
  let entries = parse::get_entries();

  for entry in entries {
    for value in entry.1 {
      let src = match resolve::resolve(&entry.0) {
        Ok(src) => src,
        Err(resolve::ResolveError::HomeDirError(err)) => {
          msg_exit!("<r>[!] Failed to resolve home directory\n==> <w>{err}</rs>")
        }
        Err(resolve::ResolveError::CurrentDirError(err)) => {
          msg_exit!("<r>[!] Failed to resolve current directory\n==> <w>{err}</rs>")
        }
      };

      if !src.exists() {
        msg!(
          "<b>[?] Skipping because the following src doesn't exist: <w>{}</rs>",
          entry.0
        );
        continue;
      }

      let dest = match resolve::resolve(&value) {
        Ok(src) => src,
        Err(resolve::ResolveError::HomeDirError(err)) => {
          msg_exit!("<r>[!] Failed to resolve home directory\n==> <w>{err}</rs>")
        }
        Err(resolve::ResolveError::CurrentDirError(err)) => {
          msg_exit!("<r>[!] Failed to resolve current directory\n==> <w>{err}</rs>")
        }
      };

      symlink(src, dest, force);
    }
  }
}

/// Symlink the src and dest.
fn symlink(src: PathBuf, dest: PathBuf, force: bool) {
  if clean_if_forced(&dest, force) {
    return;
  }

  match unix_symlink(&src, &dest) {
    Ok(_) => msg!("<g>[+] Created symbolic link successfully: <w>{src:?}<g> <==> <w>{dest:?}</rs>"),
    Err(err) => msg_exit!(
      "<r>[!] Failed to create symbolic link: <w>{src:?}<r> <==> <w>{dest:?}\n==></rs> {err}"
    ),
  };
}

/// Cleans the destination path by removing the file or directory if force is true.
/// Returns boolean based on the decision to let go or obstruct the control flow.
fn clean_if_forced(dest: &PathBuf, force: bool) -> bool {
  // If the destination doesn't exist, do not obstruct the control flow.
  if !dest.exists() {
    return false;
  }

  // If the destination exists but force is false, obstruct the control flow and print the message.
  if !force {
    msg!("<b>[?] The following path already exists: <w>{dest:?}\n<b>==> Use \"forcesync\" instead of \"sync\" to overwrite</rs>");
    return true;
  }

  if dest.is_file() {
    match remove_file(dest) {
      // Delete the file and do not obstruct the control flow.
      Ok(_) => return false,
      Err(err) => msg_exit!("<r>[!] Failed to remove file: <w>{dest:?}\n<r>==> {err}</rs>"),
    }
  }

  match remove_dir_all(dest) {
    // Delete the directory and everything inside it. Do not obstruct the control flow.
    Ok(_) => false,
    Err(err) => msg_exit!("<r>[!] Failed to remove directory: <w>{dest:?}\n<r>==> {err}</rs>"),
  }
}
