// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub use clap::ValueEnum;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    Camel,
    CasedCamel,
    UpperCamel,
    LowerCamel,
    Snake,
    CasedSnake,
    UpperSnake,
    LowerSnake,
    Kebab,
    CasedKebab,
    UpperKebab,
    LowerKebab,
    #[default]
    Hybrid,
    CasedHybrid,
    UpperHybrid,
    LowerHybrid,
}
