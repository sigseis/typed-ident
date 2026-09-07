// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::segmentation::GraphemesIterator;

// =============================================================================
// TYPES
// =============================================================================

/// Yields the graphemes of a string, assuming the string contains only
/// single-char graphemes.
///
/// This is intended to be used for types that provably have ASCII data.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Graphemes<'a> {
    string: &'a str,
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
        self.string
    }

    /// Constructs a new instance of this iterator over the provided string
    /// slice.
    #[inline(always)]
    pub fn new(s: &'a str) -> Self {
        Self { string: s }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a> Iterator for Graphemes<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.string.is_empty() {
            return None;
        }
        let idx = self.string.ceil_char_boundary(1);
        let (left, right) = self.string.split_at(idx);
        self.string = right;
        Some(left)
    }

    #[inline(always)]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

// -----------------------------------------------------------------------------
impl<'a> DoubleEndedIterator for Graphemes<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let idx = match self.string.len() {
            0 => return None,
            n => n - 1,
        };
        let idx = self.string.floor_char_boundary(idx);
        let (left, right) = self.string.split_at(idx);
        self.string = left;
        Some(right)
    }
}

// -----------------------------------------------------------------------------
impl core::iter::FusedIterator for Graphemes<'_> {}

// -----------------------------------------------------------------------------
impl<'a> GraphemesIterator<'a> for Graphemes<'a> {
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
}
