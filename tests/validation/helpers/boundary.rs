// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Boundary {
    Camel,
    DigitToLower,
    DigitToUpper,
    Hat,
    LowerToDigit,
    UpperToDigit,
}
