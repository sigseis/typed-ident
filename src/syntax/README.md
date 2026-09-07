Syntax definitions for identifiers.

This module contains traits that are needed to define an identifier. It also contains reasonable default implementations of these traits for certain obvious cases.

* [`Boundary`] defines how chunks are broken during segmentation (if at all).
* [`Delimiter`] defines the kinds of characters that always delimit a chunk regardless of the boundary.
* [`Profile`] defines a subset of the set of all valid UTF-8 characters that are valid for an identifier.

To aide in automatic zero-cost casting, a special type named [`SubsetOf`], which can dictate that one syntax element is a subset of another. This powers the [`Ident::cast`] function, which checks at compile-time if it's safe and trivial to just cast from one identifier to another.

For more details, see the respective documentation for each item.

[`Ident::cast`]: crate::core::Ident::cast
