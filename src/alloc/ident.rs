// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::IntoIntermediate;
use crate::core::error::{Error, ErrorKind};
use crate::syntax::{Boundary, CasedProfile, Delimiter};
use crate::{Fragment, FragmentBuf, Ident};
use std_alloc::boxed::Box;
use std_alloc::format;
use std_alloc::string::String;

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> Ident<B, D, P> {
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
    pub fn join<F>(&self, fragment: F) -> Result<Box<Ident<B, D, P>>, Error>
    where
        D: Default,
        F: IntoIntermediate<B, D, P>,
    {
        self.join_with(fragment, D::default())
    }

    /// Returns a heap-allocated identifier, joined with the original identifier
    /// in a way that preserves chunk boundaries.
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
    pub fn join_with<F>(&self, fragment: F, delim: D) -> Result<Box<Ident<B, D, P>>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let fragment = fragment.into_intermediate()?;
        let fragment: &Fragment<B, D, P> = fragment.as_ref();
        let mut ident = FragmentBuf::with_overhead(self, fragment.len() + 1);
        ident
            .push_bounded_with(fragment, delim)
            .map_err(|_| Error::new(ErrorKind::FailedJoin))?;
        ident.into_boxed_ident()
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

    /// Returns a heap-allocated identifier with the provided prefix and suffix
    /// attached to the original identifier.
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
    pub fn with_circumfix<F1, F2>(
        &self,
        prefix: F1,
        suffix: F2,
    ) -> Result<Box<Ident<B, D, P>>, Error>
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
        Ident::new_boxed(format!("{prefix}{self}{suffix}"))
            .map_err(|_| Error::new(ErrorKind::FailedJoin))
    }

    /// Returns a heap-allocated identifier with the provided prefix attached to
    /// the original identifier.
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
    pub fn with_prefix<F>(&self, prefix: F) -> Result<Box<Ident<B, D, P>>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let prefix = prefix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidPrefix))?;
        Ident::new_boxed(format!("{prefix}{self}")).map_err(|_| Error::new(ErrorKind::FailedJoin))
    }

    /// Returns a heap-allocated identifier with the provided suffix attached to
    /// the original identifier.
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
    pub fn with_suffix<F>(&self, suffix: F) -> Result<Box<Ident<B, D, P>>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let suffix = suffix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidSuffix))?;
        Ident::new_boxed(format!("{self}{suffix}")).map_err(|_| Error::new(ErrorKind::FailedJoin))
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
}
