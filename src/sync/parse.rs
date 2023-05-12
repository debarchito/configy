//! Parse `.configy`

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Parse the contents from `.configy` and feed it to a HashMap
pub fn get_entries() -> HashMap<String, String> {
    // TODO: Handle possible errors correctly
    let configy = File::open(".configy").unwrap();
    let lines = BufReader::new(configy).lines();
    let mut entries: HashMap<String, String> = HashMap::new();

    for line in lines {
        // TODO: Handle possible errors correctly
        let line = line.unwrap();
        let line = line.trim();

        // Only one "=>" should exist per line. Ignore comments and empty lines
        if line.is_empty() || line.starts_with("#") || line.matches("=>").count() != 1 {
            continue;
        }

        let contents = line.split("=>").collect::<Vec<&str>>();
        // contents.len() being = 2 is guarenteed, so direct access is safe
        let (a, b) = (contents[0].trim(), contents[1].trim());

        // Both values must be non-empty
        if a.is_empty() || b.is_empty() {
            continue;
        }
        
        entries.insert(a.to_owned(), b.to_owned());
    }

    entries
}
