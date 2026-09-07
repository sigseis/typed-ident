//! Utilities for the [`Fragment`] type.

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod chunked_segment_indices;
mod chunked_segments;
mod chunked_str_segment_indices;
mod chunked_str_segments;
mod fragment;
mod segment_indices;
mod segments;
mod str_segment_indices;
mod str_segments;

// =============================================================================
// TRIVIAL ITERATORS
// =============================================================================

// -----------------------------------------------------------------------------
impl_chars_iterator! {
    name=Chars,
    over=Fragment,
    function=chars,
}

// -----------------------------------------------------------------------------
impl_char_indices_iterator! {
    name=CharIndices,
    over=Fragment,
    function=char_indices,
}

// -----------------------------------------------------------------------------
impl_matches_iterator! {
    name=Matches,
    over=Fragment,
    function=matches,
    docs=concat!(
        "An iterator over matches of a fragment.",
        "\n\n",
        "This struct is created with the [`matches`] method on [`Fragment`]. ",
        "See its documentation for more.",
        "\n\n",
        "[`matches`]: Fragment::matches\n",
    ),
}

// -----------------------------------------------------------------------------
impl_matches_iterator! {
    name=RMatches,
    over=Fragment,
    function=rmatches,
    docs=concat!(
        "An iterator over the reverse matches of a fragment.",
        "\n\n",
        "This struct is created with the [`rmatches`] method on [`Fragment`]. ",
        "See its documentation for more.",
        "\n\n",
        "[`rmatches`]: Fragment::rmatches\n",
    ),
}

// -----------------------------------------------------------------------------
impl_match_indices_iterator! {
    name=MatchIndices,
    over=Fragment,
    function=match_indices,
    docs=concat!(
        "An iterator over matches of a fragment, and their positions.",
        "\n\n",
        "This struct is created with the [`match_indices`] method on [`Fragment`]. ",
        "See its documentation for more.",
        "\n\n",
        "[`match_indices`]: Fragment::match_indices\n",
    ),
}

// -----------------------------------------------------------------------------
impl_match_indices_iterator! {
    name=RMatchIndices,
    over=Fragment,
    function=rmatch_indices,
    docs=concat!(
        "An iterator over the reverse matches of a fragment, and their positions.",
        "\n\n",
        "This struct is created with the [`rmatch_indices`] method on [`Fragment`]. ",
        "See its documentation for more.",
        "\n\n",
        "[`rmatch_indices`]: Fragment::rmatch_indices\n",
    ),
}

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub use chunked_segment_indices::*;
pub use chunked_segments::*;
pub use chunked_str_segment_indices::*;
pub use chunked_str_segments::*;
pub use fragment::*;
pub use segment_indices::*;
pub use segments::*;
pub use str_segment_indices::*;
pub use str_segments::*;
