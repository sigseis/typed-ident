// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::{Boundary, GraphemeCase};

// =============================================================================
// TRAIT
// =============================================================================

/// A special version of boundary calculation where the boundary can be deduced
/// only by looking at the surrounding graphemes (prev, curr, next).
pub(crate) trait TrivialBoundary: Boundary {
    /// Tells us whether a boundary exists between `prev` and `curr` cases.
    fn is_boundary(prev: GraphemeCase, curr: GraphemeCase, next: Option<GraphemeCase>) -> bool;

    /// Tells us whether a boundary exists between `prev` and `curr` graphemes.
    ///
    /// Note that a boundary *CANNOT* exist if there's no `prev` grapheme, so
    /// that is required, however there could be a boundary formed by a run of
    /// graphemes that doesn't have a `next`.
    ///
    /// It is expected that all provided graphemes are **non-empty**.
    #[inline]
    fn is_boundary_str(prev: &str, curr: &str, next: Option<&str>) -> bool {
        Self::is_boundary(
            GraphemeCase::new(prev),
            GraphemeCase::new(curr),
            next.map(GraphemeCase::new),
        )
    }
}
