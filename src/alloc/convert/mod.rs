#![doc = include_str!("README.md")]

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod convertible_identifier;
mod to_lower_camel;
mod to_lower_hybrid;
mod to_lower_kebab;
mod to_lower_snake;
mod to_upper_camel;
mod to_upper_hybrid;
mod to_upper_kebab;
mod to_upper_snake;

// -----------------------------------------------------------------------------
#[cfg(test)]
#[cfg(feature = "presets")]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
pub use convertible_identifier::*;
pub use to_lower_camel::*;
pub use to_lower_hybrid::*;
pub use to_lower_kebab::*;
pub use to_lower_snake::*;
pub use to_upper_camel::*;
pub use to_upper_hybrid::*;
pub use to_upper_kebab::*;
pub use to_upper_snake::*;
