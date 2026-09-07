// =============================================================================
// TYPES
// =============================================================================

/// The kind of error that was experienced when an identifier fails checks.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// An identifier was flagged as invalid because it was empty.
    EmptyIdent,

    /// A fragment failed to be inserted or appended because of the data on the left.
    FailedJoinLeft,

    /// A fragment failed to be inserted or appended because of the data on the left.
    FailedJoinRight,

    /// Failed to remove a slice of data because it would invalidate the chunk,
    /// fragment, or identifier.
    FailedRemove,

    /// Failed to replace a slice of data because it would invalidate the chunk,
    /// fragment, or identifier because of the data on the left.
    FailedReplaceLeft,

    /// Failed to replace a slice of data because it would invalidate the chunk,
    /// fragment, or identifier because of the data on the right.
    FailedReplaceRight,

    /// Attempted to interpret a string slice as a delimiter, but it was not one.
    InvalidDelimiter,

    /// Contained characters that did not satisfy the profile.
    InvalidFormat,

    /// Contained characters that did not satisfy the profile for the prefix.
    InvalidPrefix,

    /// Contained characters that did not satisfy the profile for the suffix.
    InvalidSuffix,
}
