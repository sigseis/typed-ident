// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::segmentation::GraphemesIterator;
use unicode_segmentation::{Graphemes as UnicodeGraphemes, UnicodeSegmentation};

// =============================================================================
// TYPES
// =============================================================================

/// Yields the graphemes of a string.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Graphemes<'a> {
    iter: UnicodeGraphemes<'a>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a> Graphemes<'a> {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    #[inline(always)]
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }

    /// Constructs a new instance of this iterator over the provided string
    /// slice.
    #[inline(always)]
    pub fn new(s: &'a str) -> Self {
        Self {
            iter: s.graphemes(true),
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a> Iterator for Graphemes<'a> {
    type Item = &'a str;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline(always)]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a> DoubleEndedIterator for Graphemes<'a> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

// -----------------------------------------------------------------------------
impl core::iter::FusedIterator for Graphemes<'_> {}

// -----------------------------------------------------------------------------
impl<'a> GraphemesIterator<'a> for Graphemes<'a> {
    #[inline(always)]
    fn as_str(&self) -> &'a str {
        self.as_str()
    }

    #[inline(always)]
    fn new(s: &'a str) -> Self
    where
        Self: Sized,
    {
        Self::new(s)
    }
}
