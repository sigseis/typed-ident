// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::boundary::Options;

// =============================================================================
// TYPE
// =============================================================================

/// Introduces boundaries on all configurable boundary conditions.
///
/// That includes transition to/from ASCII digits, as well as `CAMEL` and `HAT`.
///
/// For more information, see the [`boundary`](crate::syntax::boundary#options) module.
pub enum AllBoundaries {}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Options for AllBoundaries {
    const CAMEL: bool = true;
    const HAT: bool = true;
    const DIGIT_TO_LOWER: bool = true;
    const DIGIT_TO_UPPER: bool = true;
    const LOWER_TO_DIGIT: bool = true;
    const UPPER_TO_DIGIT: bool = true;
}
