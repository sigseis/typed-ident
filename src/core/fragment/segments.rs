// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::fragment::StrSegments;
use crate::core::{Chunk, Fragment, Segment};
use crate::syntax::{Boundary, Delimiter, Profile};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the word-separated segments of a fragment.
///
/// This struct is created by calling [`segments`] on the [`Fragment`] type.
///
/// [`Fragment`]: Fragment
/// [`segments`]: Fragment::segments
#[repr(transparent)]
pub struct Segments<'a, B, D, P: Profile> {
    syntax: PhantomData<&'a Chunk<B, D, P>>,
    iter: StrSegments<'a, B, D, P::Segmentation>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, P: Profile> Segments<'a, B, D, P> {
    #[must_use]
    #[inline]
    pub(crate) fn new(fragment: &'a Fragment<B, D, P>) -> Self {
        Self {
            syntax: PhantomData,
            iter: StrSegments::new(fragment.as_str()),
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

    /// Drops the syntax type information associated with this iterator.
    ///
    /// This is still very useful if, for example, you want to work with
    /// yielded values that are easier to work with (like `&str`).
    #[must_use]
    #[inline]
    pub fn type_erased(self) -> StrSegments<'a, B, D, P::Segmentation> {
        self.iter
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P: Profile> core::fmt::Debug for Segments<'_, B, D, P> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Segments").field(&self.as_str()).finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P: Profile> Clone for Segments<'_, B, D, P> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            syntax: PhantomData,
            iter: self.iter.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> Iterator for Segments<'a, B, D, P> {
    type Item = Segment<D, &'a Chunk<B, D, P>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Segment::from_unchecked)
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> DoubleEndedIterator for Segments<'a, B, D, P> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Segment::from_unchecked)
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> core::iter::FusedIterator for Segments<'_, B, D, P> {}
