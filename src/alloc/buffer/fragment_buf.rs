// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::IntoIntermediate;
use crate::core::error::Error;
use crate::core::{Chunk, Fragment, Ident};
use crate::syntax::{Boundary, CasedProfile, Delimiter};
use core::marker::PhantomData;
use core::ops::RangeBounds;
use std_alloc::boxed::Box;
use std_alloc::collections::TryReserveError;
use std_alloc::string::String;

// =============================================================================
// TYPES
// =============================================================================

/// A dynamic, growable fragment.
///
/// This allows you to build a fragment dynamically, instead of having to get a
/// fragment from an identifier slice.
pub struct FragmentBuf<B, D, P> {
    config: PhantomData<(B, D, P)>,
    pub(super) inner: String,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> FragmentBuf<B, D, P> {
    /// Attempts to represent the current fragment buffer as an [`Ident`].
    ///
    /// This may fail - a fragment isn't obviously a valid identifier, plus the
    /// fragment could be empty (which is never a valid identifier).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::hybrid::HybridFragmentBuf;
    /// let mut buffer = HybridFragmentBuf::new();
    /// assert!(buffer.as_ident().is_err()); // Empty
    /// buffer.push('2')?;
    /// assert!(buffer.as_ident().is_err()); // Invalid start character
    ///
    /// buffer.clear();
    /// buffer.push('a')?;
    /// assert!(buffer.as_ident().is_ok()); // Valid!
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn as_ident(&self) -> Result<&Ident<B, D, P>, Error> {
        Ident::from_fragment(self.as_fragment())
    }

    /// Attempts to convert the current fragment buffer into a boxed [`Ident`].
    ///
    /// This may fail - a fragment isn't obviously a valid identifier, plus the
    /// fragment could be empty (which is never a valid identifier).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::hybrid::HybridFragmentBuf;
    /// let mut buffer = HybridFragmentBuf::new();
    /// assert!(buffer.into_boxed_ident().is_err()); // Empty
    ///
    /// let mut buffer = HybridFragmentBuf::new();
    /// buffer.push('2')?;
    /// assert!(buffer.into_boxed_ident().is_err()); // Invalid start character
    ///
    /// let mut buffer = HybridFragmentBuf::new();
    /// buffer.push('a')?;
    /// assert!(buffer.into_boxed_ident().is_ok()); // Valid!
    /// # Ok::<(), Error>(())
    /// ```
    pub fn into_boxed_ident(self) -> Result<Box<Ident<B, D, P>>, Error> {
        P::is_ident_fragment::<D>(self.as_str())?;
        Ok(Ident::new_boxed_unchecked(self.inner))
    }

    #[doc = include_str!("docs/methods/split_off.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    ///
    /// # Errors
    ///
    /// If the replacement of the range provided with the given fragment would
    /// lead to an invalid buffer, then the range will not be remove and instead
    /// the error `InvalidReplace` will be returned.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    /// let split = buffer.split_off(5);
    /// assert_eq!(buffer, "Upper");
    /// assert_eq!(split, "Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn split_off(&mut self, idx: usize) -> FragmentBuf<B, D, P> {
        Self::from_string_unchecked(self.inner.split_off(idx))
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> FragmentBuf<B, D, P> {
    /// Converts a fragment slice into a fragment buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::UpperCamelFragment;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// let fragment = UpperCamelFragment::new("example")?;
    /// let mut buffer = UpperCamelFragmentBuf::from_fragment(fragment);
    /// assert_eq!(buffer, "example");
    /// # Ok::<(), Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn from_fragment(fragment: &Fragment<B, D, P>) -> Self {
        Self::from_string_unchecked(String::from(fragment.as_str()))
    }

    /// Converts an allocated string into a fragment buffer, without checking if
    /// the allocated string is a valid fragment or not.
    ///
    /// # Safety
    ///
    /// You can only call this if the input string is from a valid [`Fragment`]
    /// over the same generic parameters, or if you have ensured the string
    /// *would* have been valid.
    ///
    /// Needless to say, this is difficult to know unless you are taking a slice
    /// of an existing fragment/ident/etc, or if you are testing this at compile
    /// time.
    ///
    /// [`Fragment`]: crate::core::Fragment
    #[must_use]
    #[inline]
    pub(crate) fn from_string_unchecked(orig: String) -> Self {
        Self {
            config: PhantomData,
            inner: orig,
        }
    }

    /// Convert the buffer into an owned string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragmentBuf::from_str("example")?;
    /// let string: String = fragment.into_string();
    /// assert_eq!(string, "example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_string(self) -> String {
        self.inner
    }

    /// Leaks the fragment so that it lives for the rest of the execution of the
    /// program.
    ///
    /// This is a typed wrapper over the [`String::leak`] method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragmentBuf::from_str("example")?;
    /// let string: &'static UpperCamelFragment = fragment.leak();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn leak<'a>(self) -> &'a Fragment<B, D, P> {
        Fragment::new_unchecked(self.inner.leak())
    }

    /// Truncates the buffer to the provided length.
    ///
    /// This has the same properties as [`String::truncate`].
    ///
    /// # Panics
    ///
    /// This will panic if the provided `len` value does not lie on a
    /// character sequence boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::from_str("example")?;
    /// assert_eq!(buffer, "example");
    /// buffer.truncate(4);
    /// assert_eq!(buffer, "exam");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len)
    }

    /// Constructs an empty fragment buffer with an initial capacity.
    ///
    /// This has the same properties as [`String::with_capacity`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let buffer = UpperCamelFragmentBuf::with_capacity(10);
    /// assert!(buffer.capacity() >= 10);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            config: PhantomData,
            inner: String::with_capacity(capacity),
        }
    }

    /// Constructs a fragment with enough space to hold the provided fragment,
    /// as well as `additional` bytes, then initializes the contents of this
    /// buffer to `fragment`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("example")?;
    /// let buffer = UpperCamelFragmentBuf::with_overhead(fragment, 20);
    /// assert!(buffer.capacity() >= 27);
    /// assert_eq!(buffer, "example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn with_overhead(fragment: &Fragment<B, D, P>, additional: usize) -> Self {
        let mut inner = String::with_capacity(fragment.len() + additional);
        inner.push_str(fragment.as_str());
        Self {
            config: PhantomData,
            inner,
        }
    }
}

// -----------------------------------------------------------------------------
impl_buffer_methods! {
    name=FragmentBuf,
    op=FragmentBufOp,
}

// =============================================================================
// TRAIT IMPL
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, P> core::convert::From<&'a Fragment<B, D, P>> for FragmentBuf<B, D, P> {
    #[inline]
    fn from(orig: &'a Fragment<B, D, P>) -> Self {
        Self::from_fragment(orig)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Clone for FragmentBuf<B, D, P> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            config: PhantomData,
            inner: self.inner.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> std_alloc::borrow::Borrow<Fragment<B, D, P>> for FragmentBuf<B, D, P> {
    #[inline]
    fn borrow(&self) -> &Fragment<B, D, P> {
        self.as_fragment()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> std_alloc::borrow::ToOwned for Fragment<B, D, P> {
    type Owned = FragmentBuf<B, D, P>;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.to_fragment_buf()
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> core::str::FromStr for FragmentBuf<B, D, P> {
    type Err = Error;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_fragment(Fragment::new(s)?))
    }
}

// -----------------------------------------------------------------------------
impl_buffer_traits! {
    name=FragmentBuf,
}

// -----------------------------------------------------------------------------
impl_typed_slice_traits! {
    name=FragmentBuf,
    index_target=Fragment,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=FragmentBuf,
    against=Chunk,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=FragmentBuf,
    against=Fragment,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=FragmentBuf,
    against=Ident,
}
