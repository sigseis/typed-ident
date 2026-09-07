// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::chunk::WordStrs;
use crate::syntax::{Boundary, Segmentation};

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the word string slices of a chunk and their positions.
///
/// This struct is created by calling [`type_erased`] on the [`WordIndices`]
/// iterator.
///
/// [`WordIndices`]: crate::core::chunk::WordIndices
/// [`type_erased`]: crate::core::chunk::WordIndices::type_erased
pub struct WordStrIndices<'a, B, S> {
    front_offset: usize,
    iter: WordStrs<'a, B, S>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, S> WordStrIndices<'a, B, S> {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }

    #[must_use]
    #[inline]
    pub(crate) fn new(chunk: &'a str) -> Self {
        Self {
            front_offset: 0,
            iter: WordStrs::new(chunk),
        }
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
        self.front_offset
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, S> core::fmt::Debug for WordStrIndices<'_, B, S> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WordStrIndices")
            .field(&self.as_str())
            .finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, S> Clone for WordStrIndices<'_, B, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            front_offset: self.front_offset,
            iter: self.iter.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, S: Segmentation> Iterator for WordStrIndices<'a, B, S> {
    type Item = (usize, &'a str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let pre_len = self.iter.as_str().len();
        match self.iter.next() {
            None => None,
            Some(next) => {
                let index = self.front_offset;
                let len = self.iter.as_str().len();
                self.front_offset += pre_len - len;
                Some((index, next))
            }
        }
    }

    #[inline(always)]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, S: Segmentation> DoubleEndedIterator for WordStrIndices<'a, B, S> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|next| {
            let index = self.front_offset + self.iter.as_str().len();
            (index, next)
        })
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, S: Segmentation> core::iter::FusedIterator for WordStrIndices<'_, B, S> {}
