// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "strict.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::{AppendClosed, Ascii, CharProfile, Profile, Unicode};
use crate::syntax::{SubsetOf, segmentation};
use unicode_general_category::{GeneralCategory, get_general_category};

// =============================================================================
// TYPES
// =============================================================================

/// The same as [`Unicode`], except that it disallows `Mc`, `Me`, and `Mn`
/// characters at the start of a chunk.
///
/// # The Issue With `Mc`, `Me`, and `Mn` Characters
///
/// If you allow joining characters at chunk-start, you can create some
/// surprisingly valid identifiers.
///
/// ```
/// # use typed_ident::presets::unicode::*;
/// assert!(LowerCamelIdent::new("_A").is_err()); // Doesn't work (expected).
/// assert!(LowerCamelIdent::new("_゙A").is_ok());  // Works?
/// assert!(UpperCamelIdent::new("_a").is_err()); // Doesn't work (expected).
/// assert!(UpperCamelIdent::new("_゙a").is_ok());  // Works?
/// ```
///
/// If you're okay with another dependency and slightly less optimized code, you
/// can protect against this with the [`Strict`] profile:
///
/// ```
/// # use typed_ident::presets::strict::*;
/// assert!(LowerCamelIdent::new("_A").is_err()); // Doesn't work (expected).
/// assert!(LowerCamelIdent::new("_゙A").is_err()); // Also doesn't work!
/// assert!(UpperCamelIdent::new("_a").is_err()); // Doesn't work (expected).
/// assert!(UpperCamelIdent::new("_゙a").is_err()); // Also doesn't work!
/// ```
///
/// # When To Use `Strict` vs. `Unicode`
///
/// ***You should almost always just use [`Unicode`].***
///
/// You only want [`Strict`] if you really want to enforce that upper and lower
/// camel look a certain way (e.g. that the first character is upper or lower
/// compatible).
///
/// Strict processing is a bit slower, and causes the profile to no longer be
/// `APPEND_CLOSED` for fragments (which reduces the optimizations we can do).
/// It costs you something, and *usually* it's not worth the cost.
pub enum Strict {}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Profile for Strict {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Chunk;
    type BaseProfile = Self;
    type Segmentation = segmentation::Grapheme;

    #[inline(always)]
    fn is_ident_start(c: char) -> bool {
        Unicode::is_ident_start(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        Unicode::is_chunk_start(c)
            && !matches!(
                get_general_category(c),
                GeneralCategory::EnclosingMark
                    | GeneralCategory::SpacingMark
                    | GeneralCategory::NonspacingMark,
            )
    }
    #[inline(always)]
    fn in_profile(c: char) -> bool {
        Unicode::in_profile(c)
    }
    #[inline(always)]
    fn is_chunk_continue(c: char) -> bool {
        Unicode::is_chunk_continue(c)
    }
}

// -----------------------------------------------------------------------------
impl CharProfile for Strict {}

// -----------------------------------------------------------------------------
/// Proof: ASCII is a subset of the Strict profile (which is basically Unicode).
// -----------------------------------------------------------------------------
impl SubsetOf<Strict> for Ascii {}

// -----------------------------------------------------------------------------
/// Proof: Strict is a subset of the Unicode profile (strict has more restrictions).
// -----------------------------------------------------------------------------
impl SubsetOf<Unicode> for Strict {}

// -----------------------------------------------------------------------------
/// Proof: It's always safe to implement this against yourself.
// -----------------------------------------------------------------------------
impl SubsetOf<Strict> for Strict {}
