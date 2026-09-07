// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "ascii.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::{AppendClosed, CharProfile, Profile};
use crate::syntax::{SubsetOf, segmentation};

// =============================================================================
// TYPES
// =============================================================================

/// A profile allowing only ASCII identifier characters.
///
/// # Character Set
///
/// * **Start** => any character that passes `c.is_ascii_alphabetic()`.
/// * **Continue** => any character that passes `c.is_ascii_alphanumeric()`.
pub enum Ascii {}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Ascii {
    #[inline(always)]
    fn is_ident_start(c: char) -> bool {
        c.is_ascii_alphabetic()
    }
    #[inline(always)]
    fn in_profile(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Profile for Ascii {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Fragment;
    type BaseProfile = Self;
    type Segmentation = segmentation::Char;

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
impl CharProfile for Ascii {}

// -----------------------------------------------------------------------------
/// Proof: It's always safe to implement this against yourself.
// -----------------------------------------------------------------------------
impl SubsetOf<Ascii> for Ascii {}
