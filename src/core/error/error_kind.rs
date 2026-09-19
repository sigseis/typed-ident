// =============================================================================
// TYPES
// =============================================================================

/// The kind of error that was experienced when an core type fails checks.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// An identifier was flagged as invalid because it was empty.
    EmptyIdent,

    /// Failed to affix some prefix and suffix onto a chunk, fragment, or
    /// identifier.
    FailedCircumfixing,

    /// Failed to insert a value into a buffer because it would invalidate the
    /// chunk, fragment, or identifier.
    FailedInsert,

    /// Failed to join a value with a buffer because it would invalidate the
    /// chunk, fragment, or identifier.
    FailedJoin,

    /// Failed to prefix a fragment onto a chunk, fragment, or identifier.
    FailedPrefixing,

    /// Failed to push a value onto a buffer because it would invalidate the
    /// chunk, fragment, or identifier.
    FailedPush,

    /// Failed to remove a slice of data because it would invalidate the chunk,
    /// fragment, or identifier.
    FailedRemove,

    /// Failed to replace a slice of data because it would invalidate the chunk,
    /// fragment, or identifier.
    FailedReplace,

    /// Failed to suffix a fragment onto a chunk, fragment, or identifier.
    FailedSuffixing,

    /// Attempted to interpret a string slice as a delimiter, but it was not one.
    InvalidDelimiter,

    /// Contained characters that did not satisfy the profile.
    InvalidFormat,

    /// Contained characters that did not satisfy the profile for the prefix.
    InvalidPrefix,

    /// Contained characters that did not satisfy the profile for the suffix.
    InvalidSuffix,
}
