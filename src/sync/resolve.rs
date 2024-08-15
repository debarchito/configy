//! Path resolution.

use std::env;
use std::path::PathBuf;

/// Resolve relative path (as string) to absolute path.
pub fn resolve(path: &str) -> PathBuf {
  let home_dir = match env::var("HOME") {
    Ok(home_dir) => home_dir,
    Err(err) => msg_exit!(
      "<r>[!] Failed to resolve home directory for path: <w>{}\n<r>==> <w>{}</rs>",
      path,
      err
    ),
  };
  let path = path.trim_end_matches('/');

  if path == "~" {
    return PathBuf::from(home_dir);
  }

  let path_buf = if path.starts_with('~') {
    let stripped_path = path.strip_prefix("~/").unwrap_or(path);
    PathBuf::from(home_dir).join(stripped_path)
  } else {
    PathBuf::from(path)
  };

  if path_buf.is_absolute() {
    path_buf
  } else {
    let current_dir = match env::current_dir() {
      Ok(current_dir) => current_dir,
      Err(err) => msg_exit!(
        "<r>[!] Failed to resolve current directory for path: <w>{}\n<r>==> <w>{}</rs>",
        path,
        err
      ),
    };
    current_dir.join(path_buf)
  }
}
