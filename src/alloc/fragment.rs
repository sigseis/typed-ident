// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::IntoIntermediate;
use crate::core::error::{Error, ErrorKind};
use crate::syntax::{Boundary, CasedProfile, Delimiter};
use crate::{Fragment, FragmentBuf};
use std_alloc::boxed::Box;
use std_alloc::format;
use std_alloc::string::String;

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> Fragment<B, D, P> {
    /// Returns a heap-allocated fragment, joined with the original fragment in
    /// a way that preserves chunk boundaries.
    ///
    /// This is a convenience function for cases when the delimiter value can be
    /// deduced by the `Default` trait. For more information on this operation,
    /// see the [`join_with`] method.
    ///
    /// [`join_with`]: Self::join_with
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.join("fragment")?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn join<F>(&self, fragment: F) -> Result<FragmentBuf<B, D, P>, Error>
    where
        D: Default,
        F: IntoIntermediate<B, D, P>,
    {
        self.join_with(fragment, D::default())
    }

    /// Returns a heap-allocated fragment, joined with the original fragment in
    /// a way that preserves chunk boundaries.
    ///
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// If you're working with an fragment format which has only a single valid
    /// delimiter value, you should instead be able to use the [`join`] method,
    /// and should prefer that.
    ///
    /// [`join`]: Self::join
    ///
    /// # Preserving Chunk Boundaries
    ///
    /// At the end of the operation, the total number of chunked segments
    /// present in the fragment will be equal to the sum of each fragment,
    /// potentially plus one additional fragment in the case where we needed to
    /// join using a delimiter to preserve chunk boundaries.
    ///
    /// Whether or not a delimiter is needed is found using the [`Boundary`]
    /// trait. The general strategy for joining looks like this:
    ///
    /// 1. The string is joined on the end of the current fragment.
    /// 2. [`Boundary::has_boundary_at`] is called with the old fragment length
    ///    to ensure there's still a boundary between the end of the original
    ///    fragment and the joined fragment.
    /// 3. If there's no boundary, the `delim` character is inserted at that
    ///    location to force a boundary.
    ///
    /// The goal of any joining operation is *not to merge chunks*.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the combination of `self` and `fragment` cannot produce a valid
    /// result.
    ///
    /// If the intermediate fragment is invalid, then the `InvalidFormat` error
    /// kind will be returned, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the join operation itself failed, then the `FailedJoin` error kind
    /// is returned, without setting the `byte_offset`.
    ///
    /// [`Error`]: crate::Error
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::LowLine;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.join_with("fragment", LowLine)?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn join_with<F>(&self, fragment: F, delim: D) -> Result<FragmentBuf<B, D, P>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let fragment = fragment.into_intermediate()?;
        let fragment: &Fragment<B, D, P> = fragment.as_ref();
        let mut buffer = FragmentBuf::with_overhead(self, fragment.len() + 1);
        buffer
            .push_bounded_with(fragment, delim)
            .map_err(|_| Error::new(ErrorKind::FailedJoin))?;
        Ok(buffer)
    }

    /// Converts a string into a boxed fragment if its valid.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the fragment does not satisfy the character
    /// requirements. If an invalid character is found then a `InvalidFormat`
    /// error kind is returned, with [`byte_offset`] set to the byte index for
    /// the first invalid character.
    ///
    /// [`Error`]: crate::Error
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment: Box<LowerSnakeFragment> =
    ///     Fragment::new_boxed(String::from("snake_fragment"))?;
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn new_boxed(string: String) -> Result<Box<Fragment<B, D, P>>, Error> {
        P::is_fragment::<D>(&string)?;
        Ok(Self::new_boxed_unchecked(string))
    }

    /// Returns a heap-allocated fragment, replacing the provided pattern with
    /// a fragment of the user's choice.
    ///
    /// For `from`, the pattern can be a `&str`, [`char`], a slice of [`char`]s,
    /// or a function or closure that determines if a character matches.
    ///
    /// For `to`, this function takes anything that can be represented as an
    /// intermediate fragment. That means it can take a `&str`, `char`,
    /// `Fragment`, `Chunk`, `Identifier`, or `Segment`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the replacement of `from` to `to` does not produce a valid fragment.
    ///
    /// If the intermediate fragment is invalid, then the `InvalidFormat` error
    /// kind will be returned, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the replace operation itself failed, then the `FailedReplace` error
    /// kind is returned, without setting the `byte_offset`.
    ///
    /// [`Error`]: crate::Error
    /// [`error_kind`]: crate::Error::error_kind
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::LowLine;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment = LowerSnakeFragment::new("example_snake_identifier")?;
    /// let fragment = fragment.replace("snake", "serpent")?;
    /// assert_eq!(fragment, "example_serpent_identifier");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn replace<M, F>(&self, from: M, to: F) -> Result<FragmentBuf<B, D, P>, Error>
    where
        M: crate::core::pattern::Pattern,
        F: IntoIntermediate<B, D, P>,
    {
        let to = to.into_intermediate()?;
        FragmentBuf::from_string(from.replace(self.as_str(), to.as_ref()))
            .map_err(|_| Error::new(ErrorKind::FailedReplace))
    }

    /// Returns a heap-allocated fragment with the provided prefix and suffix
    /// attached to the original fragment.
    ///
    /// The affixes provided to this function takes anything that can be
    /// represented as an intermediate fragment. That means it can take a
    /// `&str`, `char`, `Fragment`, `Chunk`, `Identifier`, or `Segment`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragments provided are invalid, or
    /// if the combination of `prefix`, `self`, and `fragment` cannot produce a
    /// valid result.
    ///
    /// If an intermediate fragment is invalid, then either `InvalidPrefix` or
    /// `InvalidSuffix` error kind will be returned (depending on which had the
    /// format error), with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the affixing operation itself failed, then the `FailedCircumfixing`
    /// error kind is returned, without setting the `byte_offset`.
    ///
    /// [`Error`]: crate::Error
    /// [`error_kind`]: crate::Error::error_kind
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.with_circumfix("lower_", "_fragment")?;
    /// assert_eq!(fragment, "lower_snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn with_circumfix<F1, F2>(
        &self,
        prefix: F1,
        suffix: F2,
    ) -> Result<FragmentBuf<B, D, P>, Error>
    where
        F1: IntoIntermediate<B, D, P>,
        F2: IntoIntermediate<B, D, P>,
    {
        let prefix = prefix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidPrefix))?;
        let suffix = suffix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidSuffix))?;
        FragmentBuf::from_string(format!("{prefix}{self}{suffix}"))
            .map_err(|_| Error::new(ErrorKind::FailedCircumfixing))
    }

    /// Returns a heap-allocated fragment with the provided prefix attached to
    /// the original fragment.
    ///
    /// The prefix provided to this function takes anything that can be
    /// represented as an intermediate fragment. That means it can take a
    /// `&str`, `char`, `Fragment`, `Chunk`, `Identifier`, or `Segment`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the combination of `prefix` and `self` cannot produce a valid result.
    ///
    /// If the intermediate fragment is invalid, then the error kind will be
    /// `InvalidPrefix`, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the affixing operation itself failed, then the `FailedPrefixing`
    /// error kind is returned, without setting the `byte_offset`.
    ///
    /// [`Error`]: crate::Error
    /// [`error_kind`]: crate::Error::error_kind
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.with_prefix("lower_")?;
    /// assert_eq!(fragment, "lower_snake");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn with_prefix<F>(&self, prefix: F) -> Result<FragmentBuf<B, D, P>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let prefix = prefix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidPrefix))?;
        FragmentBuf::from_string(format!("{prefix}{self}"))
            .map_err(|_| Error::new(ErrorKind::FailedPrefixing))
    }

    /// Returns a heap-allocated fragment with the provided suffix attached to
    /// the original fragment.
    ///
    /// The suffix provided to this function takes anything that can be
    /// represented as an intermediate fragment. That means it can take a
    /// `&str`, `char`, `Fragment`, `Chunk`, `Identifier`, or `Segment`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the combination of `self` and `suffix` cannot produce a valid result.
    ///
    /// If the intermediate fragment is invalid, then the error kind will be
    /// `InvalidSuffix`, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the affixing operation itself failed, then the `FailedSuffixing`
    /// error kind is returned, without setting the `byte_offset`.
    ///
    /// [`Error`]: crate::Error
    /// [`error_kind`]: crate::Error::error_kind
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.with_suffix("_fragment")?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn with_suffix<F>(&self, suffix: F) -> Result<FragmentBuf<B, D, P>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let suffix = suffix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidSuffix))?;
        FragmentBuf::from_string(format!("{self}{suffix}"))
            .map_err(|_| Error::new(ErrorKind::FailedSuffixing))
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Fragment<B, D, P> {
    /// Converts a boxed fragment into a boxed string slice.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment: Box<LowerSnakeFragment> =
    ///     Fragment::new_boxed(String::from("snake_fragment"))?;
    /// let fragment: Box<str> = fragment.into_boxed_str();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_str(self: Box<Fragment<B, D, P>>) -> Box<str> {
        // SAFETY: Fragment is transparent over str, so Box<Fragment> has the
        // same allocation layout, pointer metadata, alignment, and ownership
        // behavior as Box<str>.
        unsafe { Box::from_raw(Box::into_raw(self) as *mut str) }
    }

    /// Converts a boxed identifier into a fragment buffer.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment: Box<LowerSnakeFragment> =
    ///     Fragment::new_boxed(String::from("snake_fragment"))?;
    /// let buffer: LowerSnakeFragmentBuf = fragment.into_fragment_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_fragment_buf(self: Box<Fragment<B, D, P>>) -> FragmentBuf<B, D, P> {
        FragmentBuf::from_string_unchecked(self.into_string())
    }

    /// Converts a boxed identifier into a string.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment: Box<LowerSnakeFragment> =
    ///     Fragment::new_boxed(String::from("snake_fragment"))?;
    /// let buffer: String = fragment.into_string();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_string(self: Box<Fragment<B, D, P>>) -> String {
        self.into_boxed_str().into()
    }

    /// Converts a string into a boxed identifier, bypassing checks.
    #[must_use]
    #[inline]
    pub(crate) fn new_boxed_unchecked(string: String) -> Box<Fragment<B, D, P>> {
        let boxed_str = string.into_boxed_str();
        // SAFETY: Fragment is transparent over str, so Box<Fragment> has the
        // same allocation layout, pointer metadata, alignment, and ownership
        // behavior as Box<str>.
        unsafe { Box::from_raw(Box::into_raw(boxed_str) as *mut Fragment<B, D, P>) }
    }

    /// Converts a fragment into an owned boxed fragment.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: &LowerSnakeFragment = Fragment::new("snake_ident")?;
    /// let ident: Box<LowerSnakeFragment> = ident.to_boxed_fragment();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn to_boxed_fragment(&self) -> Box<Fragment<B, D, P>> {
        Fragment::new_boxed_unchecked(String::from(self.as_str()))
    }

    /// Converts a fragment into a fragment buffer.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment: &LowerSnakeFragment = Fragment::new("snake_fragment")?;
    /// let buffer: LowerSnakeFragmentBuf = fragment.to_fragment_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn to_fragment_buf(&self) -> FragmentBuf<B, D, P> {
        FragmentBuf::from_fragment(self)
    }
}
