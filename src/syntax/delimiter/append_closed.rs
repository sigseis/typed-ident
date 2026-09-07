// =============================================================================
// TYPES
// =============================================================================

/// Denotes how append-closed a delimiter is.
///
/// This is really tricky to set properly unless you know what you are doing.
/// ***DO NOT GUESS!*** You should see [`Delimiter::APPEND_CLOSED`] for details
/// on how to set this properly.
///
/// [`Delimiter::APPEND_CLOSED`]: crate::syntax::delimiter::Delimiter::APPEND_CLOSED
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AppendClosed {
    /// The delimiter is only append-closed with an empty string on one side.
    ///
    /// All delimiters are at *least* append-closed with an empty string.
    Empty,

    /// The delimiter is append-closed even from the context of a fragment, as
    /// long as one of the following is true:
    ///
    /// * the fragment consists only of delimiter characters, *or...*
    /// * if it does contain in-profile non-delimiter characters, the profile in
    ///   question is also `Fragment` append-closed.
    Fragment,

    /// The delimiter is append-closed, even from the context of an identifier,
    /// as long as one of the following is true:
    ///
    /// * the identifier consists only of delimiter characters, *or...*
    /// * if it does contain in-profile non-delimiter characters, the profile in
    ///   question is also `Identifier` append-closed.
    Identifier,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl AppendClosed {
    /// If the append-closed property is >= `Fragment`.
    ///
    /// Usually from the context of making optimization decisions, we only care
    /// if the append-closed property is over a certain minimal promise.
    pub const fn at_least_fragment(self) -> bool {
        !matches!(self, Self::Empty)
    }

    /// If the append-closed property is >= `Identifier`.
    ///
    /// Usually from the context of making optimization decisions, we only care
    /// if the append-closed property is over a certain minimal promise.
    pub const fn at_least_identifier(self) -> bool {
        matches!(self, Self::Identifier)
    }
}
