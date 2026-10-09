// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "unicode.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::{AppendClosed, Ascii, CharProfile, Profile};
use crate::syntax::{SubsetOf, segmentation};

// =============================================================================
// TYPES
// =============================================================================

/// A [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/) profile
/// (with LOW LINE removed).
///
/// # Character Set
///
/// This profile uses the [`unicode-ident`] crate.
///
/// * **Start** => any character that passes `is_xid_start(c)`.
/// * **Continue** => any character that passes `is_xid_continue(c)` (except
///   LOW LINE).
///
/// # About Low Line Omission
///
/// It's confusing if the character profile contains a delimiter.
///
/// Instead, if you want an identifier that includes the low-line character, you
/// should select such an identifier (from the presets, or by including it as a
/// delimiter in a custom identifier).
///
/// We still "include" the character, we just want it to be treated specially.
///
/// [`unicode-ident`]: unicode_ident
pub enum Unicode {}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Unicode {
    #[inline]
    fn is_chunk_char(c: char) -> bool {
        unicode_ident::is_xid_continue(c) && c != '_'
    }
    #[inline]
    fn is_ident_start_char(c: char) -> bool {
        unicode_ident::is_xid_start(c)
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Profile for Unicode {
    // It is a property of Unicode XID that continue contains start. And since
    // this type doesn't make chunk-start different from chunk-continue, it is
    // append-closed from the perspective of a whole fragment.
    const APPEND_CLOSED: AppendClosed = AppendClosed::Fragment;
    type CharProfile = Self;
    type Segmentation = segmentation::Grapheme;

    #[inline(always)]
    fn is_chunk_char(c: char) -> bool {
        Self::is_chunk_char(c)
    }
    #[inline(always)]
    fn is_chunk_continue(c: char) -> bool {
        Self::is_chunk_char(c)
    }
    #[inline(always)]
    fn is_chunk_start(c: char) -> bool {
        Self::is_chunk_char(c)
    }
    #[inline(always)]
    fn is_ident_start_char(c: char) -> bool {
        Self::is_ident_start_char(c)
    }
}

// -----------------------------------------------------------------------------
impl CharProfile for Unicode {}

// -----------------------------------------------------------------------------
/// Proof: ASCII is an obvious subset of Unicode.
// -----------------------------------------------------------------------------
impl SubsetOf<Unicode> for Ascii {}

// -----------------------------------------------------------------------------
/// Proof: It's always safe to implement this against yourself.
// -----------------------------------------------------------------------------
impl SubsetOf<Unicode> for Unicode {}
