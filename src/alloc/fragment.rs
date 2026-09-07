// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::error::{Error, ErrorKind};
use crate::syntax::{Boundary, Delimiter, Profile};
use crate::{Fragment, FragmentBuf};
use std_alloc::boxed::Box;
use std_alloc::string::String;

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> Fragment<B, D, P> {
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
    /// It can be a bit cumbersome to use this function in most cases. Instead,
    /// if you find it easier to work with string data (or you don't have any
    /// fragments that you're joining with), you can use [`join_str`].
    ///
    /// [`join_str`]: Self::join_str
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
    pub fn join(&self, fragment: &Fragment<B, D, P>) -> Result<FragmentBuf<B, D, P>, Error>
    where
        D: Default,
    {
        self.join_with(fragment, D::default())
    }

    /// Returns a heap-allocated fragment, joined with the original fragment in
    /// a way that preserves chunk boundaries. The provided string is first
    /// converted to a fragment before attempting to append it.
    ///
    /// At the end of the operation, the total number of chunked segments
    /// present in the fragment will be equal to the sum of each fragment,
    /// potentially plus one additional fragment in the case where we needed to
    /// join using a delimiter to preserve chunk boundaries.
    ///
    /// This call is identical to [`join_str_with`] with the default delimiter.
    ///
    /// [`join_str_with`]: Self::join_str_with
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `fragment` is invalid. If invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `FailedJoinLeft`.
    ///
    /// The value [`byte_offset`] *MAY* be set on this function. If the joining
    /// string contained invalid characters, this will be set to the byte index
    /// (from the start of the joining string) that was invalid.
    ///
    /// However, if all characters are independently valid, but one side failed
    /// to join (because the join itself would make the following character
    /// invalid), then `byte_offset` will be set to `None`.
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
    /// let fragment = fragment.join_str("fragment")?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn join_str(&self, s: &str) -> Result<FragmentBuf<B, D, P>, Error>
    where
        D: Default,
    {
        self.join_str_with(s, D::default())
    }

    /// Returns a heap-allocated fragment, joined with the original fragment in
    /// a way that preserves chunk boundaries. The provided string is first
    /// converted to a fragment before attempting to append it.
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
    /// [`error_kind`] set to `InvalidFormat` or `FailedJoinLeft`.
    ///
    /// The value [`byte_offset`] *MAY* be set on this function. If the joining
    /// string contained invalid characters, this will be set to the byte index
    /// (from the start of the joining string) that was invalid.
    ///
    /// However, if all characters are independently valid, but one side failed
    /// to join (because the join itself would make the following character
    /// invalid), then `byte_offset` will be set to `None`.
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
    /// let fragment = fragment.join_str_with("fragment", LowLine)?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn join_str_with(&self, s: &str, delim: D) -> Result<FragmentBuf<B, D, P>, Error> {
        let fragment = Fragment::new(s)?;
        self.join_with(fragment, delim)
    }

    /// Returns a heap-allocated fragment, joined with the original fragment in
    /// a way that preserves chunk boundaries.
    ///
    /// At the end of the operation, the total number of chunked segments
    /// present in the fragment will be equal to the sum of each fragment,
    /// potentially plus one additional fragment in the case where we needed to
    /// join using a delimiter to preserve chunk boundaries.
    ///
    /// It can be a bit cumbersome to use this function in most cases. Instead,
    /// if you find it easier to work with string data (or you don't have any
    /// fragments that you're joining with), you can use [`join_str_with`].
    ///
    /// [`join_str_with`]: Self::join_str_with
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
    pub fn join_with(
        &self,
        fragment: &Fragment<B, D, P>,
        delim: D,
    ) -> Result<FragmentBuf<B, D, P>, Error> {
        let mut buffer = FragmentBuf::with_overhead(self, fragment.len() + 1);
        buffer.push_bounded_fragment_with(fragment, delim)?;
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
    /// It can be a bit cumbersome to use this function in most cases. Instead,
    /// if you find it easier to work with string data (or you don't have any
    /// fragments that you're replacing with), you can use [`replace_str`].
    ///
    /// [`replace_str`]: Self::replace_str
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
    pub fn replace<M>(&self, from: M, to: &Fragment<B, D, P>) -> Result<FragmentBuf<B, D, P>, Error>
    where
        M: crate::core::pattern::Pattern,
    {
        let mut buffer = FragmentBuf::with_capacity(self.len());
        let mut last_end = 0;
        for (start, part) in self.match_indices(from) {
            buffer.push_fragment(&self[last_end..start]).map_err(|_| {
                Error::new(ErrorKind::FailedReplaceRight).with_byte_offset(last_end)
            })?;
            buffer
                .push_fragment(to)
                .map_err(|_| Error::new(ErrorKind::FailedReplaceLeft).with_byte_offset(start))?;
            last_end = start + part.len();
        }
        buffer
            .push_fragment(&self[last_end..self.len()])
            .map_err(|_| Error::new(ErrorKind::FailedReplaceRight).with_byte_offset(last_end))?;
        Ok(buffer)
    }

    /// Returns a heap-allocated fragment, replacing the provided pattern with
    /// a fragment of the user's choice. The provided string is first converted
    /// to a fragment before attempting to append it.
    ///
    /// It can be a bit cumbersome to use this function in most cases. Instead,
    /// if you find it easier to work with string data (or you don't have any
    /// fragments that you're replacing with), you can use [`replace_str`].
    ///
    /// [`replace_str`]: Self::replace_str
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `to` is invalid at any replacement index. If invalid, an [`Error`] is
    /// returned with the [`error_kind`] set to `InvalidFormat` if the
    /// provided fragment was invalid, or `InvalidReplace` if the replacement
    /// failed.
    ///
    /// The value [`byte_offset`] *WILL* be set from this function. On invalid
    /// fragment, it will be set to the byte index from the start of the
    /// fragment which was invalid. On invalid replacement, it will be set to
    /// the byte index that caused the failure from the original fragment
    /// (`self`).
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
    /// let fragment = fragment.replace_str("snake", "serpent")?;
    /// assert_eq!(fragment, "example_serpent_identifier");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn replace_str<M>(&self, from: M, to: &str) -> Result<FragmentBuf<B, D, P>, Error>
    where
        M: crate::core::pattern::Pattern,
    {
        self.replace(from, Fragment::new(to)?)
    }

    /// Returns a heap-allocated fragment with the provided prefix and suffix
    /// attached to the original fragment.
    ///
    /// It can be a bit cumbersome to use this function in most cases. Instead,
    /// if you find it easier to work with string data (or you don't have any
    /// fragments that you're joining with), you can use [`with_circumfix_str`].
    ///
    /// [`with_circumfix_str`]: Self::with_circumfix_str
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
    pub fn with_circumfix(
        &self,
        prefix: &Fragment<B, D, P>,
        suffix: &Fragment<B, D, P>,
    ) -> Result<FragmentBuf<B, D, P>, Error> {
        let mut buffer = FragmentBuf::with_overhead(prefix, self.len() + suffix.len());
        buffer.push_fragment(self)?;
        buffer
            .push_fragment(suffix)
            .map_err(|_| Error::new(ErrorKind::FailedJoinRight))?;
        Ok(buffer)
    }

    /// Returns a heap-allocated fragment with the provided prefix and suffix
    /// strings attached to the original fragment. The provided strings are
    /// first converted to fragments before attempting to append them.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the either of the provided fragments are invalid.
    /// If one of them is invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `InvalidPrefix` or `InvalidSuffix` depending on
    /// which was invalid (prefix takes precedence if both are invalid).
    ///
    /// Returns `Err` if the fragment formed from the combination of `prefix`,
    /// `self`, and `suffix` is invalid. If invalid, an [`Error`] is returned
    /// with the [`error_kind`] set either to `FailedJoinLeft` or
    /// `FailedJoinRight` (depending on which side caused the failure).
    ///
    /// The value [`byte_offset`] *MAY* be set on this function. If the prefix
    /// or suffix strings contained invalid characters, this will be set to the
    /// byte index (from the start of either the prefix or suffix, depending on
    /// which `error_kind` was set) that was invalid.
    ///
    /// However, if all characters are independently valid, but one side failed
    /// to join (because the join itself would make the following character
    /// invalid), then `byte_offset` will be set to `None`.
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
    /// let fragment = fragment.with_circumfix_str("lower_", "_fragment")?;
    /// assert_eq!(fragment, "lower_snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn with_circumfix_str(
        &self,
        prefix: &str,
        suffix: &str,
    ) -> Result<FragmentBuf<B, D, P>, Error> {
        let prefix =
            Fragment::new(prefix).map_err(|e| e.with_error_kind(ErrorKind::InvalidPrefix))?;
        let suffix =
            Fragment::new(suffix).map_err(|e| e.with_error_kind(ErrorKind::InvalidSuffix))?;
        self.with_circumfix(prefix, suffix)
    }

    /// Returns a heap-allocated fragment with the provided prefix attached to
    /// the original fragment.
    ///
    /// It can be a bit cumbersome to use this function in most cases. Instead,
    /// if you find it easier to work with string data (or you don't have any
    /// fragments that you're joining with), you can use [`with_prefix_str`].
    ///
    /// [`with_prefix_str`]: Self::with_prefix_str
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
    pub fn with_prefix(&self, prefix: &Fragment<B, D, P>) -> Result<FragmentBuf<B, D, P>, Error> {
        let mut buffer = FragmentBuf::with_overhead(prefix, self.len());
        buffer.push_fragment(self)?;
        Ok(buffer)
    }

    /// Returns a heap-allocated fragment with the provided prefix string
    /// attached to the original fragment. The provided string is first
    /// converted to a fragment before attempting to append it.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `prefix`
    /// and `self` is invalid. If invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `InvalidPrefix`.
    ///
    /// The value [`byte_offset`] *MAY* be set on this function. If the prefix
    /// string contained invalid characters, this will be set to the byte index
    /// (from the start of the prefix string) that was invalid.
    ///
    /// However, if all characters are independently valid, but one side failed
    /// to join (because the join itself would make the following character
    /// invalid), then `byte_offset` will be set to `None`.
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
    /// let fragment = fragment.with_prefix_str("lower_")?;
    /// assert_eq!(fragment, "lower_snake");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn with_prefix_str(&self, prefix: &str) -> Result<FragmentBuf<B, D, P>, Error> {
        self.with_prefix(Fragment::new(prefix)?)
    }

    /// Returns a heap-allocated fragment with the provided suffix attached to
    /// the original fragment.
    ///
    /// It can be a bit cumbersome to use this function in most cases. Instead,
    /// if you find it easier to work with string data (or you don't have any
    /// fragments that you're joining with), you can use [`with_suffix_str`].
    ///
    /// [`with_suffix_str`]: Self::with_suffix_str
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
    pub fn with_suffix(&self, suffix: &Fragment<B, D, P>) -> Result<FragmentBuf<B, D, P>, Error> {
        let mut buffer = FragmentBuf::with_overhead(self, suffix.len());
        buffer.push_fragment(suffix)?;
        Ok(buffer)
    }

    /// Returns a heap-allocated fragment with the provided suffix string
    /// attached to the original fragment. The provided string is first
    /// converted to a fragment before attempting to append it.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `suffix` is invalid. If invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `InvalidPrefix`.
    ///
    /// The value [`byte_offset`] *MAY* be set on this function. If the suffix
    /// string contained invalid characters, this will be set to the byte index
    /// (from the start of the suffix string) that was invalid.
    ///
    /// However, if all characters are independently valid, but one side failed
    /// to join (because the join itself would make the following character
    /// invalid), then `byte_offset` will be set to `None`.
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
    /// let fragment = fragment.with_suffix_str("_fragment")?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn with_suffix_str(&self, suffix: &str) -> Result<FragmentBuf<B, D, P>, Error> {
        self.with_suffix(Fragment::new(suffix)?)
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
