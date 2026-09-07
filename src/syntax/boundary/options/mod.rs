//! Options for configuring how the provided boundary breaking algorithm works.
//!
//! For more information, see the [`boundary`](crate::syntax::boundary) module.

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod all_boundaries;
mod default;
mod no_boundaries;
mod options;

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub use all_boundaries::*;
pub use default::*;
pub use no_boundaries::*;
pub use options::*;
