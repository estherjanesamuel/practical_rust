# Formatting

We've seen that formatting is specified via a format string.

- format!("{}", foo) -> "3735.."
- format!("0x{:X}", foo) -> "0xDEAD.."
- format!("0o{:o}", foo) -> "0o3363..."

these same variable (foo) can be formatted differently depending on which argument type is used: x vs o vs unspecified.

This formatting functionallity is implemented via traits, and there is one trait for each argument type. the most common formatting trait is Display, which handles cases where the argument type is left unspecified: {} for instance. see main.rs
