// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::fragment::ChunkedStrSegmentIndices;
use crate::core::{Chunk, Fragment, Segment};
use crate::syntax::{Boundary, Delimiter, Profile};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the contiguous segments of a fragment and their positions.
///
/// This struct is created by calling [`chunked_segment_indices`] on the
/// [`Fragment`] type.
///
/// [`Fragment`]: Fragment
/// [`chunked_segment_indices`]: Fragment::chunked_segment_indices
#[repr(transparent)]
pub struct ChunkedSegmentIndices<'a, B, D, P> {
    syntax: PhantomData<&'a Chunk<B, D, P>>,
    iter: ChunkedStrSegmentIndices<'a, D>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, P> ChunkedSegmentIndices<'a, B, D, P> {
    #[must_use]
    #[inline]
    pub(crate) fn new(fragment: &'a Fragment<B, D, P>) -> Self {
        Self {
            syntax: PhantomData,
            iter: ChunkedStrSegmentIndices::new(fragment.as_str()),
        }
    }

    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    #[must_use]
    #[inline]
    pub fn as_fragment(&self) -> &'a Fragment<B, D, P> {
        Fragment::new_unchecked(self.iter.as_str())
    }

    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }

    /// Returns the byte position of the next element, or the total number of
    /// bytes that have been returned via [`next()`](Self::next).
    ///
    /// This means that, when the iterator has not been fully consumed, the
    /// returned value will match the index that will be returned by the next
    /// call to [`next()`](Self::next).
    #[must_use]
    #[inline]
    pub fn offset(&self) -> usize {
        self.iter.offset()
    }

    /// Drops the syntax type information associated with this iterator.
    ///
    /// This is still very useful if, for example, you want to work with
    /// yielded values that are easier to work with (like `&str`).
    #[must_use]
    #[inline]
    pub fn type_erased(self) -> ChunkedStrSegmentIndices<'a, D> {
        self.iter
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P> core::fmt::Debug for ChunkedSegmentIndices<'_, B, D, P> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChunkedSegmentIndices")
            .field(&self.as_str())
            .finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Clone for ChunkedSegmentIndices<'_, B, D, P> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            syntax: PhantomData,
            iter: self.iter.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> Iterator for ChunkedSegmentIndices<'a, B, D, P> {
    type Item = (usize, Segment<D, &'a Chunk<B, D, P>>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(i, s)| (i, Segment::from_unchecked(s)))
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> DoubleEndedIterator
    for ChunkedSegmentIndices<'a, B, D, P>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|(i, s)| (i, Segment::from_unchecked(s)))
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> core::iter::FusedIterator
    for ChunkedSegmentIndices<'_, B, D, P>
{
}
