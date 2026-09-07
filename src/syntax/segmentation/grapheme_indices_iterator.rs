// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use core::fmt::Debug;
use core::iter::FusedIterator;

// =============================================================================
// TRAIT
// =============================================================================

/// A trait for providing an implementation for traversing graphemes for a
/// character profile with position information.
pub trait GraphemeIndicesIterator<'a>:
    Clone + Debug + DoubleEndedIterator<Item = (usize, &'a str)> + FusedIterator
{
    /// Get the remainder of the string from the iterator.
    #[must_use]
    fn as_str(&self) -> &'a str;

    /// Construct a new iterator.
    #[must_use]
    fn new(s: &'a str) -> Self
    where
        Self: Sized;

    /// Find the byte offset from the front of the original input.
    #[must_use]
    fn offset(&self) -> usize;
}
