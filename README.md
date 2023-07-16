# MultiGREP

Multigrep is an extension to the Rust project “minigrep”, outlined within the official Rust book. The goal of multigrep is to make searching for keywords across multiple files / directories faster and easier than the standard file explorer. Currently, the application runs via a CLI, so to start the program enter the directory via CMD / terminal, and run the file, adding an argument for the `needle`, or the key term you’re searching for:

```rust
cargo run -- {needle}
```

*You must have Cargo installed on your computer to run the file.

If the `needle` is found, it will print the line containing the `needle` to the standard output, as well as containing the file name(s) and line number(s) in which they occur.

## Current Features:

- Saving input arguments from CLI
- Reading openable files within a directory (not `DS_Store` or `.lock`)
    - Indicates which file is being opened / read from
    - Displays line containing the `needle` as well as it’s location in-file (line #)

## Future Implementations:

- [x]  Saving input arguments from command line
- [x]  Reading all openable files in a directory
    - [x]  Indicate which file is being opened / read from
    - [x]  Displaying line(s) containing the `needle` as well as line number
- [ ]  Enter into subdirectories from within parent directory
    - [ ]  Scan files within that directory for the same criteria
    - [ ]  (?) Ignore hidden folders
- [ ]  Create a GUI similar to file explorer, listing selectable files
    - [ ]  Allows for searching, displaying only selectable files containing the `needle`
- [ ]  Add the ability to read from more file types
- [ ]  Add environmental arguments for searching with or without case sensitivity