# configy

A simple (zero-dependency) file and directory syncing utility.

## Build (Linux)

This project uses the [mold](https://github.com/rui314/mold) linker. In case you don't want to use `mold`, edit the `rustflags` in [.cargo/config.toml](/.cargo/config.toml) to use a linker of your choice. To build, run:

```bash
cargo build --release
```

## How to use?

1. Create a `.configy` file in the directory of your choice (e.g. a .dotfiles directory)
2. The syntax is rather simple. Here is all you need to know:

```sh
#> Syntax:

# <src> => <dest>
# Altogether its called a link.

#> Valid Examples:

../relative/path/to/file.rs => ../relative-two/path-two/to/file.rs
/absolute/path/to/directory => ../relative/path/to/directory
/absolute/path/to/directory => /absolute-two/path-two/to/directory
../relative/path/to/file.rs => /absolute/path/to/file.rs

#> Properties:

# 1. Only one "=>" is allowed per line.
# 2. Comments MUST start from the beginning of a new line. This example is invalid:
../a/b/c => /d/e/f # The parser will try to parse it as a valid link not a comment.
# 3. Empty lines are ignored just like comments.
# 4. `src` and `dest` can't be empty. These examples are invalid:
/a/b/c =>
=> ../d/e/f
=>
# 5. Links cannot span to multiple lines. It MUST be a single line.
# 6. Link are insensitive to leading and trailing white spaces.
```

3. After you have added all you desired links in `.configy`, just run `configy sync` (or just `s`) to create the symlinks. If the destination already exists, it will not overwrite by default. To overwrite, use `config forcesync` (or `fsync`, `fs`).

4. Use `configy list` (or just `l`) to list all the links in `.configy`.

## Example usage

I manage my [.dotfiles](https://github.com/debarchito/.dotfiles) using `configy`.

## License

[MIT](/LICENSE)
