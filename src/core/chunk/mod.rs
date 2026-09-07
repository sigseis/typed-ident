//! Utilities for the [`Chunk`] type.

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod chunk;
mod word_indices;
mod word_str_indices;
mod word_strs;
mod words;

// =============================================================================
// TRIVIAL ITERATORS
// =============================================================================

// -----------------------------------------------------------------------------
impl_chars_iterator! {
    name=Chars,
    over=Chunk,
    function=chars,
}

// -----------------------------------------------------------------------------
impl_char_indices_iterator! {
    name=CharIndices,
    over=Chunk,
    function=char_indices,
}

// -----------------------------------------------------------------------------
impl_matches_iterator! {
    name=Matches,
    over=Chunk,
    function=matches,
    docs=concat!(
        "An iterator over matches of a chunk.",
        "\n\n",
        "This struct is created with the [`matches`] method on [`Chunk`]. ",
        "See its documentation for more.",
        "\n\n",
        "[`matches`]: Chunk::matches\n",
    ),
}

// -----------------------------------------------------------------------------
impl_matches_iterator! {
    name=RMatches,
    over=Chunk,
    function=rmatches,
    docs=concat!(
        "An iterator over the reverse matches of a chunk.",
        "\n\n",
        "This struct is created with the [`rmatches`] method on [`Chunk`]. ",
        "See its documentation for more.",
        "\n\n",
        "[`rmatches`]: Chunk::rmatches\n",
    ),
}

// -----------------------------------------------------------------------------
impl_match_indices_iterator! {
    name=MatchIndices,
    over=Chunk,
    function=match_indices,
    docs=concat!(
        "An iterator over matches of a chunk, and their positions.",
        "\n\n",
        "This struct is created with the [`match_indices`] method on [`Chunk`]. ",
        "See its documentation for more.",
        "\n\n",
        "[`match_indices`]: Chunk::match_indices\n",
    ),
}

// -----------------------------------------------------------------------------
impl_match_indices_iterator! {
    name=RMatchIndices,
    over=Chunk,
    function=rmatch_indices,
    docs=concat!(
        "An iterator over the reverse matches of a chunk, and their positions.",
        "\n\n",
        "This struct is created with the [`rmatch_indices`] method on [`Chunk`]. ",
        "See its documentation for more.",
        "\n\n",
        "[`rmatch_indices`]: Chunk::rmatch_indices\n",
    ),
}

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub use chunk::*;
pub use word_indices::*;
pub use word_str_indices::*;
pub use word_strs::*;
pub use words::*;
