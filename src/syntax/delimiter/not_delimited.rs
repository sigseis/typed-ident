// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "not_delimited.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::delimiter::{AppendClosed, Delimiter, TryFromCharError};

// =============================================================================
// TYPES
// =============================================================================

/// Represents the explicit lack of a delimiter.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord)]
pub enum NotDelimited {}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Delimiter for NotDelimited {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Identifier;

    #[inline]
    fn as_char(&self) -> char {
        panic!("as_char() is not defined for the `NotDelimited` delimiter")
    }

    #[inline(always)]
    fn from_char(_: char) -> Option<Self> {
        None
    }
    #[inline(always)]
    fn from_ident_start(_: char) -> Option<Self> {
        None
    }
    #[inline(always)]
    fn from_chunk_delim(_: char) -> Option<Self> {
        None
    }

    #[inline(always)]
    fn is_delim(_: char) -> bool {
        false
    }
    #[inline(always)]
    fn is_ident_start(_: char) -> bool {
        false
    }
    #[inline(always)]
    fn is_chunk_delim(_: char) -> bool {
        false
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl core::cmp::PartialEq<char> for NotDelimited {
    #[inline(always)]
    fn eq(&self, _: &char) -> bool {
        false
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialEq<NotDelimited> for char {
    #[inline(always)]
    fn eq(&self, _: &NotDelimited) -> bool {
        false
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter> core::cmp::PartialEq<D> for NotDelimited {
    #[inline(always)]
    fn eq(&self, _: &D) -> bool {
        false
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter> core::cmp::PartialOrd<D> for NotDelimited {
    #[inline(always)]
    fn partial_cmp(&self, _: &D) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Less)
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialOrd<char> for NotDelimited {
    #[inline(always)]
    fn partial_cmp(&self, _: &char) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Less)
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialOrd<NotDelimited> for char {
    #[inline(always)]
    fn partial_cmp(&self, _: &NotDelimited) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Less)
    }
}

// -----------------------------------------------------------------------------
impl TryFrom<char> for NotDelimited {
    type Error = TryFromCharError;

    #[inline(always)]
    fn try_from(_: char) -> Result<Self, Self::Error> {
        Err(TryFromCharError(()))
    }
}

// -----------------------------------------------------------------------------
impl core::fmt::Display for NotDelimited {
    #[inline]
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}
