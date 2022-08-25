# Debug

All types which want to use `std::fmt` formatting `traits` require an implementation to be printable. Automatic implementations are only provided for types such as in the `std` library. All other must be maunally implemented somehow.

The `fmt::Debug` trait makes this very straightforward. ALl types can derive (automatically create) the fmt::Debug implementation. This is not true for fmt::Display which must be manually implemented.

// This structure cannot be printed either with `fmt::Display` or with `fmt::Debug`
```rust  
struct UnPrintable(i32);
```

// The `derive` attribut automatically creates the implematations 
// required to make this `struct` printable with `fmt:Debug`

```rust  
#[derive(Debug)]
struct DEbugPrintable(i32);
```