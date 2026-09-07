// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Segment;
use crate::core::chunk::WordStrs;
use crate::core::fragment::ChunkedStrSegments;
use crate::syntax::{Boundary, Delimiter, Segmentation};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the word-separated segment string slices of a fragment.
///
/// This struct is created by calling [`type_erased`] on the [`Segments`]
/// iterator.
///
/// [`Segments`]: crate::core::fragment::Segments
/// [`type_erased`]: crate::core::fragment::Segments::type_erased
#[repr(transparent)]
pub struct StrSegments<'a, B, D, S> {
    bounds: PhantomData<(&'a B, &'a D, &'a S)>,
    inner: &'a str,
}

// =============================================================================
// IMPLS
// =============================================================================

// ---------------------------------------------------------------------
impl<'a, B, D, S> StrSegments<'a, B, D, S> {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.inner
    }

    #[must_use]
    #[inline]
    pub(crate) fn new(fragment: &'a str) -> Self {
        Self {
            bounds: PhantomData,
            inner: fragment,
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, S> core::fmt::Debug for StrSegments<'_, B, D, S> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StrSegments").field(&self.as_str()).finish()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, S> Clone for StrSegments<'_, B, D, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.inner)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, S: Segmentation> core::iter::Iterator
    for StrSegments<'a, B, D, S>
{
    type Item = Segment<char, &'a str>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut delimited = ChunkedStrSegments::<D>::new(self.inner);
        let chunk = match delimited.next()? {
            Segment::Chunk(chunk) => chunk,
            Segment::Delim(delim) => {
                self.inner = delimited.as_str();
                return Some(Segment::Delim(delim));
            }
        };

        let mut bounded = WordStrs::<B, S>::new(chunk);
        let next = bounded.next()?;
        self.inner = &self.inner[next.len()..];
        Some(Segment::Chunk(next))
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, S: Segmentation> core::iter::DoubleEndedIterator
    for StrSegments<'a, B, D, S>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut delimited = ChunkedStrSegments::<D>::new(self.inner);
        let chunk = match delimited.next_back()? {
            Segment::Chunk(chunk) => chunk,
            Segment::Delim(delim) => {
                self.inner = delimited.as_str();
                return Some(Segment::Delim(delim));
            }
        };

        let mut bounded = WordStrs::<B, S>::new(chunk);
        let next = bounded.next_back()?;
        self.inner = &self.inner[..self.inner.len() - next.len()];
        Some(Segment::Chunk(next))
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, S: Segmentation> core::iter::FusedIterator
    for StrSegments<'_, B, D, S>
{
}
