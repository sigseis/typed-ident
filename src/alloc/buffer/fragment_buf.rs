// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::IntoIntermediate;
use crate::alloc::buffer::FragmentBufOp;
use crate::core::{Chunk, Error, ErrorKind, Fragment, Ident};
use crate::syntax::{Boundary, CasedProfile, Delimiter};
use core::marker::PhantomData;
use core::ops::RangeBounds;
use std_alloc::boxed::Box;
use std_alloc::collections::TryReserveError;
use std_alloc::string::String;

// =============================================================================
// TYPES
// =============================================================================

/// A dynamic, growable [`Fragment`].
///
/// This type can either be constructed directly, or can be returned as a result
/// from some operation on another chunk, fragment, or identifier which may not
/// leave it in the original checked format (like removing part of an identifier
/// and ending up with something which is not validly an identifier any more).
///
/// This allows you to build a fragment dynamically, instead of having to get a
/// fragment from an identifier slice. Every operation performed to mutate a
/// fragment will be checked to ensure that the result is still a valid
/// fragment.
///
/// This is also how one is expected to build a dynamic identifier from scratch.
/// Start with a `FragmentBuf`, make the modifications you want to make, then
/// attempt to convert it into either an `&Ident` or `Box<Ident>`.
///
/// [`Fragment`]: crate::core::Fragment
///
/// # Construction
///
/// If you want to construct a new fragment buffer, it is expected that you do
/// so through some type alias which fully defines the fragment buffer.
///
/// The common way to get a reasonable alias is through the [`presets`] module.
///
/// [`presets`]: crate::presets
///
/// ```
/// use typed_ident::syntax::delimiter::*;
/// use typed_ident::presets::unicode::lower_snake::LowerSnakeFragmentBuf;
///
/// // You can build a fragment buffer dynamically.
/// let mut buffer = LowerSnakeFragmentBuf::new();
/// buffer.push("lower")?;
/// buffer.push(LowLine)?;
/// buffer.push("snake")?;
///
/// // When you're done with it, you can cast it to a fragment.
/// let fragment = buffer.as_fragment();
/// assert_eq!(fragment, "lower_snake");
///
/// // Or, if it forms a valid identifier, to a identifier.
/// let ident = buffer.as_ident()?;
/// assert_eq!(fragment, "lower_snake");
/// # Ok::<(), typed_ident::Error>(())
/// ```
///
/// These presets are all just type aliases to `FragmentBuf` with the generic
/// type parameters filled-in. If you would like to define your own custom
/// fragment buffer, it's recommended that you do so by defining your own type
/// alias.
///
/// ```
/// use typed_ident::FragmentBuf;
/// use typed_ident::syntax::{boundary, delimiter, profile};
///
/// // An ASCII fragment using the uppercase ASCII profile.
/// // Any ASCII punctuation is rejected as delimiters.
/// type CustomFragmentBuf = FragmentBuf<
///     boundary::Standard,
///     delimiter::AsciiPunctuation,
///     profile::Upper<profile::Ascii>,
/// >;
///
/// let ident = CustomFragmentBuf::from_str("UPPER#@IDENT")?;
/// # Ok::<(), typed_ident::Error>(())
/// ```
///
/// # Type Parameters
///
/// The type parameters used on this type are:
///
/// * `B`: [`Boundary`] (a boundary definition; usually [`Standard`])
/// * `D`: [`Delimiter`] (a delimiter type; [`HyphenMinus`], [`LowLine`], etc.)
/// * `P`: [`CasedProfile`] (a cased profile; which is...)
///   * A wrapping case profile (e.g. [`Mixed`], [`Lower`], etc.)
///   * A specific character profile (e.g. [`Ascii`], [`Unicode`], etc.)
///
/// See the [`syntax`] module definition if you plan on defining your own type
/// aliases to understand better what these types mean and how they work.
///
/// [`Ascii`]: crate::syntax::profile::Ascii
/// [`Boundary`]: crate::syntax::boundary::Boundary
/// [`Delimiter`]: crate::syntax::delimiter::Delimiter
/// [`HyphenMinus`]: crate::syntax::delimiter::HyphenMinus
/// [`Ident`]: crate::core::Ident
/// [`LowLine`]: crate::syntax::delimiter::LowLine
/// [`Lower`]: crate::syntax::profile::Lower
/// [`Mixed`]: crate::syntax::profile::Mixed
/// [`Profile`]: crate::syntax::profile::Profile
/// [`Standard`]: crate::syntax::boundary::Standard
/// [`SubsetOf`]: crate::syntax::SubsetOf
/// [`Unicode`]: crate::syntax::profile::Unicode
/// [`presets`]: crate::presets
/// [`syntax`]: crate::syntax
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
    /// # Errors
    ///
    /// Returns [`Error`] if the fragment does not satisfy the identifier
    /// character requirements. If the fragment is empty, this will return an
    /// `Empty` error kind. If an invalid character is found then a
    /// `InvalidFormat` error kind is returned, with [`byte_offset`] set to the
    /// byte index for the first invalid character.
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

    /// Constructs a fragment buffer, initializing the contents to a provided
    /// string slice (attempting first to convert the string slice to a valid
    /// fragment).
    ///
    /// This is equivalent to `FragmentBuf::from_fragment(Fragment::new(s)?)`.
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
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// assert!(UpperCamelFragmentBuf::from_str("").is_ok());
    /// assert!(UpperCamelFragmentBuf::from_str("ValidUpperCamel").is_ok());
    /// assert!(UpperCamelFragmentBuf::from_str("continuingUpperCamel").is_ok());
    /// assert!(UpperCamelFragmentBuf::from_str("not_validUpperCamel").is_err());
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    #[allow(clippy::should_implement_trait)] // It *does* implement the trait.
    pub fn from_str(s: &str) -> Result<Self, Error> {
        core::str::FromStr::from_str(s)
    }

    /// Constructs a fragment buffer, initializing the contents to a provided
    /// buffered string (checking first that the string is a valid fragment).
    ///
    /// This is similar to [`from_str`], except that it will not allocate a
    /// separate string. It will use the provided string, if it's valid.
    ///
    /// [`from_str`]: Self::from_str
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
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// assert!(UpperCamelFragmentBuf::from_string(String::from("")).is_ok());
    /// assert!(UpperCamelFragmentBuf::from_string(String::from("ValidUpperCamel")).is_ok());
    /// assert!(UpperCamelFragmentBuf::from_string(String::from("continuingUpperCamel")).is_ok());
    /// assert!(UpperCamelFragmentBuf::from_string(String::from("not_validUpperCamel")).is_err());
    /// # Ok::<(), Error>(())
    #[inline]
    pub fn from_string(s: String) -> Result<Self, Error> {
        P::is_fragment::<D>(&s)?;
        Ok(Self::from_string_unchecked(s))
    }

    /// Attempts to insert a fragment into the buffer.
    ///
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// # Panics
    ///
    /// This function will panic if the byte index is larger than the element's
    /// length, *or* if it does not fall on a character sequence boundary.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the insertion of `fragment` into `self` cannot produce a valid
    /// result.
    ///
    /// If the intermediate fragment is invalid, then the `InvalidFormat` error
    /// kind will be returned, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the insertion operation itself failed, then the `FailedInsert` error
    /// kind is returned, without setting the `byte_offset`.
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
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend.
    /// let mut example = buffer.clone();
    /// assert!(example.insert(0, 'V').is_ok());
    /// assert!(example.insert(0, '_').is_ok());
    /// assert_eq!(example, "_VUpperCamel");
    ///
    /// // Inserting at the end is ~append.
    /// let mut example = buffer.clone();
    /// example.push('_')?;
    /// assert!(example.insert(example.len(), 'i').is_err());
    /// assert!(example.insert(example.len(), 'V').is_ok());
    /// assert_eq!(example, "UpperCamel_V");
    ///
    /// // Inserting in the middle can be tricky, as your insertions
    /// // may invalidate the buffer's invariants in surprising ways.
    /// let mut example = buffer.clone();
    /// assert!(example.insert(2, '_').is_err()); // "Up_perCamel" != UpperCamel casing
    /// assert!(example.insert(5, '_').is_ok());
    /// assert_eq!(example, "Upper_Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert<F>(&mut self, idx: usize, fragment: F) -> Result<(), Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let fragment = fragment.into_intermediate()?;
        FragmentBufOp::new(self).insert_str(idx, fragment.as_ref())
    }

    /// Attempts to insert a fragment into the buffer, preserving chunk
    /// boundaries by conditionally inserting delimiters where needed.
    ///
    /// This is a convenience function for cases when the delimiter value can be
    /// deduced by the `Default` trait. For more information on this operation,
    /// see the [`insert_bounded_with`] method.
    ///
    /// [`insert_bounded_with`]: Self::insert_bounded_with
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded(0, 'V').is_ok()); // Bounded because of `HAT` rules.
    /// assert!(example.insert_bounded(0, 'V').is_ok()); // But another `V` would not be.
    /// assert_eq!(example, "V_VUpperCamel");
    ///
    /// // Inserting at the end is ~append.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded(example.len(), 'i').is_err()); // "UpperCamel_i" != UpperCamel casing
    /// assert!(example.insert_bounded(example.len(), 'V').is_ok()); // Because of `CAMEL` boundary.
    /// assert_eq!(example, "UpperCamelV");
    ///
    /// // Inserting in the middle can be tricky, as your insertions
    /// // may invalidate the buffer's invariants in surprising ways.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded(2, 'V').is_err()); // "Up_V_perCamel" != UpperCamel casing
    /// assert!(example.insert_bounded(5, 'V').is_ok()); // Surprisingly a `CAMEL` & `HAT` boundary.
    /// assert_eq!(example, "UpperVCamel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_bounded<F>(&mut self, idx: usize, fragment: F) -> Result<(), Error>
    where
        D: Default,
        F: IntoIntermediate<B, D, P>,
    {
        self.insert_bounded_with(idx, fragment, Default::default())
    }

    /// Attempts to insert a fragment into the buffer, preserving chunk
    /// boundaries by conditionally inserting delimiters where needed.
    ///
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// If you're working with a fragment format which has only a single valid
    /// delimiter value, you should instead be able to use the
    /// [`insert_bounded`] method, and should prefer that.
    ///
    /// [`insert_bounded`]: Self::insert_bounded
    ///
    /// # Preserving Chunk Boundaries
    ///
    /// If the fragment already contains delimiters on both ends, it will
    /// always be inserted verbatim.
    ///
    /// If the fragment does *NOT* contain delimiters on both ends, the fragment
    /// will first be inserted, and then it will be tested to ensure that both
    /// sides of the fragment don't merge into the surrounding chunks. If they
    /// would, a delimiter will be inserted to force separation.
    ///
    /// Whether or not a delimiter is needed is found using the [`Boundary`]
    /// trait. The general strategy for bounded insertion looks like this:
    ///
    /// 1. The string is inserted into the current fragment.
    /// 2. [`Boundary::has_boundary_at`] is called on both the beginning and end
    ///    of the insertion indices, to ensure there's still a boundary between
    ///    the ends of the original fragment and the inserted fragment.
    /// 3. If there's no boundary, the `delim` character is inserted at the
    ///    necessary locations to force a boundary.
    ///
    /// The goal of any bounded insertion operation is *not to merge chunks*.
    ///
    /// # Panics
    ///
    /// This function will panic if the byte index is larger than the element's
    /// length, *or* if it does not fall on a character sequence boundary.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the insertion of `fragment` into `self` cannot produce a valid
    /// result.
    ///
    /// If the intermediate fragment is invalid, then the `InvalidFormat` error
    /// kind will be returned, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the insertion operation itself failed, then the `FailedInsert` error
    /// kind is returned, without setting the `byte_offset`.
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
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_with(
    ///     0,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok()); // Bounded because of `HAT` rules.
    /// assert!(example.insert_bounded_with(
    ///     0,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok()); // But another would not be.
    /// assert_eq!(example, "HAT_HATUpperCamel");
    ///
    /// // Inserting at the end is ~append.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_with(
    ///     example.len(),
    ///     UpperCamelFragment::new("lower")?,
    ///     LowLine,
    /// ).is_err()); // "UpperCamel_lower" != UpperCamel casing
    /// assert!(example.insert_bounded_with(
    ///     example.len(),
    ///     UpperCamelFragment::new("Camel")?,
    ///     LowLine,
    /// ).is_ok()); // Because of `CAMEL` boundary.
    /// assert_eq!(example, "UpperCamelCamel");
    ///
    /// // Inserting in the middle can be tricky, as your insertions
    /// // may invalidate the buffer's invariants in surprising ways.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_with(
    ///     2,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_err()); // "UpHAT_perCamel" != UpperCamel casing
    /// assert!(example.insert_bounded_with(
    ///     5,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok()); // Surprisingly a `CAMEL` & `HAT` boundary.
    /// assert_eq!(example, "UpperHATCamel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_bounded_with<F>(&mut self, idx: usize, fragment: F, delim: D) -> Result<(), Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let fragment = fragment.into_intermediate()?;
        FragmentBufOp::new(self).insert_bounded_str(idx, fragment.as_ref(), delim.as_char())
    }

    /// Attempts to insert a fragment into the buffer, preserving chunk
    /// boundaries by ensuring a delimiter is present on each side (where
    /// needed).
    ///
    /// This is a convenience function for cases when the delimiter value can be
    /// deduced by the `Default` trait. For more information on this operation,
    /// see the [`insert_delimited_with`] method.
    ///
    /// [`insert_delimited_with`]: Self::insert_delimited_with
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited(0, 'V').is_ok());
    /// assert_eq!(example, "V_UpperCamel");
    ///
    /// // Inserting at the end is ~append.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited(example.len(), 'i').is_err()); // "UpperCamel_i" != UpperCamel casing
    /// assert!(example.insert_delimited(example.len(), 'V').is_ok());
    /// assert_eq!(example, "UpperCamel_V");
    ///
    /// // Inserting in the middle can be tricky, as your insertions
    /// // may invalidate the buffer's invariants in surprising ways.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited(2, 'V').is_err()); // "Up_V_perCamel" != UpperCamel casing
    /// assert!(example.insert_delimited(5, 'V').is_ok());
    /// assert_eq!(example, "Upper_V_Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_delimited<F>(&mut self, idx: usize, fragment: F) -> Result<(), Error>
    where
        D: Default,
        F: IntoIntermediate<B, D, P>,
    {
        self.insert_delimited_with(idx, fragment, D::default())
    }

    /// Attempts to insert a fragment into the buffer, preserving chunk
    /// boundaries by ensuring a delimiter is present on each side (where
    /// needed).
    ///
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// If you're working with a fragment format which has only a single valid
    /// delimiter value, you should instead be able to use the
    /// [`insert_delimited`] method, and should prefer that.
    ///
    /// [`insert_delimited`]: Self::insert_delimited
    ///
    /// # Preserving Chunk Boundaries
    ///
    /// If the fragment already contains delimiters on both ends, it will always
    /// be inserted verbatim.
    ///
    /// If the fragment does *NOT* contain delimiters on both ends, then a
    /// delimiter will be inserted a number of times depending on how many sides
    /// have chunk data immediately next to the insertion points. Finally, the
    /// fragment itself will be inserted such that the inserted delimiters will
    /// fall on the left or right depending on where they were intended.
    ///
    /// # Panics
    ///
    /// This function will panic if the byte index is larger than the element's
    /// length, *or* if it does not fall on a character sequence boundary.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the insertion of `fragment` into `self` cannot produce a valid
    /// result.
    ///
    /// If the intermediate fragment is invalid, then the `InvalidFormat` error
    /// kind will be returned, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the insertion operation itself failed, then the `FailedInsert` error
    /// kind is returned, without setting the `byte_offset`.
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
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_with(
    ///     0,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok());
    /// assert_eq!(example, "HAT_UpperCamel");
    ///
    /// // Inserting at the end is ~append.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_with(
    ///     example.len(),
    ///     UpperCamelFragment::new("lower")?,
    ///     LowLine,
    /// ).is_err()); // "UpperCamel_lower" != UpperCamel casing
    /// assert!(example.insert_delimited_with(
    ///     example.len(),
    ///     UpperCamelFragment::new("Camel")?,
    ///     LowLine,
    /// ).is_ok());
    /// assert_eq!(example, "UpperCamel_Camel");
    ///
    /// // Inserting in the middle can be tricky, as your insertions
    /// // may invalidate the buffer's invariants in surprising ways.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_with(
    ///     2,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_err()); // "Up_HAT_perCamel" != UpperCamel casing
    /// assert!(example.insert_delimited_with(
    ///     5,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok());
    /// assert_eq!(example, "Upper_HAT_Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_delimited_with<F>(
        &mut self,
        idx: usize,
        fragment: F,
        delim: D,
    ) -> Result<(), Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        let fragment = fragment.into_intermediate()?;
        FragmentBufOp::new(self).insert_delimited_str(idx, fragment.as_ref(), delim.as_char())
    }

    /// Attempts to convert the current fragment buffer into a boxed [`Ident`].
    ///
    /// This may fail - a fragment isn't obviously a valid identifier, plus the
    /// fragment could be empty (which is never a valid identifier).
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the fragment does not satisfy the identifier
    /// character requirements. If the fragment is empty, this will return an
    /// `Empty` error kind. If an invalid character is found then a
    /// `InvalidFormat` error kind is returned, with [`byte_offset`] set to the
    /// byte index for the first invalid character.
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

    /// Attempts to push a fragment onto the buffer.
    ///
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the pushing of `fragment` onto `self` cannot produce a valid result.
    ///
    /// If the intermediate fragment is invalid, then the `InvalidFormat` error
    /// kind will be returned, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the push operation itself failed, then the `FailedPush` error kind is
    /// returned, without setting the `byte_offset`.
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
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::new();
    ///
    /// // You can only push characters that are valid at the given position.
    /// assert!(buffer.push('O').is_ok());
    /// assert!(buffer.push('k').is_ok());
    /// assert_eq!(buffer, "Ok");
    ///
    /// // But you have to be mindful of the format to avoid pushing
    /// // invalid characters. Most commonly, after delimiters.
    /// buffer.push('_')?;
    /// assert!(buffer.push('i').is_err());
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn push<F>(&mut self, fragment: F) -> Result<(), Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        self.insert(self.len(), fragment).map_err(|e| {
            e.with_error_kind(match e.error_kind() {
                ErrorKind::FailedInsert => ErrorKind::FailedPush,
                other => other,
            })
        })
    }

    /// Attempts to push a fragment onto the buffer, preserving chunk boundaries
    /// by conditionally inserting delimiters if needed.
    ///
    /// This is a convenience function for cases when the delimiter value can be
    /// deduced by the `Default` trait. For more information on this operation,
    /// see the [`push_bounded_with`] method.
    ///
    /// [`push_bounded_with`]: Self::push_bounded_with
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::new();
    ///
    /// // If there's no data at the start, there's nothing to bound against.
    /// assert!(buffer.push_bounded('O').is_ok());
    /// assert_eq!(buffer, "O");
    ///
    /// // But the very next bounded push would need to bound the contents.
    /// // As such, you cannot select an invalid character.
    /// assert!(buffer.push_bounded('k').is_err()); // "O_k" != UpperCamel casing
    /// assert!(buffer.push_bounded('K').is_ok());
    /// assert_eq!(buffer, "O_K");
    ///
    /// // This won't add a delimiter if the chunks are already bounded.
    /// assert!(buffer.push('o').is_ok()); // Regular push to get a lowercase.
    /// assert!(buffer.push_bounded('K').is_ok()); // A `CAMEL` boundary char.
    /// assert_eq!(buffer, "O_KoK");
    ///
    /// // If there's already a delimiter, another would not be inserted.
    /// assert!(buffer.push(LowLine).is_ok());
    /// assert!(buffer.push_bounded('K').is_ok());
    /// assert_eq!(buffer, "O_KoK_K");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn push_bounded<F>(&mut self, fragment: F) -> Result<(), Error>
    where
        D: Default,
        F: IntoIntermediate<B, D, P>,
    {
        self.insert_bounded(self.len(), fragment).map_err(|e| {
            e.with_error_kind(match e.error_kind() {
                ErrorKind::FailedInsert => ErrorKind::FailedPush,
                other => other,
            })
        })
    }

    /// Attempts to push a fragment into the buffer, preserving chunk boundaries
    /// by conditionally inserting delimiters if needed.
    ///
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// If you're working with a fragment format which has only a single valid
    /// delimiter value, you should instead be able to use the
    /// [`push_bounded`] method, and should prefer that.
    ///
    /// [`push_bounded`]: Self::push_bounded
    ///
    /// # Preserving Chunk Boundaries
    ///
    /// If the fragment already contains delimiters on the left, it will always
    /// be inserted verbatim.
    ///
    /// If the fragment does *NOT* contain delimiters on the left, the fragment
    /// will first be pushed, and then it will be tested to ensure that the left
    /// side of the fragment didn't merge into the prior chunk. If it did, a
    /// delimiter will be inserted to force separation.
    ///
    /// Whether or not a delimiter is needed is found using the [`Boundary`]
    /// trait. The general strategy for bounded push looks like this:
    ///
    /// 1. The string is pushed onto the current fragment.
    /// 2. [`Boundary::has_boundary_at`] is called with the original fragment
    ///    length, to ensure there's still a boundary between the end of the
    ///    original fragment and the pushed fragment.
    /// 3. If there's no boundary, the `delim` character is inserted at the
    ///    necessary location to force a boundary.
    ///
    /// The goal of any bounded push operation is *not to merge chunks*.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the pushing of `fragment` onto `self` cannot produce a valid result.
    ///
    /// If the intermediate fragment is invalid, then the `InvalidFormat` error
    /// kind will be returned, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the push operation itself failed, then the `FailedPush` error kind is
    /// returned, without setting the `byte_offset`.
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
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::new();
    ///
    /// // If there's no data at the start, there's nothing to bound against.
    /// assert!(buffer.push_bounded_with('O', LowLine).is_ok());
    /// assert_eq!(buffer, "O");
    ///
    /// // But the very next bounded push would need to bound the contents.
    /// // As such, you cannot select an invalid character.
    /// assert!(buffer.push_bounded_with('k', LowLine).is_err()); // "O_k" != UpperCamel casing
    /// assert!(buffer.push_bounded_with('K', LowLine).is_ok());
    /// assert_eq!(buffer, "O_K");
    ///
    /// // This won't add a delimiter if the chunks are already bounded.
    /// assert!(buffer.push('o').is_ok()); // Regular push to get a lowercase.
    /// assert!(buffer.push_bounded_with('K', LowLine).is_ok()); // A `CAMEL` boundary char.
    /// assert_eq!(buffer, "O_KoK");
    ///
    /// // If there's already a delimiter, another would not be inserted.
    /// assert!(buffer.push(LowLine).is_ok());
    /// assert!(buffer.push_bounded_with('K', LowLine).is_ok());
    /// assert_eq!(buffer, "O_KoK_K");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn push_bounded_with<F>(&mut self, fragment: F, delim: D) -> Result<(), Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        self.insert_bounded_with(self.len(), fragment, delim)
            .map_err(|e| {
                e.with_error_kind(match e.error_kind() {
                    ErrorKind::FailedInsert => ErrorKind::FailedPush,
                    other => other,
                })
            })
    }

    /// Attempts to push a fragment into the buffer, preserving chunk boundaries
    /// by ensuring a delimiter is present on the left side (if needed).
    ///
    /// This is a convenience function for cases when the delimiter value can be
    /// deduced by the `Default` trait. For more information on this operation,
    /// see the [`push_delimited_with`] method.
    ///
    /// [`push_delimited_with`]: Self::push_delimited_with
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::new();
    ///
    /// // If there's no data at the start, there's nothing to delimit against.
    /// assert!(buffer.push_delimited('O').is_ok());
    /// assert_eq!(buffer, "O");
    ///
    /// // But the very next delimited push would need to delimit the contents.
    /// // As such, you cannot select an invalid character.
    /// assert!(buffer.push_delimited('k').is_err()); // "O_k" != UpperCamel casing
    /// assert!(buffer.push_delimited('K').is_ok());
    /// assert_eq!(buffer, "O_K");
    ///
    /// // If there's already a delimiter, another would not be inserted.
    /// assert!(buffer.push(LowLine).is_ok());
    /// assert!(buffer.push_delimited('K').is_ok());
    /// assert_eq!(buffer, "O_K_K");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn push_delimited<F>(&mut self, fragment: F) -> Result<(), Error>
    where
        D: Default,
        F: IntoIntermediate<B, D, P>,
    {
        self.insert_delimited(self.len(), fragment).map_err(|e| {
            e.with_error_kind(match e.error_kind() {
                ErrorKind::FailedInsert => ErrorKind::FailedPush,
                other => other,
            })
        })
    }

    /// Attempts to push a fragment into the buffer, preserving chunk boundaries
    /// by ensuring a delimiter is present on the left side (if needed).
    ///
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// If you're working with a fragment format which has only a single valid
    /// delimiter value, you should instead be able to use the
    /// [`push_delimited`] method, and should prefer that.
    ///
    /// [`push_delimited`]: Self::push_delimited
    ///
    /// # Preserving Chunk Boundaries
    ///
    /// If the fragment already contains delimiters on the left, it will be
    /// inserted verbatim.
    ///
    /// If the fragment does *NOT* contain delimiters on the left, and the prior
    /// fragment did not contain a delimiter on the right, then a delimiter will
    /// be pushed first. Finally, the fragment itself will be pushed.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the intermediate fragment provided is invalid, or
    /// if the pushing of `fragment` onto `self` cannot produce a valid result.
    ///
    /// If the intermediate fragment is invalid, then the `InvalidFormat` error
    /// kind will be returned, with the [`byte_offset`] set to the first invalid
    /// character of the intermediate fragment.
    ///
    /// If the push operation itself failed, then the `FailedPush` error kind is
    /// returned, without setting the `byte_offset`.
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
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::new();
    ///
    /// // If there's no data at the start, there's nothing to delimit against.
    /// assert!(buffer.push_delimited_with('O', LowLine).is_ok());
    /// assert_eq!(buffer, "O");
    ///
    /// // But the very next delimited push would need to delimit the contents.
    /// // As such, you cannot select an invalid character.
    /// assert!(buffer.push_delimited_with('k', LowLine).is_err()); // "O_k" != UpperCamel casing
    /// assert!(buffer.push_delimited_with('K', LowLine).is_ok());
    /// assert_eq!(buffer, "O_K");
    ///
    /// // If there's already a delimiter, another would not be inserted.
    /// assert!(buffer.push(LowLine).is_ok());
    /// assert!(buffer.push_delimited_with('K', LowLine).is_ok());
    /// assert_eq!(buffer, "O_K_K");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn push_delimited_with<F>(&mut self, fragment: F, delim: D) -> Result<(), Error>
    where
        F: IntoIntermediate<B, D, P>,
    {
        self.insert_delimited_with(self.len(), fragment, delim)
            .map_err(|e| {
                e.with_error_kind(match e.error_kind() {
                    ErrorKind::FailedInsert => ErrorKind::FailedPush,
                    other => other,
                })
            })
    }

    /// Removes a character from the buffer at a given index.
    ///
    /// # Panics
    ///
    /// This function will panic if the byte index is larger than the element's
    /// length, *or* if it does not fall on a character sequence boundary.
    ///
    /// # Errors
    ///
    /// If the removal of the character at the provided index would lead to an
    /// invalid buffer, then the character will not be remove and instead the
    /// error `FailedRemove` will be returned.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("Upper_Camel")?;
    ///
    /// // This would be valid, because it might be a continuation fragment.
    /// assert!(buffer.remove(0).is_ok());
    /// assert_eq!(buffer, "pper_Camel");
    ///
    /// // However, attempting to remove `C` would fail for `UpperCamel`.
    /// assert!(buffer.remove(5).is_err());
    /// assert_eq!(buffer, "pper_Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn remove(&mut self, idx: usize) -> Result<(), Error> {
        FragmentBufOp::new(self).remove(idx)
    }

    /// Replace a range of characters with a provided replacement fragment.
    ///
    /// This function takes anything that can be represented as an intermediate
    /// fragment. That means it can take a `&str`, `char`, `Fragment`, `Chunk`,
    /// `Identifier`, or `Segment`.
    ///
    /// # Panics
    ///
    /// This function will panic if either end of the range specified lies
    /// outside of the length of the fragment, *or* if either end does not lie
    /// on a character sequence boundary.
    ///
    /// # Errors
    ///
    /// If the replacement of the range provided with the given fragment would
    /// lead to an invalid buffer, then the range will not be remove and instead
    /// the error `FailedReplace` will be returned.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("Upper_Camel")?;
    ///
    /// // Examples replacing various ranges.
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range(4..7, "R").is_ok());
    /// assert_eq!(example, "UppeRamel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range(4..=7, "R").is_ok());
    /// assert_eq!(example, "UppeRmel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range(..7, "R").is_ok());
    /// assert_eq!(example, "Ramel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range(..=7, "R").is_ok());
    /// assert_eq!(example, "Rmel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range(4.., "R").is_ok());
    /// assert_eq!(example, "UppeR");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn replace_range<R, F>(&mut self, range: R, replace_with: F) -> Result<(), Error>
    where
        R: RangeBounds<usize>,
        F: IntoIntermediate<B, D, P>,
    {
        let replace_with = replace_with.into_intermediate()?;
        FragmentBufOp::new(self).replace_range_str(range, replace_with.as_ref())
    }

    /// Splits the buffer into two halves at a given byte index.
    ///
    /// Returns a newly allocated buffer. `self` contains bytes `[0, at)`, and
    /// the returned buffer contains bytes `[at, len)`. `at` must be on the
    /// boundary of a UTF-8 code point.
    ///
    /// Note that the capacity of `self` does not change.
    ///
    /// # Panics
    ///
    /// This function will panic if the byte index is larger than the element's
    /// length, *or* if it does not fall on a character sequence boundary.
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
    /// Returns a reference to the fragment slice from the buffer.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::from_str("example")?;
    /// let fragment: &HybridFragment = buffer.as_fragment();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_fragment(&self) -> &Fragment<B, D, P> {
        Fragment::new_unchecked(self.inner.as_str())
    }

    /// Returns a string slice representation of the buffer data.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::from_str("example")?;
    /// let fragment: &str = buffer.as_str();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Returns the underlying capacity of the buffer.
    ///
    /// This has the same properties as [`String::capacity`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::new();
    /// assert_eq!(buffer.capacity(), 0);
    /// buffer.reserve(10);
    /// assert!(buffer.capacity() >= 10);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the underlying fragment buffer, making it empty.
    ///
    /// This has the same properties as [`String::clear`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::from_str("example")?;
    /// assert_eq!(buffer, "example");
    /// buffer.clear();
    /// assert_eq!(buffer, "");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

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

    /// Convert the buffer into a boxed fragment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::from_str("example")?;
    /// let boxed: Box<HybridFragment> = buffer.into_boxed_fragment();
    /// assert_eq!(boxed.as_ref(), "example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_fragment(self) -> Box<Fragment<B, D, P>> {
        Fragment::new_boxed_unchecked(self.into_string())
    }

    /// Convert the buffer into a boxed string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let buffer = HybridFragmentBuf::from_str("example")?;
    /// let boxed: Box<str> = buffer.into_boxed_str();
    /// assert_eq!(boxed.as_ref(), "example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_str(self) -> Box<str> {
        self.inner.into_boxed_str()
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

    /// Returns `true` if the underlying buffer is empty.
    ///
    /// This has the same properties as [`String::is_empty`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::from_str("example")?;
    /// assert!(!buffer.is_empty());
    /// buffer.clear();
    /// assert!(buffer.is_empty());
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
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

    /// Returns the byte length of the buffer
    ///
    /// This has the same properties as [`String::len`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::from_str("example")?;
    /// assert_eq!(buffer.len(), 7);
    /// buffer.clear();
    /// assert_eq!(buffer.len(), 0);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    /// Constructs a new buffer with the capacity set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::new();
    /// assert_eq!(buffer, "");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Pops the right-most character off the buffer and returns it.
    ///
    /// This has the same properties as [`String::pop`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::from_str("test")?;
    /// assert_eq!(buffer.pop(), Some('t'));
    /// assert_eq!(buffer.pop(), Some('s'));
    /// assert_eq!(buffer.pop(), Some('e'));
    /// assert_eq!(buffer.pop(), Some('t'));
    /// assert_eq!(buffer.pop(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        self.inner.pop()
    }

    /// Reserves enough buffer space for `additional` more bytes.
    ///
    /// This has the same properties as [`String::reserve`].
    ///
    /// # Note
    ///
    /// This will over-allocate in most situations. If you need to
    /// reserve an *exact* amount of bytes, see [`reserve_exact`].
    ///
    /// [`reserve_exact`]: Self::reserve_exact
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::new();
    /// assert_eq!(buffer.capacity(), 0);
    /// buffer.reserve(10);
    /// assert!(buffer.capacity() >= 10);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Reserves exact buffer space for `additional` more bytes.
    ///
    /// This has the same properties as [`String::reserve_exact`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::new();
    /// assert_eq!(buffer.capacity(), 0);
    /// buffer.reserve_exact(10);
    /// assert_eq!(buffer.capacity(), 10);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }

    /// Shrinks the buffer to the minimum of the actual length or the
    /// provided `min_capacity` value.
    ///
    /// This has the same properties as [`String::shrink_to`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::with_capacity(10);
    /// assert!(buffer.capacity() >= 10);
    /// buffer.shrink_to(5);
    /// assert_eq!(buffer.capacity(), 5);
    /// buffer.push("example")?;
    /// assert!(buffer.capacity() >= 7);
    /// buffer.shrink_to(0);
    /// assert_eq!(buffer.capacity(), 7);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity)
    }

    /// Shrinks the buffer to the size of the content.
    ///
    /// This has the same properties as [`String::shrink_to_fit`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let mut buffer = HybridFragmentBuf::with_capacity(10);
    /// assert!(buffer.capacity() >= 10);
    /// buffer.shrink_to_fit();
    /// assert_eq!(buffer.capacity(), 0);
    /// buffer.push("example")?;
    /// assert!(buffer.capacity() >= 7);
    /// buffer.shrink_to_fit();
    /// assert_eq!(buffer.capacity(), 7);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
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

    /// Attempts to reserve buffer space for `additional` more bytes.
    ///
    /// This has the same properties as [`String::try_reserve`].
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    /// Attempts to reserve exact buffer space for `additional` more bytes.
    ///
    /// This has the same properties as [`String::try_reserve_exact`].
    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
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
impl<B, D, P> Default for FragmentBuf<B, D, P> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> From<FragmentBuf<B, D, P>> for String {
    #[inline]
    fn from(orig: FragmentBuf<B, D, P>) -> Self {
        orig.into_string()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> From<FragmentBuf<B, D, P>> for std_alloc::boxed::Box<Fragment<B, D, P>> {
    #[inline]
    fn from(orig: FragmentBuf<B, D, P>) -> Self {
        orig.into_boxed_fragment()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> From<FragmentBuf<B, D, P>> for std_alloc::boxed::Box<str> {
    #[inline]
    fn from(orig: FragmentBuf<B, D, P>) -> Self {
        orig.into_boxed_str()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> core::ops::Deref for FragmentBuf<B, D, P> {
    type Target = Fragment<B, D, P>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        Fragment::new_unchecked(self.inner.as_str())
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> core::cmp::PartialEq<&str> for FragmentBuf<B, D, P> {
    #[inline]
    fn eq(&self, rhs: &&str) -> bool {
        self.as_str() == *rhs
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> core::cmp::PartialEq<FragmentBuf<B, D, P>> for &str {
    #[inline]
    fn eq(&self, rhs: &FragmentBuf<B, D, P>) -> bool {
        *self == rhs.as_str()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> core::cmp::PartialOrd<&str> for FragmentBuf<B, D, P> {
    #[inline]
    fn partial_cmp(&self, rhs: &&str) -> Option<core::cmp::Ordering> {
        self.as_str().partial_cmp(*rhs)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> core::cmp::PartialOrd<FragmentBuf<B, D, P>> for &str {
    #[inline]
    fn partial_cmp(&self, rhs: &FragmentBuf<B, D, P>) -> Option<core::cmp::Ordering> {
        (*self).partial_cmp(rhs.as_str())
    }
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
