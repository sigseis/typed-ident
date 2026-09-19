// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::ErrorKind;
use crate::syntax::SyntaxError;
use core::fmt::{Display, Formatter};

// =============================================================================
// TYPES
// =============================================================================

/// An error when performing some operation on a core type.
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
            ErrorKind::FailedCircumfixing => "failed to append a set of circumfixes",
            ErrorKind::FailedPrefixing => "failed to prepend a prefix",
            ErrorKind::FailedSuffixing => "failed to append a suffix",
            ErrorKind::FailedInsert => "failed to insert a fragment",
            ErrorKind::FailedJoin => "failed to join with another fragment",
            ErrorKind::FailedPush => "failed to push a fragment onto a buffer",
            ErrorKind::FailedRemove => "failed to remove a range of fragment data",
            ErrorKind::FailedReplace => "failed to replace a range of fragment data",
            ErrorKind::InvalidFormat => "invalid format",
            ErrorKind::InvalidPrefix => "invalid format for the prefix fragment",
            ErrorKind::InvalidDelimiter => "invalid delimiter string",
            ErrorKind::InvalidSuffix => "invalid format for the suffix fragment",
        };
        match self.byte_offset() {
            None => write!(f, "{msg}"),
            Some(idx) => write!(f, "{msg}: at byte offset {idx}"),
        }
    }
}

// -----------------------------------------------------------------------------
impl From<SyntaxError> for Error {
    fn from(orig: SyntaxError) -> Self {
        match orig {
            SyntaxError::Empty => Error::new(ErrorKind::EmptyIdent),
            SyntaxError::Format(idx) => Error::new(ErrorKind::InvalidFormat).with_byte_offset(idx),
        }
    }
}
