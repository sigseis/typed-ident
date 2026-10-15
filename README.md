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
use typed_ident::fmt::AsLowerSnake;
use typed_ident::presets::unicode::*;

// Validate the format of input to see that it matches expectations.
let upper_camel = UpperCamelIdent::new("HTTPServer")?;

// Inspect all of the delimiters and words of the identifier.
println!("identifier segments:");
for segment in upper_camel.segments() {
    println!("* {segment:?}"); // [Chunk("HTTP"), Chunk("Server")]
}

// Interpret as a different displayable format ("http_server").
// (This simply reformats for display purposes, no allocation.)
println!("as lower_snake: {}", upper_camel.as_lower_snake());

// Zero-cost cast to any format that represents a superset.
// (This does not do any work, so it's basically a type cast.)
let cased_camel: &CasedCamelIdent = upper_camel.cast();
let mixed_camel: &CamelIdent = upper_camel.cast();

// ---------------------------------------------------------------------
// Mutation functionality (requires "alloc" feature).
// ---------------------------------------------------------------------
use typed_ident::convert::ToLowerKebab;

// Convert to a different identifier preset format.
let lower_kebab: String = upper_camel.to_lower_kebab()?;
assert_eq!(lower_kebab, "http-server");

// The conversion result adapts to the target variable.
let lower_kebab: Box<LowerKebabIdent> = upper_camel.to_lower_kebab()?;
assert_eq!(lower_kebab, "http-server");

// Attach a new word to the end of an identifier.
let lower_kebab_joined = lower_kebab.join("context")?;
assert_eq!(lower_kebab_joined, "http-server-context");

// Decorate an identifier with some affixed strings.
let decorated = upper_camel.with_circumfix("__", "__")?;
assert_eq!(decorated, "__HTTPServer__");
```

Once constructed, identifiers can be inspected, converted, and mutated through
their methods while preserving the type's validity guarantees. See the
[full documentation](https://docs.rs/typed-ident) for more information.

If the provided presets do not match your requirements, you can
[define your own syntax](https://docs.rs/typed-ident/latest/typed_ident/syntax/index.html).

## When to Use

`typed-ident` is intended for use with programming identifiers that must obey a
defined syntax.

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

For ordinary string conversion, use a general-purpose crate such as
[`convert_case`](https://crates.io/crates/convert_case/).

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
