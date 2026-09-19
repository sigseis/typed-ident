# typed-ident

A Rust crate for type-safe identifier validation, inspection, and mutation.

> **Alpha Software:** The API is unstable and may contain breaking changes in
> any release. Do not depend on `typed-ident` for a stable public interface yet.

## Unicode Standards

The Unicode-based identifier profiles and boundary calculations follow:

* [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/)
  for identifier character profiles
* [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks)
  for identifier boundary calculation

## Example

The most common way to use this crate is through its
[predefined identifier presets](https://docs.rs/typed-ident/latest/typed_ident/presets/index.html).
You can choose the preset that matches the syntax you want to enforce.

```rust
use typed_ident::core::fmt::*;
use typed_ident::presets::unicode::*;

// Validate the format of input to see that it matches expectations.
let identifier = UpperCamelIdent::new("HTTPServer")?;

// Look at all of the segments of the identifier.
println!("identifier segments:");
for segment in identifier.segments() {
    println!("* {segment:?}"); // [Chunk("HTTP"), Chunk("Server")]
}

// Convert to a different displayable format.
let lower_snake = identifier.as_lower_snake();
println!("as lower-snake: {lower_snake}"); // "http_server"

// Create a decorated identifier.
let decorated = identifier.with_circumfix("__", "__")?;
println!("with decoration: {decorated}"); // "__HTTPServer__"
```

Once constructed, identifiers can be inspected, converted, and mutated through
their methods while preserving the type's validity guarantees.

If the provided presets do not match your requirements, you can
[define your own syntax](https://docs.rs/typed-ident/latest/typed_ident/syntax/index.html).

## When to Use

`typed-ident` is intended for values that represent programming identifiers and
must obey a defined identifier syntax.

Use it when you need to:

* Check that an input is a valid identifier
* Distinguish between different identifier casing conventions at the type level
* Inspect identifier boundaries, delimiters, or segments
* Convert between different identifier casing conventions while preserving
  identifier semantics
* Safely build or modify identifiers without accidentally producing invalid
  values

Typical use cases include:

* Programming language implementations
* Procedural macros and code generation
* Compiler, formatter, or linter tooling
* Schema, API, or serialization code that generates fields, types, or methods
* Refactoring and rename tools

For ordinary strings that only need case conversion, use a general-purpose crate
such as [`convert_case`](https://crates.io/crates/convert_case/).

For URLs, REST paths, filesystem paths, or other domain-specific syntax, use a
crate that understands that syntax instead.

## Changelog

See [CHANGELOG](https://github.com/sigseis/typed-ident/blob/main/CHANGELOG.md)
for release history.

## Contributing

We welcome contributions!

Please read the
[CONTRIBUTING](https://github.com/sigseis/typed-ident/blob/main/CONTRIBUTING.md)
guide before contributing.

## License

Licensed under either the
[Apache License, Version 2.0](https://github.com/sigseis/typed-ident/blob/main/LICENSE-APACHE)
or [MIT license](https://github.com/sigseis/typed-ident/blob/main/LICENSE-MIT),
at your option.
