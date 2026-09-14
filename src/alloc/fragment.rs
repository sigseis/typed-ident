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
    /// At the end of the operation, the total number of chunked segments
    /// present in the fragment will be equal to the sum of each fragment,
    /// potentially plus one additional fragment in the case where we needed to
    /// join using a delimiter to preserve chunk boundaries.
    ///
    /// This call is identical to [`join_with`] with the default delimiter.
    ///
    /// [`join_with`]: Self::join_with
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `fragment` is invalid. If invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `FailedJoinLeft`.
    ///
    /// The value [`byte_offset`] will *NOT* be set from this function. None of
    /// the individual characters are invalid, it's just that the combination of
    /// joining the fragments themselves is invalid.
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
    /// let fragment = fragment.join(
    ///     LowerSnakeFragment::new("fragment")?,
    /// )?;
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
    /// At the end of the operation, the total number of chunked segments
    /// present in the fragment will be equal to the sum of each fragment,
    /// potentially plus one additional fragment in the case where we needed to
    /// join using a delimiter to preserve chunk boundaries.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `fragment` is invalid. If invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `FailedJoinLeft`.
    ///
    /// The value [`byte_offset`] will *NOT* be set from this function. None of
    /// the individual characters are invalid, it's just that the combination of
    /// joining the fragments themselves is invalid.
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
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.join_with(
    ///     LowerSnakeFragment::new("fragment")?,
    ///     LowLine,
    /// )?;
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

    /// Converts a string into a boxed identifier if its valid.
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
        let _ = Fragment::<B, D, P>::new(&string)?;
        Ok(Self::new_boxed_unchecked(string))
    }

    /// Returns a heap-allocated fragment, replacing the provided pattern with
    /// a fragment of the user's choice.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `to` is invalid at any replacement index. If invalid, an [`Error`] is
    /// returned with the [`error_kind`] set to either `FailedReplaceLeft` (if
    /// `to` was invalid at a specific replacement) or `FailedReplaceRight` (if
    /// `to` was valid, but the remainder was not valid after `to`).
    ///
    /// The value [`byte_offset`] *WILL* be set from this function, and it will
    /// be set to the index that caused the failure from the original fragment
    /// (`self`).
    ///
    /// So for `FailedReplaceLeft`, this is the byte index of the replacement.
    /// For `FailedReplaceRight`, this is the byte index of the residual that
    /// failed to join with the replacement.
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
    /// let fragment = fragment.replace(
    ///     "snake",
    ///     LowerSnakeFragment::new("serpent")?,
    /// )?;
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
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `prefix`,
    /// `self`, and `suffix` is invalid. If invalid, an [`Error`] is returned
    /// with the [`error_kind`] set either to `FailedJoinLeft` or
    /// `FailedJoinRight` (depending on which side caused the failure).
    ///
    /// The value [`byte_offset`] will *NOT* be set from this function. None of
    /// the individual characters are invalid, it's just that the combination of
    /// joining the fragments themselves is invalid.
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
    /// let fragment = fragment.with_circumfix(
    ///     LowerSnakeFragment::new("lower_")?,
    ///     LowerSnakeFragment::new("_fragment")?,
    /// )?;
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
            .map_err(|_| Error::new(ErrorKind::FailedJoin))
    }

    /// Returns a heap-allocated fragment with the provided prefix attached to
    /// the original fragment.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `prefix`
    /// and `self` is invalid. If invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `InvalidPrefix`.
    ///
    /// The value [`byte_offset`] will *NOT* be set from this function. None of
    /// the individual characters are invalid, it's just that the combination of
    /// joining the fragments themselves is invalid.
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
    /// let fragment = fragment.with_prefix(
    ///     LowerSnakeFragment::new("lower_")?,
    /// )?;
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
            .map_err(|_| Error::new(ErrorKind::FailedJoin))
    }

    /// Returns a heap-allocated fragment with the provided suffix attached to
    /// the original fragment.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `suffix` is invalid. If invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `InvalidPrefix`.
    ///
    /// The value [`byte_offset`] will *NOT* be set from this function. None of
    /// the individual characters are invalid, it's just that the combination of
    /// joining the fragments themselves is invalid.
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
    /// let fragment = fragment.with_suffix(
    ///     LowerSnakeFragment::new("_fragment")?,
    /// )?;
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
            .map_err(|_| Error::new(ErrorKind::FailedJoin))
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

    /// Converts an identifier into a fragment buffer.
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
