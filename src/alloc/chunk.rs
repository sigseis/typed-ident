// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::{Boundary, CasedProfile, Delimiter};
use crate::{Chunk, Error, FragmentBuf};
use std_alloc::boxed::Box;
use std_alloc::string::String;

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> Chunk<B, D, P> {
    /// Converts a string into a boxed chunk if it's valid.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeChunk> = Chunk::new_boxed("snake")?;
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn new_boxed<S>(string: S) -> Result<Box<Self>, Error>
    where
        S: Into<String>,
    {
        let string = string.into();
        let _ = Chunk::<B, D, P>::new(&string)?;
        Ok(Self::new_boxed_unchecked(string))
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Chunk<B, D, P> {
    /// Zero-cost conversion into a different boxed, type-configured chunk.
    ///
    /// This is the boxed, by-value equivalent of [`cast`], see that function
    /// for details on how this works. If you don't need to consume the boxed
    /// value, you can instead use that function to get a reference to a
    /// different type-configured chunk.
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
    /// let original = LowerSnakeChunk::new_boxed("apple")?;
    /// let converted: Box<LowerCamelChunk> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::lower_camel::*;
    /// // Bad Cast (Fails Compilation):
    /// let original = LowerCamelChunk::new_boxed("apple")?;
    /// let converted: Box<LowerSnakeChunk> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Example traversing character profile boundary:
    ///
    /// ```
    /// # use typed_ident::presets::ascii::lower_snake as ascii;
    /// # use typed_ident::presets::unicode::lower_snake as unicode;
    /// // Compilable Cast:
    /// let original = ascii::LowerSnakeChunk::new_boxed("apple")?;
    /// let converted: Box<unicode::LowerSnakeChunk> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::ascii::lower_snake as ascii;
    /// # use typed_ident::presets::unicode::lower_snake as unicode;
    /// // Bad Cast (Fails Compilation):
    /// let original = unicode::LowerSnakeChunk::new_boxed("apple")?;
    /// let converted: Box<ascii::LowerSnakeChunk> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Example traversing delimiter boundary:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// // Compilable Cast:
    /// let original = LowerSnakeChunk::new_boxed("apple")?;
    /// let converted: Box<HybridChunk> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// # use typed_ident::presets::unicode::hybrid::*;
    /// // Bad Cast (Fails Compilation):
    /// let original = HybridChunk::new_boxed("apple")?;
    /// let converted: Box<LowerSnakeChunk> = original.convert();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn convert<B2, D2, P2>(self: Box<Self>) -> Box<Chunk<B2, D2, P2>>
    where
        B2: crate::syntax::Boundary,
        D: crate::syntax::SubsetOf<D2>,
        P: crate::syntax::SubsetOf<P2>,
    {
        Chunk::new_boxed_unchecked(self.into_string())
    }

    /// Converts a boxed chunk into a boxed string slice.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeChunk> = Chunk::new_boxed("snake")?;
    /// let ident: Box<str> = ident.into_boxed_str();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_boxed_str(self: Box<Self>) -> Box<str> {
        // SAFETY: Chunk is transparent over Fragment, which is itself
        // transparent over str, so Box<Chunk> has the same allocation layout,
        // pointer metadata, alignment, and ownership behavior as Box<str>.
        unsafe { Box::from_raw(Box::into_raw(self) as *mut str) }
    }

    /// Converts a boxed chunk into a fragment buffer.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeChunk> = Chunk::new_boxed("snake")?;
    /// let buffer: LowerSnakeFragmentBuf = ident.into_fragment_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_fragment_buf(self: Box<Self>) -> FragmentBuf<B, D, P> {
        FragmentBuf::from_string_unchecked(self.into_string())
    }

    /// Converts a boxed chunk into a string.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: Box<LowerSnakeChunk> = Chunk::new_boxed("snake")?;
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
        // SAFETY: Chunk is transparent over Fragment, which is itself
        // transparent over str, so Box<Chunk> has the same allocation layout,
        // pointer metadata, alignment, and ownership behavior as Box<str>.
        unsafe { Box::from_raw(Box::into_raw(boxed_str) as *mut Self) }
    }

    /// Converts a chunk into an owned boxed chunk.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: &LowerSnakeChunk = Chunk::new("snake")?;
    /// let ident: Box<LowerSnakeChunk> = ident.to_boxed_chunk();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn to_boxed_chunk(&self) -> Box<Self> {
        Chunk::new_boxed_unchecked(String::from(self.as_str()))
    }

    /// Converts a chunk into a fragment buffer.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::presets::unicode::lower_snake::*;
    /// let ident: &LowerSnakeChunk = Chunk::new("snake")?;
    /// let buffer: LowerSnakeFragmentBuf = ident.to_fragment_buf();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn to_fragment_buf(&self) -> FragmentBuf<B, D, P> {
        FragmentBuf::from_fragment(self)
    }

    /// Attempts a fallible convert into a different boxed type-configured
    /// chunk.
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
    pub fn try_convert<B2, D2, P2>(self: Box<Self>) -> Result<Box<Chunk<B2, D2, P2>>, Error>
    where
        B2: Boundary,
        D2: Delimiter,
        P2: CasedProfile,
    {
        Chunk::new_boxed(self.into_string())
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P> crate::type_erased::TypeErased for Box<Chunk<B, D, P>> {
    type Erased = String;
    #[inline(always)]
    fn type_erased(self) -> Self::Erased {
        self.into_string()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> std_alloc::borrow::ToOwned for Chunk<B, D, P> {
    type Owned = Box<Chunk<B, D, P>>;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.to_boxed_chunk()
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: CasedProfile> TryFrom<String> for Box<Chunk<B, D, P>> {
    type Error = crate::Error;
    #[inline]
    fn try_from(orig: String) -> Result<Self, Self::Error> {
        Chunk::new_boxed(orig)
    }
}

// -----------------------------------------------------------------------------
impl_boxed_slice_traits! {
    name=Chunk,
    index_target=Chunk,
}
