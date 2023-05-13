# configy
A simple (zero-dependency) file and folder syncing (local) utility (created specially to manage my .files and configuration in general)

## Build

```bash
cargo build
```

## How to use?

1. Create a `.configy` file in the root of your project.
2. The syntax is very simple. Here is all you need to know ->
```bash
# NOTE: The folder in which the configy executable was executed is
# considered as the root for relative file resolution
# The following examples are parsed correctly:
../relative/path/to/file.rs => ../relative2/path2/to2/file.rs
/absolute/path/to/folder => ../relative/path/to/folder
/absolute/path/to/folder => /absolute2/path2/to2/folder
../relative/path/to/file.rs => /absolute/path/to/file.rs
# The following examples are not parsed and ignored:
# Comments are ignored as well as empty lines

=> /some/path
/some/path =>
# Only one "=>" is allowed per line
/some/path => ../some2/path2 => ../other
# Comments MUST start from the beginning of a line
/some/path => /some2/path2 # This is not invalid
# Here, the comment part is going to treated as part of the 2nd link
# Instead, comments should be placed on top of the links, like this:
# This is valid
/some/path => /some2/path2
```
3. After you have added all you desired links in `.configy`, just run `configy sync` and you are done! If the destination already exists, it will not overwrite it and issue info to console. If you want to overwrite, use `config forcesync` (or `fsync`).
