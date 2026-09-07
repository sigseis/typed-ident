#![doc = include_str!("README.md")]
// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
pub mod boundary;
mod char_case;
pub mod delimiter;
mod grapheme_case;
pub mod profile;
pub mod segmentation;
mod subset_of;

// =============================================================================
// RE-EXPORT
// =============================================================================

// -----------------------------------------------------------------------------
#[doc(inline)]
pub use boundary::Boundary;
pub(crate) use boundary::TrivialBoundary;
pub(crate) use char_case::*;
#[doc(inline)]
pub use delimiter::{Delimiter, UnitDelimiter};
pub(crate) use grapheme_case::*;
#[doc(inline)]
pub use profile::{CasedProfile, CharProfile, Profile};
#[doc(inline)]
pub use segmentation::Segmentation;
#[doc(inline)]
pub use subset_of::SubsetOf;
