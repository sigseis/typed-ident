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
use crate::syntax::{SubsetOf, generated, segmentation};

// =============================================================================
// CONSTANTS
// =============================================================================

/// The version of `unicode-ident` that was compiled in this crate.
pub const UNICODE_IDENT_VERSION: (u32, u32, u32) = (
    unicode_ident::UNICODE_VERSION.0 as u32,
    unicode_ident::UNICODE_VERSION.1 as u32,
    unicode_ident::UNICODE_VERSION.2 as u32,
);

/// The version of `unicode-segmentation` that was compiled in this crate.
pub const UNICODE_SEGMENTATION_VERSION: (u32, u32, u32) = {
    assert!(unicode_segmentation::UNICODE_VERSION.0 <= u32::MAX as u64);
    assert!(unicode_segmentation::UNICODE_VERSION.1 <= u32::MAX as u64);
    assert!(unicode_segmentation::UNICODE_VERSION.2 <= u32::MAX as u64);
    (
        unicode_segmentation::UNICODE_VERSION.0 as u32,
        unicode_segmentation::UNICODE_VERSION.1 as u32,
        unicode_segmentation::UNICODE_VERSION.2 as u32,
    )
};

/// The minimal common dependency version (lesser of [`UNICODE_IDENT_VERSION`]
/// and [`UNICODE_SEGMENTATION_VERSION`]).
///
/// [`UNICODE_IDENT_VERSION`]: crate::syntax::profile::UNICODE_IDENT_VERSION
/// [`UNICODE_SEGMENTATION_VERSION`]: crate::syntax::profile::UNICODE_SEGMENTATION_VERSION
pub const UNICODE_DEPENDENCIES_VERSION: (u32, u32, u32) = {
    if UNICODE_IDENT_VERSION.0 < UNICODE_SEGMENTATION_VERSION.0 {
        UNICODE_IDENT_VERSION
    } else if UNICODE_SEGMENTATION_VERSION.0 < UNICODE_IDENT_VERSION.0 {
        UNICODE_SEGMENTATION_VERSION
    } else if UNICODE_IDENT_VERSION.1 < UNICODE_SEGMENTATION_VERSION.1 {
        UNICODE_IDENT_VERSION
    } else if UNICODE_SEGMENTATION_VERSION.1 < UNICODE_IDENT_VERSION.1 {
        UNICODE_SEGMENTATION_VERSION
    } else if UNICODE_IDENT_VERSION.2 < UNICODE_SEGMENTATION_VERSION.2 {
        UNICODE_IDENT_VERSION
    } else {
        UNICODE_SEGMENTATION_VERSION
    }
};

/// The version of Unicode used to generate functionality provided by this crate.
///
/// Specifically we provide the calls for:
/// * `is_titlecase` (eventually the standard will provide this).
/// * `is_titlecase_greek_variant`
/// * `is_combining_mark`
pub const UNICODE_GENERATED_VERSION: (u32, u32, u32) = generated::UNICODE_VERSION;

/// The minimal common Unicode version (lesser of
/// [`UNICODE_DEPENDENCIES_VERSION`] and [`UNICODE_GENERATED_VERSION`]).
///
/// [`UNICODE_DEPENDENCIES_VERSION`]: crate::syntax::profile::UNICODE_DEPENDENCIES_VERSION
/// [`UNICODE_GENERATED_VERSION`]: crate::syntax::profile::UNICODE_GENERATED_VERSION
pub const UNICODE_VERSION: (u32, u32, u32) = {
    if UNICODE_DEPENDENCIES_VERSION.0 < UNICODE_GENERATED_VERSION.0 {
        UNICODE_DEPENDENCIES_VERSION
    } else if UNICODE_GENERATED_VERSION.0 < UNICODE_DEPENDENCIES_VERSION.0 {
        UNICODE_GENERATED_VERSION
    } else if UNICODE_DEPENDENCIES_VERSION.1 < UNICODE_GENERATED_VERSION.1 {
        UNICODE_DEPENDENCIES_VERSION
    } else if UNICODE_GENERATED_VERSION.1 < UNICODE_DEPENDENCIES_VERSION.1 {
        UNICODE_GENERATED_VERSION
    } else if UNICODE_DEPENDENCIES_VERSION.2 < UNICODE_GENERATED_VERSION.2 {
        UNICODE_DEPENDENCIES_VERSION
    } else {
        UNICODE_GENERATED_VERSION
    }
};

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
