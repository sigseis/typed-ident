// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::segmentation::GraphemeIndicesIterator;
use crate::syntax::segmentation::r#char::Graphemes;

// =============================================================================
// TYPES
// =============================================================================

/// Yields the graphemes of a string and their positions, assuming the string
/// contains only single-char graphemes.
///
/// This is intended to be used for types that provably have ASCII data.
#[derive(Clone, Debug)]
pub struct GraphemeIndices<'a> {
    front_offset: usize,
    iter: Graphemes<'a>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a> GraphemeIndices<'a> {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }

    /// Constructs a new instance of this iterator over the provided string
    /// slice.
    #[inline]
    pub fn new(s: &'a str) -> Self {
        Self {
            front_offset: 0,
            iter: Graphemes::new(s),
        }
    }

    /// Returns the byte position of the next element, or the total number of
    /// bytes that have been returned via [`next()`](Self::next).
    ///
    /// This means that, when the iterator has not been fully consumed, the
    /// returned value will match the index that will be returned by the next
    /// call to [`next()`](Self::next).
    #[inline]
    pub fn offset(&self) -> usize {
        self.front_offset
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a> Iterator for GraphemeIndices<'a> {
    type Item = (usize, &'a str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let pre_len = self.iter.as_str().len();
        match self.iter.next() {
            None => None,
            Some(grapheme) => {
                let index = self.front_offset;
                let len = self.iter.as_str().len();
                self.front_offset += pre_len - len;
                Some((index, grapheme))
            }
        }
    }

    #[inline(always)]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a> DoubleEndedIterator for GraphemeIndices<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|segment| {
            let index = self.front_offset + self.iter.as_str().len();
            (index, segment)
        })
    }
}

// -----------------------------------------------------------------------------
impl core::iter::FusedIterator for GraphemeIndices<'_> {}

// -----------------------------------------------------------------------------
impl<'a> GraphemeIndicesIterator<'a> for GraphemeIndices<'a> {
    #[inline]
    fn as_str(&self) -> &'a str {
        self.as_str()
    }

    #[inline]
    fn new(s: &'a str) -> Self
    where
        Self: Sized,
    {
        Self::new(s)
    }

    #[inline]
    fn offset(&self) -> usize {
        self.offset()
    }
}
