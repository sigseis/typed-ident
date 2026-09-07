// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::FragmentBuf;
use crate::core::error::{Error, ErrorKind};
use crate::core::{Chunk, Fragment, Ident};
use crate::syntax::{Boundary, Delimiter, Profile, UnitDelimiter};
use core::ops::{Bound, RangeBounds};
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
    inner: FragmentBuf<B, D, P>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> IdentBuf<B, D, P> {
    #[must_use]
    #[inline]
    pub(crate) fn can_push_fragment_char(left: Option<&Ident<B, D, P>>, right: char) -> bool {
        match left {
            None => D::is_ident_start(right) || P::is_ident_start(right),
            Some(left) => FragmentBuf::can_push_fragment_char(left, right),
        }
    }

    #[must_use]
    #[inline]
    pub(crate) fn can_join(left: Option<&Ident<B, D, P>>, right: &Fragment<B, D, P>) -> bool {
        right
            .chars()
            .next()
            .is_none_or(|c| Self::can_push_fragment_char(left, c))
    }

    /// Constructs an ident buffer, initializing the contents to a provided
    /// string slice (attempting first to convert the string slice to a valid
    /// ident).
    ///
    /// This is equivalent to `IdentBuf::from_fragment(Fragment::new(s)?)`.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::UpperCamelIdentBuf;
    /// assert!(UpperCamelIdentBuf::from_str("").is_err());
    /// assert!(UpperCamelIdentBuf::from_str("ValidUpperCamel").is_ok());
    /// assert!(UpperCamelIdentBuf::from_str("continuingUpperCamel").is_err());
    /// assert!(UpperCamelIdentBuf::from_str("not_validUpperCamel").is_err());
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    #[allow(clippy::should_implement_trait)] // It *does* implement the trait.
    pub fn from_str(s: &str) -> Result<Self, Error> {
        core::str::FromStr::from_str(s)
    }

    /// Constructs an ident buffer, initializing the contents to a provided
    /// buffered string (checking first that the string is a valid ident).
    ///
    /// This is similar to [`from_str`], except that it will not allocate a
    /// separate string. It will use the provided string, if it's valid.
    ///
    /// [`from_str`]: Self::from_str
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::UpperCamelIdentBuf;
    /// assert!(UpperCamelIdentBuf::from_string(String::from("")).is_err());
    /// assert!(UpperCamelIdentBuf::from_string(String::from("ValidUpperCamel")).is_ok());
    /// assert!(UpperCamelIdentBuf::from_string(String::from("continuingUpperCamel")).is_err());
    /// assert!(UpperCamelIdentBuf::from_string(String::from("not_validUpperCamel")).is_err());
    /// # Ok::<(), Error>(())
    #[inline]
    pub fn from_string(s: String) -> Result<Self, Error> {
        let _ = Ident::<B, D, P>::new(&s)?;
        Ok(Self::from_string_unchecked(s))
    }

    #[doc = include_str!("docs/methods/insert_fragment.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    #[doc = include_str!("docs/sections/errors.md")]
    ///
    /// If the insertion targeted the beginning of the identifier, but the first
    /// character was not a valid identifier start character, then the error
    /// kind will be `InvalidFormat`.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelIdentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend, and thus must be `UpperCamel`.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_fragment(0, UpperCamelFragment::new("Valid")?).is_ok());
    ///
    /// // Inserting at the end is ~append, so it must not accidentally produce a `lowerCamel`.
    /// let mut example = buffer.clone();
    /// example.push('_')?;
    /// assert!(example.insert_fragment(example.len(), UpperCamelFragment::new("invalid")?).is_err());
    /// assert!(example.insert_fragment(example.len(), UpperCamelFragment::new("Valid")?).is_ok());
    ///
    /// // But note, that if it would simply append an ongoing chunk, that's fine.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_fragment(example.len(), UpperCamelFragment::new("valid")?).is_ok());
    ///
    /// // Inserting in the middle is tricky, you must ensure it forms a valid `UpperCamel` ident.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_fragment(2, UpperCamelFragment::new("_")?).is_err());
    /// assert!(example.insert_fragment(5, UpperCamelFragment::new("_")?).is_ok());
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_fragment(
        &mut self,
        idx: usize,
        fragment: &Fragment<B, D, P>,
    ) -> Result<(), Error> {
        let Some(first) = fragment.chars().next() else {
            return Ok(());
        };
        if idx == 0 && !D::is_ident_start(first) && !P::is_ident_start(first) {
            return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(0));
        }
        self.inner.insert_fragment(idx, fragment)
    }

    #[doc = include_str!("docs/methods/insert_bounded_fragment.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    #[doc = include_str!("docs/sections/errors.md")]
    ///
    /// If the insertion targeted the beginning of the identifier, but the first
    /// character was not a valid identifier start character, then the error
    /// kind will be `InvalidFormat`.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelIdentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting a character that needs no separation introduces no delims.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_fragment(5, UpperCamelFragment::new("Formatted")?).is_ok());
    /// assert_eq!(example, "UpperFormattedCamel");
    ///
    /// // Inserting a delimiter works as long as you don't break the formatting.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_fragment(5, UpperCamelFragment::new("_")?).is_ok());
    /// assert!(example.insert_bounded_fragment(2, UpperCamelFragment::new("_")?).is_err());
    /// assert_eq!(example, "Upper_Camel");
    ///
    /// // It's most common to push characters onto the end though.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_fragment(10, UpperCamelFragment::new("1")?).is_ok());
    /// assert_eq!(example, "UpperCamel_1");
    /// assert!(example.insert_bounded_fragment(12, UpperCamelFragment::new("FRAGMENT")?).is_ok());
    /// assert_eq!(example, "UpperCamel_1_FRAGMENT");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// There's an interesting case where fixing one side makes the other side
    /// combine with the chunk to its left (only impacts certain syntaxes).
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::camel::*;
    /// let mut buffer = CamelIdentBuf::from_str("UpperCamel")?;
    /// assert!(buffer.insert_bounded_str(1, "U").is_ok());
    /// assert_eq!(buffer, "U_U_pperCamel"); // Instead of "UU_pperCamel"
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_bounded_fragment_with(
        &mut self,
        idx: usize,
        fragment: &Fragment<B, D, P>,
        delim: D,
    ) -> Result<(), Error> {
        let Some(first) = fragment.chars().next() else {
            return Ok(());
        };
        if idx == 0 && !D::is_ident_start(first) && !P::is_ident_start(first) {
            return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(0));
        }
        self.inner
            .insert_bounded_fragment_with(idx, fragment, delim)
    }

    #[doc = include_str!("docs/methods/insert_delimited_fragment.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    #[doc = include_str!("docs/sections/errors.md")]
    ///
    /// If the insertion targeted the beginning of the identifier, but the first
    /// character was not a valid identifier start character, then the error
    /// kind will be `InvalidFormat`.
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
    ///
    /// // Inserting a character that needs no separation introduces no delims.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_fragment_with(5, UpperCamelFragment::new("Formatted")?, LowLine).is_ok());
    /// assert_eq!(example, "Upper_Formatted_Camel");
    ///
    /// // Inserting a delimiter works as long as you don't break the formatting.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_fragment_with(5, UpperCamelFragment::new("_")?, LowLine).is_ok());
    /// assert!(example.insert_delimited_fragment_with(2, UpperCamelFragment::new("_")?, LowLine).is_err());
    /// assert_eq!(example, "Upper_Camel");
    ///
    /// // It's most common to push characters onto the end though.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_fragment_with(10, UpperCamelFragment::new("1")?, LowLine).is_ok());
    /// assert_eq!(example, "UpperCamel_1");
    /// assert!(example.insert_delimited_fragment_with(12, UpperCamelFragment::new("Fragment")?, LowLine).is_ok());
    /// assert_eq!(example, "UpperCamel_1_Fragment");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_delimited_fragment_with(
        &mut self,
        idx: usize,
        fragment: &Fragment<B, D, P>,
        delim: D,
    ) -> Result<(), Error> {
        let Some(first) = fragment.chars().next() else {
            return Ok(());
        };
        if idx == 0 && !D::is_ident_start(first) && !P::is_ident_start(first) {
            return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(0));
        }
        self.inner
            .insert_delimited_fragment_with(idx, fragment, delim)
    }

    #[doc = include_str!("docs/methods/push_delim.md")]
    #[doc = include_str!("docs/methods/push_delim.errors.md")]
    ///
    /// If the insertion targeted the beginning of the identifier, but the first
    /// character was not a valid identifier start character, then the error
    /// kind will be `InvalidFormat`.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use syntax::delimiter::LowLine;
    /// # use presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelIdentBuf::new();
    ///
    /// // For all preset and provided delimiters, you can push them anywhere in
    /// // an identifier. Unless you have a custom delimiter, it's always safe to push.
    /// assert!(buffer.push_delim_with(LowLine).is_ok());
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// Example Failure:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::*;
    /// # use syntax::delimiter::LowLine;
    /// # use presets::unicode::upper_camel::*;
    /// #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// struct DollarStart;
    ///
    /// impl Delimiter for DollarStart {
    ///     fn as_char(&self) -> char {
    ///         '$'
    ///     }
    ///     fn from_ident_start(c: char) -> Option<Self> {
    ///         match c {
    ///             '$' => Some(Self),
    ///             _ => None,
    ///         }
    ///     }
    ///     fn from_chunk_delim(c: char) -> Option<Self> {
    ///         None
    ///     }
    /// }
    ///
    /// type DollarStartIdentBuf = IdentBuf<
    ///     boundary::Standard,
    ///     DollarStart,
    ///     profile::Unicode,
    /// >;
    ///
    /// let mut buffer = DollarStartIdentBuf::new();
    ///
    /// // Okay to push one `$` in, because it may be the start fragment.
    /// assert!(buffer.push_delim_with(DollarStart).is_ok());
    ///
    /// // But you definitely cannot push another in - that's invalid.
    /// assert!(buffer.push_delim_with(DollarStart).is_err());
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn push_delim_with(&mut self, delim: D) -> Result<(), Error> {
        if !D::APPEND_CLOSED.at_least_identifier()
            && self.inner.is_empty()
            && !D::is_ident_start(delim.as_char())
        {
            return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(0));
        }
        self.inner.push_delim_with(delim)
    }

    #[doc = include_str!("docs/methods/remove.md")]
    #[doc = include_str!("docs/sections/panics.md")]
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
        let (left, right) = self
            .as_ident()
            .expect(
                "attempted to remove a character from an identifier buffer that was out of bounds",
            )
            .split_at(idx);
        let mut chars = right.chars();
        let _ = chars.next();
        let right = chars.as_fragment();
        if !Self::can_join(left, right) {
            return Err(Error::new(ErrorKind::FailedRemove).with_byte_offset(idx));
        }
        self.remove_unchecked(idx);
        Ok(())
    }

    #[doc = include_str!("docs/methods/replace_range.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    ///
    /// # Errors
    ///
    /// If the replacement of the range provided with the given fragment would
    /// lead to an invalid buffer, then the range will not be remove and instead
    /// the error `InvalidReplace` will be returned.
    ///
    /// If the insertion targeted the beginning of the identifier, but the first
    /// character was not a valid identifier start character, then the error
    /// kind will be `InvalidFormat`.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::*;
    /// let buffer = UpperCamelIdentBuf::from_str("Upper_Camel")?;
    ///
    /// // Examples replacing various ranges.
    /// let replacement = UpperCamelFragment::new("R")?;
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(4..7, replacement).is_ok());
    /// assert_eq!(example, "UppeRamel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(4..=7, replacement).is_ok());
    /// assert_eq!(example, "UppeRmel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(..7, replacement).is_ok());
    /// assert_eq!(example, "Ramel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(..=7, replacement).is_ok());
    /// assert_eq!(example, "Rmel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(4.., replacement).is_ok());
    /// assert_eq!(example, "UppeR");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn replace_range_fragment<R>(
        &mut self,
        range: R,
        replace_with: &Fragment<B, D, P>,
    ) -> Result<(), Error>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(idx) => *idx,
            Bound::Excluded(idx) => *idx + 1,
            Bound::Unbounded => 0,
        };
        if start == 0 {
            let end = match range.end_bound() {
                Bound::Included(idx) => *idx + 1,
                Bound::Excluded(idx) => *idx,
                Bound::Unbounded => self.len(),
            };
            let valid_first_char = replace_with
                .chars()
                .next()
                .or_else(|| self[end..].chars().next())
                .is_none_or(|c| D::is_ident_start(c) || P::is_ident_start(c));
            if !valid_first_char {
                return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(0));
            }
        }
        self.inner.replace_range_fragment(range, replace_with)
    }

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
    pub fn split_off(&mut self, idx: usize) -> FragmentBuf<B, D, P> {
        self.inner.split_off(idx)
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

    #[inline]
    pub(crate) fn remove_unchecked(&mut self, idx: usize) {
        self.inner.remove_unchecked(idx)
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
}

// =============================================================================
// TRAIT IMPL
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> core::str::FromStr for IdentBuf<B, D, P> {
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
