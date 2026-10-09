// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "word_indices.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fragment::StrWordIndices;
use crate::syntax::{Boundary, Delimiter, Profile};
use crate::{Chunk, Fragment};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the words of a fragment and their positions.
///
/// This struct is created with the [`word_indices`] method on [`Fragment`]. See
/// its documentation for more.
///
/// Unlike other `*_indices` iterators, this one does not have the ability to
/// access the remaining underlying string data, or offsets. Not because this is
/// not possible, but since the iterator skips sections of the underlying string
/// it can be hard to reason about these values.
///
/// [`word_indices`]: Fragment::word_indices
#[repr(transparent)]
pub struct WordIndices<'a, B, D, P: Profile> {
    syntax: PhantomData<&'a Chunk<B, D, P>>,
    iter: StrWordIndices<'a, B, D, P::Segmentation>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, P: Profile> WordIndices<'a, B, D, P> {
    #[must_use]
    #[inline]
    pub(crate) fn new(fragment: &'a Fragment<B, D, P>) -> Self {
        Self {
            syntax: PhantomData,
            iter: StrWordIndices::new(fragment.as_str()),
        }
    }

    /// Drops the syntax type information associated with this iterator.
    ///
    /// This is still very useful if, for example, you want to work with
    /// yielded values that are easier to work with (like `&str`).
    #[must_use]
    #[inline]
    pub fn type_erased(self) -> StrWordIndices<'a, B, D, P::Segmentation> {
        self.iter
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P: Profile> core::fmt::Debug for WordIndices<'_, B, D, P> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WordIndices").finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P: Profile> Clone for WordIndices<'_, B, D, P> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            syntax: PhantomData,
            iter: self.iter.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> Iterator for WordIndices<'a, B, D, P> {
    type Item = (usize, &'a Chunk<B, D, P>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, c)| (i, Chunk::new_unchecked(c)))
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> DoubleEndedIterator for WordIndices<'a, B, D, P> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|(i, c)| (i, Chunk::new_unchecked(c)))
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> core::iter::FusedIterator for WordIndices<'_, B, D, P> {}
