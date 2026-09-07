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
use crate::syntax::{Boundary, Delimiter, Profile};

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
/// # Type Parameters
///
/// The type parameters used on this type are:
///
/// * `B`: [`Boundary`] (a boundary definition; usually [`Standard`])
/// * `D`: [`Delimiter`] (a delimiter type; [`HyphenMinus`], [`LowLine`], etc.)
/// * `P`: [`Profile`] (a character profile; [`Ascii`], [`Unicode`], etc.)
///
/// # Useful Properties
///
/// Some useful properties to be aware of when dealing with chunks:
///
/// * An empty string slice is always a valid chunk.
/// * A slice of any chunk is itself a chunk over the same generics.
///   * e.g. as long as we don't change the type parameters, you can slice a
///     chunk and get another valid chunk over the same types.
/// * You can trivially [`cast`] one chunk to another as long as the
///   chunk's generic types are [`SubsetOf`] the target chunk's generics.
///   * e.g. as long as we are casting to a more broad format, it's trivial and
///     we do not need to check the format again (enforced by the trait system).
///
/// # Examples
///
/// It is recommended that you configure a type alias to work with chunks, so
/// that you don't need to provide the type parameters everywhere (or use one of
/// the provided [`presets`]).
///
/// ```
/// // Custom Chunk Example
/// use typed_ident::core::Chunk;
/// use typed_ident::syntax::{boundary, delimiter, profile};
/// type CustomChunk = Chunk<
///     boundary::Standard,
///     delimiter::LowLine,
///     profile::Lower<profile::Unicode>,
/// >;
/// assert!(CustomChunk::new("onlyacceptslowercase").is_ok());
///
/// // Preset Chunk Example
/// use typed_ident::presets::unicode::upper_camel::UpperCamelChunk;
/// assert!(UpperCamelChunk::new("AcceptsUppercase").is_ok());
/// ```
///
/// [`Ascii`]: crate::syntax::profile::Ascii
/// [`Boundary`]: crate::syntax::boundary::Boundary
/// [`Delimiter`]: crate::syntax::delimiter::Delimiter
/// [`Standard`]: crate::syntax::boundary::Standard
/// [`HyphenMinus`]: crate::syntax::delimiter::HyphenMinus
/// [`Ident`]: crate::core::Ident
/// [`LowLine`]: crate::syntax::delimiter::LowLine
/// [`Profile`]: crate::syntax::profile::Profile
/// [`SubsetOf`]: crate::syntax::SubsetOf
/// [`Unicode`]: crate::syntax::profile::Unicode
/// [`cast`]: Self::cast
/// [`presets`]: crate::presets
#[repr(transparent)]
pub struct Chunk<B, D, P> {
    inner: Fragment<B, D, P>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> Chunk<B, D, P> {
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
    /// Returns `Err` if the fragment contains any delimiters. If a delimiter is
    /// found, [`Error`] is returned with [`byte_offset`] set to the byte index
    /// for the first invalid character.
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
        // This is not a hot function - so no need to over-optimize it. Just
        // call `Fragment::new`, even though it's technically looping over this
        // twice. We don't expect user's to build chunks very often.
        Self::from_fragment(Fragment::new(s)?)
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D, P: Profile> Chunk<B, D, P> {
    /// Returns `true` if the current chunk is a "word", `false` otherwise.
    ///
    /// This is not a word in a linguistic sense, rather this is an *identifier
    /// word*. An identifier word is a chunk of an identifier which contains no
    /// boundaries (no natural split points).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// assert!(UpperCamelChunk::new("")?.is_word());
    /// assert!(UpperCamelChunk::new("Word")?.is_word());
    /// assert!(!UpperCamelChunk::new("NotWord")?.is_word());
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn is_word(&self) -> bool {
        B::find_boundary::<P::Segmentation>(self.as_str()).is_none()
    }

    /// Produces an iterator over the words of a chunk, and their positions.
    ///
    /// This is not a word in a linguistic sense, rather this is an *identifier
    /// word*. An identifier word is a chunk of an identifier which contains no
    /// boundaries (no natural split points).
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

    /// Produces an iterator over the words of a chunk.
    ///
    /// This is not a word in a linguistic sense, rather this is an *identifier
    /// word*. An identifier word is a chunk of an identifier which contains no
    /// boundaries (no natural split points).
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
}

// -----------------------------------------------------------------------------
impl<B, D: Delimiter, P> Chunk<B, D, P> {
    /// Converts a fragment to a chunk.
    ///
    /// A chunk is a slice of a fragment that contains no delimiters, this
    /// function converts between the two. Not all fragments are valid chunks,
    /// however. `new` checks to ensure the fragment contains no delimiters
    /// before the conversion.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment contains any delimiters. If a delimiter is
    /// found, [`Error`] is returned with [`byte_offset`] set to the byte index
    /// for the first invalid character.
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
    ///
    /// [`Error`]: crate::core::Error
    /// [`byte_offset`]: crate::core::Error::byte_offset
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
impl<B1: Boundary, D1: Delimiter, P1: Profile, B2, D2, P2> AsRef<Chunk<B2, D2, P2>>
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
impl<'a, B: Boundary, D: Delimiter, P: Profile> core::convert::TryFrom<&'a Fragment<B, D, P>>
    for &'a Chunk<B, D, P>
{
    type Error = Error;

    #[inline(always)]
    fn try_from(orig: &'a Fragment<B, D, P>) -> Result<Self, Self::Error> {
        Chunk::from_fragment(orig)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> core::convert::TryFrom<&'a str>
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
