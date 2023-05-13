//! Parse `.configy`

use crate::colors::{RED_FG, RESET};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

/// Parse the contents from `.configy` and feeds it to a HashMap of HashSets
///
/// # Example
///
/// ```bash
/// # NOTE: The folder in which the configy executable was executed is
/// # considered as the root for relative file resolution
/// # The following examples are parsed correctly:
/// ../relative/path/to/file.rs => ../relative2/path2/to2/file.rs
/// /absolute/path/to/folder => ../relative/path/to/folder
/// /absolute/path/to/folder => /absolute2/path2/to2/folder
/// ../relative/path/to/file.rs => /absolute/path/to/file.rs
/// # The following examples are not parsed and ignored:
/// # Comments are ignored as well as empty lines
///
/// => /some/path
/// /some/path =>
/// # Only one "=>" is allowed per line
/// /some/path => ../some2/path2 => ../other
/// # Comments MUST start from the beginning of a line
/// /some/path => /some2/path2 # This is not invalid
/// # Here, the comment part is going to treated as part of the 2nd link
/// # Instead, comments should be placed on top of the links, like this:
/// # This is valid
/// /some/path => /some2/path2
/// ```
pub fn get_entries() -> HashMap<String, HashSet<String>> {
    let configy = File::open(".configy").unwrap_or_else(|err| {
        eprintln!(
            "{}[!] Failed to read \".configy\"\n==> {}{}",
            RED_FG, err, RESET,
        );
        exit(1);
    });
    let lines = BufReader::new(configy).lines();
    let mut entries: HashMap<String, HashSet<String>> = HashMap::new();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap_or_else(|err| {
            eprintln!(
                "{}[!] Failed while reading line no. {} in \".configy\"\n==> {}{}",
                RED_FG,
                i + 1,
                err,
                RESET,
            );
            exit(1);
        });
        let line = line.trim();

        // Only one "=>" should exist per line. Ignore comments and empty lines
        if line.is_empty() || line.starts_with("#") || line.matches("=>").count() != 1 {
            continue;
        }

        let contents = line.split("=>").collect::<Vec<&str>>();
        // contents.len() being = 2 is guarenteed, so direct access is safe
        let (a, b) = (contents[0].trim(), contents[1].trim());

        // Values must not be empty
        if a.is_empty() || b.is_empty() {
            continue;
        }

        insert(&mut entries, a, b);
    }

    entries
}

/// Inserts a value into a HashMap with HashSet values, allowing duplicates for each key.
fn insert(entries: &mut HashMap<String, HashSet<String>>, key: &str, value: &str) {
    let entry = entries.entry(key.into()).or_insert_with(HashSet::new);
    entry.insert(value.into());

    let value_count = entry.len();
    let new_values = (2..value_count).map(|i| format!("value{}", i));
    entry.extend(new_values);
}
