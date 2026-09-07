// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::{Boundary, Segmentation};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the word string slices of a chunk.
///
/// This struct is created by calling [`type_erased`] on the [`Words`] iterator.
///
/// [`Words`]: crate::core::chunk::Words
/// [`type_erased`]: crate::core::chunk::Words::type_erased
#[repr(transparent)]
pub struct WordStrs<'a, B, S> {
    boundary: PhantomData<(B, S)>,
    inner: &'a str,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, S> WordStrs<'a, B, S> {
    #[must_use]
    #[inline]
    pub(crate) fn new(chunk: &'a str) -> Self {
        Self {
            boundary: PhantomData,
            inner: chunk,
        }
    }

    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.inner
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, S> core::fmt::Debug for WordStrs<'_, B, S> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WordStrs").field(&self.as_str()).finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, S> Clone for WordStrs<'_, B, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.inner)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, S: Segmentation> Iterator for WordStrs<'a, B, S> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        let chunk = self.inner;

        if let Some(split) = B::find_boundary::<S>(chunk) {
            let (left, right) = self.inner.split_at(split.get());
            self.inner = right;
            return Some(left);
        }

        self.inner = "";
        Some(chunk)
    }

    #[inline(always)]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, S: Segmentation> DoubleEndedIterator for WordStrs<'a, B, S> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        let chunk = self.inner;

        if let Some(split) = B::rfind_boundary::<S>(chunk) {
            let (left, right) = self.inner.split_at(split.get());
            self.inner = left;
            return Some(right);
        }

        self.inner = "";
        Some(chunk)
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, S: Segmentation> core::iter::FusedIterator for WordStrs<'_, B, S> {}
