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
relative/folder => ../realtive2/folder2
relative/file.rs => ../relative2/file2.rs
# This is a comment and is ignored. Empty lines are also going to be ignored
/absolute/file/path/file.rs => ../relative3/file3.rs
# To note, on top "relative" is calculated from the folder where the executable is launched
# Mixture of relative and absolute is allowed. Glob support doesn't exist yet
# Relative paths are all converted to absolute cause symlinks in Windows don't support relative paths
# The parser is VERY simple. No syntax errors, stuff that don't line up are just ignored. for e.g.
=> 56
1 =>
/djdjd => ../eieie => ...
# Only one "=>" is allowed per line
```
3. After you have added all you desired links in `.configy`, just run `configy sync` and you are done! Currently, on Windows, re-trying to link already linkeds files and folders cause a panic, but this will be fixed in future commits cause all the code so far have been an evening's work. It currently works for the sake of working. The code quality is questionable but everything will improve. Also, configy needs to run in admin mode in windows to use symlinks cause that's how windows does it.
