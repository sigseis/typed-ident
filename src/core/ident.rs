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
use crate::core::error::Error;
use crate::core::{Chunk, Fragment, Identifier, Segment};
use crate::syntax::{Boundary, CasedProfile, Delimiter, UnitDelimiter};

// =============================================================================
// TYPES
// =============================================================================

/// An immutable, UTF-8 encoded, valid identifier string slice.
///
/// # Construction
///
/// It's not expected that you interact with this type directly. Instead, you
/// should interact with it through a type alias which fully defines the
/// identifier.
///
/// The common way to get a reasonable alias is through the [`presets`] module.
///
/// [`presets`]: crate::presets
///
/// ```
/// use typed_ident::presets::unicode::LowerSnakeIdent;
///
/// // By-Reference (`&LowerSnakeIdent`)
/// let ident = LowerSnakeIdent::new("lower_snake")?;
///
/// // Owned (`Box<LowerSnakeIdent>`) (requires `alloc` feature)
/// let owned = ident.to_boxed_ident(); // From pre-validated reference
/// let owned = LowerSnakeIdent::new_boxed(String::from("lower_snake"))?;
/// # Ok::<(), typed_ident::Error>(())
/// ```
///
/// These presets are all just type aliases to `Ident` with the generic type
/// parameters filled-in. If you would like to define your own custom
/// identifier, it's recommended that you do so by defining your own type alias.
///
/// ```
/// use typed_ident::Ident;
/// use typed_ident::syntax::{boundary, delimiter, profile};
///
/// // An ASCII identifier using the uppercase ASCII profile.
/// // Any ASCII punctuation is permitted as delimiters.
/// type CustomIdent = Ident<
///     boundary::Standard,
///     delimiter::AsciiPunctuation,
///     profile::Upper<profile::Ascii>,
/// >;
///
/// let ident = CustomIdent::new("UPPER#@IDENT")?;
/// # Ok::<(), typed_ident::Error>(())
/// ```
///
/// See the [`core`] module definition to understand how this type relates to
/// other types (such as [`Fragment`] and [`Chunk`]), as well as additional
/// information for how to use these types effectively.
///
/// [`core`]: crate::core
///
/// # Type Conversion
///
/// There's several ways to convert between types depending on what you want to
/// accomplish. These type conversions ***DO NOT*** reformat or change the input
/// string. They just produce a new type of the target identifier syntax.
///
/// | **I Have a...**   | **I Want a...**   | **Method**                | **Validation Cost**  | **Allocation Cost** |
/// |-------------------|-------------------|---------------------------|----------------------|---------------------|
/// | `&Ident<A..>`     | `&Ident<B..>`     | [`cast`]  / [`as_ref`]    | None                 | None                |
/// | `&Ident<A..>`     | `&Ident<B..>`     | [`try_cast`]              | Same as [`new`]      | None                |
/// | `&Ident`          | `Box<Ident>`      | [`to_boxed_ident`]        | None                 | `O(strlen)`         |
/// | `&Box<Ident>`     | `&Ident`          | [`as_ident`] / [`as_ref`] | None                 | None                |
/// | `Box<Ident<A..>>` | `Box<Ident<B..>>` | [`convert`]               | None                 | None                |
/// | `Box<Ident<A..>>` | `Box<Ident<B..>>` | [`try_convert`]           | Same as [`new`]      | None                |
///
/// [`as_ident`]: Ident::as_ident
/// [`as_ref`]: Ident::as_ref
/// [`cast`]: Ident::cast
/// [`convert`]: Ident::convert
/// [`new`]: Ident::new
/// [`to_boxed_ident`]: Ident::to_boxed_ident
/// [`try_cast`]: Ident::try_cast
/// [`try_convert`]: Ident::try_convert
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
/// [`LowLine`]: crate::syntax::delimiter::LowLine
/// [`Lower`]: crate::syntax::profile::Lower
/// [`Mixed`]: crate::syntax::profile::Mixed
/// [`Profile`]: crate::syntax::profile::Profile
/// [`Standard`]: crate::syntax::boundary::Standard
/// [`Unicode`]: crate::syntax::profile::Unicode
/// [`syntax`]: crate::syntax
#[repr(transparent)]
pub struct Ident<B, D, P> {
    inner: Fragment<B, D, P>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B: 'a, D: UnitDelimiter + 'a, P: 'a> Ident<B, D, P> {
    /// An anonymous identifier filled with a single unit delimiter.
    pub const ANONYMOUS: &'a Ident<B, D, P> = Ident::new_unchecked(D::STR);
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> Ident<B, D, P> {
    /// Returns the first segment of an identifier.
    ///
    /// Since an identifier is always non-empty, there's always at least one
    /// available segment. This function will simply return the first segment.
    ///
    /// # Examples
    ///
    /// ```
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
        // We could `unwrap_unchecked` here, but this is not a hot path, and it
        // is probably better to catch if someone constructed this invalidly
        // using unsafe instead of saving a few cycles in a cold path.
        self.segments().next().unwrap()
    }

    /// Converts a fragment to an identifier.
    ///
    /// An identifier is made of a fragment, this function converts between the
    /// two. Not all fragments are valid identifiers, however. An identifier has
    /// additional requirements.
    ///
    /// `new` checks to ensure these are satisfied before the conversion.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the fragment does not satisfy the character
    /// requirements. If the fragment is empty, this will return an `Empty`
    /// error kind. If an invalid character is found then a `InvalidFormat`
    /// error kind is returned, with [`byte_offset`] set to the byte index for
    /// the first invalid character.
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
        P::is_ident_fragment::<D>(fragment.as_str())?;
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
        // We could `unwrap_unchecked` here, but this is not a hot path, and it
        // is probably better to catch if someone constructed this invalidly
        // using unsafe instead of saving a few cycles in a cold path.
        self.segments().next_back().unwrap()
    }

    /// Converts a string slice to an identifier.
    ///
    /// An identifier is made of a [`Fragment`], which itself is made of a
    /// string slice ([`&str`]), this function converts between the two. Not all
    /// string slices are valid identifiers, however. An identifier requires
    /// that the characters it is comprised of satisfy certain requirements.
    ///
    /// `new` checks to ensure these are satisfied before the conversion.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the identifier does not satisfy the character
    /// requirements. If the identifier is empty, this will return an `Empty`
    /// error kind. If an invalid character is found then a `InvalidFormat`
    /// error kind is returned, with [`byte_offset`] set to the byte index for
    /// the first invalid character.
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
        P::is_ident::<D>(s)?;
        Ok(Self::new_unchecked(s))
    }

    /// Trims any decorative delimiters from the identifier.
    ///
    /// This function ***cannot*** leave you with an invalid identifier, it
    /// explicitly only trims delimiters that it considers to be non-essential,
    /// or decorative.
    ///
    /// If there's multiple kinds of delimiters, the right-most delimiter will
    /// be preserved. This is essentially the same as calling
    /// [`trim_leading_decorative_delims`] followed by
    /// [`trim_trailing_decorative_delims`].
    ///
    /// If you want all delimiters to be trimmed regardless of whether or not it
    /// will leave you with a valid identifier, you can instead call the
    /// [`trim_delims`] method, which returns a [`Fragment`].
    ///
    /// [`trim_delims`]: Fragment::trim_delims
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
    /// This function ***cannot*** leave you with an invalid identifier, it
    /// explicitly only trims delimiters that it considers to be non-essential,
    /// or decorative.
    ///
    /// If there's multiple kinds of delimiters, the right-most delimiter will
    /// be preserved (e.g. the trimming happens from left-to-right).
    ///
    /// If you want all leading delimiters to be trimmed regardless of whether
    /// or not it will leave you with a valid identifier, you can instead call
    /// the [`trim_leading_delims`] method, which returns a [`Fragment`].
    ///
    /// [`trim_leading_delims`]: Fragment::trim_leading_delims
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
            if !D::is_ident_start_delim(c) {
                break;
            }

            // If there's no next char, don't trim - don't leave it empty.
            let mut peek = chars.clone();
            let Some(next) = peek.next() else {
                break;
            };

            // If the next character is not a valid start character, don't trim.
            if !D::is_ident_start_delim(next) && !P::is_ident_start_char(next) {
                break;
            }

            trimmed = chars.as_str();
        }
        Self::new_unchecked(trimmed)
    }

    /// Trims any trailing decorative delimiters from the identifier.
    ///
    /// This function ***cannot*** leave you with an invalid identifier, it
    /// explicitly only trims delimiters that it considers to be non-essential,
    /// or decorative.
    ///
    /// If there's multiple kinds of delimiters, the left-most delimiter will
    /// be preserved (e.g. the trimming happens from right-to-left).
    ///
    /// If you want all trailing delimiters to be trimmed regardless of whether
    /// or not it will leave you with a valid identifier, you can instead call
    /// the [`trim_trailing_delims`] method, which returns a [`Fragment`].
    ///
    /// [`trim_trailing_delims`]: Fragment::trim_trailing_delims
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
    /// Note that `Ident` implements `Deref<Target = Fragment>`, so usually you
    /// don't need to call this function explicitly.
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

    /// Returns the identifier as a reference.
    ///
    /// This method exists for convenience use when dealing with `Box<Ident>`
    /// types. That way there's a simple way to get the underlying reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let ident = UpperCamelIdent::new_boxed(String::from("ExampleIdent"))?;
    /// assert_eq!(ident.as_ident(), UpperCamelIdent::new("ExampleIdent")?);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline(always)]
    pub const fn as_ident(&self) -> &Self {
        self
    }

    /// Returns a string slice representation of the identifier.
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

    /// Zero-cost cast into a different type-configured identifier.
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
    /// // A `HybridIdent` accepts any of the other preset formats!
    /// fn expect_hybrid_ident<I: AsRef<HybridIdent> + ?Sized>(ident: &I) {
    ///     // ...
    /// }
    /// expect_hybrid_ident(LowerSnakeIdent::new("lower_snake")?);
    /// expect_hybrid_ident(UpperCamelIdent::new("UpperCamel")?);
    /// expect_hybrid_ident(KebabIdent::new("kebab-ident")?);
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
    /// See the checked version, [`new`], for more information.
    ///
    /// [`new`]: Self::new
    ///
    /// # Safety
    ///
    /// This function is always memory-safe, even when provided with an invalid
    /// string. However, it should only be used in situations where you are
    /// certain of the format of your string.
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
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// let original = LowerCamelIdent::new("apple")?;
    /// let casted: &LowerSnakeIdent = original.try_cast()?;
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn try_cast<B2, D2, P2>(&self) -> Result<&Ident<B2, D2, P2>, Error>
    where
        B2: Boundary,
        D2: Delimiter,
        P2: CasedProfile,
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
impl<B1: Boundary, D1: Delimiter, P1: CasedProfile, B2, D2, P2> AsRef<Ident<B2, D2, P2>>
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
impl<'a, B: Boundary, D: Delimiter, P: CasedProfile> core::convert::TryFrom<&'a str>
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
impl<'a, B: Boundary, D: Delimiter, P: CasedProfile> core::convert::TryFrom<&'a Fragment<B, D, P>>
    for &'a Ident<B, D, P>
{
    type Error = Error;

    #[inline(always)]
    fn try_from(orig: &'a Fragment<B, D, P>) -> Result<Self, Self::Error> {
        Ident::from_fragment(orig)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: CasedProfile> core::convert::TryFrom<&'a Chunk<B, D, P>>
    for &'a Ident<B, D, P>
{
    type Error = Error;

    #[inline]
    fn try_from(orig: &'a Chunk<B, D, P>) -> Result<Self, Self::Error> {
        Ident::from_fragment(orig.as_fragment())
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> Identifier for Ident<B, D, P> {
    type Boundary = B;
    type Delimiter = D;
    type Profile = P;

    #[inline(always)]
    fn as_ident(&self) -> &Ident<Self::Boundary, Self::Delimiter, Self::Profile> {
        self
    }
}
