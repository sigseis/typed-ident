// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::StrSegment;
use crate::fragment::StrSegments;
use crate::syntax::{Boundary, Delimiter, Segmentation};

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the word string slices of a fragment.
///
/// This struct is created by calling [`type_erased`] on the [`Words`] iterator.
///
/// Unlike other similar iterators, this one does not have the ability to access
/// the remaining underlying string data. Not because this is not possible, but
/// since the iterator skips sections of the underlying string it can be hard to
/// reason about these values.
///
/// [`Words`]: crate::fragment::Words
/// [`type_erased`]: crate::fragment::Words::type_erased
#[repr(transparent)]
pub struct StrWords<'a, B, D, S> {
    iter: StrSegments<'a, B, D, S>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, S> StrWords<'a, B, D, S> {
    #[must_use]
    #[inline]
    pub(crate) fn new(fragment: &'a str) -> Self {
        Self {
            iter: StrSegments::new(fragment),
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, S> core::fmt::Debug for StrWords<'_, B, D, S> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StrWords").finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, S> Clone for StrWords<'_, B, D, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, S: Segmentation> Iterator for StrWords<'a, B, D, S> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for next in self.iter.by_ref() {
            if let StrSegment::Chunk(next) = next {
                return Some(next);
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
impl<'a, B: Boundary, D: Delimiter, S: Segmentation> DoubleEndedIterator for StrWords<'a, B, D, S> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.iter.next_back() {
            if let StrSegment::Chunk(next) = next {
                return Some(next);
            }
        }
        None
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, S: Segmentation> core::iter::FusedIterator
    for StrWords<'_, B, D, S>
{
}
