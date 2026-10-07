// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::StrSegment;
use crate::core::fragment::StrSegmentIndices;
use crate::syntax::{Boundary, Delimiter, Segmentation};

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the word string slices of a fragment and their positions.
///
/// This struct is created by calling [`type_erased`] on the [`WordIndices`]
/// iterator.
///
/// Unlike other `*_indices` iterators, this one does not have the ability to
/// access the remaining underlying string data, or offsets. Not because this is
/// not possible, but since the iterator skips sections of the underlying string
/// it can be hard to reason about these values.
///
/// [`WordIndices`]: crate::core::fragment::WordIndices
/// [`type_erased`]: crate::core::fragment::WordIndices::type_erased
pub struct WordStrIndices<'a, B, D, S> {
    iter: StrSegmentIndices<'a, B, D, S>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, S> WordStrIndices<'a, B, D, S> {
    #[must_use]
    #[inline]
    pub(crate) fn new(fragment: &'a str) -> Self {
        Self {
            iter: StrSegmentIndices::new(fragment),
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, S> core::fmt::Debug for WordStrIndices<'_, B, D, S> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WordStrIndices").finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, S> Clone for WordStrIndices<'_, B, D, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, S: Segmentation> Iterator for WordStrIndices<'a, B, D, S> {
    type Item = (usize, &'a str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for (idx, next) in self.iter.by_ref() {
            if let StrSegment::Chunk(next) = next {
                return Some((idx, next));
            }
        }
        None
    }

    #[inline(always)]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, S: Segmentation> DoubleEndedIterator
    for WordStrIndices<'a, B, D, S>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((idx, next)) = self.iter.next_back() {
            if let StrSegment::Chunk(next) = next {
                return Some((idx, next));
            }
        }
        None
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, S: Segmentation> core::iter::FusedIterator
    for WordStrIndices<'_, B, D, S>
{
}
