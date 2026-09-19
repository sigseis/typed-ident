# typed-ident

A Rust crate for type-safe identifier validation, inspection, and mutation.

Conforming to:

* [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/) for Unicode character profiles
* [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks) for boundary calculation

<div class="warning">

**NOTE:** This crate is currently in alpha. Until the crate enters beta, the crate has no stability promise and may contain breaking changes with every update.

</div>

# Using `typed-ident`

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

## Basic Usage

The most common way to use this crate is through its predefined identifier presets. You can choose the preset that matches the syntax you want to enforce.

```rust
use typed_ident::presets::unicode::*;

// Examples of lower-camel identifiers:
assert!(LowerCamelIdent::new("iAm_123").is_ok());     // Delimiters         (allowed)
assert!(LowerCamelIdent::new("iAm_lower").is_ok());   // Consistent Chunks  (allowed)

// Examples of non-lower-camel identifiers:
assert!(LowerCamelIdent::new("").is_err());           // Empty              (disallowed)
assert!(LowerCamelIdent::new("2").is_err());          // Invalid Start      (disallowed)
assert!(LowerCamelIdent::new("UpperCamel").is_err()); // Upper Camel        (disallowed)
assert!(LowerCamelIdent::new("iAm_Mixed").is_err());  // Mixed Chunks       (disallowed)
assert!(LowerCamelIdent::new("kebab-case").is_err()); // Hyphen Delimiter   (disallowed)
# Ok::<(), typed_ident::Error>(())
```

If you know the general shape of the identifier, but do not want to enforce a specific casing convention, less-specific presets are also available.

```rust
use typed_ident::presets::unicode::*;

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

## Advanced Usage

If the predefined presets do not match your use case, you can define custom identifiers with customized boundaries, delimiters, and character profiles. Prefer a predefined preset whenever one already matches your requirements.

```rust
use typed_ident::*;
use typed_ident::syntax::*;

// Defining your own custom delimiter (the `@` symbol).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AtDelimiter;
impl UnitDelimiter for AtDelimiter {
    const CHAR: char = '@';
    const STR: &'static str = "@";
}
impl SubsetOf<AtDelimiter> for AtDelimiter {}

// Defining your own custom type that uses the `@` delimiter.
// For this example, use a predefined boundary and character profile.
type CustomIdent = Ident<
    boundary::Standard,             // Use the standard boundary algorithm
    AtDelimiter,                    // The delimiter used by this identifier is `@`.
    profile::Lower<profile::Ascii>, // ASCII-only, disallowing uppercase and titlecase.
>;

// Examples of lower-at identifiers:
assert!(CustomIdent::new("iam@123").is_ok());     // Delimiters         (allowed)
assert!(CustomIdent::new("iam@lower").is_ok());   // Consistent Chunks  (allowed)
assert!(CustomIdent::new("onechunk").is_ok());    // One Chunk          (allowed)

// Examples of non-lower-at identifiers:
assert!(CustomIdent::new("").is_err());           // Empty              (disallowed)
assert!(CustomIdent::new("2").is_err());          // Invalid Start      (disallowed)
assert!(CustomIdent::new("iam@Mixed").is_err());  // Uppercase Char     (disallowed)
assert!(CustomIdent::new("kebab-case").is_err()); // Hyphen Delimiter   (disallowed)
assert!(CustomIdent::new("snake_case").is_err()); // Low Line Delimiter (disallowed)
assert!(CustomIdent::new("CamelCase").is_err());  // Camel Casing       (disallowed)
assert!(CustomIdent::new("UPPERCASE").is_err());  // Upper Casing       (disallowed)
assert!(CustomIdent::new("日本語").is_err());      // Non-ASCII          (disallowed)
# Ok::<(), typed_ident::Error>(())
```

Creating custom identifiers can be difficult. See the [`syntax`] module documentation for more details.

## Selecting a Preset

There are four predefined formats, each available with different case restrictions.

| **Delimiter**             | **Mixed**       | **Cased**            | **Lower** or **Upper**                      |
|---------------------------|-----------------|----------------------|---------------------------------------------|
| **Low Line (`_`)**        | [`CamelIdent`]  | [`CasedCamelIdent`]  | [`LowerCamelIdent`]  / [`UpperCamelIdent`]  |
| **Flat Line (`_` / `-`)** | [`HybridIdent`] | [`CasedHybridIdent`] | [`LowerHybridIdent`] / [`UpperHybridIdent`] |
| **Hyphen-Minus (`-`)**    | [`KebabIdent`]  | [`CasedKebabIdent`]  | [`LowerKebabIdent`]  / [`UpperKebabIdent`]  |
| **Low Line (`_`)**        | [`SnakeIdent`]  | [`CasedSnakeIdent`]  | [`LowerSnakeIdent`]  / [`UpperSnakeIdent`]  |

To break down the selection process, you can first start by considering the general form of the identifier you want.

**Start by selecting one of...**

| **Format**      | **Delimiters**        | **Case restrictions apply to...** |
|-----------------|-----------------------|-----------------------------------|
| [`CamelIdent`]  | Low Line (`_`)        | ...only chunk start characters.   |
| [`HybridIdent`] | Flat Line (`_` / `-`) | ...only chunk start characters.   |
| [`KebabIdent`]  | Hyphen-Minus (`-`)    | ...all characters.                |
| [`SnakeIdent`]  | Low Line (`_`)        | ...all characters.                |

Then select a case profile based on the restrictions you want to apply to the in-scope characters.

**Finalize your selection based on your required case restrictions...**

| **Casing**           | **Case Restrictions**                 |
|----------------------|---------------------------------------|
| Mixed *(Unprefixed)* | No specific case restrictions.        |
| Cased                | Either lower or upper, but not mixed. |
| Lower                | Must be lowercase or uncased.         |
| Upper                | Must be uppercase or uncased.         |

All predefined presets follow this naming pattern. Select the strictest preset that matches your requirement.

```rust
# use typed_ident::presets::unicode::*;
// A basic camel identifier is the most permissive choice.
assert!(CamelIdent::new("lowerCamel").is_ok());
assert!(CamelIdent::new("UpperCamel").is_ok());
assert!(CamelIdent::new("lowerCamel_UpperCamel").is_ok());

// A cased-camel identifier must be consistently either lower- or upper-cased.
assert!(CasedCamelIdent::new("lowerCamel").is_ok());
assert!(CasedCamelIdent::new("UpperCamel").is_ok());
assert!(CasedCamelIdent::new("lowerCamel_UpperCamel").is_err());

// A lower-cased camel identifier requires that it's lowerCamelCase
assert!(LowerCamelIdent::new("lowerCamel").is_ok());
assert!(LowerCamelIdent::new("UpperCamel").is_err());
assert!(LowerCamelIdent::new("lowerCamel_UpperCamel").is_err());

// An upper-cased camel identifier requires that it's UpperCamelCase
assert!(UpperCamelIdent::new("lowerCamel").is_err());
assert!(UpperCamelIdent::new("UpperCamel").is_ok());
assert!(UpperCamelIdent::new("lowerCamel_UpperCamel").is_err());
```

[`CamelIdent`]: crate::presets::unicode::CamelIdent
[`HybridIdent`]: crate::presets::unicode::HybridIdent
[`KebabIdent`]: crate::presets::unicode::KebabIdent
[`SnakeIdent`]: crate::presets::unicode::SnakeIdent
[`CasedCamelIdent`]: crate::presets::unicode::CasedCamelIdent
[`CasedHybridIdent`]: crate::presets::unicode::CasedHybridIdent
[`CasedKebabIdent`]: crate::presets::unicode::CasedKebabIdent
[`CasedSnakeIdent`]: crate::presets::unicode::CasedSnakeIdent
[`LowerCamelIdent`]: crate::presets::unicode::LowerCamelIdent
[`LowerHybridIdent`]: crate::presets::unicode::LowerHybridIdent
[`LowerKebabIdent`]: crate::presets::unicode::LowerKebabIdent
[`LowerSnakeIdent`]: crate::presets::unicode::LowerSnakeIdent
[`UpperCamelIdent`]: crate::presets::unicode::UpperCamelIdent
[`UpperHybridIdent`]: crate::presets::unicode::UpperHybridIdent
[`UpperKebabIdent`]: crate::presets::unicode::UpperKebabIdent
[`UpperSnakeIdent`]: crate::presets::unicode::UpperSnakeIdent

# Crate Details

## Core Types

The predefined presets are configurations of more general types provided by this crate.

| **Type**        | **Brief Description**                                                            |
|-----------------|----------------------------------------------------------------------------------|
| [`Ident`]       | An immutable, syntax-validated UTF-8 string slice representing an identifier.    |
| [`Fragment`]    | An immutable slice of an identifier, which itself may not be an identifier.      |
| [`FragmentBuf`] | A mutable, owned buffer for constructing or modifying fragments and identifiers. |
| [`Chunk`]       | An immutable slice of a [`Fragment`] which contains no delimiters.               |
| [`Segment`]     | A single chunk or delimiter from an [`Ident`] or [`Fragment`].                   |

There is no `IdentBuf` type. This is because a mutable identifier is able to be made invalid by clearing the buffer, and enforcing the buffer is non-empty makes such a type difficult to use.

For an owned `Ident`, use `Box<Ident>`.

* [`Ident::to_boxed_ident`] to construct a boxed identifier from an immutable identifier reference
* [`Ident::new_boxed`] to fallibly construct a boxed identifier from a plain string buffer
* [`FragmentBuf::into_boxed_ident`] to fallibly construct a boxed identifier from a fragment buffer

[`Chunk`]: crate::core::Chunk
[`Fragment`]: crate::core::Fragment
[`FragmentBuf`]: crate::alloc::FragmentBuf
[`FragmentBuf::into_boxed_ident`]: crate::alloc::FragmentBuf::into_boxed_ident
[`Ident`]: crate::core::Ident
[`Ident::new_boxed`]: crate::core::Ident::new_boxed
[`Ident::to_boxed_ident`]: crate::core::Ident::to_boxed_ident
[`Segment`]: crate::core::Segment

## Boundary Definition

Though the boundary implementation can be customized, this crate provides a configurable predefined implementation with defaults based on [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks).

This means that predefined identifiers will create the following segment boundaries.

* A **delimiter boundary** places a boundary before and after any delimiter defined by the identifier
* A **camel boundary** places a boundary after a lowercase or non-Greek titlecase letter, followed by an uppercase or titlecase letter
* A **hat boundary** places a boundary before an uppercase or titlecase letter followed by a lowercase letter, or before a non-Greek titlecase letter

**NOTE:** UTS #55 distinguishes Greek and non-Greek titlecase characters when determining identifier chunks. Greek titlecase characters are treated as visually uppercase (`ᾈ`, `ᾨ`, `ῌ`, `ᾚ`, etc), while non-Greek titlecase characters are treated as beginning uppercase and continuing lowercase within the same scalar value (`ǅ`, `ǈ`, `ǋ`, `ǲ`, etc).

```rust
# use typed_ident::core::*;
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

**NOTE:** The specific handling of titlecase here comes from how [UTS #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks) defines where boundaries are introduced. For consistency, we either consider a titlecase character entirely visually uppercase (Greek), or that it visually start uppercase but ends lowercase (non-Greek).

```rust
# use typed_ident::syntax::profile::*;
// A mixed profile allows any casing of valid characters.
assert!(Mixed::<Ascii>::is_chunk_start('a'));
assert!(Mixed::<Ascii>::is_chunk_start('A'));
assert!(Mixed::<Ascii>::is_chunk_start('0'));
assert!(Mixed::<Ascii>::is_chunk_continue('a'));
assert!(Mixed::<Ascii>::is_chunk_continue('A'));
assert!(Mixed::<Ascii>::is_chunk_continue('0'));

// A lower profile allows only lower or uncased valid characters.
assert!(Lower::<Ascii>::is_chunk_start('a'));
assert!(!Lower::<Ascii>::is_chunk_start('A'));
assert!(Lower::<Ascii>::is_chunk_start('0'));
assert!(Lower::<Ascii>::is_chunk_continue('a'));
assert!(!Lower::<Ascii>::is_chunk_continue('A'));
assert!(Lower::<Ascii>::is_chunk_continue('0'));

// An upper profile allows only upper-like or uncased valid characters.
assert!(!Upper::<Ascii>::is_chunk_start('a'));
assert!(Upper::<Ascii>::is_chunk_start('A'));
assert!(Upper::<Ascii>::is_chunk_start('0'));
assert!(!Upper::<Ascii>::is_chunk_continue('a'));
assert!(Upper::<Ascii>::is_chunk_continue('A'));
assert!(Upper::<Ascii>::is_chunk_continue('0'));

// A lower camel profile restricts only the first character of each chunk.
assert!(LowerCamel::<Ascii>::is_chunk_start('a'));
assert!(!LowerCamel::<Ascii>::is_chunk_start('A'));
assert!(LowerCamel::<Ascii>::is_chunk_start('0'));
assert!(LowerCamel::<Ascii>::is_chunk_continue('a'));
assert!(LowerCamel::<Ascii>::is_chunk_continue('A'));
assert!(LowerCamel::<Ascii>::is_chunk_continue('0'));

// An upper camel profile restricts only the first character of each chunk.
assert!(!UpperCamel::<Ascii>::is_chunk_start('a'));
assert!(UpperCamel::<Ascii>::is_chunk_start('A'));
assert!(UpperCamel::<Ascii>::is_chunk_start('0'));
assert!(UpperCamel::<Ascii>::is_chunk_continue('a'));
assert!(UpperCamel::<Ascii>::is_chunk_continue('A'));
assert!(UpperCamel::<Ascii>::is_chunk_continue('0'));
```

See the [`profile`] module documentation for information on other less-common predefined case profiles, or how to create your own case and character profile implementations.

[`profile`]: crate::syntax::profile

## How Validation Works

Validation works on one Unicode scalar value (`char`) at a time, and checks if each character is either a valid delimiter or within the profile selected. Some case checks are interdependent, particularly for the `Cased*` types, but this is a useful high-level explanation.

* There must be one or more characters to form a valid identifier.
* Every character belonging to the target `Delimiter` is valid.
* Non-delimiter characters must match based on the following definitions:
  * [`Profile::is_ident_start_char`] is called if it's the very first character.
  * [`Profile::is_chunk_start`] is called if it's the first character after a delimiter.
  * [`Profile::is_chunk_continue`] is called for any remaining non-delimiter characters.

[`Profile::is_ident_start_char`]: crate::syntax::profile::Profile::is_ident_start_char
[`Profile::is_chunk_start`]: crate::syntax::profile::Profile::is_chunk_start
[`Profile::is_chunk_continue`]: crate::syntax::profile::Profile::is_chunk_continue

Validation can be described generally with the following pseudo-code:

```rust
# use typed_ident::syntax::*;
# use typed_ident::syntax::delimiter::*;
# use typed_ident::syntax::profile::*;
// First, we must define some target syntax definitions.
type TargetDelimiter = LowLine;
type TargetProfile = Mixed<Unicode>;

// Then we can validate some target string.
let mut chars = "targetString".char_indices();

// The first character is handled specially.
let Some((_, first)) = chars.next() else {
    return Err(SyntaxError::Format(0));
};
let first_is_delim = TargetDelimiter::is_ident_start_delim(first);
if !first_is_delim && !TargetProfile::is_ident_start_char(first) {
    return Err(SyntaxError::Format(0));
}

// Remaining characters are handled generally.
let mut last_was_delim = first_is_delim;
for (idx, c) in chars {
    let profile_check = match last_was_delim {
        true => TargetProfile::is_chunk_start,
        false => TargetProfile::is_chunk_continue,
    };
    last_was_delim = TargetDelimiter::is_chunk_delim(c);
    if last_was_delim {
        continue;
    }
    if !profile_check(c) {
        return Err(SyntaxError::Format(idx));
    }
}
# Ok::<(), typed_ident::syntax::SyntaxError>(())
```

## Normalization

This crate does not provide utilities for [Unicode normalization](https://www.unicode.org/faq/normalization.html).

Normalization is optional, but if canonical-equivalent spellings should be treated identically, normalize the input before validation.

**NFC** *(for storage and general use)*

NFC composes canonically equivalent sequences into a standard canonical form. It is commonly useful for storing and comparing text when canonical equivalence should be treated as equality. It does not resolve case differences, compatibility equivalences, or visually confusable characters.

**NFKC** *(for compatibility normalization)*

NFKC converts text to a canonical form while also removing compatibility distinctions, such as mapping full-width `１` to ASCII `1`. Because this can discard distinctions that may matter, it should be used only when compatibility equivalence is desired.

For caseless, compatibility-insensitive matching, use Unicode’s `NFKC_Casefold` operation, preferably through a library API that implements it directly.
