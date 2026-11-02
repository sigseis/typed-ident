// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod impl_ascii_flat_line_delimiter;
#[cfg(feature = "alloc")]
mod impl_boxed_slice_traits;
mod impl_char_indices_iterator;
mod impl_chars_iterator;
mod impl_displayable_type;
mod impl_match_indices_iterator;
mod impl_matches_iterator;
mod impl_typed_slice_functions;
mod impl_typed_slice_traits;
#[cfg(test)]
#[cfg(feature = "unicode")]
mod test_as_ref;

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub(crate) use impl_ascii_flat_line_delimiter::*;
#[cfg(feature = "alloc")]
pub(crate) use impl_boxed_slice_traits::*;
pub(crate) use impl_char_indices_iterator::*;
pub(crate) use impl_chars_iterator::*;
pub(crate) use impl_displayable_type::*;
pub(crate) use impl_match_indices_iterator::*;
pub(crate) use impl_matches_iterator::*;
pub(crate) use impl_typed_slice_functions::*;
pub(crate) use impl_typed_slice_traits::*;
#[cfg(test)]
#[cfg(feature = "unicode")]
pub(crate) use test_as_ref::*;
