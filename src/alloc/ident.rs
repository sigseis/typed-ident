// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::intermediate::IntoIntermediate;
use crate::syntax::{Boundary, CasedProfile, Delimiter};
use crate::{Error, ErrorKind, Fragment, FragmentBuf, Ident};
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
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// # Preserving Chunk Boundaries
    ///
    /// At the end of the operation, the total number of chunked segments
    /// present in the identifier will be equal to the sum of each fragment,
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
    /// let ident = ident.join("ident")?;
    /// assert_eq!(ident, "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn join<F>(&self, fragment: F) -> Result<Box<Self>, Error>
    where
        D: Default,
        F: IntoIntermediate<B, D, P>,
    {
        self.join_with(fragment, D::default())
    }

    /// Returns a heap-allocated identifier, joined with the original identifier
    /// in a way that preserves chunk boundaries.
    ///
    /// This function is identical to [`join`], except that you can specify the
    /// delimiter to use. This may be needed in cases where there can be more
    /// than one delimiter, and as such there is no default.
    ///
    /// [`join`]: Self::join
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::LowLine;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.join_with("ident", LowLine)?;
    /// assert_eq!(ident, "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn join_with<F>(&self, fragment: F, delim: D) -> Result<Box<Self>, Error>
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

    /// Converts a string into a boxed identifier if it's valid.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeIdent> = Ident::new_boxed("snake_ident")?;
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn new_boxed<S>(string: S) -> Result<Box<Self>, Error>
    where
        S: Into<String>,
    {
        let string = string.into();
        let _ = Ident::<B, D, P>::new(&string)?;
        Ok(Self::new_boxed_unchecked(string))
    }

    /// Returns a heap-allocated identifier with the provided prefix and suffix
    /// attached to the original identifier.
    ///
    /// Unlike [`join`], this function will *NOT* ensure that words remain
    /// separated after affixing. The values provided will be affixed verbatim.
    /// This is often useful for providing quick decoration like `"__"`.
    ///
    /// The affixes provided to this function takes anything that can be
    /// represented as an intermediate fragment. That means it can take a
    /// `&str`, `char`, `Fragment`, `Chunk`, `Identifier`, or `Segment`.
    ///
    /// [`join`]: Self::join
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_circumfix("lower_", "_ident")?;
    /// assert_eq!(ident, "lower_snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_circumfix<F1, F2>(&self, prefix: F1, suffix: F2) -> Result<Box<Self>, Error>
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
            .map_err(|_| Error::new(ErrorKind::FailedCircumfixing))
    }

    /// Returns a heap-allocated identifier with the provided prefix attached to
    /// the original identifier.
    ///
    /// Unlike [`join`], this function will *NOT* ensure that words remain
    /// separated after affixing. The values provided will be affixed verbatim.
    /// This is often useful for providing quick decoration like `"__"`.
    ///
    /// The affixes provided to this function takes anything that can be
    /// represented as an intermediate fragment. That means it can take a
    /// `&str`, `char`, `Fragment`, `Chunk`, `Identifier`, or `Segment`.
    ///
    /// [`join`]: Self::join
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_prefix("lower_")?;
    /// assert_eq!(ident, "lower_snake");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_prefix<F>(&self, prefix: F) -> Result<Box<Self>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let prefix = prefix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidPrefix))?;
        Ident::new_boxed(format!("{prefix}{self}"))
            .map_err(|_| Error::new(ErrorKind::FailedPrefixing))
    }

    /// Returns a heap-allocated identifier with the provided suffix attached to
    /// the original identifier.
    ///
    /// Unlike [`join`], this function will *NOT* ensure that words remain
    /// separated after affixing. The values provided will be affixed verbatim.
    /// This is often useful for providing quick decoration like `"__"`.
    ///
    /// The affixes provided to this function takes anything that can be
    /// represented as an intermediate fragment. That means it can take a
    /// `&str`, `char`, `Fragment`, `Chunk`, `Identifier`, or `Segment`.
    ///
    /// [`join`]: Self::join
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
    /// let ident = LowerSnakeIdent::new("snake")?;
    /// let ident = ident.with_suffix("_ident")?;
    /// assert_eq!(ident, "snake_ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated identifier, it does not mutate the original"]
    #[inline]
    pub fn with_suffix<F>(&self, suffix: F) -> Result<Box<Self>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let suffix = suffix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidSuffix))?;
        Ident::new_boxed(format!("{self}{suffix}"))
            .map_err(|_| Error::new(ErrorKind::FailedSuffixing))
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Ident<B, D, P> {
    /// Zero-cost conversion into a different boxed, type-configured identifier.
    ///
    /// This is the boxed, by-value equivalent of [`cast`], see that function
    /// for details on how this works. If you don't need to consume the boxed
    /// value, you can instead use that function to get a reference to a
    /// different type-configured identifier.
    ///
    /// [`cast`]: Self::cast
    ///
    /// # Examples
    ///
    /// Example traversing case profile boundary:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// // Compilable Cast:
    /// let original = LowerSnakeIdent::new_boxed("apple")?;
    /// let converted: Box<LowerCamelIdent> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// // Bad Cast (Fails Compilation):
    /// let original = LowerCamelIdent::new_boxed("apple")?;
    /// let converted: Box<LowerSnakeIdent> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Example traversing character profile boundary:
    ///
    /// ```
    /// # use typed_ident::presets::ascii::lower_snake as ascii;
    /// # use typed_ident::presets::unicode::lower_snake as unicode;
    /// // Compilable Cast:
    /// let original = ascii::LowerSnakeIdent::new_boxed("apple")?;
    /// let converted: Box<unicode::LowerSnakeIdent> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::ascii::lower_snake as ascii;
    /// # use typed_ident::presets::unicode::lower_snake as unicode;
    /// // Bad Cast (Fails Compilation):
    /// let original = unicode::LowerSnakeIdent::new_boxed("apple")?;
    /// let converted: Box<ascii::LowerSnakeIdent> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Example traversing delimiter boundary:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// // Compilable Cast:
    /// let original = LowerSnakeIdent::new_boxed("apple")?;
    /// let converted: Box<HybridIdent> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// // Bad Cast (Fails Compilation):
    /// let original = HybridIdent::new_boxed("apple")?;
    /// let converted: Box<LowerSnakeIdent> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn convert<B2, D2, P2>(self: Box<Self>) -> Box<Ident<B2, D2, P2>>
    where
        B2: crate::syntax::Boundary,
        D: crate::syntax::SubsetOf<D2>,
        P: crate::syntax::SubsetOf<P2>,
    {
        Ident::new_boxed_unchecked(self.into_string())
    }

    /// Converts a boxed identifier into a boxed fragment.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeIdent> = Ident::new_boxed("snake_ident")?;
    /// let ident: Box<LowerSnakeFragment> = ident.into_boxed_fragment();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_fragment(self: Box<Self>) -> Box<Fragment<B, D, P>> {
        Fragment::new_boxed_unchecked(self.into_string())
    }

    /// Converts a boxed identifier into a boxed string slice.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeIdent> = Ident::new_boxed("snake_ident")?;
    /// let ident: Box<str> = ident.into_boxed_str();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_str(self: Box<Self>) -> Box<str> {
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
    /// let ident: Box<LowerSnakeIdent> = Ident::new_boxed("snake_ident")?;
    /// let buffer: LowerSnakeFragmentBuf = ident.into_fragment_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_fragment_buf(self: Box<Self>) -> FragmentBuf<B, D, P> {
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
    /// let ident: Box<LowerSnakeIdent> = Ident::new_boxed("snake_ident")?;
    /// let ident: String = ident.into_string();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_string(self: Box<Self>) -> String {
        self.into_boxed_str().into()
    }

    /// Converts a string into a boxed identifier, bypassing checks.
    #[must_use]
    #[inline]
    pub(crate) fn new_boxed_unchecked(string: String) -> Box<Self> {
        let boxed_str = string.into_boxed_str();
        // SAFETY: Ident is transparent over Fragment, which is itself
        // transparent over str, so Box<Ident> has the same allocation layout,
        // pointer metadata, alignment, and ownership behavior as Box<str>.
        unsafe { Box::from_raw(Box::into_raw(boxed_str) as *mut Self) }
    }

    /// Converts an identifier into an owned boxed identifier.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: &LowerSnakeIdent = Ident::new("snake_ident")?;
    /// let ident: Box<LowerSnakeIdent> = ident.to_boxed_ident();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn to_boxed_ident(&self) -> Box<Self> {
        Ident::new_boxed_unchecked(String::from(self.as_str()))
    }

    /// Attempts a fallible convert into a different boxed type-configured
    /// identifier.
    ///
    /// You should first attempt to call [`convert`] on a type, if that compiles
    /// it is preferred to this function (and you will not need to call this
    /// function), because it is truly zero-cost.
    ///
    /// This is equivalent to just calling [`new_boxed`] on the target type with
    /// the current type's string contents. This function is provided for
    /// ergonomic convenience.
    ///
    /// [`convert`]: Self::convert
    /// [`new_boxed`]: Self::new_boxed
    #[inline]
    pub fn try_convert<B2, D2, P2>(self: Box<Self>) -> Result<Box<Ident<B2, D2, P2>>, Error>
    where
        B2: Boundary,
        D2: Delimiter,
        P2: CasedProfile,
    {
        Ident::new_boxed(self.into_string())
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P> crate::type_erased::TypeErased for Box<Ident<B, D, P>> {
    type Erased = String;
    #[inline(always)]
    fn type_erased(self) -> Self::Erased {
        self.into_string()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> std_alloc::borrow::ToOwned for Ident<B, D, P> {
    type Owned = Box<Ident<B, D, P>>;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.to_boxed_ident()
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> TryFrom<String> for Box<Ident<B, D, P>> {
    type Error = crate::Error;
    #[inline]
    fn try_from(orig: String) -> Result<Self, Self::Error> {
        Ident::new_boxed(orig)
    }
}

// -----------------------------------------------------------------------------
impl_boxed_slice_traits! {
    name=Ident,
    index_target=Fragment,
}
