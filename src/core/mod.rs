// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
pub mod chunk;
mod error;
pub mod fmt;
pub mod fragment;
mod ident;
mod identifier;
pub(crate) mod pattern;
mod segment;
pub(crate) mod slice_index;

// =============================================================================
// Common Helper Definition
// =============================================================================

/// A borrowed, type-erased representation of a [`Segment`].
pub type StrSegment<'a> = Segment<char, &'a str>;

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
#[doc(inline)]
pub use chunk::Chunk;
pub use error::*;
#[doc(inline)]
pub use fmt::FormattableIdentifier;
#[doc(inline)]
pub use fragment::Fragment;
pub use ident::*;
pub use identifier::*;
pub use segment::*;
