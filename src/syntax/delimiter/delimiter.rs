// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::delimiter::AppendClosed;
use core::fmt::Debug;
use core::hash::Hash;

// =============================================================================
// TYPES
// =============================================================================

/// A trait for signifying that a type can be used as a delimiter in an
/// identifier.
///
/// # Must Be a Char!
///
/// A delimiter must be exactly one [`char`] (one unicode code point).
///
/// If joining characters are present to the left or right of a delimiter that,
/// this crate will consider them incomplete parts of the surrounding chunk,
/// *NOT* a part of the delimiter (even if they visually appear that way).
///
/// # Must be Optional!
///
/// A valid identifier is allowed to have no delimiters whatsoever.
///
/// This crate does not enforce the structure of an identifier past validating
/// start and continue characters. If you need something more explicit, like an
/// identifier that *MUST* start with a specific character, it's recommended you
/// wrap the identifier in a new-type and handle that validation explicitly.
///
/// For example, for PHP it would be recommended to do the following:
///
/// ```
/// # #[derive(Copy, Clone, Debug)]
/// # enum MyError { NoDollarStart, FormatError }
/// # impl From<typed_ident::Error> for MyError {
/// #     fn from(_: typed_ident::Error) -> Self {
/// #         Self::FormatError
/// #     }
/// # }
/// use typed_ident::presets::ascii::CamelIdent;
///
/// struct PhpVariable<'a>(&'a CamelIdent);
///
/// impl<'a> PhpVariable<'a> {
///     pub fn new(s: &'a str) -> Result<Self, MyError> {
///         let Some(remaining) = s.strip_prefix('$') else {
///             return Err(MyError::NoDollarStart);
///         };
///         Ok(Self(CamelIdent::new(remaining)?))
///     }
/// }
/// ```
///
/// # Must *NOT* Be Stateful!
///
/// A delimiter character must not be different depending on whether it's been
/// parsed from [`is_ident_start`] vs. [`is_chunk_delim`].
///
/// If you were to use stateful delimiters, then you might be surprised by the
/// values returned by segmentation (they may not be what you expect). It will
/// still segment chunks properly, but the delimiter values may be strange.
///
/// For example, this would be an ***INCORRECT*** implementation:
///
/// ```
/// # use typed_ident::syntax::delimiter::Delimiter;
/// #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// enum BadDelimiter {
///     FoundAtStart,
///     FoundInChunk,
/// }
///
/// impl Delimiter for BadDelimiter {
///     fn as_char(&self) -> char {
///         '_'
///     }
///     fn from_ident_start(c: char) -> Option<Self> {
///         if c == '_' {
///             Some(Self::FoundAtStart)
///         } else {
///             None
///         }
///     }
///     fn from_chunk_delim(c: char) -> Option<Self> {
///         if c == '_' {
///             Some(Self::FoundInChunk)
///         } else {
///             None
///         }
///     }
/// }
/// ```
///
/// [`is_chunk_delim`]: Delimiter::is_chunk_delim
/// [`is_ident_start`]: Delimiter::is_ident_start
pub trait Delimiter:
    Copy + Clone + Debug + Eq + Hash + Ord + PartialEq + PartialOrd + Sized
{
    /// Whether or not a fragment containing only this type of delimiters would
    /// be considered append-closed.
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
    /// # When Is a Delimiter `Fragment` Append-Closed?
    ///
    /// Simply put, if [`is_chunk_delim`] is a superset (or equal to) the set of
    /// valid characters for [`is_ident_start`], then set this to `Fragment`.
    ///
    /// Imagine we represent delimiter characters in a fragment as:
    ///
    /// * `I` = Ident Start Delimiter
    /// * `D` = Chunk Delimiter
    ///
    /// Abstractly, all valid fragments containing only delimiters would be:
    ///
    /// * `` `` and `ID*` (empty string, or `I` followed by one or more `D`)
    ///
    /// A valid fragment of delimiters may be `IDD`, after appending some other
    /// string we may be left with `IDDID`, but if `I ⊆ D`, then we can
    /// interpret any `I` character as a `D` character, and thus it's
    /// syntactically `IDDDD` (which is still valid and within this set).
    ///
    /// # When Is a Delimiter `Identifier` Append-Closed?
    ///
    /// This is a wider promise, which states that if the delimiter were used
    /// with a profile that *also* claimed to be `Identifier` append-closed,
    /// then even identifiers themselves would be append-closed.
    ///
    /// Simply put, the delimiter needs to have the following properties:
    ///
    /// * It must be `Fragment` append-closed (see above), *and...*
    /// * [`is_ident_start`] must be identical to [`is_chunk_delim`].
    ///
    /// # What Happens If This Is Set Incorrectly?
    ///
    /// If this is set incorrectly, then it may be possible for some invalid
    /// fragments (and thus, identifiers) to be formed. It cannot lead to any
    /// memory issues or anything like that, just logical issues.
    ///
    /// [`is_chunk_delim`]: Delimiter::is_chunk_delim
    /// [`is_ident_start`]: Delimiter::is_ident_start
    const APPEND_CLOSED: AppendClosed = AppendClosed::Empty;

    /// Returns the original character representation of this delimiter value.
    #[must_use]
    fn as_char(&self) -> char;

    /// Finds the next index that a delimiter would be present on.
    ///
    /// # Returns
    ///
    /// If a delimiter could be found, it is returned with the byte index that
    /// the delimiter exists at. If there are no remaining delimiters, `None` is
    /// returned.
    #[must_use]
    #[inline]
    fn find_delimiter(fragment: &str) -> Option<usize> {
        fragment.find(Self::is_delim)
    }

    /// Finds the next index from the back that a delimiter would be present on.
    ///
    /// # Returns
    ///
    /// If a delimiter could be found, it is returned with the byte index that
    /// the delimiter exists at. If there are no remaining delimiters, `None` is
    /// returned.
    #[must_use]
    #[inline]
    fn rfind_delimiter(fragment: &str) -> Option<usize> {
        fragment.rfind(Self::is_delim)
    }

    /// Attempts to convert a character to any valid delimiter value.
    ///
    /// This should always be a superset (or equal-to) both [`from_ident_start`]
    /// *AND* [`from_chunk_delim`].
    ///
    /// [`from_chunk_delim`]: Delimiter::from_chunk_delim
    /// [`from_ident_start`]: Delimiter::from_ident_start
    #[must_use]
    #[inline]
    fn from_char(c: char) -> Option<Self> {
        Self::from_ident_start(c).or_else(|| Self::from_chunk_delim(c))
    }

    /// Attempts to convert a character to a delimiter which is valid only at
    /// the absolute start of an identifier.
    ///
    /// # Important
    ///
    /// This is at the start of an *identifier*, not at the start of a
    /// *fragment*. Valid at fragment-start is defined by [`from_char`].
    ///
    /// However, note that it doesn't *require* presence. So it would be bad to
    /// depend on this for a required character (like PHP's dollar sign).
    ///
    /// [`from_char`]: Delimiter::from_char
    #[must_use]
    fn from_ident_start(c: char) -> Option<Self>;

    /// Attempts to convert a character to a delimiter which is valid at any
    /// position other than the absolute start of an identifier.
    #[must_use]
    fn from_chunk_delim(c: char) -> Option<Self>;

    /// Whether or not the provided character is a delimiter.
    ///
    /// e.g. this should always be true:
    ///
    /// * `assert_eq!(D::from_char(c).is_some(), D::is_delim(c))`
    #[must_use]
    #[inline]
    fn is_delim(c: char) -> bool {
        Self::from_char(c).is_some()
    }

    /// Whether or not the provided character is a valid ident-start delimiter.
    ///
    /// e.g. this should always be true:
    ///
    /// * `assert_eq!(D::from_ident_start(c).is_some(), D::is_ident_start(c))`
    #[must_use]
    #[inline]
    fn is_ident_start(c: char) -> bool {
        Self::from_ident_start(c).is_some()
    }

    /// Whether or not the provided character is a valid chunk delimiter.
    ///
    /// e.g. this should always be true:
    ///
    /// * `assert_eq!(D::from_chunk_delim(c).is_some(), D::is_chunk_delim(c))`
    #[must_use]
    #[inline]
    fn is_chunk_delim(c: char) -> bool {
        Self::from_chunk_delim(c).is_some()
    }
}
