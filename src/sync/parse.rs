//! Parse `.configy`.

use crate::colors::{RED_FG, RESET};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

/// Parse the contents from `.configy` and feeds it to a HashMap of HashSets.
///
/// # Example
///
/// ```bash
/// # Syntax:
/// # <link-from> => <link-to>
/// # Altogether its called a link.
///
/// # The following examples are valid:
///
/// ../relative/path/to/file.rs => ../relative-two/path-two/to/file.rs
/// /absolute/path/to/folder => ../relative/path/to/folder
/// /absolute/path/to/folder => /absolute-two/path-two/to/folder
/// ../relative/path/to/file.rs => /absolute/path/to/file.rs
///
/// # Rules:
/// # 1. Only one "=>" is allowed per line.
/// # 2. Comments MUST start from the beginning of a line. This example is invalid:
/// ../a/b/c => /d/e/f # The parser will try to parse it as a valid link not a comment.
/// # 3. Empty lines are ignored just like comments.
/// # 4. "link-from" and "link-to" can't be empty. These examples are invalid:
/// /a/b/c =>
/// => ../d/e/f
/// =>
/// # 5. Links cannot span to multiple lines. It MUST be a single line.
/// # 6. Link are insensitive to leading and trailing white spaces.
/// ```
pub fn get_entries() -> HashMap<String, HashSet<String>> {
    // Nothing to do if this fails. So just exit.
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
        // Nothing to do if this fails. So just exit.
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

        // Ignore comments and empty lines.
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // "=>" is reserved for to-from distinction and are disallowed for paths.
        if line.matches("=>").count() != 1 {
            eprintln!(
                "{}[!] Only one \"=>\" is allowed per line and is reserved for to-from distinction.{}",
                RED_FG,
                RESET,
            );
            exit(1);
        }

        let contents = line.split("=>").collect::<Vec<&str>>();
        // contents.len() being = 2 is guaranteed, so direct access is safe.
        let (a, b) = (contents[0].trim(), contents[1].trim());

        // Values must not be empty.
        if a.is_empty() || b.is_empty() {
            continue;
        }

        insert(&mut entries, a, b);
    }

    entries
}

/// Inserts a value into a HashMap with HashSet values, allowing duplicates for each key.
fn insert(entries: &mut HashMap<String, HashSet<String>>, key: &str, value: &str) {
    let entry = entries.entry(key.into()).or_default();

    entry.insert(value.into());

    let value_count = entry.len();
    let new_values = (2..value_count).map(|i| format!("value{}", i));

    entry.extend(new_values);
}
