Syntax definitions for chunk character profiles.

Profiles are broken into two sub-categories:

* [`case`], which limits a character profile based on some casing rules.
* [`chars`], which presents a base set of rules for a chunk character profile.

For more details, see the respective documentation for each item.

## Using Profiles

You should almost always use profiles by passing them into [`Fragment`], [`Chunk`], or [`Ident`] type aliases to define custom identifier syntaxes.

Attempting to call their functions directly to roll your own validation is tricky, and you're more likely to be wrong than you think.

Take for instance the [`Uniform`] and [`Camel`] profiles.

These profiles have special implementations for checking `is_fragment`, `is_ident`, and `is_ident_fragment`. Not calling those functions is incorrect for those cased profiles if you are attempting to validate an identifier or fragment.

[`Chunk`]: crate::core::Chunk
[`Fragment`]: crate::core::Fragment
[`Ident`]: crate::core::Ident

## Customizing Profiles

The expected way to customize profiles is by type composition.

```rust
use typed_ident::syntax::profile::{case, chars};
type LowerAsciiProfile = case::Lower<chars::Ascii>;
```

Try to keep the concept of case and character profiles separate from one-another.

### Implementing a `CharProfile`

If the provided character profiles are insufficient for your needs, you can define a custom profile. All you need to do is implement the [`Profile`] and [`CharProfile`] traits on an un-constructable marker type.

Some notes:

* `CharProfile` should be `Self` if you are implementing a character profile.
* `Segmentation` should be...
  * [`segmentation::Char`] if your profile only contains ASCII characters.
  * [`segmentation::Grapheme`] if your profile contains any non-ASCII characters.

[`segmentation::Char`]: crate::syntax::segmentation::Char
[`segmentation::Grapheme`]: crate::syntax::segmentation::Grapheme

```rust
use typed_ident::syntax::*;
use typed_ident::syntax::profile::*;

enum Binary {}
impl Profile for Binary {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Identifier;
    type CharProfile = Self;
    type Segmentation = segmentation::Char;

    fn is_chunk_char(c: char) -> bool {
        matches!(c, '0' | '1')
    }
    fn is_chunk_continue(c: char) -> bool {
        Self::is_chunk_char(c)
    }
    fn is_chunk_start(c: char) -> bool {
        Self::is_chunk_char(c)
    }
    fn is_ident_start_char(c: char) -> bool {
        Self::is_chunk_char(c)
    }
}
impl CharProfile for Binary {}
impl SubsetOf<Binary> for Binary {}

// Use the custom character profile with the generic presets.
use typed_ident::presets::generic::*;
type BinarySnakeIdentFromPresets = SnakeIdent<
    Binary,             // Select a character profile (required).
    boundary::Standard, // Select a boundary implementation (optional).
>;

// Or you can use it with a custom combination of syntax rules.
use typed_ident::*;
type BinarySnakeIdent = Ident<
    boundary::Standard,
    delimiter::LowLine,
    profile::Mixed<Binary>,
>;
```

### Implementing a `CasedProfile`

Cased profiles are significantly more difficult to implement.

This implements the logic for validating whether something is a valid chunk, fragment, or identifier. If you need more complex validation (like checking emoji profiles, for instance) you will need to implement a cased profiles.

You will need to implement [`Profile`] and [`CasedProfile`] on some un-constructable marker type. It's common (though not required) to make your case profile generic over all `CharProfile` types.

We highly recommend against implementing a custom cased profile, but if you truly need to you should see the existing [cased profiles] for inspiration.

[cased profiles]: case
