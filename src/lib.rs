#![doc = include_str!("README.md")]
// =============================================================================
// CRATE CONFIG
// =============================================================================

// -----------------------------------------------------------------------------
#![allow(clippy::module_inception)] // Common pattern for module organization.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]
#![no_std]

// =============================================================================
// CRATES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
extern crate alloc as std_alloc;

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[macro_use]
mod macros;

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
mod alloc;
mod core;

// -----------------------------------------------------------------------------
pub mod presets;
pub mod syntax;

// =============================================================================
// CONSTANTS
// =============================================================================

// -----------------------------------------------------------------------------
/// The version of Unicode used to generate functionality provided by this crate.
///
/// Specifically we provide the calls for:
///
/// * `is_combining_mark`
/// * `is_titlecase`
/// * `is_titlecase_greek_variant`
///
/// # What Does This Mean for Combining Marks?
///
/// This could change across versions, but it only impacts one profile.
///
/// * **Profiles Impacted:** [`Strict`]
/// * **What Changes:** What may or may not be considered valid for different
///   kinds of identifiers involving combining marks at the start of a chunk.
///
/// [`Strict`]: crate::syntax::profile::Strict
/// [`Unicode`]: crate::syntax::profile::Unicode
///
/// # What Does This Mean For Titlecase?
///
/// Titlecase characters don't tend to change very frequently (if ever).
///
/// * **Profiles Impacted:** [`Strict`], [`Unicode`]
/// * **What Changes:** What may or may not be considered valid for different
///   kinds of identifiers involving titlecase characters, or it may change
///   where some boundary splits happen involving titlecase characters.
#[cfg(feature = "unicode")]
pub const UNICODE_VERSION: (u32, u32, u32) = syntax::generated::UNICODE_VERSION;

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
pub use alloc::*;
pub use core::*;
