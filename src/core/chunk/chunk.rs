// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "chunk.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Fragment;
use crate::core::chunk::{
    CharIndices, Chars, MatchIndices, Matches, RMatchIndices, RMatches, WordIndices, Words,
};
use crate::core::error::{Error, ErrorKind};
use crate::syntax::{Boundary, CasedProfile, Delimiter};

// =============================================================================
// TYPES
// =============================================================================

/// An immutable UTF-8 encoded slice of an [`Ident`] which contains no
/// delimiters.
///
/// This type can be returned during segmentation operations on a fragment or
/// identifier (such as [`chunked_segments`], [`segments`], and their
/// `*_indices` variants).
///
/// Since a chunk is a subset of a fragment, it can also be represented as a
/// fragment if need-be (as it implements `Deref`).
///
/// [`chunked_segments`]: Fragment::chunked_segments
/// [`segments`]: Fragment::segments
///
/// # Construction
///
/// It's not usually recommended that you construct a chunk directly, usually it
/// will be constructed for you as a result of doing some operation that returns
/// a chunk (like the aforementioned [`segments`] method).
///
/// However, if you *do* want to construct a chunk, it is expected that you do
/// so through some type alias which fully defines the chunk.
///
/// The common way to get a reasonable alias is through the [`presets`] module.
///
/// [`presets`]: crate::presets
///
/// ```
/// use typed_ident::presets::unicode::lower_camel::*;
///
/// // This appears to be a valid identifier, in this case also a valid chunk!
/// let chunk = LowerCamelChunk::new("lowerCamel")?;
///
/// // But not all chunks will appear to be valid identifiers.
/// // Take for instance following, which is a slice of `lowerCamel`.
/// // It appears to instead be `UpperCamel` - a valid lower-camel chunk.
/// let chunk = LowerCamelChunk::new("Camel")?;
/// # Ok::<(), typed_ident::Error>(())
/// ```
///
/// These presets are all just type aliases to `Chunk` with the generic type
/// parameters filled-in. If you would like to define your own custom chunk,
/// it's recommended that you do so by defining your own type alias.
///
/// ```
/// use typed_ident::Chunk;
/// use typed_ident::syntax::{boundary, delimiter, profile};
///
/// // An ASCII chunk using the uppercase ASCII profile.
/// // Any ASCII punctuation is rejected as delimiters.
/// type CustomChunk = Chunk<
///     boundary::Standard,
///     delimiter::AsciiPunctuation,
///     profile::Upper<profile::Ascii>,
/// >;
///
/// let ident = CustomChunk::new("UPPERIDENT")?;
/// # Ok::<(), typed_ident::Error>(())
/// ```
///
/// # Type Conversion
///
/// There's a few ways to convert between types depending on what you want to
/// accomplish. These type conversions ***DO NOT*** reformat or change the input
/// string. They just produce a new type of the target chunk syntax.
///
/// | **I Have a...**   | **I Want a...**   | **Method**                | **Validation Cost**  | **Allocation Cost** |
/// |-------------------|-------------------|---------------------------|----------------------|---------------------|
/// | `&Chunk<A..>`     | `&Chunk<B..>`     | [`cast`]  / [`as_ref`]    | None                 | None                |
/// | `&Chunk<A..>`     | `&Chunk<B..>`     | [`try_cast`]              | Same as [`new`]      | None                |
///
/// [`as_ref`]: Chunk::as_ref
/// [`cast`]: Chunk::cast
/// [`new`]: Chunk::new
/// [`try_cast`]: Chunk::try_cast
///
/// This type contains fewer conversion functions than [`Ident`], because it's
/// less common to need as much functionality with a `Chunk`. However, it might
/// be nice to have common conversion functions regardless, so follow issue
/// [#19](https://github.com/sigseis/typed-ident/issues/19) to track this.
///
/// # Type Parameters
///
/// The type parameters used on this type are:
///
/// * `B`: [`Boundary`] (a boundary definition; usually [`Standard`])
/// * `D`: [`Delimiter`] (a delimiter type; [`HyphenMinus`], [`LowLine`], etc.)
/// * `P`: [`CasedProfile`] (a cased profile; which is...)
///   * A wrapping case profile (e.g. [`Mixed`], [`Lower`], etc.)
///   * A specific character profile (e.g. [`Ascii`], [`Unicode`], etc.)
///
/// See the [`syntax`] module definition if you plan on defining your own type
/// aliases to understand better what these types mean and how they work.
///
/// [`Ascii`]: crate::syntax::profile::Ascii
/// [`Boundary`]: crate::syntax::boundary::Boundary
/// [`Delimiter`]: crate::syntax::delimiter::Delimiter
/// [`HyphenMinus`]: crate::syntax::delimiter::HyphenMinus
/// [`Ident`]: crate::core::Ident
/// [`LowLine`]: crate::syntax::delimiter::LowLine
/// [`Lower`]: crate::syntax::profile::Lower
/// [`Mixed`]: crate::syntax::profile::Mixed
/// [`Profile`]: crate::syntax::profile::Profile
/// [`Standard`]: crate::syntax::boundary::Standard
/// [`SubsetOf`]: crate::syntax::SubsetOf
/// [`Unicode`]: crate::syntax::profile::Unicode
/// [`cast`]: Self::cast
/// [`syntax`]: crate::syntax
#[repr(transparent)]
pub struct Chunk<B, D, P> {
    inner: Fragment<B, D, P>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B: 'a, D: 'a, P: 'a> Chunk<B, D, P> {
    /// A default-empty `Chunk` value (which is always valid).
    pub const EMPTY: &'a Chunk<B, D, P> = Chunk::new_unchecked("");
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> Chunk<B, D, P> {
    /// Converts a string slice to a chunk.
    ///
    /// A chunk is a slice of a fragment, which itself is made of a string slice
    /// ([`&str`]), this function converts between the two. Not all string
    /// slices are valid chunks, however. They must first be valid fragments,
    /// then secondly they must contain no delimiters.
    ///
    /// `new` checks to ensure these are satisfied before the conversion.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the chunk does not satisfy the character
    /// requirements. If an invalid character is found then a `InvalidFormat`
    /// error kind is returned, with [`byte_offset`] set to the byte index for
    /// the first invalid character.
    ///
    /// [`Error`]: crate::Error
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let chunk = UpperCamelChunk::new("AnUpperCamelChunk")?;
    /// assert_eq!(chunk, "AnUpperCamelChunk");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn new(s: &str) -> Result<&Self, Error> {
        P::is_chunk::<D>(s)?;
        Ok(Self::new_unchecked(s))
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> Chunk<B, D, P> {
    /// Returns `true` if the current chunk is a "word", `false` otherwise.
    ///
    /// This is not a word in a linguistic sense, rather this is an *identifier
    /// word*. An identifier word is a non-empty chunk of an identifier which
    /// contains no boundaries (no natural split points).
    ///
    /// See the [`core`] module documentation for more information.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// assert!(!UpperCamelChunk::new("")?.is_word());
    /// assert!(UpperCamelChunk::new("Word")?.is_word());
    /// assert!(!UpperCamelChunk::new("NotWord")?.is_word());
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn is_word(&self) -> bool {
        !self.is_empty() && B::find_boundary::<P::Segmentation>(self.as_str()).is_none()
    }

    /// Produces an iterator over the words of a chunk.
    ///
    /// This is not a word in a linguistic sense, rather this is an *identifier
    /// word*. An identifier word is a chunk of an identifier which contains no
    /// boundaries (no natural split points).
    ///
    /// See the [`core`] module documentation for more information.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let chunk = UpperCamelChunk::new("UpperCamelChunk")?;
    /// let mut words = chunk.words().type_erased();
    /// assert_eq!(words.next(), Some("Upper"));
    /// assert_eq!(words.next(), Some("Camel"));
    /// assert_eq!(words.next(), Some("Chunk"));
    /// assert_eq!(words.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn words(&self) -> Words<'_, B, D, P> {
        Words::new(self)
    }

    /// Produces an iterator over the words of a chunk, and their positions.
    ///
    /// This is not a word in a linguistic sense, rather this is an *identifier
    /// word*. An identifier word is a chunk of an identifier which contains no
    /// boundaries (no natural split points).
    ///
    /// See the [`core`] module documentation for more information.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let chunk = UpperCamelChunk::new("UpperCamelChunk")?;
    /// let mut words = chunk.word_indices().type_erased();
    /// assert_eq!(words.next(), Some((0, "Upper")));
    /// assert_eq!(words.next(), Some((5, "Camel")));
    /// assert_eq!(words.next(), Some((10, "Chunk")));
    /// assert_eq!(words.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn word_indices(&self) -> WordIndices<'_, B, D, P> {
        WordIndices::new(self)
    }
}

// -----------------------------------------------------------------------------
impl<B, D: Delimiter, P> Chunk<B, D, P> {
    /// Converts a fragment to a chunk.
    ///
    /// A chunk is a slice of a fragment that contains no delimiters, this
    /// function converts between the two. Not all fragments are valid chunks,
    /// however.
    ///
    /// `from_fragment` checks to ensure the fragment contains no delimiters
    /// before the conversion.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the chunk does not satisfy the character
    /// requirements. If a delimiter character is found then a `InvalidFormat`
    /// error kind is returned, with [`byte_offset`] set to the byte index for
    /// the first delimiter character.
    ///
    /// [`Error`]: crate::Error
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("AnUpperCamelFragment")?;
    /// let chunk = UpperCamelChunk::from_fragment(fragment)?;
    /// assert_eq!(chunk, "AnUpperCamelFragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn from_fragment(fragment: &Fragment<B, D, P>) -> Result<&Self, Error> {
        for (idx, c) in fragment.char_indices() {
            if D::is_delim(c) {
                return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(idx));
            }
        }
        Ok(Self::new_unchecked(fragment.as_str()))
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Chunk<B, D, P> {
    /// Casts a chunk into a fragment.
    ///
    /// Since all chunks are a slice of a fragment that contains no delimiters,
    /// all chunks can be trivially casted to fragments. This function performs
    /// that cast.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let chunk = UpperCamelChunk::new("AnUpperCamelChunk")?;
    /// let fragment: &UpperCamelFragment = chunk.as_fragment();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_fragment(&self) -> &Fragment<B, D, P> {
        &self.inner
    }

    /// Casts a chunk into a string slice.
    ///
    /// Since all chunks are a slice of a fragment, and all fragments are a
    /// UTF-8 string slice, all chunks can be trivially casted to string slices.
    /// This function performs that cast.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let chunk = UpperCamelChunk::new("AnUpperCamelChunk")?;
    /// let string: &str = chunk.as_str();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

// -----------------------------------------------------------------------------
impl_typed_slice_common! {
    name=Chunk,
    name_lowercase=chunk,
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P> Default for &Chunk<B, D, P> {
    #[inline(always)]
    fn default() -> Self {
        Chunk::new_unchecked("")
    }
}

// -----------------------------------------------------------------------------
impl<B1: Boundary, D1: Delimiter, P1: CasedProfile, B2, D2, P2> AsRef<Chunk<B2, D2, P2>>
    for Chunk<B1, D1, P1>
where
    D1: crate::syntax::SubsetOf<D2>,
    P1: crate::syntax::SubsetOf<P2>,
{
    #[inline(always)]
    fn as_ref(&self) -> &Chunk<B2, D2, P2> {
        self.cast()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: CasedProfile> core::convert::TryFrom<&'a Fragment<B, D, P>>
    for &'a Chunk<B, D, P>
{
    type Error = Error;

    #[inline(always)]
    fn try_from(orig: &'a Fragment<B, D, P>) -> Result<Self, Self::Error> {
        Chunk::from_fragment(orig)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: CasedProfile> core::convert::TryFrom<&'a str>
    for &'a Chunk<B, D, P>
{
    type Error = Error;
    #[inline(always)]
    fn try_from(orig: &'a str) -> Result<Self, Self::Error> {
        Chunk::new(orig)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> core::ops::Deref for Chunk<B, D, P> {
    type Target = Fragment<B, D, P>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_fragment()
    }
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=Chunk,
    against=Fragment,
}
