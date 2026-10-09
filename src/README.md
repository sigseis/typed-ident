# Getting Started

A Rust crate for type-safe identifier validation, inspection, and mutation.

Conforming to:

* [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/) for Unicode character profiles
* [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks) for boundary calculation

<div class="warning">

**NOTE:** This crate is currently in alpha. Until the crate enters beta, the crate has no stability promise and may contain breaking changes with every update.

</div>

## Basic Usage

The most common way to use this crate is through its predefined identifier presets. You can choose the preset that matches the syntax you want to enforce.

```rust
use typed_ident::fmt::AsLowerSnake;
use typed_ident::presets::unicode::UpperCamelIdent;

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

// Create a decorated identifier (requires "alloc" feature).
let decorated = identifier.with_circumfix("__", "__")?;
println!("with decoration: {decorated}"); // "__HTTPServer__"
# Ok::<(), typed_ident::Error>(())
```

If you know the general shape of the identifier, but not the casing, less-specific presets are also available.

```rust
use typed_ident::presets::unicode::CamelIdent;

// Examples of camel identifiers:
assert!(CamelIdent::new("iAm_123").is_ok());     // Delimiters         (allowed)
assert!(CamelIdent::new("iAm_lower").is_ok());   // Consistent Chunks  (allowed)
assert!(CamelIdent::new("iAm_Mixed").is_ok());   // Mixed Chunks       (allowed)
assert!(CamelIdent::new("UpperCamel").is_ok());  // Upper Camel        (allowed)

// Examples of non-camel identifiers:
assert!(CamelIdent::new("").is_err());           // Empty              (disallowed)
assert!(CamelIdent::new("2").is_err());          // Invalid Start      (disallowed)
assert!(CamelIdent::new("kebab-case").is_err()); // Hyphen Delimiter   (disallowed)
# Ok::<(), typed_ident::Error>(())
```

See the [`presets`] module documentation for a guided tour of different predefined identifier formats.

See the [`syntax`] module documentation if you want to build your own custom identifier formats.

## When To Use

`typed-ident` is intended for values that represent programming identifiers and must obey a defined identifier syntax.

Use it when you need to:

* Check that an input is a valid identifier
* Distinguish between different identifier casing conventions at the type level
* Inspect identifier boundaries, delimiters, or segments
* Convert between different identifier casing conventions while preserving identifier semantics
* Safely build or modify identifiers without accidentally producing invalid values

Typical use cases include:

* Programming language implementations
* Procedural macros and code generation
* Compiler, formatter, or linter tooling
* Schema, API, or serialization code that generates fields, types, or methods
* Refactoring and rename tools

For ordinary strings that only need case conversion, use a general-purpose crate such as [`convert_case`](https://crates.io/crates/convert_case/).

For URLs, REST paths, filesystem paths, or other domain-specific syntax, use a crate that understands that syntax instead.

## Core Types

The predefined presets are configurations of more general types provided by this crate.

| **Type**        | **Brief Description**                                                            |
|-----------------|----------------------------------------------------------------------------------|
| [`Ident`]       | An immutable, syntax-validated UTF-8 string slice representing an identifier.    |
| [`Fragment`]    | An immutable slice of an identifier, which itself may not be an identifier.      |
| [`FragmentBuf`] | A mutable, owned buffer for constructing or modifying fragments and identifiers. |
| [`Chunk`]       | An immutable slice of a [`Fragment`] which contains no delimiters.               |
| [`Segment`]     | A single chunk or delimiter from an [`Ident`] or [`Fragment`].                   |

> **HINT:** You almost always want to start with an `Ident`, usually through some predefined identifier [`presets`].

There is no `IdentBuf` type. This is because a mutable identifier is able to be made invalid by clearing the buffer, and enforcing the buffer is non-empty makes such a type difficult to use.

For an owned `Ident`, use `Box<Ident>` (these require enabling the "alloc" feature).

* [`Ident::to_boxed_ident`] to construct a boxed identifier from an immutable identifier reference
* [`Ident::new_boxed`] to fallibly construct a boxed identifier from a plain string buffer
* [`FragmentBuf::into_boxed_ident`] to fallibly construct a boxed identifier from a fragment buffer

[`Chunk`]: crate::Chunk
[`Fragment`]: crate::Fragment
[`FragmentBuf`]: crate::FragmentBuf
[`FragmentBuf::into_boxed_ident`]: crate::FragmentBuf::into_boxed_ident
[`Ident`]: crate::Ident
[`Ident::new_boxed`]: crate::Ident::new_boxed
[`Ident::to_boxed_ident`]: crate::Ident::to_boxed_ident
[`Segment`]: crate::Segment

# Crate Details

## Boundary Definition

Though the boundary implementation can be customized, this crate provides a configurable predefined implementation with defaults based on [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks).

This means that predefined identifiers will create the following segment boundaries.

* A **delimiter boundary** places a boundary before and after any delimiter defined by the identifier
* A **camel boundary** places a boundary after a lowercase or non-Greek titlecase letter, followed by an uppercase or titlecase letter
* A **hat boundary** places a boundary before an uppercase or titlecase letter followed by a lowercase letter, or before a non-Greek titlecase letter

> **NOTE:** UTS #55 distinguishes Greek and non-Greek titlecase characters when determining identifier chunks. Greek titlecase characters are treated as visually uppercase (`ᾈ`, `ᾨ`, `ῌ`, `ᾚ`, etc), while non-Greek titlecase characters are treated as beginning uppercase and continuing lowercase within the same scalar value (`ǅ`, `ǈ`, `ǋ`, `ǲ`, etc).

```rust
# use typed_ident::*;
# use typed_ident::presets::unicode::*;
// A camel boundary splits `Camel` from `Boundary`.
let segments: Vec<_> = CamelIdent::new("CamelBoundary")?
    .segments()
    .type_erased()
    .collect();
assert_eq!(segments, [
    StrSegment::Chunk("Camel"),
    StrSegment::Chunk("Boundary")
]);

// A hat boundary splits `HAT` from `Boundary`.
let segments: Vec<_> = CamelIdent::new("HATBoundary")?
    .segments()
    .type_erased()
    .collect();
assert_eq!(segments, [
    StrSegment::Chunk("HAT"),
    StrSegment::Chunk("Boundary")
]);

// A delimiter boundary splits `delimiter` from `_`, and `_` from `boundary`.
let segments: Vec<_> = CamelIdent::new("delimiter_boundary")?
    .segments()
    .type_erased()
    .collect();
assert_eq!(segments, [
    StrSegment::Chunk("delimiter"),
    StrSegment::Delim('_'),
    StrSegment::Chunk("boundary")
]);
# Ok::<(), typed_ident::Error>(())
```

See the [`boundary`] module documentation for information on configuring this predefined boundary implementation, or how to create your own boundary implementation.

[`boundary`]: crate::syntax::boundary

## Character Profiles

Though the character profile can be customized, this crate provides a predefined Unicode character profile based on [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/). It also provides an ASCII profile that restrict the Unicode profile to only ASCII characters.

A character profile must be combined with a case profile to form a **cased profile**, which is then usable for identifier validation.

* A **mixed** profile allows any casing supported by the character profile
* A **lower** profile allows only lowercase and uncased characters
* An **upper** profile allows only uppercase, uncased, and Greek titlecase characters
* A **lower-camel** profile allows only lowercase and uncased for the first character after a delimiter
* An **upper-camel** profile allows only uppercase, uncased, and titlecase for the first character after a delimiter

> **NOTE:** The specific handling of titlecase here comes from how [UTS #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks) defines where boundaries are introduced. For consistency, we either consider a titlecase character entirely visually uppercase (Greek), or that it visually start uppercase but ends lowercase (non-Greek).

See the [`profile`] module documentation for information on other less-common predefined case profiles, or how to create your own case and character profile implementations.

[`profile`]: crate::syntax::profile

## Identifier "Words"

Common terminology used in this crate is calling a specific kind of slice of a chunk a "word". This is slightly non-specific terminology, but it's succinct so it is often used regardless.

When we refer to a "word", it's really a non-empty chunk which has been made to contain no boundaries (as defined by the [`Boundary`] definition on the chunk). Commonly this is the largest slice of a chunk that satisfies this requirement, but it doesn't have to be.

In that sense, this is not a linguistic word, but more like an "identifier word", or an "identifier chunk word". A specific definition of words useful for identifier syntax inspection.

[`Boundary`]: crate::syntax::boundary::Boundary

## Normalization

This crate does not provide utilities for [Unicode normalization](https://www.unicode.org/faq/normalization.html).

Normalization is optional, but if canonical-equivalent spellings should be treated identically, normalize the input before validation.

**NFC** *(for storage and general use)*

NFC composes canonically equivalent sequences into a standard canonical form. It is commonly useful for storing and comparing text when canonical equivalence should be treated as equality. It does not resolve case differences, compatibility equivalences, or visually confusable characters.

**NFKC** *(for compatibility normalization)*

NFKC converts text to a canonical form while also removing compatibility distinctions, such as mapping full-width `１` to ASCII `1`. Because this can discard distinctions that may matter, it should be used only when compatibility equivalence is desired.

For caseless, compatibility-insensitive matching, use Unicode’s `NFKC_Casefold` operation, preferably through a library API that implements it directly.
