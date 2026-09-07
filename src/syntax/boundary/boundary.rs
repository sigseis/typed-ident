// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::Segmentation;
use core::num::NonZero;

// =============================================================================
// TRAIT
// =============================================================================

/// Defines how chunks are split during segmentation.
///
/// This trait is defined in terms of `&str`, despite acting on chunk data. This
/// is to reduce the number of functions that need to be generated. There's no
/// real need to have the `Chunk` type information, so we just cast to a `&str`
/// before passing data into these functions.
///
/// # About Segmentation
///
/// The functions on this type are all passed a [`Segmentation`] type parameter.
///
/// The reason for this is - the profile dictates the ***code*** segmentation
/// properties, but the boundary dictates the ***chunk*** segmentation
/// properties. As such, [`Profile`] needs to feed the segmentation type into
/// these functions.
///
/// [`Profile`]: crate::syntax::profile::Profile
/// [`Segmentation`]: crate::syntax::segmentation::Segmentation
pub trait Boundary {
    /// Whether or not this boundary definition could *ever* find a boundary
    /// within any provided, valid chunk.
    ///
    /// For all practical purposes, if you are implementing a custom boundary,
    /// this should be set to `true`, so it is defaulted to that. It exists as a
    /// way to optimize some operations when there's provable no boundaries.
    const CAN_FIND_BOUNDARIES: bool = true;

    /// Finds the next index that a boundary should be introduced on.
    ///
    /// # Returns
    ///
    /// If a boundary could be found, it is returned with the byte index that
    /// the boundary exists at. If there are no remaining boundaries, `None` is
    /// returned.
    ///
    /// This function should never return `Some(s.len())`. If it does, then some
    /// functionality may not work as expected.
    fn find_boundary<S: Segmentation>(chunk: &str) -> Option<NonZero<usize>>;

    /// Finds the next index from the back of the string that a boundary should
    /// be introduced on.
    ///
    /// # Returns
    ///
    /// If a boundary could be found, it is returned with the byte index that
    /// the boundary exists at. If there are no remaining boundaries, `None` is
    /// returned.
    ///
    /// This function should never return `Some(s.len())`. If it does, then some
    /// functionality may not work as expected.
    ///
    /// # Important
    ///
    /// This should be implemented in a way, such that it identifies the same
    /// boundaries as `find_boundary`, just starting from the back of the string
    /// instead.
    ///
    /// If it's not implemented that way, how your chunk will split could change
    /// depending on whether you are forward or reverse iterating.
    fn rfind_boundary<S: Segmentation>(chunk: &str) -> Option<NonZero<usize>>;

    /// Returns whether a boundary exists at a given byte offset of a string.
    ///
    /// # Returns
    ///
    /// Returns `true` if a chunk boundary exists within a provided chunk at a
    /// given index. Returns false otherwise.
    ///
    /// This function should never return `true` at index `0` or `s.len()`. If
    /// it does, then some functionality may not work as expected.
    ///
    /// # Panics
    ///
    /// This will usually perform a string splitting operation at `idx`, so if
    /// you provide an index which is out of bounds, *or* that doesn't align on
    /// a UTF-8 sequence boundary, this will likely panic.
    fn has_boundary_at<S: Segmentation>(chunk: &str, idx: usize) -> bool;
}
