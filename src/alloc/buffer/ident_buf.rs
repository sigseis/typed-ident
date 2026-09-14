// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::FragmentBuf;
use crate::core::error::{Error, ErrorKind};
use crate::core::{Chunk, Fragment, Ident};
use crate::syntax::{Boundary, CasedProfile, Delimiter, UnitDelimiter};
use core::ops::RangeBounds;
use std_alloc::borrow::ToOwned;
use std_alloc::boxed::Box;
use std_alloc::collections::TryReserveError;
use std_alloc::string::String;

// =============================================================================
// TYPES
// =============================================================================

/// A dynamic, growable identifier.
///
/// This allows you to build a identifier dynamically, instead of having to get
/// one from a string slice.
///
/// # Not Inherently an `Ident`
///
/// Unlike [`Fragment`]s, [`Ident`]s have a requirement not only of being
/// comprised of certain characters in a certain order, but *also* that the
/// identifier itself is *not empty*.
///
/// Because of that, and because `IdentBuf` *can* be empty, it might surprise
/// you to realize that `IdentBuf` does *NOT* implement `Deref` to `Ident` (as
/// `String`, `PathBuf`, or indeed, `FragmentBuf` would for their respective
/// immutably borrowed counterparts).
///
/// Instead, you must attempt a fallible cast to an identifier, which can only
/// fail if the buffer itself is empty.
///
/// ```
/// # use typed_ident::presets::unicode::upper_camel::*;
/// let mut buffer = UpperCamelIdentBuf::new();
/// assert!(buffer.as_ident().is_none());
///
/// buffer.push('V')?;
/// assert!(buffer.as_ident().is_some());
/// # Ok::<(), typed_ident::Error>(())
/// ```
///
/// If your chosen delimiter implements `Default`, there's a nice helper
/// function for this, which does what I believe most people would do in this
/// situation; see [`as_ident_or_anonymous`].
///
/// [`as_ident_or_anonymous`]: Self::as_ident_or_anonymous
pub struct IdentBuf<B, D, P> {
    pub(super) inner: FragmentBuf<B, D, P>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> IdentBuf<B, D, P> {
    #[doc = include_str!("docs/methods/split_off.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelIdentBuf::from_str("UpperCamel")?;
    /// let split = buffer.split_off(5);
    /// assert_eq!(buffer, "Upper");
    /// assert_eq!(split, "Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn split_off(&mut self, mid: usize) -> FragmentBuf<B, D, P> {
        assert!(
            P::is_ident_split_boundary::<D>(self.as_str(), mid),
            "provided index `mid` is not a valid split point for this identifier",
        );
        self.inner.split_off(mid)
    }

    /// Truncates the buffer to the provided length.
    ///
    /// This has the same properties as [`String::truncate`].
    ///
    /// # Panics
    ///
    /// This will panic if the provided `len` value does not lie on a
    /// character sequence boundary, or if the remainder of the string is no
    /// longer a valid identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridIdentBuf::from_str("example")?;
    /// assert_eq!(buffer, "example");
    /// buffer.truncate(4);
    /// assert_eq!(buffer, "exam");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        assert!(
            P::is_ident_split_boundary::<D>(self.as_str(), len),
            "provided index `len` is not a valid truncation point for this identifier",
        );
        self.inner.truncate(len)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> IdentBuf<B, D, P> {
    /// Fallibly casts the buffer to an identifier.
    ///
    /// This can fail because the buffer can be empty. As long as the buffer is
    /// *not* empty, than this will succeed (because this type enforces that
    /// what is added to the buffer conforms to the requirements of a valid
    /// identifier).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// let mut buffer = LowerCamelIdentBuf::new();
    /// assert_eq!(buffer.as_ident(), None);
    ///
    /// buffer.push_str("ident")?;
    /// assert_eq!(buffer.as_ident(), Some(LowerCamelIdent::new("ident")?));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn as_ident(&self) -> Option<&Ident<B, D, P>> {
        match self.is_empty() {
            true => None,
            false => Some(Ident::new_unchecked(self.inner.as_str())),
        }
    }

    /// Either returns the identifier contained by this buffer (if non-empty),
    /// or return the provided default identifier.
    ///
    /// This is equivalent to `as_ident().unwrap_or(_)`, but it's provided for
    /// convenience and to parallel the [`as_ident_or_anonymous`].
    ///
    /// [`as_ident_or_anonymous`]: Self::as_ident_or_anonymous
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// let fallback = LowerCamelIdent::new("fallback")?;
    /// let mut buffer = LowerCamelIdentBuf::new();
    /// assert_eq!(buffer.as_ident_or(fallback), fallback);
    ///
    /// buffer.push_str("ident")?;
    /// assert_eq!(buffer.as_ident_or(fallback), "ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn as_ident_or<'a>(&'a self, default: &'a Ident<B, D, P>) -> &'a Ident<B, D, P> {
        match self.is_empty() {
            true => default,
            false => Ident::new_unchecked(self.inner.as_str()),
        }
    }

    /// Either returns the identifier contained by this buffer (if non-empty),
    /// or return a single delimiter representing an anonymous value.
    ///
    /// This can only be called if it's obvious which delimiter should be
    /// provided, and that is only possible for [`UnitDelimiter`] delimiters
    /// (Like [`LowLine`] and [`HyphenMinus`]).
    ///
    /// [`HyphenMinus`]: crate::syntax::delimiter::HyphenMinus
    /// [`LowLine`]: crate::syntax::delimiter::LowLine
    /// [`UnitDelimiter`]: crate::syntax::delimiter::UnitDelimiter
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// let mut buffer = LowerCamelIdentBuf::new();
    /// assert_eq!(buffer.as_ident_or_anonymous(), "_");
    ///
    /// buffer.push_str("ident")?;
    /// assert_eq!(buffer.as_ident_or_anonymous(), "ident");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn as_ident_or_anonymous(&self) -> &Ident<B, D, P>
    where
        D: UnitDelimiter,
    {
        match self.is_empty() {
            true => Ident::new_unchecked(D::STR),
            false => Ident::new_unchecked(self.inner.as_str()),
        }
    }

    /// Converts an ident into an ident buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("Example")?;
    /// let mut buffer = UpperCamelIdentBuf::from_ident(ident);
    /// assert_eq!(buffer, "Example");
    /// # Ok::<(), Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn from_ident(orig: &Ident<B, D, P>) -> Self {
        Self {
            inner: FragmentBuf::from_fragment(orig.as_fragment()),
        }
    }

    /// Converts an allocated string into an ident buffer, without checking if
    /// the allocated string is a valid identifier or not.
    ///
    /// # Safety
    ///
    /// You can only call this if the input string is from a valid [`Ident`]
    /// over the same generic parameters, or if you have ensured the string
    /// *would* have been valid.
    ///
    /// Needless to say, this is difficult to know unless you are taking a
    /// prefixed slice of an existing ident, or if you are testing this at
    /// compile time.
    ///
    /// [`Ident`]: crate::core::Ident
    #[must_use]
    #[inline]
    pub(crate) fn from_string_unchecked(s: String) -> Self {
        Self {
            inner: FragmentBuf::from_string_unchecked(s),
        }
    }

    /// Convert the buffer into a boxed identifier (if possible).
    ///
    /// This can fail if the buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridIdentBuf::from_str("example")?;
    /// let boxed: Box<HybridIdent> = buffer.into_boxed_ident().unwrap();
    /// assert_eq!(boxed.as_ref(), "example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_ident(self) -> Option<Box<Ident<B, D, P>>> {
        if self.inner.is_empty() {
            return None;
        }
        Some(Ident::new_boxed_unchecked(self.into_string()))
    }

    /// Convert the buffer into an owned string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let buffer = UpperCamelIdentBuf::from_str("Example")?;
    /// let string: String = buffer.into_string();
    /// assert_eq!(string, "Example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_string(self) -> String {
        self.inner.into_string()
    }

    /// Leaks the identifier so that it lives for the rest of the execution of
    /// the program.
    ///
    /// This can fail if the buffer is empty.
    ///
    /// This is a typed wrapper over the [`String::leak`] method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelIdentBuf::from_str("Example")?;
    /// let string: &'static UpperCamelFragment = fragment.leak().unwrap();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn leak<'a>(self) -> Option<&'a Ident<B, D, P>> {
        match self.is_empty() {
            true => None,
            false => Some(Ident::new_unchecked(self.inner.into_string().leak())),
        }
    }

    /// Constructs an empty ident buffer with an initial capacity.
    ///
    /// This has the same properties as [`String::with_capacity`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let buffer = UpperCamelIdentBuf::with_capacity(10);
    /// assert!(buffer.capacity() >= 10);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: FragmentBuf::with_capacity(capacity),
        }
    }

    /// Constructs an ident buffer with enough space to hold the provided ident,
    /// as well as `additional` bytes, then initializes the contents of this
    /// buffer to `ident`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("Example")?;
    /// let buffer = UpperCamelIdentBuf::with_overhead(ident, 20);
    /// assert!(buffer.capacity() >= 27);
    /// assert_eq!(buffer, "Example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn with_overhead(ident: &Ident<B, D, P>, additional: usize) -> Self {
        Self {
            inner: FragmentBuf::with_overhead(ident, additional),
        }
    }
}

// -----------------------------------------------------------------------------
impl_buffer_methods! {
    name=IdentBuf,
    op=IdentBufOp,
}

// =============================================================================
// TRAIT IMPL
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> core::str::FromStr for IdentBuf<B, D, P> {
    type Err = Error;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_ident(Ident::new(s)?))
    }
}

// -----------------------------------------------------------------------------
impl<'a, B, D, P> core::convert::From<&'a Ident<B, D, P>> for IdentBuf<B, D, P> {
    #[inline]
    fn from(orig: &'a Ident<B, D, P>) -> Self {
        Self::from_ident(orig)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Clone for IdentBuf<B, D, P> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> ToOwned for Ident<B, D, P> {
    type Owned = Box<Ident<B, D, P>>;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        IdentBuf::from(self)
            .into_boxed_ident()
            .expect("it should never be the case that an ident fails conversion to a boxed ident")
    }
}

// -----------------------------------------------------------------------------
impl_buffer_traits! {
    name=IdentBuf,
}

// -----------------------------------------------------------------------------
impl_typed_slice_traits! {
    name=IdentBuf,
    index_target=Fragment,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=IdentBuf,
    against=Chunk,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=IdentBuf,
    against=Fragment,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=IdentBuf,
    against=FragmentBuf,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=IdentBuf,
    against=Ident,
}
