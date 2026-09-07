// =============================================================================
// TYPES
// =============================================================================

/// Denotes how append-closed a profile is.
///
/// This is really tricky to set properly unless you know what you are doing.
/// ***DO NOT GUESS!*** You should see [`Profile::APPEND_CLOSED`] for details
/// on how to set this properly.
///
/// [`Profile::APPEND_CLOSED`]: crate::syntax::profile::Profile::APPEND_CLOSED
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AppendClosed {
    /// The profile is only append-closed with an empty string on one side.
    ///
    /// All profiles are at *least* append-closed with an empty string.
    Empty,

    /// The profile is only append-closed from the context of a chunk (a run
    /// of characters containing only profile characters, and no delimiters).
    Chunk,

    /// The profile is append-closed even from the context of a fragment, as
    /// long as one of the following is true:
    ///
    /// * the fragment consists only of non-delimiter characters, *or...*
    /// * if it does contain delimiter characters, the delimiter in question is
    ///   also `Fragment` append-closed.
    Fragment,

    /// The profile is append-closed even from the context of an identifier, as
    /// long as one of the following is true:
    ///
    /// * the identifier consists only of non-delimiter characters, *or...*
    /// * if it does contain delimiter characters, the delimiter in question is
    ///   also `Identifier` append-closed.
    Identifier,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl AppendClosed {
    /// If the append-closed property is >= `Chunk`.
    ///
    /// Usually from the context of making optimization decisions, we only care
    /// if the append-closed property is over a certain minimal promise.
    pub const fn at_least_chunk(self) -> bool {
        !matches!(self, Self::Empty)
    }

    /// If the append-closed property is >= `Fragment`.
    ///
    /// Usually from the context of making optimization decisions, we only care
    /// if the append-closed property is over a certain minimal promise.
    pub const fn at_least_fragment(self) -> bool {
        match self {
            Self::Empty | Self::Chunk => false,
            Self::Fragment | Self::Identifier => true,
        }
    }

    /// If the append-closed property is >= `Identifier`.
    ///
    /// Usually from the context of making optimization decisions, we only care
    /// if the append-closed property is over a certain minimal promise.
    pub const fn at_least_identifier(self) -> bool {
        matches!(self, Self::Identifier)
    }
}
