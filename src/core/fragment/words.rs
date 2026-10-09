// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "words.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::fragment::StrWords;
use crate::syntax::{Boundary, Delimiter, Profile};
use crate::{Chunk, Fragment};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the words of a fragments.
///
/// This struct is created with the [`words`] method on [`Chunk`]. See its
/// documentation for more.
///
/// Unlike other similar iterators, this one does not have the ability to access
/// the remaining underlying string data. Not because this is not possible, but
/// since the iterator skips sections of the underlying string it can be hard to
/// reason about these values.
///
/// [`words`]: Chunk::words
#[repr(transparent)]
pub struct Words<'a, B, D, P: Profile> {
    syntax: PhantomData<&'a Chunk<B, D, P>>,
    iter: StrWords<'a, B, D, P::Segmentation>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, P: Profile> Words<'a, B, D, P> {
    #[must_use]
    #[inline]
    pub(crate) fn new(fragment: &'a Fragment<B, D, P>) -> Self {
        Self {
            syntax: PhantomData,
            iter: StrWords::new(fragment.as_str()),
        }
    }

    /// Drops the syntax type information associated with this iterator.
    ///
    /// This is still very useful if, for example, you want to work with
    /// yielded values that are easier to work with (like `&str`).
    #[must_use]
    #[inline]
    pub fn type_erased(self) -> StrWords<'a, B, D, P::Segmentation> {
        self.iter
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P: Profile> core::fmt::Debug for Words<'_, B, D, P> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Words").finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P: Profile> Clone for Words<'_, B, D, P> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            syntax: PhantomData,
            iter: self.iter.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> Iterator for Words<'a, B, D, P> {
    type Item = &'a Chunk<B, D, P>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Chunk::new_unchecked)
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> DoubleEndedIterator for Words<'a, B, D, P> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Chunk::new_unchecked)
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> core::iter::FusedIterator for Words<'_, B, D, P> {}
