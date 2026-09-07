#![doc = include_str!("README.md")]
// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
pub mod r#char;
#[cfg(feature = "unicode")]
pub mod grapheme;
mod grapheme_indices_iterator;
mod graphemes_iterator;
mod segmentation;

// =============================================================================
// RE-EXPORT
// =============================================================================

// -----------------------------------------------------------------------------
pub use r#char::Char;
#[cfg(feature = "unicode")]
pub use grapheme::Grapheme;
pub use grapheme_indices_iterator::*;
pub use graphemes_iterator::*;
pub use segmentation::*;
