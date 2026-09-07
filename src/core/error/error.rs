// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::ErrorKind;
use core::fmt::{Display, Formatter};

// =============================================================================
// TYPES
// =============================================================================

/// An error when performing some operation on an identifier (or fragment).
///
/// You can inspect [`error_kind`] to see specifically what went wrong. And
/// depending on the failure, [`byte_offset`] might have been set. Please refer
/// to the function documentation.
///
/// [`error_kind`]: Self::error_kind
/// [`byte_offset`]: Self::byte_offset
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    byte_offset: Option<usize>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Error {
    /// The byte offset that caused the error (starting from the beginning of
    /// the offending string).
    #[must_use]
    #[inline]
    pub fn byte_offset(&self) -> Option<usize> {
        self.byte_offset
    }

    /// The error kind that led to this failure.
    #[must_use]
    #[inline]
    pub fn error_kind(&self) -> ErrorKind {
        self.kind
    }

    /// Constructs a new error.
    #[must_use]
    #[inline]
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            byte_offset: None,
        }
    }

    /// Sets the byte offset of the error.
    #[must_use]
    #[inline]
    pub(crate) fn with_byte_offset(mut self, idx: usize) -> Self {
        self.byte_offset = Some(idx);
        self
    }

    /// Sets the error kind of the error.
    #[cfg(feature = "alloc")] // Only used w/ alloc feature currently.
    #[must_use]
    #[inline]
    pub(crate) fn with_error_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let msg = match self.kind {
            ErrorKind::EmptyIdent => "empty identifier",
            ErrorKind::FailedJoinLeft => "failed to join because of the data on the left",
            ErrorKind::FailedJoinRight => "failed to join because of the data on the right",
            ErrorKind::FailedRemove => "failed to remove a range of data",
            ErrorKind::FailedReplaceLeft => {
                "failed to replace a range of data because of data on the left"
            }
            ErrorKind::FailedReplaceRight => {
                "failed to replace a range of data because of data on the right"
            }
            ErrorKind::InvalidFormat => "character doesn't satisfy the character profile",
            ErrorKind::InvalidPrefix => {
                "character doesn't satisfy the character profile for the prefix fragment"
            }
            ErrorKind::InvalidDelimiter => {
                "delimiter string contained more data than a single delimiter"
            }
            ErrorKind::InvalidSuffix => {
                "character doesn't satisfy the character profile for the suffix fragment"
            }
        };
        match self.byte_offset() {
            None => write!(f, "{msg}"),
            Some(idx) => write!(f, "{msg}: at byte offset {idx}"),
        }
    }
}
