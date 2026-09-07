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
/// (with LOW LINE, ZWJ, and ZWNJ removed).
///
/// # Character Set
///
/// This profile uses the [`unicode-ident`] crate.
///
/// * **Start** => any character that passes `is_xid_start(c)`
///   (except LOW LINE, ZWJ, and ZWNJ).
/// * **Continue** => any character that passes `is_xid_continue(c)`
///   (except LOW LINE, ZWJ, and ZWNJ).
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
/// # About ZWJ+ZWNJ Omissions
///
/// The zero-width joiner characters (U+200D and U+200C) have special usage
/// within this profile, as detailed in the above specification. However, it
/// requires extra processing to use them properly - we simply omit them.
///
/// If you wish to support these extended use-cases, you can create a custom
/// profile which allows these characters (simply by not disallowing them, just
/// call into `unicode-ident` functions). But you'll have to implement
/// additional validation on top of the identifiers you form (perhaps as a
/// newtype).
///
/// In my reading of this standard, the zero-width characters are just not worth
/// the headache. I really strongly recommend *against* their inclusion.
///
/// [`unicode-ident`]: unicode_ident
pub enum Unicode {}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Unicode {
    #[inline]
    fn is_disallowed(c: char) -> bool {
        const ZWNJ: char = '\u{200C}';
        const ZWJ: char = '\u{200D}';
        c == '_' || c == ZWNJ || c == ZWJ
    }
    #[inline]
    fn is_ident_start(c: char) -> bool {
        unicode_ident::is_xid_start(c) && !Self::is_disallowed(c)
    }
    #[inline]
    fn in_profile(c: char) -> bool {
        unicode_ident::is_xid_continue(c) && !Self::is_disallowed(c)
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
    type BaseProfile = Self;
    type Segmentation = segmentation::Grapheme;

    #[inline(always)]
    fn is_ident_start(c: char) -> bool {
        Self::is_ident_start(c)
    }
    #[inline(always)]
    fn is_chunk_start(c: char) -> bool {
        Self::in_profile(c)
    }
    #[inline(always)]
    fn in_profile(c: char) -> bool {
        Self::in_profile(c)
    }
    #[inline(always)]
    fn is_chunk_continue(c: char) -> bool {
        Self::in_profile(c)
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
