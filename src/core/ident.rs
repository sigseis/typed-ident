// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "ident.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::error::{Error, ErrorKind};
use crate::core::{Chunk, Fragment, Identifier, Segment};
use crate::syntax::{Boundary, Delimiter, Profile, UnitDelimiter};

// =============================================================================
// TYPES
// =============================================================================

/// An immutable, UTF-8 encoded, valid identifier string slice.
///
/// # Type Parameters
///
/// The type parameters used on this type are:
///
/// * `B`: [`Boundary`] (a boundary definition; usually [`Standard`])
/// * `D`: [`Delimiter`] (a delimiter type; [`HyphenMinus`], [`LowLine`], etc.)
/// * `P`: [`Profile`] (a character profile; [`Ascii`], [`Unicode`], etc.)
///
/// # Character Requirements
///
/// This is effectively a special-case of a [`Fragment`]. So in addition to the
/// character requirements of that type, this type adds the following additional
/// requirements:
///
/// * Non-empty, *and...*
/// * Starts with either [`D::is_ident_start`] or [`P::is_ident_start`].
///
/// [`D::is_ident_start`]: crate::syntax::delimiter::Delimiter::is_ident_start
/// [`P::is_ident_start`]: crate::syntax::profile::Profile::is_ident_start
///
/// [`Ascii`]: crate::syntax::profile::Ascii
/// [`Boundary`]: crate::syntax::boundary::Boundary
/// [`Standard`]: crate::syntax::boundary::Standard
/// [`Delimiter`]: crate::syntax::delimiter::Delimiter
/// [`HyphenMinus`]: crate::syntax::delimiter::HyphenMinus
/// [`LowLine`]: crate::syntax::delimiter::LowLine
/// [`Profile`]: crate::syntax::profile::Profile
/// [`Unicode`]: crate::syntax::profile::Unicode
#[repr(transparent)]
pub struct Ident<B, D, P> {
    inner: Fragment<B, D, P>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: 'static, D: UnitDelimiter + 'static, P: 'static> Ident<B, D, P> {
    /// An anonymous identifier filled with a single unit delimiter.
    pub const ANONYMOUS: &'static Ident<B, D, P> = Ident::new_unchecked(D::STR);
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> Ident<B, D, P> {
    /// Returns the first segment of an identifier.
    ///
    /// Since an identifier is always non-empty, there's always at least one
    /// available segment. This function will simply return the first segment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("UpperCamelIdent")?;
    /// assert_eq!(
    ///     ident.first_segment(),
    ///     UpperCamelSegment::Chunk(UpperCamelChunk::new("Upper")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn first_segment(&self) -> Segment<D, &Chunk<B, D, P>> {
        // An ident is always non-empty, so there must be at least one segment.
        self.segments().next().unwrap()
    }

    /// Converts a fragment to an identifier.
    ///
    /// An identifier is made of a fragment, this function converts between the
    /// two. Not all fragments are valid identifiers, however. An identifier has
    /// additional [requirements].
    ///
    /// `new` checks to ensure these are satisfied before the conversion.
    ///
    /// [requirements]: Self#character-requirements
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment does not satisfy the character
    /// requirements. If an invalid character is found then an [`Error`] is
    /// returned, with [`byte_offset`] set to the byte index for the first
    /// invalid character (for this function, this is always `0`).
    ///
    /// [`Error`]: crate::Error
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("AnUpperCamel_Fragment")?;
    /// let ident = UpperCamelIdent::from_fragment(fragment)?;
    /// assert_eq!(ident, "AnUpperCamel_Fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn from_fragment(fragment: &Fragment<B, D, P>) -> Result<&Self, Error> {
        let Some(first) = fragment.chars().next() else {
            return Err(Error::new(ErrorKind::EmptyIdent));
        };
        if !D::is_ident_start(first) && !P::is_ident_start(first) {
            return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(0));
        }
        Ok(Self::new_unchecked(fragment.as_str()))
    }

    /// Returns the last segment of an identifier.
    ///
    /// Since an identifier is always non-empty, there's always at least one
    /// available segment. This function will simply return the last segment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("UpperCamelIdent")?;
    /// assert_eq!(
    ///     ident.last_segment(),
    ///     UpperCamelSegment::Chunk(UpperCamelChunk::new("Ident")?),
    /// );
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn last_segment(&self) -> Segment<D, &Chunk<B, D, P>> {
        // An ident is always non-empty, so there must be at least one segment.
        self.segments().next_back().unwrap()
    }

    /// Converts a string slice to an identifier.
    ///
    /// An identifier is made of a string slice ([`&str`]), this function
    /// converts between the two. Not all string slices are valid fragments,
    /// however. A fragment requires that the characters it is comprised of
    /// satisfy certain [requirements].
    ///
    /// `new` checks to ensure these are satisfied before the conversion.
    ///
    /// [requirements]: Self#character-requirements
    ///
    /// # Errors
    ///
    /// Returns `Err` if the string slice does not satisfy the character
    /// requirements. If an invalid character is found then an [`Error`] is
    /// returned, with [`byte_offset`] set to the byte index for the first
    /// invalid character.
    ///
    /// [`Error`]: crate::Error
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("AnUpperCamel_Identifier")?;
    /// assert_eq!(ident, "AnUpperCamel_Identifier");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn new(s: &str) -> Result<&Self, Error> {
        Self::from_fragment(Fragment::new(s)?)
    }

    /// Trims any decorative delimiters from the identifier.
    ///
    /// This function cannot leave you with an invalid identifier, it
    /// explicitly only trims delimiters that it considers to be non-essential,
    /// or decorative.
    ///
    /// If there's multiple kinds of delimiters, the right-most delimiter will
    /// be preserved. This is essentially the same as calling
    /// [`trim_leading_decorative_delims`] followed by
    /// [`trim_trailing_decorative_delims`].
    ///
    /// [`trim_leading_decorative_delims`]: Self::trim_leading_decorative_delims
    /// [`trim_trailing_decorative_delims`]: Self::trim_trailing_decorative_delims
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("__DecoratedIdent__")?;
    /// assert_eq!(ident.trim_decorative_delims(), "DecoratedIdent");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Delimiters will be preserved if the following character is not valid at
    /// ident-start:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("__2DecoratedIdent__")?;
    /// assert_eq!(ident.trim_decorative_delims(), "_2DecoratedIdent");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// A delimiter-only identifier becomes a single-character identifier:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("____")?;
    /// assert_eq!(ident.trim_decorative_delims(), "_");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// If there's multiple allowed delimiters, the right-most is preserved:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// assert_eq!(HybridIdent::new("--__")?.trim_decorative_delims(), "_");
    /// assert_eq!(HybridIdent::new("__--")?.trim_decorative_delims(), "-");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this returns the remaining sub-identifier as a new identifier, without modifying the original"]
    #[inline]
    pub fn trim_decorative_delims(&self) -> &Self {
        self.trim_leading_decorative_delims()
            .trim_trailing_decorative_delims()
    }

    /// Trims any leading decorative delimiters from the identifier.
    ///
    /// This function can not leave you with an invalid identifier, it
    /// explicitly only trims delimiters that it considers to be non-essential,
    /// or decorative.
    ///
    /// If there's multiple kinds of delimiters, the right-most delimiter will
    /// be preserved (e.g. the trimming happens from left-to-right).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("__DecoratedIdent__")?;
    /// assert_eq!(ident.trim_leading_decorative_delims(), "DecoratedIdent__");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Delimiters will be preserved if the following character is not valid at
    /// ident-start:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("__2DecoratedIdent__")?;
    /// assert_eq!(ident.trim_leading_decorative_delims(), "_2DecoratedIdent__");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// A delimiter-only identifier becomes a single-character identifier:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("____")?;
    /// assert_eq!(ident.trim_leading_decorative_delims(), "_");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// If there's multiple allowed delimiters, the right-most is preserved:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// assert_eq!(HybridIdent::new("--__")?.trim_leading_decorative_delims(), "_");
    /// assert_eq!(HybridIdent::new("__--")?.trim_leading_decorative_delims(), "-");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this returns the remaining sub-identifier as a new identifier, without modifying the original"]
    #[inline]
    pub fn trim_leading_decorative_delims(&self) -> &Self {
        let mut chars = self.chars();
        let mut trimmed = self.as_str();
        while let Some(c) = chars.next() {
            // If it's not a start delim, stop - we've trimmed them all.
            if !D::is_ident_start(c) {
                break;
            }

            // If there's no next char, don't trim - don't leave it empty.
            let mut peek = chars.clone();
            let Some(next) = peek.next() else {
                break;
            };

            // If the next character is not a valid start character, don't trim.
            if !D::is_ident_start(next) && !P::is_ident_start(next) {
                break;
            }

            trimmed = chars.as_str();
        }
        Self::new_unchecked(trimmed)
    }

    /// Trims any trailing decorative delimiters from the identifier.
    ///
    /// This function can not leave you with an invalid identifier, it
    /// explicitly only trims delimiters that it considers to be non-essential,
    /// or decorative.
    ///
    /// If there's multiple kinds of delimiters, the left-most delimiter will
    /// be preserved (e.g. the trimming happens from right-to-left).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("__DecoratedIdent__")?;
    /// assert_eq!(ident.trim_trailing_decorative_delims(), "__DecoratedIdent");
    /// let ident = UpperCamelIdent::new("__2DecoratedIdent__")?;
    /// assert_eq!(ident.trim_trailing_decorative_delims(), "__2DecoratedIdent");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// A delimiter-only identifier becomes a single-character identifier:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("____")?;
    /// assert_eq!(ident.trim_trailing_decorative_delims(), "_");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// If there's multiple allowed delimiters, the right-most is preserved:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// assert_eq!(HybridIdent::new("--__")?.trim_trailing_decorative_delims(), "-");
    /// assert_eq!(HybridIdent::new("__--")?.trim_trailing_decorative_delims(), "_");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this returns the remaining sub-identifier as a new identifier, without modifying the original"]
    #[inline]
    pub fn trim_trailing_decorative_delims(&self) -> &Self {
        let mut chars = self.chars();
        let mut trimmed = self.as_str();
        while let Some(c) = chars.next_back() {
            // If it's not a delim, stop - we've trimmed them all.
            if !D::is_delim(c) {
                break;
            }

            // If there's no next char, don't trim - don't leave it empty.
            if chars.as_str().is_empty() {
                break;
            };

            trimmed = chars.as_str();
        }
        Self::new_unchecked(trimmed)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Ident<B, D, P> {
    /// Returns a fragment representation of the identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("ExampleIdent")?;
    /// assert_eq!(ident.as_fragment(), UpperCamelFragment::new("ExampleIdent")?);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_fragment(&self) -> &Fragment<B, D, P> {
        &self.inner
    }

    /// Returns a string slice representation of the fragment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new("ExampleIdent")?;
    /// assert_eq!(ident.as_str(), "ExampleIdent");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Zero-cost cast into the type-configured identifier.
    ///
    /// This function does not perform any checks that the format matches the
    /// expectations of the target type. The way it's able to be provided
    /// depends on implementation of the [`SubsetOf`] trait.
    ///
    /// [`SubsetOf`]: crate::syntax::SubsetOf
    ///
    /// # Casting Requirements
    ///
    /// This function will be able to be called, if:
    ///
    /// * `Source::D: SubsetOf<Target::D>`, *and...*
    /// * `Source::P: SubsetOf<Target::P>`
    ///
    /// If these invariants are not upheld, attempting to call this function
    /// will result in a compilation failure.
    ///
    /// # Pro-Tip
    ///
    /// If an identifier can perform a zero-cost cast, then the identifier also
    /// will implement `AsRef` to the target identifier. Because of this, if
    /// you know the shape of identifier that you want, but also want to accept
    /// the widest range of inputs, you can use an `AsRef` trait bounds.
    ///
    /// ```
    /// # use typed_ident::presets::unicode::*;
    /// fn expect_hybrid_ident<I: AsRef<HybridIdent> + ?Sized>(ident: &I) {
    ///     // ...
    /// }
    /// expect_hybrid_ident(LowerSnakeIdent::new("apple")?);
    /// expect_hybrid_ident(UpperCamelIdent::new("Apple")?);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// # Examples
    ///
    /// Example traversing case profile boundary:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// // Compilable Cast:
    /// let original = LowerSnakeIdent::new("apple")?;
    /// let casted: &LowerCamelIdent = original.cast();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// // Bad Cast (Fails Compilation):
    /// let original = LowerCamelIdent::new("apple")?;
    /// let casted: &LowerSnakeIdent = original.cast();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Example traversing character profile boundary:
    ///
    /// ```
    /// # use typed_ident::presets::ascii::lower_snake as ascii;
    /// # use typed_ident::presets::unicode::lower_snake as unicode;
    /// // Compilable Cast:
    /// let original = ascii::LowerSnakeIdent::new("apple")?;
    /// let casted: &unicode::LowerSnakeIdent = original.cast();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::ascii::lower_snake as ascii;
    /// # use typed_ident::presets::unicode::lower_snake as unicode;
    /// // Bad Cast (Fails Compilation):
    /// let original = unicode::LowerSnakeIdent::new("apple")?;
    /// let casted: &ascii::LowerSnakeIdent = original.cast();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Example traversing delimiter boundary:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// // Compilable Cast:
    /// let original = LowerSnakeIdent::new("apple")?;
    /// let casted: &HybridIdent = original.cast();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// // Bad Cast (Fails Compilation):
    /// let original = HybridIdent::new("apple")?;
    /// let casted: &LowerSnakeIdent = original.cast();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn cast<B2, D2, P2>(&self) -> &Ident<B2, D2, P2>
    where
        D: crate::syntax::SubsetOf<D2>,
        P: crate::syntax::SubsetOf<P2>,
    {
        Ident::new_unchecked(self.as_str())
    }

    /// Returns the length of `self`.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words, it
    /// might not be what a human considers the length of the identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let slice = HybridIdent::new("foo")?;
    /// let len = slice.len();
    /// assert_eq!(len, 3);
    ///
    /// let slice = HybridIdent::new("ƒoo")?;
    /// assert_eq!(slice.len(), 4); // fancy f!
    /// assert_eq!(slice.chars().count(), 3);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    #[allow(clippy::len_without_is_empty)] // Identifiers are never empty.
    pub const fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Converts a string slice to an identifier without checking that the
    /// contents are a valid chunk.
    ///
    /// See the safe version, [`new`], for more information.
    ///
    /// [`new`]: Self::new
    ///
    /// # Safety
    ///
    /// Needless to say, this is very difficult to deduce on your own.
    ///
    /// The easiest way to uphold this invariant is when taking a slice (which
    /// must contain the first character) of an identifier represented as a
    /// string - *or* if you have checked the identifier in a proc-macro during
    /// compilation.
    ///
    /// Those are the intended use-cases for this function.
    #[must_use]
    #[inline(always)]
    pub(crate) const fn new_unchecked(s: &str) -> &Self {
        // SAFETY: Transparent over Fragment, which itself is transparent over
        // str. Because of this, the layout, alignment, metadata, and validity
        // are identical.
        unsafe { core::mem::transmute::<&str, &Ident<B, D, P>>(s) }
    }

    /// Divides one identifier at an index, leaving an optional identifier on
    /// the left, and a fragment on the right.
    ///
    /// The argument, `mid`, should be a byte offset from the start of the
    /// identifier. It must also be on the boundary of a UTF-8 code point.
    ///
    /// The two slices returned go from the start of the identifier to `mid`,
    /// and from `mid` to the end of the identifier.
    ///
    /// # Panics
    ///
    /// Panics if `mid` is not on a UTF-8 code point boundary, or if it is past
    /// the end of the last code point of the identifier. For a non-panicking
    /// alternative see [`split_at_checked`].
    ///
    /// [`split_at_checked`]: Self::split_at_checked
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let slice = HybridIdent::new("こんにちは世界")?;
    ///
    /// let (first, last) = slice.split_at(15);
    /// assert_eq!(first, Some(HybridIdent::new("こんにちは")?));
    /// assert_eq!(last, HybridIdent::new("世界")?);
    ///
    /// let (first, last) = slice.split_at(0);
    /// assert_eq!(first, None);
    /// assert_eq!(last, HybridIdent::new("こんにちは世界")?);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub const fn split_at(&self, mid: usize) -> (Option<&Self>, &Fragment<B, D, P>) {
        let (left, right) = self.as_fragment().split_at(mid);
        (Self::from_fragment_unchecked_opt(left), right)
    }

    /// Divides one identifier at an index, leaving an optional identifier on
    /// the left, and a fragment on the right.
    ///
    /// The argument, `mid`, should be a byte offset from the start of the
    /// identifier. It must also be on the boundary of a UTF-8 code point. The
    /// method returns `None` if that's not the case.
    ///
    /// The two slices returned go from the start of the identifier to `mid`,
    /// and from `mid` to the end of the identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// let slice = HybridIdent::new("こんにちは世界")?;
    ///
    /// let (first, last) = slice.split_at_checked(15).unwrap();
    /// assert_eq!(first, Some(HybridIdent::new("こんにちは")?));
    /// assert_eq!(last, HybridIdent::new("世界")?);
    ///
    /// let (first, last) = slice.split_at_checked(0).unwrap();
    /// assert_eq!(first, None);
    /// assert_eq!(last, HybridIdent::new("こんにちは世界")?);
    ///
    /// assert!(slice.split_at_checked(16).is_none()); // Inside "世"
    /// assert!(slice.split_at_checked(42).is_none()); // Beyond the length
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    #[allow(clippy::type_complexity)] // I thought about this a lot - a helper type only hurts here.
    pub const fn split_at_checked(
        &self,
        mid: usize,
    ) -> Option<(Option<&Self>, &Fragment<B, D, P>)> {
        match self.as_fragment().split_at_checked(mid) {
            Some((l, r)) => Some((Self::from_fragment_unchecked_opt(l), r)),
            None => None,
        }
    }

    #[must_use]
    #[inline(always)]
    const fn from_fragment_unchecked_opt(fragment: &Fragment<B, D, P>) -> Option<&Self> {
        match fragment.is_empty() {
            true => None,
            false => Some(Self::new_unchecked(fragment.as_str())),
        }
    }

    /// Attempts a fallible cast into the type-configured target.
    ///
    /// You should first attempt to call [`cast`] on a type, if that compiles it
    /// is preferred to this function (and you will not need to call this
    /// function), because it is truly zero-cost.
    ///
    /// This is equivalent to just calling [`new`] on the target type with the
    /// current type's string contents. This function is provided for ergonomic
    /// convenience.
    ///
    /// [`cast`]: Self::cast
    /// [`new`]: Self::new
    #[inline]
    pub fn try_cast<B2, D2, P2>(&self) -> Result<&Ident<B2, D2, P2>, Error>
    where
        B2: Boundary,
        D2: Delimiter,
        P2: Profile,
    {
        Ident::new(self.as_str())
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl_typed_slice_traits! {
    name=Ident,
    index_target=Fragment,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=Ident,
    against=Chunk,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=Ident,
    against=Fragment,
}

// -----------------------------------------------------------------------------
impl<B1: Boundary, D1: Delimiter, P1: Profile, B2, D2, P2> AsRef<Ident<B2, D2, P2>>
    for Ident<B1, D1, P1>
where
    D1: crate::syntax::SubsetOf<D2>,
    P1: crate::syntax::SubsetOf<P2>,
{
    #[inline(always)]
    fn as_ref(&self) -> &Ident<B2, D2, P2> {
        self.cast()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> core::convert::TryFrom<&'a str>
    for &'a Ident<B, D, P>
{
    type Error = Error;
    #[inline(always)]
    fn try_from(orig: &'a str) -> Result<Self, Self::Error> {
        Ident::new(orig)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> core::ops::Deref for Ident<B, D, P> {
    type Target = Fragment<B, D, P>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_fragment()
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> core::convert::TryFrom<&'a Fragment<B, D, P>>
    for &'a Ident<B, D, P>
{
    type Error = Error;

    #[inline(always)]
    fn try_from(orig: &'a Fragment<B, D, P>) -> Result<Self, Self::Error> {
        Ident::from_fragment(orig)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> core::convert::TryFrom<&'a Chunk<B, D, P>>
    for &'a Ident<B, D, P>
{
    type Error = Error;

    #[inline]
    fn try_from(orig: &'a Chunk<B, D, P>) -> Result<Self, Self::Error> {
        Ident::from_fragment(orig.as_fragment())
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> Identifier for Ident<B, D, P> {
    type Boundary = B;
    type Delimiter = D;
    type Profile = P;

    #[inline(always)]
    fn as_ident(&self) -> &Ident<Self::Boundary, Self::Delimiter, Self::Profile> {
        self
    }
}
