# Getting Started

A Rust crate for type-safe identifier validation, inspection, and mutation.

<div class="warning">

**Alpha Software:** This crate is currently in alpha. Until the crate enters beta, the crate has no stability promise and may contain breaking changes with every update.

</div>

## Unicode Standards

The Unicode-based identifier profiles and boundary calculations follow:

* [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/)
  for identifier character profiles
* [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks)
  for identifier boundary calculation


## Core Features

The most common way to use this crate is through its predefined presets.

You can choose the preset that matches the syntax you want to enforce.

```rust
use typed_ident::fmt::AsLowerSnake;
use typed_ident::presets::unicode::{CamelIdent, CasedCamelIdent, UpperCamelIdent};

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
# Ok::<(), typed_ident::Error>(())
```

See the [`presets`] module documentation for other predefined identifier formats.

See the [`syntax`] module documentation if you want to build a custom identifier formats.

## Alloc Features

By enabling the `alloc` feature, you can use functionality to safely mutate identifiers.

Identifiers are validated throughout mutation to ensure they continue to satisfy the format requirements.

```rust
use typed_ident::convert::ToLowerKebab;
use typed_ident::presets::unicode::{LowerKebabIdent, UpperCamelIdent};
let upper_camel = UpperCamelIdent::new("HTTPServer")?;

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
# Ok::<(), typed_ident::Error>(())
```

## Main Types

The presets are type aliases to more complex generic types provided by this crate.

| **Type**        | **Brief Description**                                                            |
|-----------------|----------------------------------------------------------------------------------|
| [`Ident`]       | An immutable, syntax-validated UTF-8 string slice representing an identifier.    |
| [`Fragment`]    | An immutable slice of an identifier, which itself may not be an identifier.      |
| [`FragmentBuf`] | A mutable, owned buffer for constructing or modifying fragments and identifiers. |
| [`Chunk`]       | An immutable slice of a [`Fragment`] which contains no delimiters.               |
| [`Segment`]     | A single chunk or delimiter from an [`Ident`] or [`Fragment`].                   |

> **HINT:** You almost always want to start with an `Ident`, usually through some predefined identifier [`presets`].

There is no `IdentBuf` type. Since you can clear a buffer, and since an empty string is not a valid identifier, it's too easy for mutable buffers to form invalid identifiers. You should use `FragmentBuf` to build an identifier from nothing, and use one of its helper functions to fallibly convert to an `Ident` of some form.

For an owned, dynamically-produced `Ident`; use `Box<Ident>` (requires "alloc" feature).

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

## When To Use

`typed-ident` is intended for use with programming identifiers that must obey a defined syntax.

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

For ordinary string conversion, use a general-purpose crate such as [`convert_case`](https://crates.io/crates/convert_case/).

# Crate Details

## Identifier Parts

Identifiers can be loosely defined as having the following parts:

- **Chunks** which are the parts of the identifier which satisfy some character profile.
- **Delimiters** which are additional valid characters that force separation between chunks.
- **Words** which are the largest subslices of a chunk that contain no obvious boundaries.

**Example:**

```text
                 Chunk    Chunk
                ___|___   __|__
               /       \ /     \
Identifier -> "SQLServer_context"
               \_/\____/|\_____/
                |    |  |   |
              Word Word |  Word
                        |
                    Delimiter
```

## Boundary Definition

**A boundary says how a single chunk should break into words.**

Boundaries are used in types like [`CamelIdent`] and [`HybridIdent`], which prefer using casing to separate words instead of solely relying on delimiters (in comparison to [`KebabIdent`] and [`SnakeIdent`]).

This crate provides a configurable [`Standard`] implementation with defaults based on [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks). The provided implementation can be configured if you disagree with the defaults.

See the [`boundary`] module documentation for information on configuring the provided boundary implementation, or how to create your own boundary implementation.

[`CamelIdent`]: crate::presets::unicode::CamelIdent
[`HybridIdent`]: crate::presets::unicode::HybridIdent
[`KebabIdent`]: crate::presets::unicode::KebabIdent
[`SnakeIdent`]: crate::presets::unicode::SnakeIdent
[`Standard`]: crate::syntax::boundary::Standard
[`boundary`]: crate::syntax::boundary

## Delimiter Types

**Delimiters are special characters that *force* word separation.**

You don't always need to be aware of these types, just know that they are ways to configure identifiers to say what a valid syntax may be. For most presets, the types [`LowLine`] (`_`) and [`HyphenMinus`] (`-`) are used.

It is possible to say an identifier allows no valid delimiter ([`NotDelimited`]), but it is of the opinion of the author of this crate that you avoid that unless dealing with some already set syntax. Having a special character which is able to force word separation in otherwise ambiguous cases is very important.

See the [`delimiter`] module documentation for information for more provided delimiter types, or how to create your own delimiter types.

[`HyphenMinus`]: crate::syntax::delimiter::HyphenMinus
[`LowLine`]: crate::syntax::delimiter::LowLine
[`NotDelimited`]: crate::syntax::delimiter::NotDelimited
[`delimiter`]: crate::syntax::delimiter

## Character Profiles

**Character profiles define the set of characters that are valid within a chunk.**

This crate provides a predefined [`Unicode`] character profile based on [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/). It also provides an [`Ascii`] profile that restrict the Unicode profile to only ASCII characters.

A character profile on its own is not enough to define a valid identifier syntax, it needs to be adapted by a case adapter to limit the character profile to a subset of characters based on casing rules. The [`Mixed`] case adapter says not to limit by casing at all, but other adapters (like [`Lower`] or [`Upper`]) will limit the valid characters by their casing as named.

See the [`profile`] module documentation for other provided case profiles, or how to create your own case and character profiles.

[`Ascii`]: crate::syntax::profile::chars::Ascii
[`Lower`]: crate::syntax::profile::case::Lower
[`Mixed`]: crate::syntax::profile::case::Mixed
[`Unicode`]: crate::syntax::profile::chars::Unicode
[`Upper`]: crate::syntax::profile::case::Upper
[`profile`]: crate::syntax::profile

## Normalization

This crate *does not* provide utilities for [Unicode normalization](https://www.unicode.org/faq/normalization.html).

Normalization is optional, but it can be useful to normalize text before processing them as identifiers, especially if the input is from some text that you or the rust compiler has no control over (like user provided text from a command-line tool, for instance).

A brief explanation of the different forms is provided for those who are unfamiliar with it.

**NFC Normalized** *(for storage and general use)*

NFC composes canonically equivalent sequences into a standard canonical form. It is commonly useful for storing and comparing text when canonical equivalence should be treated as equality. It *does not* resolve case differences, compatibility equivalences, or visually confusable characters.

For example, NFC would map `CaféPatron` to `CaféPatron` - these appear identical, but are different on a binary level (and thus would not satisfy an equality check).

**NFKC Normalized** *(for compatibility normalization)*

NFKC converts text to a canonical form while also removing compatibility distinctions. Because this can discard distinctions that may matter, it should be used only when compatibility equivalence is desired. For caseless, compatibility-insensitive matching, use Unicode’s `NFKC_Casefold` operation, preferably through a library API that implements it directly.

For example, NFKC would map `Magic⑧Ball` to `Magic8Ball` - this demonstrates the obvious visual lossiness.
