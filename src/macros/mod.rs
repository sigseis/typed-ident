// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[macro_use]
mod impl_ascii_flat_line_delimiter;
#[macro_use]
mod impl_match_indices_iterator;
#[macro_use]
mod impl_matches_iterator;
#[macro_use]
mod impl_chars_iterator;
#[macro_use]
mod impl_char_indices_iterator;
#[macro_use]
mod impl_typed_slice_cmp;
#[macro_use]
mod impl_typed_slice_common;
#[macro_use]
mod impl_typed_slice_traits;
#[macro_use]
#[cfg(test)]
#[cfg(feature = "unicode-strict")]
mod test_as_ref_comprehensive;
