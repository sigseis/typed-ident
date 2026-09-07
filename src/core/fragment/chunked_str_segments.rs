// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Segment;
use crate::syntax::Delimiter;
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An iterator over the contiguous segment string slices of a fragment.
///
/// This struct is created by calling [`type_erased`] on the [`ChunkedSegments`]
/// iterator.
///
/// [`ChunkedSegments`]: crate::core::fragment::ChunkedSegments
/// [`type_erased`]: crate::core::fragment::ChunkedSegments::type_erased
#[repr(transparent)]
pub struct ChunkedStrSegments<'a, D> {
    delimiter: PhantomData<D>,
    inner: &'a str,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, D> ChunkedStrSegments<'a, D> {
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
    pub(super) fn new(fragment: &'a str) -> Self {
        Self {
            delimiter: PhantomData,
            inner: fragment,
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D> core::fmt::Debug for ChunkedStrSegments<'_, D> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChunkedStrSegments")
            .field(&self.as_str())
            .finish()
    }
}

// -----------------------------------------------------------------------------
impl<D> Clone for ChunkedStrSegments<'_, D> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            delimiter: PhantomData,
            inner: self.inner,
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, D: Delimiter> core::iter::Iterator for ChunkedStrSegments<'a, D> {
    type Item = Segment<char, &'a str>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (left, right) = D::find_delimiter(self.inner)
            .map(|idx| self.inner.split_at(idx))
            .unwrap_or((self.inner, ""));
        if !left.is_empty() {
            self.inner = right;
            return Some(Segment::Chunk(left));
        }
        let mut chars = right.chars();
        let next = chars.next().map(Segment::Delim);
        self.inner = chars.as_str();
        next
    }

    #[inline(always)]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a, D: Delimiter> core::iter::DoubleEndedIterator for ChunkedStrSegments<'a, D> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (left, right) = D::rfind_delimiter(self.inner)
            .map(|idx| {
                self.inner.split_at(
                    idx + self.inner[idx..]
                        .chars()
                        .next()
                        .map(char::len_utf8)
                        .unwrap_or_default(),
                )
            })
            .unwrap_or(("", self.inner));
        if !right.is_empty() {
            self.inner = left;
            return Some(Segment::Chunk(right));
        }
        let mut chars = left.chars();
        let next = chars.next_back().map(Segment::Delim);
        self.inner = chars.as_str();
        next
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter> core::iter::FusedIterator for ChunkedStrSegments<'_, D> {}
