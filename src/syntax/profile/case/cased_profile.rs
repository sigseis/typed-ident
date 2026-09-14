// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::{Delimiter, Profile, SyntaxError};

// =============================================================================
// TRAITS
// =============================================================================

/// A trait which further distinguishes a profile as an adapter of another base
/// profile, with some new casing rules mixed in.
///
/// # Note
///
/// A `CasedProfile` should not also be a [`CharProfile`], it's either one or
/// the other (either it defines a base set of characters, *OR* it defines an
/// adapter for a provided set of base characters).
///
/// Defining both won't lead to logical errors, but it may not compile (due to
/// overlapping trait implementations), and at worst you may cause code bloat by
/// virtue of your profile being usable in places where it's not expected.
///
/// [`CharProfile`]: crate::syntax::profile::CharProfile
pub trait CasedProfile: Profile {
    /// Given a UTF-8 string, check whether or not it's a valid chunk.
    ///
    /// # Errors
    ///
    /// On error, [`SyntaxError`] will be returned. Since chunks are allowed to
    /// be empty, this should only ever be `SyntaxError::Format(byte_offset)`,
    /// where the `byte_offset` provided is the offset from the start of the
    /// string to the first invalid character.
    #[inline]
    fn is_chunk<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        // A simple default implementation of this is to check if the fragment
        // contains any delimiters, and then see if it's a valid fragment. It's
        // likely to be less-optimal than checking in one pass, but usually this
        // is fine because of how infrequently users are expected to use this.
        for (idx, c) in s.char_indices() {
            if D::is_delim(c) {
                return Err(SyntaxError::Format(idx));
            }
        }
        Self::is_fragment::<D>(s)
    }

    /// Given a UTF-8 string, check whether or not it's a valid fragment.
    ///
    /// # Errors
    ///
    /// On error, [`SyntaxError`] will be returned. Since fragments are allowed
    /// to be empty, this should only ever be `SyntaxError::Format(byte_offset)`,
    /// where the `byte_offset` provided is the offset from the start of the
    /// string to the first invalid character.
    fn is_fragment<D: Delimiter>(s: &str) -> Result<(), SyntaxError>;

    /// Given a UTF-8 string, check whether or not it's a valid identifier.
    ///
    /// Something that is a valid fragment is not necessarily a valid
    /// identifier. These two can disagree on many complex properties from one
    /// another aside from just the start character, so don't try to outsmart
    /// the implementation here.
    ///
    /// If you have a fragment and you want to see if it's a valid identifier,
    /// you should always call [`is_ident_fragment`], which will attempt to not
    /// redo unnecessary checks (where possible).
    ///
    /// [`is_ident_fragment`]: Self::is_ident_fragment
    ///
    /// # Errors
    ///
    /// On error, [`SyntaxError`] will be returned. Since identifiers are *NOT*
    /// allowed to be empty, this ***MUST*** return `SyntaxError::Empty` on an
    /// empty string. If something else is wrong with the format, it may also
    /// return a `SyntaxError::Format(byte_offset)` pointing to the location of
    /// the byte within the string that caused the failure.
    fn is_ident<D: Delimiter>(s: &str) -> Result<(), SyntaxError>;

    /// Given a UTF-8 string that is known to pass [`is_fragment`], check
    /// whether or not it's a valid identifier.
    ///
    /// Some implementations can optimize this instead of having to check the
    /// entire string again. However, you should be sure to only call this
    /// function when you are *certain* that the string passes [`is_fragment`].
    ///
    /// [`is_fragment`]: Self::is_fragment
    #[inline(always)]
    fn is_ident_fragment<D: Delimiter>(fragment: &str) -> Result<(), SyntaxError> {
        // It's always appropriate to check this again to see if it's valid.
        // Though it might be less optimal depending on the cased profile.
        Self::is_ident::<D>(fragment)
    }

    /// Given a UTF-8 string that is known to pass [`is_ident`], check whether
    /// or not the provided `mid` would be a split point that would leave the
    /// left-hand side either empty, or would still pass an [`is_ident`] check.
    ///
    /// Some implementations may check additional properties, like for unicode
    /// values that they properly use things like `ZWJ` and `ZWNJ` characters.
    /// So we cannot assume that just any index is valid to split on.
    ///
    /// [`is_ident`]: Self::is_ident
    fn is_ident_split_boundary<D: Delimiter>(ident: &str, mid: usize) -> bool;
}
