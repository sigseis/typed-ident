Syntax definitions for identifiers.

This module contains traits that are needed to define an identifier. It also contains reasonable default implementations of these traits for certain obvious cases.

* [`boundary`] defines how chunks are broken during segmentation (if at all)
* [`delimiter`] defines the kinds of characters that always delimit a chunk regardless of the boundary
* [`profile`] defines a subset of the set of all valid UTF-8 characters that are valid for an identifier
* [`segmentation`] defines the valid ways in which you can iterate over graphemes

For more details, especially if you plan on implementing any of these syntax traits, see the respective documentation for each item.

## Zero-Cost Casting

To aide in automatic zero-cost casting, a special type named [`SubsetOf`], which can dictate that one syntax element is a subset of another. This powers the [`Ident::cast`] function, which checks at compile-time if it's safe and trivial to just cast from one identifier to another.

[`Ident::cast`]: crate::core::Ident::cast

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
