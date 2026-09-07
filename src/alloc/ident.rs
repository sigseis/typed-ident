// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::error::{Error, ErrorKind};
use crate::syntax::{Boundary, Delimiter, Profile};
use crate::{Fragment, FragmentBuf, Ident, IdentBuf};
use std_alloc::boxed::Box;
use std_alloc::string::String;

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> Ident<B, D, P> {
    /// Returns a heap-allocated identifier, joined with the original identifier
    /// in a way that preserves chunk boundaries.
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
    /// [`error_kind`] set to `FailedLeftJoin`.
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.join(
    ///     LowerSnakeFragment::new("ident")?,
    /// )?;
    /// assert_eq!(ident.as_ref(), "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn join(
        &self,
        fragment: &Fragment<B, D, P>,
    ) -> Result<std_alloc::boxed::Box<Ident<B, D, P>>, Error>
    where
        D: Default,
    {
        self.join_with(fragment, D::default())
    }

    /// Returns a heap-allocated identifier, joined with the original identifier
    /// in a way that preserves chunk boundaries. The provided string is first
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
    /// [`error_kind`] set to `FailedLeftJoin`.
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.join_str("ident")?;
    /// assert_eq!(ident.as_ref(), "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn join_str(&self, s: &str) -> Result<Box<Ident<B, D, P>>, Error>
    where
        D: Default,
    {
        self.join_with(Fragment::new(s)?, D::default())
    }

    /// Returns a heap-allocated identifier, joined with the original identifier
    /// in a way that preserves chunk boundaries. The provided string is first
    /// converted to a fragment before attempting to append it.
    ///
    /// At the end of the operation, the total number of chunked segments
    /// present in the identifier will be equal to the sum of each fragment,
    /// potentially plus one additional fragment in the case where we needed to
    /// join using a delimiter to preserve chunk boundaries.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `fragment` is invalid. If invalid, an [`Error`] is returned with the
    /// [`error_kind`] set to `InvalidFormat` or `FailedLeftJoin`.
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.join_str_with("ident", LowLine)?;
    /// assert_eq!(ident.as_ref(), "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn join_str_with(&self, s: &str, delim: D) -> Result<Box<Ident<B, D, P>>, Error> {
        self.join_with(Fragment::new(s)?, delim)
    }

    /// Returns a heap-allocated identifier, joined with the original identifier
    /// in a way that preserves chunk boundaries.
    ///
    /// At the end of the operation, the total number of chunked segments
    /// present in the identifier will be equal to the sum of each fragment,
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
    /// [`error_kind`] set to `FailedLeftJoin`.
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.join_with(
    ///     LowerSnakeFragment::new("ident")?,
    ///     LowLine,
    /// )?;
    /// assert_eq!(ident.as_ref(), "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn join_with(
        &self,
        fragment: &Fragment<B, D, P>,
        delim: D,
    ) -> Result<Box<Ident<B, D, P>>, Error> {
        let mut ident = IdentBuf::with_overhead(self, fragment.len() + 1);
        ident.push_bounded_fragment_with(fragment, delim)?;
        Ok(ident.into_boxed_ident().unwrap())
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
    /// let ident: Box<LowerSnakeIdent> =
    ///     Ident::new_boxed(String::from("snake_ident"))?;
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn new_boxed(string: String) -> Result<Box<Ident<B, D, P>>, Error> {
        let _ = Ident::<B, D, P>::new(&string)?;
        Ok(Self::new_boxed_unchecked(string))
    }

    /// Returns a heap-allocated identifier, replacing the provided pattern with
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
    /// returned with the [`error_kind`] set to `FailedReplaceLeft` or
    /// `FailedReplaceRight` (if the replacement succeeded, but the remainder
    /// could not be appended).
    ///
    /// The value [`byte_offset`] *WILL* be set from this function, and it will
    /// be set to the index that caused the failure from the original fragment
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
    /// let ident = LowerSnakeIdent::new("example_snake_identifier")?;
    /// let ident = ident.replace(
    ///     "snake",
    ///     LowerSnakeFragment::new("serpent")?,
    /// )?;
    /// assert_eq!(ident.as_ident_or_anonymous(), "example_serpent_identifier");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn replace<M>(&self, from: M, to: &Fragment<B, D, P>) -> Result<IdentBuf<B, D, P>, Error>
    where
        M: crate::core::pattern::Pattern,
    {
        let mut buffer = IdentBuf::with_capacity(self.len());
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

    /// Returns a heap-allocated identifier, replacing the provided pattern with
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
    /// let ident = LowerSnakeIdent::new("example_snake_identifier")?;
    /// let ident = ident.replace_str("snake", "serpent")?;
    /// assert_eq!(ident.as_ident_or_anonymous(), "example_serpent_identifier");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn replace_str<M>(&self, from: M, to: &str) -> Result<IdentBuf<B, D, P>, Error>
    where
        M: crate::core::pattern::Pattern,
    {
        self.replace(from, Fragment::new(to)?)
    }

    /// Returns a heap-allocated identifier with the provided prefix and suffix
    /// attached to the original identifier.
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
    /// with the [`error_kind`] set either to `InvalidPrefix` or `InvalidSuffix`
    /// (depending on which has caused the failure).
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_circumfix(
    ///     LowerSnakeFragment::new("lower_")?,
    ///     LowerSnakeFragment::new("_ident")?,
    /// )?;
    /// assert_eq!(ident.as_ref(), "lower_snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_circumfix(
        &self,
        prefix: &Fragment<B, D, P>,
        suffix: &Fragment<B, D, P>,
    ) -> Result<Box<Ident<B, D, P>>, Error> {
        let mut ident = IdentBuf::with_capacity(self.len() + prefix.len() + suffix.len());
        ident.push_fragment(prefix)?;
        ident.push_fragment(self)?;
        ident.push_fragment(suffix)?;
        Ok(ident.into_boxed_ident().unwrap())
    }

    /// Returns a heap-allocated identifier with the provided prefix and suffix
    /// strings attached to the original identifier. The provided strings are
    /// first converted to fragments before attempting to append them.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `prefix`,
    /// `self`, and `suffix` is invalid. If invalid, an [`Error`] is returned
    /// with the [`error_kind`] set either to `InvalidPrefix` or `InvalidSuffix`
    /// (depending on which has caused the failure).
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_circumfix_str("lower_", "_ident")?;
    /// assert_eq!(ident.as_ref(), "lower_snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_circumfix_str(
        &self,
        prefix: &str,
        suffix: &str,
    ) -> Result<Box<Ident<B, D, P>>, Error> {
        self.with_circumfix(Fragment::new(prefix)?, Fragment::new(suffix)?)
    }

    /// Returns a heap-allocated identifier with the provided prefix attached to
    /// the original identifier.
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_prefix(
    ///     LowerSnakeFragment::new("lower_")?,
    /// )?;
    /// assert_eq!(ident.as_ref(), "lower_snake");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_prefix(&self, prefix: &Fragment<B, D, P>) -> Result<Box<Ident<B, D, P>>, Error> {
        let mut ident = IdentBuf::with_capacity(self.len() + prefix.len());
        ident.push_fragment(prefix)?;
        ident.push_fragment(self)?;
        Ok(ident.into_boxed_ident().unwrap())
    }

    /// Returns a heap-allocated identifier with the provided prefix string
    /// attached to the original identifier. The provided string is first
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_prefix_str("lower_")?;
    /// assert_eq!(ident.as_ref(), "lower_snake");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_prefix_str(&self, prefix: &str) -> Result<Box<Ident<B, D, P>>, Error> {
        self.with_prefix(Fragment::new(prefix)?)
    }

    /// Returns a heap-allocated identifier with the provided suffix attached to
    /// the original identifier.
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_suffix(
    ///     LowerSnakeFragment::new("_ident")?,
    /// )?;
    /// assert_eq!(ident.as_ref(), "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_suffix(&self, suffix: &Fragment<B, D, P>) -> Result<Box<Ident<B, D, P>>, Error> {
        let mut ident = IdentBuf::with_overhead(self, suffix.len());
        ident.push_fragment(suffix)?;
        Ok(ident.into_boxed_ident().unwrap())
    }

    /// Returns a heap-allocated identifier with the provided suffix string
    /// attached to the original identifier. The provided string is first
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_suffix_str("_ident")?;
    /// assert_eq!(ident.as_ref(), "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_suffix_str(&self, suffix: &str) -> Result<Box<Ident<B, D, P>>, Error> {
        self.with_suffix(Fragment::new(suffix)?)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Ident<B, D, P> {
    /// Converts a boxed identifier into a boxed string slice.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeIdent> =
    ///     Ident::new_boxed(String::from("snake_ident"))?;
    /// let ident: Box<str> = ident.into_boxed_str();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_str(self: Box<Ident<B, D, P>>) -> Box<str> {
        // SAFETY: Ident is transparent over Fragment, which is itself
        // transparent over str, so Box<Ident> has the same allocation layout,
        // pointer metadata, alignment, and ownership behavior as Box<str>.
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
    /// let ident: Box<LowerSnakeIdent> =
    ///     Ident::new_boxed(String::from("snake_ident"))?;
    /// let buffer: LowerSnakeFragmentBuf = ident.into_fragment_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_fragment_buf(self: Box<Ident<B, D, P>>) -> FragmentBuf<B, D, P> {
        FragmentBuf::from_string_unchecked(self.into_string())
    }

    /// Converts a boxed identifier into an identifier buffer.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeIdent> =
    ///     Ident::new_boxed(String::from("snake_ident"))?;
    /// let buffer: LowerSnakeIdentBuf = ident.into_ident_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_ident_buf(self: Box<Ident<B, D, P>>) -> IdentBuf<B, D, P> {
        IdentBuf::from_string_unchecked(self.into_string())
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
    /// let ident: Box<LowerSnakeIdent> =
    ///     Ident::new_boxed(String::from("snake_ident"))?;
    /// let ident: String = ident.into_string();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_string(self: Box<Ident<B, D, P>>) -> String {
        self.into_boxed_str().into()
    }

    /// Converts a string into a boxed identifier, bypassing checks.
    #[must_use]
    #[inline]
    pub(crate) fn new_boxed_unchecked(string: String) -> Box<Ident<B, D, P>> {
        let boxed_str = string.into_boxed_str();
        // SAFETY: Ident is transparent over Fragment, which is itself
        // transparent over str, so Box<Ident> has the same allocation layout,
        // pointer metadata, alignment, and ownership behavior as Box<str>.
        unsafe { Box::from_raw(Box::into_raw(boxed_str) as *mut Ident<B, D, P>) }
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
    /// let ident: &LowerSnakeIdent = Ident::new("snake_ident")?;
    /// let buffer: LowerSnakeFragmentBuf = ident.to_fragment_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn to_fragment_buf(&self) -> FragmentBuf<B, D, P> {
        FragmentBuf::from_fragment(self)
    }

    /// Converts an identifier into an identifier buffer.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: &LowerSnakeIdent = Ident::new("snake_ident")?;
    /// let buffer: LowerSnakeIdentBuf = ident.to_ident_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn to_ident_buf(&self) -> IdentBuf<B, D, P> {
        IdentBuf::from_ident(self)
    }
}
