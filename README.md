# typed-ident

A Rust crate for type-safe identifier validation, inspection, and mutation.

## Example

The main benefit of this crate is that it *enforces* the format of identifiers and checks that they are valid given a profile.

```rust
use typed_ident::alloc::convert::*;
use typed_ident::presets::unicode::*;

// Validate the format of input to see that it matches expectations.
let identifier = UpperCamelIdent::new("HTTPServer")?;

// Look at all of the atomic segments of the identifier.
println!("identifier segments:");
for segment in identifier.segments() {
    println!("* {segment:?}"); // [Chunk("HTTP"), Chunk("Server")]
}

// Convert to a different format.
let lower_snake = identifier.to_lower_snake_canonical();
println!("as lower-snake: {lower_snake}"); // "http_server"

// Decorate and mutate.
let decorated = identifier.with_circumfix_str("__", "__")?;
println!("with decoration: {decorated}"); // "__HTTPServer__"

// And more! :)
```

See the [api example commands](./examples/api/commands/) for more examples!

See the [documentation](https://docs.rs/typed_ident) for more information (we saved most of the important information for there - this is just repo information and a brief introduction in this file)!

## Contributing

We welcome contributions!

However, we'll ask that you read the [CONTRIBUTING](./CONTRIBUTING.md) guide first, before contributing.

## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.
