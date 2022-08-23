# Formatted print

Printing is handled by a series of macros defined in std::fmt some of which include:

- format!: write formatted text to String
- print!: same as format! but the text is printed to the console (io::stdout).
- println!: same as print! but a newline is appended.
- eprint!: same as print! but the text is printed to the standard error (io::stderr).
- eprintln!: same as eprint! but a newline is appended.

All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.