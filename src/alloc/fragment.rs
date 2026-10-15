// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::intermediate::IntoIntermediate;
use crate::syntax::{Boundary, CasedProfile, Delimiter};
use crate::{Error, ErrorKind, Fragment, FragmentBuf};
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
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// If you're working with a fragment format which has only a single valid
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
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.join("fragment")?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn join<F>(&self, fragment: F) -> Result<Box<Self>, Error>
    where
        D: Default,
        F: IntoIntermediate<B, D, P>,
    {
        self.join_with(fragment, D::default())
    }

    /// Returns a heap-allocated fragment, joined with the original fragment in
    /// a way that preserves chunk boundaries.
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
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.join_with("fragment", LowLine)?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn join_with<F>(&self, fragment: F, delim: D) -> Result<Box<Self>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let fragment = fragment.into_intermediate()?;
        let fragment: &Self = fragment.as_ref();
        let mut buffer = FragmentBuf::with_overhead(self, fragment.len() + 1);
        buffer
            .push_bounded_with(fragment, delim)
            .map_err(|_| Error::new(ErrorKind::FailedJoin))?;
        Ok(buffer.into_boxed_fragment())
    }

    /// Converts a string into a boxed fragment if it's valid.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the fragment does not satisfy the character
    /// requirements. If an invalid character is found then an `InvalidFormat`
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
    /// let fragment: Box<LowerSnakeFragment> = Fragment::new_boxed("snake_fragment")?;
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn new_boxed<S>(string: S) -> Result<Box<Self>, Error>
    where
        S: Into<String>,
    {
        let string = string.into();
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
    /// let fragment = LowerSnakeFragment::new("example_snake_fragment")?;
    /// let fragment = fragment.replace("snake", "serpent")?;
    /// assert_eq!(fragment, "example_serpent_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn replace<M, F>(&self, from: M, to: F) -> Result<Box<Self>, Error>
    where
        M: crate::pattern::Pattern,
        F: IntoIntermediate<B, D, P>,
    {
        let to = to.into_intermediate()?;
        Fragment::new_boxed(from.replace(self.as_str(), to.as_ref()))
            .map_err(|_| Error::new(ErrorKind::FailedReplace))
    }

    /// Returns a heap-allocated fragment with the provided prefix and suffix
    /// attached to the original fragment.
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
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.with_circumfix("lower_", "_fragment")?;
    /// assert_eq!(fragment, "lower_snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
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
        Fragment::new_boxed(format!("{prefix}{self}{suffix}"))
            .map_err(|_| Error::new(ErrorKind::FailedCircumfixing))
    }

    /// Returns a heap-allocated fragment with the provided prefix attached to
    /// the original fragment.
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
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.with_prefix("lower_")?;
    /// assert_eq!(fragment, "lower_snake");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn with_prefix<F>(&self, prefix: F) -> Result<Box<Self>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let prefix = prefix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidPrefix))?;
        Fragment::new_boxed(format!("{prefix}{self}"))
            .map_err(|_| Error::new(ErrorKind::FailedPrefixing))
    }

    /// Returns a heap-allocated fragment with the provided suffix attached to
    /// the original fragment.
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
    /// let fragment = LowerSnakeFragment::new("snake")?;
    /// let fragment = fragment.with_suffix("_fragment")?;
    /// assert_eq!(fragment, "snake_fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this function returns an allocated fragment, it does not mutate the original"]
    #[inline]
    pub fn with_suffix<F>(&self, suffix: F) -> Result<Box<Self>, Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let suffix = suffix
            .into_intermediate()
            .map_err(|e| e.with_error_kind(ErrorKind::InvalidSuffix))?;
        Fragment::new_boxed(format!("{self}{suffix}"))
            .map_err(|_| Error::new(ErrorKind::FailedSuffixing))
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Fragment<B, D, P> {
    /// Zero-cost conversion into a different boxed, type-configured fragment.
    ///
    /// This is the boxed, by-value equivalent of [`cast`], see that function
    /// for details on how this works. If you don't need to consume the boxed
    /// value, you can instead use that function to get a reference to a
    /// different type-configured fragment.
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
    /// let original = LowerSnakeFragment::new_boxed("apple")?;
    /// let converted: Box<LowerCamelFragment> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// // Bad Cast (Fails Compilation):
    /// let original = LowerCamelFragment::new_boxed("apple")?;
    /// let converted: Box<LowerSnakeFragment> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Example traversing character profile boundary:
    ///
    /// ```
    /// # use typed_ident::presets::ascii::lower_snake as ascii;
    /// # use typed_ident::presets::unicode::lower_snake as unicode;
    /// // Compilable Cast:
    /// let original = ascii::LowerSnakeFragment::new_boxed("apple")?;
    /// let converted: Box<unicode::LowerSnakeFragment> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::ascii::lower_snake as ascii;
    /// # use typed_ident::presets::unicode::lower_snake as unicode;
    /// // Bad Cast (Fails Compilation):
    /// let original = unicode::LowerSnakeFragment::new_boxed("apple")?;
    /// let converted: Box<ascii::LowerSnakeFragment> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Example traversing delimiter boundary:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// // Compilable Cast:
    /// let original = LowerSnakeFragment::new_boxed("apple")?;
    /// let converted: Box<HybridFragment> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// // Bad Cast (Fails Compilation):
    /// let original = HybridFragment::new_boxed("apple")?;
    /// let converted: Box<LowerSnakeFragment> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn convert<B2, D2, P2>(self: Box<Self>) -> Box<Fragment<B2, D2, P2>>
    where
        B2: crate::syntax::Boundary,
        D: crate::syntax::SubsetOf<D2>,
        P: crate::syntax::SubsetOf<P2>,
    {
        Fragment::new_boxed_unchecked(self.into_string())
    }

    /// Converts a boxed fragment into a boxed string slice.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment: Box<LowerSnakeFragment> = Fragment::new_boxed("snake_fragment")?;
    /// let fragment: Box<str> = fragment.into_boxed_str();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_str(self: Box<Self>) -> Box<str> {
        // SAFETY: Fragment is transparent over str, so Box<Fragment> has the
        // same allocation layout, pointer metadata, alignment, and ownership
        // behavior as Box<str>.
        unsafe { Box::from_raw(Box::into_raw(self) as *mut str) }
    }

    /// Converts a boxed fragment into a fragment buffer.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment: Box<LowerSnakeFragment> = Fragment::new_boxed("snake_fragment")?;
    /// let buffer: LowerSnakeFragmentBuf = fragment.into_fragment_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_fragment_buf(self: Box<Self>) -> FragmentBuf<B, D, P> {
        FragmentBuf::from_string_unchecked(self.into_string())
    }

    /// Converts a boxed fragment into a string.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let fragment: Box<LowerSnakeFragment> = Fragment::new_boxed("snake_fragment")?;
    /// let buffer: String = fragment.into_string();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_string(self: Box<Self>) -> String {
        self.into_boxed_str().into()
    }

    /// Converts a string into a boxed fragment, bypassing checks.
    #[must_use]
    #[inline]
    pub(crate) fn new_boxed_unchecked(string: String) -> Box<Self> {
        let boxed_str = string.into_boxed_str();
        // SAFETY: Fragment is transparent over str, so Box<Fragment> has the
        // same allocation layout, pointer metadata, alignment, and ownership
        // behavior as Box<str>.
        unsafe { Box::from_raw(Box::into_raw(boxed_str) as *mut Self) }
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
    pub fn to_boxed_fragment(&self) -> Box<Self> {
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

    /// Attempts a fallible convert into a different boxed type-configured
    /// fragment.
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
    pub fn try_convert<B2, D2, P2>(self: Box<Self>) -> Result<Box<Fragment<B2, D2, P2>>, Error>
    where
        B2: Boundary,
        D2: Delimiter,
        P2: CasedProfile,
    {
        Fragment::new_boxed(self.into_string())
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P> crate::type_erased::TypeErased for Box<Fragment<B, D, P>> {
    type Erased = String;
    #[inline(always)]
    fn type_erased(self) -> Self::Erased {
        self.into_string()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> std_alloc::borrow::ToOwned for Fragment<B, D, P> {
    type Owned = Box<Fragment<B, D, P>>;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.to_boxed_fragment()
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> TryFrom<String> for Box<Fragment<B, D, P>> {
    type Error = crate::Error;
    #[inline]
    fn try_from(orig: String) -> Result<Self, Self::Error> {
        Fragment::new_boxed(orig)
    }
}

// -----------------------------------------------------------------------------
impl_boxed_slice_traits! {
    name=Fragment,
    index_target=Fragment,
}
