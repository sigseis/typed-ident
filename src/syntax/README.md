Syntax definitions for identifiers.

This module contains traits that are needed to define an identifier. It also contains reasonable default implementations of these traits for certain obvious cases.

* [`boundary`] defines how chunks are broken during segmentation (if at all)
* [`delimiter`] defines the kinds of characters that always delimit a chunk regardless of the boundary
* [`profile`] defines a subset of the set of all valid UTF-8 characters that are valid for an identifier
* [`segmentation`] defines the valid ways in which you can iterate over graphemes

For more details, especially if you plan on implementing any of these syntax traits, see the respective documentation for each item.

## Defining Custom Identifiers

You should prefer predefined [presets] whenever one matches your requirements.

But if the predefined presets do not match your use case, you can define custom identifiers with customized boundaries, delimiters, and character profiles.

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

[presets]: crate::presets

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

## Zero-Cost Casting

To aide in automatic zero-cost casting, a special type named [`SubsetOf`], which can dictate that one syntax element is a subset of another. This powers the [`Ident::cast`] function, which checks at compile-time if it's safe and trivial to just cast from one identifier to another.

[`Ident::cast`]: crate::Ident::cast

```rust
# use typed_ident::presets::unicode::*;
let ident = LowerSnakeIdent::new("lower_snake")?;

// Can, without any additional checks, cast to...
let _ident: &CasedSnakeIdent = ident.cast();
let _ident: &SnakeIdent = ident.cast();

// Perhaps surprisingly, it's also a valid camel-ident...
let _ident: &LowerCamelIdent = ident.cast();
let _ident: &CasedCamelIdent = ident.cast();
let _ident: &CamelIdent = ident.cast();

// Similarly, it's also a valid hybrid-ident...
let _ident: &LowerHybridIdent = ident.cast();
let _ident: &CasedHybridIdent = ident.cast();
let _ident: &HybridIdent = ident.cast();

// However, the following would lead to compilation failure...
// let _ident: &UpperSnakeIdent = ident.cast();
// let _ident: &UpperCamelIdent = ident.cast();
// let _ident: &UpperHybridIdent = ident.cast();
// let _ident: &KebabIdent = ident.cast();
// let _ident: &CasedKebabIdent = ident.cast();
// let _ident: &LowerKebabIdent = ident.cast();
// let _ident: &UpperKebabIdent = ident.cast();
# Ok::<(), typed_ident::Error>(())
```

If you are implementing a custom delimiter or profile, you should consider how it relates to others and define `SubsetOf` where sensible.
