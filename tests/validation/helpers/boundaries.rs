// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Boundaries {
    pub camel: bool,
    pub digit_to_lower: bool,
    pub digit_to_upper: bool,
    pub hat: bool,
    pub lower_to_digit: bool,
    pub upper_to_digit: bool,
}
