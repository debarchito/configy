# configy

A simple (zero-dependency) file and directory syncing utility.

## Build

Be sure to have [mold](https://github.com/rui314/mold) installed. If you don't want to use `mold`, edit the rustflags in [.cargo/config.toml](/.cargo/config.toml) to use a linker of your choice. Once done, you can just run:

```bash
cargo build --release
```

## How to use?

1. Create a `.configy` file in the root of your project.
2. The syntax is very simple. Here is all you need to know:

```sh
# Syntax:
# <link-from> => <link-to>
# Altogether its called a link.

# The following examples are valid:

../relative/path/to/file.rs => ../relative-two/path-two/to/file.rs
/absolute/path/to/directory => ../relative/path/to/directory
/absolute/path/to/directory => /absolute-two/path-two/to/directory
../relative/path/to/file.rs => /absolute/path/to/file.rs

# Rules:
# 1. Only one "=>" is allowed per line.
# 2. Comments MUST start from the beginning of a line. This example is invalid:
../a/b/c => /d/e/f # The parser will try to parse it as a valid link not a comment.
# 3. Empty lines are ignored just like comments.
# 4. "link-from" and "link-to" can't be empty. These examples are invalid:
/a/b/c =>
=> ../d/e/f
=>
# 5. Links cannot span to multiple lines. It MUST be a single line.
# 6. Link are insensitive to leading and trailing white spaces.
```

3. After you have added all you desired links in `.configy`, just run `configy sync`. If the destination already exists, it will not overwrite by default. If you want to overwrite, use `config forcesync` (or `fsync`).

## License

[MIT](/LICENSE)
