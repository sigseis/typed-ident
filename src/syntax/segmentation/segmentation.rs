// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::segmentation::{GraphemeIndicesIterator, GraphemesIterator};

// =============================================================================
// TRAITS
// =============================================================================

/// A trait which contains type definitions for segmentation iterators.
pub trait Segmentation {
    /// The kind of iterator to use for segmentation where you only need the
    /// individual "graphemes" (or grapheme-equivalents).
    type Graphemes<'a>: GraphemesIterator<'a>;

    /// Same as `Graphemes`, except that it additionally will contain byte
    /// offset information (`(usize, &str)`).
    type GraphemeIndices<'a>: GraphemeIndicesIterator<'a>;
}
