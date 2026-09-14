// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use core::fmt::{Display, Formatter};

// =============================================================================
// TYPES
// =============================================================================

/// A type for reporting simple string syntax errors.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SyntaxError {
    /// The value is empty, and an empty string is not allowed.
    Empty,

    /// The format of the string is invalid at the provided byte index.
    Format(usize),
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty identifier"),
            Self::Format(idx) => write!(
                f,
                "character doesn't satisfy the identifier profile: at byte offset {idx}"
            ),
        }
    }
}
