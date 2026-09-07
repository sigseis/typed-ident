// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use core::error::Error;
use core::fmt::{Display, Formatter};

// =============================================================================
// TYPES
// =============================================================================

/// An error that is returned when failing a conversion from a `char` to a
/// delimiter.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TryFromCharError(pub(crate) ());

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Error for TryFromCharError {}

// -----------------------------------------------------------------------------
impl Display for TryFromCharError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("attempted to convert a non-delimiter character to a delimiter")
    }
}
