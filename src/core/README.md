Core type definitions that make up this crate.

The main types defined by this module are:

* [`Chunk`] is a subslice of an identifier which contains no delimiter characters.
* [`Fragment`] is a subslice of an identifier which itself may not be a valid identifier.
* [`Ident`] is a reference to a string slice that forms a valid identifier.
* [`Segment`] is a part of an identifier (either a delimiter, or a chunk).

For more details, see the respective documentation for each item.

# How Types Are Related

TL;DR: You almost always want to use [`Ident`], and you almost always want it as-provided through the type aliases in the [`presets`] module. Other types are mostly incidental depending on what methods you call on `Ident`.

* [`Ident`]: An identifier validated to satisfy some configured syntax rules.
  * **Allowed to be Empty:** No
  * **Deref Target:** [`Fragment`]
  * **Slice Target:** [`Fragment`] (`&ident[start..end]`)
  * **Purpose:** This is the main type of this crate.
  * **When to Use:** Almost always. You usually will be using this type through some type alias (commonly provided by the [`presets`] module).
* [`Fragment`]: A slice of an identifier, which itself may not be a valid identifier.
  * **Allowed to be Empty:** Yes
  * **Deref Target:** None
  * **Slice Target:** [`Fragment`] (`&fragment[start..end]`)
  * **Purpose:** To safely encode in the type system that you have something which may not be an entirely valid identifier (either empty, or a subslice which does not satisfy the format requirements).
  * **When to Use:** Usually this is used incidentally, without explicitly naming the type. For instance, when you slice an identifier or another fragment, or perform some action that returns a part of an identifier.
* [`Chunk`]: A slice of a fragment which contains no delimiters.
  * **Allowed to be Empty:** Yes (Though uncommon)
  * **Deref Target:** [`Fragment`]
  * **Slice Target:** [`Chunk`] (`&chunk[start..end]`)
  * **Purpose:** To safely encode in the type system that there are no delimiters to deal with.
  * **When to Use:** Usually this is used incidentally, without explicitly naming the type. For instance, when you segment an identifier or another fragment.
* [`Segment`]: An enum containing either a [`Chunk`] or a [`Delimiter`].
  * **Allowed to be Empty:** Yes (Though uncommon)
  * **Deref Target:** None
  * **Slice Target:** None
  * **Purpose:** To allow for segmentation functionality that picks apart an identifier or fragment.
  * **When to Use:** Usually this is used incidentally, without explicitly naming the type. For instance, when you segment an identifier or another fragment.

[`Delimiter`]: crate::syntax::delimiter::Delimiter
[`presets`]: crate::presets

# Identifier "Words"

Common terminology used in this crate is calling a specific kind of slice of a chunk a "word". This is slightly non-specific terminology, but it's succinct so it is often used regardless.

When we refer to a "word", it's really a non-empty chunk which has been made to contain no boundaries (as defined by the [`Boundary`] definition on the chunk). Commonly this is the largest slice of a chunk that satisfies this requirement, but it doesn't have to be.

In that sense, this is not a linguistic word, but more like an "identifier word", or an "identifier chunk word". A specific definition of words useful for identifier syntax inspection.

[`Boundary`]: crate::syntax::boundary::Boundary
