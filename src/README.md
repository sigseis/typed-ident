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
# Ok::<(), typed_ident::Error>(())
```

## Operations

After constructing a valid [`Ident`], there are *many* operations that you can perform on it.

* [`cast`] - Zero-cost conversion from one identifier to another (in cases where the target is a superset representation).
* [`join_str`] - Append a string onto the end of an identifier while preserving chunk boundaries.
* [`segments`] - Break the identifier into its smallest semantically meaningful segments ("identifier words", and delimiters).
* [`trim_decorative_delims`] - Remove unnecessary delimiters from each end of the identifier.
* [`as_<case>_<format>`] - display formatting identifiers as different formats (e.g. [`as_lower_camel`], [`as_upper_kebab`], etc.)
* [`to_<case>_<format>`] - Conversion to strings for other preset identifier formats (e.g. [`to_lower_camel`], [`to_upper_kebab`], etc.)
* ***string-like operations*** - Many of the operations you expect on a regular string slice are also here, except they now ensure you cannot produce an invalid identifier (e.g. [`split_at`], [`replace`], [`with_suffix`], etc.)
* ***typed slicing*** - Typed information about the identifier travels with [`Fragment`]s (sub-slices) of an identifier.
* ***and much more!*** - See the crate documentation for more information.

[`to_<case>_<format>`]: crate::alloc::convert
[`to_lower_camel`]: crate::alloc::convert::ToLowerCamel
[`to_upper_kebab`]: crate::alloc::convert::ToUpperKebab
[`as_<case>_<format>`]: crate::core::fmt
[`as_lower_camel`]: crate::core::fmt::AsLowerCamel
[`as_upper_kebab`]: crate::core::fmt::AsUpperKebab
[`Fragment`]: crate::core::Fragment
[`Ident`]: crate::core::Ident
[`cast`]: crate::core::Ident::cast
[`join_str`]: crate::core::Ident::join_str
[`replace`]: crate::core::Ident::replace
[`segments`]: crate::core::Fragment::segments
[`split_at`]: crate::core::Ident::split_at
[`trim_decorative_delims`]: crate::core::Ident::trim_decorative_delims
[`with_suffix`]: crate::core::Fragment::with_suffix

## Design

This crate works with base configurable identifier types; [`Ident`] and [`IdentBuf`].

The idea is that most common identifier formats can be defined by 3 things:

1. [`Boundary`] - how chunks are broken during segmentation (if at all).
2. [`Delimiter`] - the kinds of characters which *always* delimit chunks of an identifier, regardless of boundary configuration.
3. [`Profile`] - the set of valid UTF-8 code points allowed in this format.

We define an "identifier format" as a specific combination of characters, which is constrained to fit an expected pattern. The [`Delimiter`] and [`Profile`] types define the format, while [`Boundary`] defines the segmentation properties.

See the [`syntax`] module for more information on configuring identifier syntax.

[`Boundary`]: crate::syntax::boundary::Boundary
[`Delimiter`]: crate::syntax::delimiter::Delimiter
[`Profile`]: crate::syntax::profile::Profile
[`syntax`]: crate::syntax

### Design: Profiles

The crate splits profiles into two categories; character profiles and case profiles.

If no explicit case is expected, you can use the character profiles directly. Otherwise, if you expect some specific case properties, you can wrap the character profile in a case profile (e.g. `Lower<Unicode>`).

**Character Profiles**

There are three provided character profiles.

* [`Ascii`] - `ascii_alphabetic` start, `ascii_alphanumeric` chunk start/continue.
* [`Unicode`] - `xid_start` start, `xid_continue` chunk start/continue (with some documented omissions).
* [`Strict`] - Same as [`Unicode`], except chunk start is not allowed to have combining marks (must be an actual, literal character).

[`Ascii`]: crate::syntax::profile::chars::Ascii
[`Unicode`]: crate::syntax::profile::chars::Unicode
[`Strict`]: crate::syntax::profile::chars::Strict

**Case Profiles**

There are four provided casing profiles.

* [`Lower`] - only accepts non-uppercase/titlecase characters.
* [`LowerCamel`] - The first character in a chunk (or start of an identifier) must be non-uppercase/titlecase.
* [`Upper`] - only accepts non-lowercase/greek-titlecase characters.
* [`UpperCamel`] - The first character in a chunk (or start of an identifier) must be non-lowercase.

[`Lower`]: crate::syntax::profile::case::Lower
[`LowerCamel`]: crate::syntax::profile::case::LowerCamel
[`Upper`]: crate::syntax::profile::case::Upper
[`UpperCamel`]: crate::syntax::profile::case::UpperCamel

### Design: Delimiters

There are five provided delimiters.

* [`AsciiPunctuation`] - accepts any ASCII punctuation as a delimiter. *(no preset uses this, it's mostly for reference and to have a common ASCII punctuation delimiter)*
* [`AsciiFlatLine`] - either the LOW LINE (`_`) or HYPHEN-MINUS (`-`) characters.
* [`HyphenMinus`] - the HYPHEN-MINUS (`-`) character.
* [`LowLine`] - the LOW LINE (`_`) character.
* [`NotDelimited`] - disallow delimiters of any kind for this identifier. *(no preset uses this, it's provided because this is tricky to implement, and I could see a want for this)*

[`AsciiPunctuation`]: crate::syntax::delimiter::AsciiPunctuation
[`AsciiFlatLine`]: crate::syntax::delimiter::AsciiFlatLine
[`HyphenMinus`]: crate::syntax::delimiter::HyphenMinus
[`LowLine`]: crate::syntax::delimiter::LowLine
[`NotDelimited`]: crate::syntax::delimiter::NotDelimited

### Presets

The presets defined by this crate are broken into two categories based on whether or not they prefer delimiters.

All provided presets allow for some kind of delimiter, because in all cases delimiters are valuable in cases of ambiguity (consider for instance `Tls1.3` - we likely want a break of some kind between the `1` and `3`). Though, if you would like, you can produce a version of an identifier completely disallowing delimiters by using the [`NotDelimited`] delimiter.

**Prefers Delimiters**

These are identifiers which prefer there be a delimiter between chunks.

| **Delimiter\\Casing**  | **Mixed-Case**  | **Lower-Case**  | **Upper-Case**  |
|------------------------|-----------------|-----------------|-----------------|
| **Low Line (`_`)**     | [`Mixed_Snake`] | [`lower_snake`] | [`UPPER_SNAKE`] |
| **Hyphen-Minus (`-`)** | [`Mixed-Kebab`] | [`lower-kebab`] | [`UPPER-KEBAB`] |

[`Mixed_Snake`]: crate::presets::generic::SnakeIdent
[`lower_snake`]: crate::presets::generic::LowerSnakeIdent
[`UPPER_SNAKE`]: crate::presets::generic::UpperSnakeIdent
[`Mixed-Kebab`]: crate::presets::generic::KebabIdent
[`lower-kebab`]: crate::presets::generic::LowerKebabIdent
[`UPPER-KEBAB`]: crate::presets::generic::UpperKebabIdent

**Prefers Chunk Boundary**

These are identifiers which prefer there *NOT* to be delimiters between chunks, instead depending on casing to separate chunks. (However, delimiters are still allowed, they are only introduced whenever casing would not be able to separate two chunks).

| **Delimiter\\Casing**     | **Mixed-Case**  | **Lower-Case**  | **Upper-Case**  |
|---------------------------|-----------------|-----------------|-----------------|
| **Low Line (`_`)**        | [`MixedCamel`]  | [`lowerCamel`]  | [`UpperCamel`]  |
| **Flat Line (`-` / `_`)** | [`MixedHybrid`] | [`lowerHybrid`] | [`UpperHybrid`] |

[`MixedCamel`]: crate::presets::generic::CamelIdent
[`lowerCamel`]: crate::presets::generic::LowerCamelIdent
[`UpperCamel`]: crate::presets::generic::UpperCamelIdent
[`MixedHybrid`]: crate::presets::generic::HybridIdent
[`lowerHybrid`]: crate::presets::generic::LowerHybridIdent
[`UpperHybrid`]: crate::presets::generic::UpperHybridIdent

# Normalization

This crate does not provide utilities for [Unicode normalization](https://www.unicode.org/faq/normalization.html).

It seemed like something that would probably be best provided from some other utility crate, and the developer in question could handle using the two features together (e.g. `LowerCamelIdent::new_boxed(string.nfc().collect())`, or something similar).

If identifiers should treat canonically equivalent spellings as equal, normalize them consistently before validation, storage, and comparison.

## Normalization: NFC

*(for storage and general use)*

NFC composes canonically equivalent sequences into a standard canonical form. It is commonly useful for storing and comparing text when canonical equivalence should be treated as equality. It does not resolve case differences, compatibility equivalences, or visually confusable characters.

## Normalization: NFKC

*(for compatibility normalization)*

NFKC converts text to a canonical form while also removing compatibility distinctions, such as mapping full-width `１` to ASCII `1`. Because this can discard distinctions that may matter, it should be used only when compatibility equivalence is desired.

For caseless matching, use Unicode’s `NFKC_Casefold` operation, preferably through a library API that implements it directly.

## Examples

Instead of listing a bunch of examples here on the main README, we have some examples prepared under the examples directory. Specifically, we've made a single util with commands that should serve as a good collection of example applications.

You can run any of these with: `cargo run --all-features --example api -- {your CLI args here}`

You can type `--help` for details on the commands the util can run.
