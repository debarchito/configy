//! Path resolution.

use std::env;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
/// Possible errors returned during path resolution.
pub enum ResolveError {
  HomeDirError(env::VarError),
  CurrentDirError(io::Error),
}

impl fmt::Display for ResolveError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl std::error::Error for ResolveError {}

/// Resolve relative path (as string) to absolute path.
pub fn resolve(path: &str) -> Result<PathBuf, ResolveError> {
  let home_dir = env::var("HOME").map_err(ResolveError::HomeDirError)?;
  let path = path.trim().trim_end_matches('/');

  if path == "~" {
    return Ok(PathBuf::from(home_dir));
  }

  let path_buf = if path.starts_with('~') {
    let stripped_path = path.strip_prefix("~/").unwrap_or(path);
    PathBuf::from(home_dir).join(stripped_path)
  } else {
    PathBuf::from(path)
  };

  if path_buf.is_absolute() {
    Ok(path_buf)
  } else {
    let current_dir = env::current_dir().map_err(ResolveError::CurrentDirError)?;
    Ok(current_dir.join(path_buf))
  }
}
