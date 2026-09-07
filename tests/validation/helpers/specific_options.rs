// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use typed_ident::syntax::boundary::Options;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub struct SpecificOptions<
    const CAMEL: bool,
    const HAT: bool,
    const DIGIT_TO_LOWER: bool,
    const DIGIT_TO_UPPER: bool,
    const LOWER_TO_DIGIT: bool,
    const UPPER_TO_DIGIT: bool,
>(());

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<
    const CAMEL: bool,
    const HAT: bool,
    const DIGIT_TO_LOWER: bool,
    const DIGIT_TO_UPPER: bool,
    const LOWER_TO_DIGIT: bool,
    const UPPER_TO_DIGIT: bool,
> Options
    for SpecificOptions<CAMEL, HAT, DIGIT_TO_LOWER, DIGIT_TO_UPPER, LOWER_TO_DIGIT, UPPER_TO_DIGIT>
{
    const CAMEL: bool = CAMEL;
    const HAT: bool = HAT;

    const DIGIT_TO_LOWER: bool = DIGIT_TO_LOWER;
    const DIGIT_TO_UPPER: bool = DIGIT_TO_UPPER;
    const LOWER_TO_DIGIT: bool = LOWER_TO_DIGIT;
    const UPPER_TO_DIGIT: bool = UPPER_TO_DIGIT;
}
