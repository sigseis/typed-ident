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
use crate::core::chunk::WordStrs;
use crate::core::{Chunk, Fragment};
use crate::syntax::{Boundary, Delimiter, Profile};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the words of a chunk.
///
/// This struct is created with the [`words`] method on [`Chunk`]. See its
/// documentation for more.
///
/// [`words`]: Chunk::words
#[repr(transparent)]
pub struct Words<'a, B, D, P: Profile> {
    syntax: PhantomData<&'a Chunk<B, D, P>>,
    iter: WordStrs<'a, B, P::Segmentation>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, P: Profile> Words<'a, B, D, P> {
    #[must_use]
    #[inline]
    pub(crate) fn new(chunk: &'a Chunk<B, D, P>) -> Self {
        Self {
            syntax: PhantomData,
            iter: WordStrs::new(chunk.as_str()),
        }
    }

    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    #[must_use]
    #[inline]
    pub fn as_chunk(&self) -> &'a Chunk<B, D, P> {
        Chunk::new_unchecked(self.iter.as_str())
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

    /// Drops the syntax type information associated with this iterator.
    ///
    /// This is still very useful if, for example, you want to work with
    /// yielded values that are easier to work with (like `&str`).
    #[must_use]
    #[inline]
    pub fn type_erased(self) -> WordStrs<'a, B, P::Segmentation> {
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
        f.debug_tuple("Words").field(&self.as_str()).finish()
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
