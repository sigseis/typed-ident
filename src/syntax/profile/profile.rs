// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::Segmentation;
use crate::syntax::profile::{AppendClosed, CharProfile};

// =============================================================================
// TYPES
// =============================================================================

/// Defines the valid composition of characters for a chunk (a slice of an
/// identifier that contains no delimiters).
///
/// # Works on Character Code Points
///
/// Character profiles are defined in accordance with the default profile in
/// [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/), which
/// defines validation along a string of character code-points, not necessarily
/// along a string of valid graphemes.
///
/// If you need more complex validation, such as the emoji profile which must
/// validate a specific ordering of special characters for emoji handling, you
/// need to use a wrapper type to add that extra validation.
pub trait Profile: Sized {
    /// Whether or not a fragment containing characters in this profile are
    /// append-closed.
    ///
    /// It's always valid to set this to `Empty`, it just will lead to less
    /// optimal codegen for your identifier.
    ///
    /// # What Is Append-Closed?
    ///
    /// Append-closed is a string property, where if we consider a set that
    /// contains all valid permutations of a string, appending one value from
    /// that set onto another equals some other value from the set.
    ///
    /// It's called "Append-Closed" because the operation we're doing is
    /// "appending" one value onto another, and the operation will never result
    /// in a value outside of the set (e.g. the set is closed given this
    /// operation).
    ///
    /// # When Is a Profile `Chunk` Append-Closed?
    ///
    /// Simply put, if [`is_chunk_continue`] is a superset (or equal to) the set
    /// of valid characters for [`is_ident_start`] *and* [`is_chunk_start`],
    /// then you can set this to `Chunk`.
    ///
    /// Imagine we represent profile characters in a chunk as:
    ///
    /// * `S` = Chunk/Ident Start Character
    /// * `C` = Chunk Continue Character
    ///
    /// Abstractly, all valid chunks would be:
    ///
    /// * `` `` and `SC*` (empty string, or `S` followed by one or more `C`)
    ///
    /// A valid chunk of characters may be `SCC`, after appending some other
    /// string we may be left with `SCCSC`, but if `S ⊆ C`, then we can
    /// interpret any `S` character as a `C` character, and thus it's
    /// syntactically `SCCCC` (which is still valid and within this set).
    ///
    /// # When Is a Profile `Fragment` Append-Closed?
    ///
    /// This is a wider promise, which states that when used with a delimiter
    /// that is `APPEND_CLOSED`, then all valid fragments are themselves also
    /// append-closed (regardless of whether they contain delimiters or not).
    ///
    /// Simply put, the profile needs to have the following properties:
    ///
    /// * It must be `Chunk` append-closed (see above), *and...*
    /// * [`is_chunk_start`] must be identical to [`is_chunk_continue`].
    ///
    /// It's easiest to understand this by looking at a case where it's *not*
    /// true - `UpperCamel` (or `lowerCamel`, but let's just focus on upper).
    ///
    /// Two valid `UpperCamel` fragments are:
    ///
    /// * `Upper_` (a perfectly valid start, but also has a trailing delimiter).
    /// * `camel` (a continuation of a chunk, not the start of one)
    ///
    /// If we append one onto the other, it may lead to an invalid fragment:
    ///
    /// * `Upper_` + `camel` = `Upper_camel` *(which is not valid)*
    ///
    /// # When Is a Profile `Identifier` Append-Closed?
    ///
    /// This is a wider promise, which states that if the profile were used
    /// with a delimiter that *also* claimed to be `Identifier` append-closed,
    /// then even identifiers themselves would be append-closed.
    ///
    /// Simply put, the profile needs to have the following properties:
    ///
    /// * It must be `Fragment` append-closed (see above), *and...*
    /// * [`is_ident_start`] must be identical to [`is_chunk_start`] *and*
    ///   [`is_chunk_continue`].
    ///
    /// # What Happens If This Is Set Incorrectly?
    ///
    /// If this is set incorrectly, then it may be possible for some invalid
    /// fragments (and thus, identifiers) to be formed. It cannot lead to any
    /// memory issues or anything like that, just logical issues.
    ///
    /// [`is_chunk_continue`]: Profile::is_chunk_continue
    /// [`is_chunk_start`]: Profile::is_chunk_start
    /// [`is_ident_start`]: Profile::is_ident_start
    const APPEND_CLOSED: AppendClosed = AppendClosed::Empty;

    /// The underlying profile that this profile is based on.
    ///
    /// This can be useful in identifying the core underlying profile for a type
    /// in a generic way (instead of handling each profile type specially). It
    /// comes in handy when there are case-conversions already present overtop
    /// of a profile.
    ///
    /// # Implementation Suggestion
    ///
    /// If you are defining a *new* profile (e.g., like [`Ascii`], [`Unicode`]),
    /// then you should set this to `Self`. If you are defining a new
    /// `CasedProfile` (e.g., like [`Lower`], [`Upper`], etc). This should be
    /// set to the profile that the cased profile wraps.
    ///
    /// [`Ascii`]: crate::syntax::profile::Ascii
    /// [`Unicode`]: crate::syntax::profile::Unicode
    /// [`Lower`]: crate::syntax::profile::Lower
    /// [`Upper`]: crate::syntax::profile::Upper
    type BaseProfile: CharProfile;

    /// The segmentation strategy that this profile uses.
    ///
    /// Depending on the valid characters, different segmentation strategies
    /// could apply. For instance, the [`Ascii`] profile rightly uses the
    /// [`Char`] segmentation strategy (because it's safe for it to).
    ///
    /// The recommendation is:
    ///
    /// * If your profile only consists of ASCII characters, use [`Char`].
    /// * Otherwise you should use the [`Grapheme`] strategy.
    ///
    /// # What Happens If This Is Set Incorrectly?
    ///
    /// If this is set incorrectly, validation will not be impacted, but things
    /// relating to segmentation would be (so any of the [`segments`]-like
    /// methods may appear to be missing boundaries, for instance).
    ///
    /// [`Ascii`]: crate::syntax::profile::Ascii
    /// [`Char`]: crate::syntax::segmentation::Char
    /// [`Grapheme`]: crate::syntax::segmentation::Grapheme
    /// [`segments`]: crate::core::fragment::Fragment::segments
    type Segmentation: Segmentation;

    /// Whether or not the provided character can appear at the absolute start
    /// of an identifier.
    ///
    /// # Important
    ///
    /// This is at the start of an *identifier*, not at the start of a
    /// *fragment* or *chunk*. Valid at fragment-start is defined by
    /// [`in_profile`]. Valid at chunk-start is defined by [`is_chunk_start`].
    ///
    /// However, note that it doesn't *require* presence. So it would be bad to
    /// depend on this for a required character (like PHP's dollar sign).
    ///
    /// [`in_profile`]: Profile::in_profile
    /// [`is_chunk_start`]: Profile::is_chunk_start
    fn is_ident_start(c: char) -> bool;

    /// Whether or not the provided character can appear at the start of a new
    /// chunk (e.g. right after a delimiter).
    ///
    /// # Important
    ///
    /// This is at the start of a *chunk*, not at the start of a *fragment* or
    /// *identifier*. Valid at fragment-start is defined by [`in_profile`].
    /// Valid at identifier-start is defined by [`is_ident_start`].
    ///
    /// However, note that it doesn't *require* presence. So it would be bad to
    /// depend on this for a required character (like PHP's dollar sign).
    ///
    /// [`in_profile`]: Profile::in_profile
    /// [`is_ident_start`]: Profile::is_ident_start
    fn is_chunk_start(c: char) -> bool;

    /// Whether or not the provided character can appear at any point in a
    /// chunk.
    ///
    /// This should always be a superset (or equal-to) [`is_ident_start`],
    /// [`is_chunk_start`], *AND* [`is_chunk_continue`].
    ///
    /// [`is_chunk_start`]: Profile::is_chunk_start
    /// [`is_ident_start`]: Profile::is_ident_start
    /// [`is_chunk_continue`]: Profile::is_chunk_continue
    #[inline]
    fn in_profile(c: char) -> bool {
        Self::is_ident_start(c) || Self::is_chunk_start(c) || Self::is_chunk_continue(c)
    }

    /// Whether or not the provided character is valid at any position after the
    /// first character of a chunk (a run of non-delimiter characters).
    fn is_chunk_continue(c: char) -> bool;
}
