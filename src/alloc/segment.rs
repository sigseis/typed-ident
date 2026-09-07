// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Segment;
use crate::syntax::Delimiter;
use std_alloc::string::String;

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D> Segment<D, &str> {
    /// Converts a type-erased segment into an owned representation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased_chunk()
    ///     .into_owned();
    /// assert_eq!(segment, CamelSegment::Delim(LowLine).type_erased_chunk());
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased_chunk()
    ///     .into_owned();
    /// assert_eq!(segment, StringSegment::Chunk(String::from("Upper")));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_owned(self) -> Segment<D, String> {
        match self {
            Segment::Chunk(chunk) => Segment::Chunk(String::from(chunk)),
            Segment::Delim(delim) => Segment::Delim(delim),
        }
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter> Segment<D, String> {
    /// Returns the byte length of the string representation of `self`.
    ///
    /// For delimiters, this will first cast as a character, then return the
    /// UTF-8 length of the character. For chunks, the length of the chunk is
    /// used directly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased_chunk()
    ///     .into_owned();
    /// assert_eq!(segment.len(), 1);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased_chunk()
    ///     .into_owned();
    /// assert_eq!(segment.len(), 5);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    pub fn len(&self) -> usize {
        match self {
            Self::Delim(delim) => delim.as_char().len_utf8(),
            Self::Chunk(chunk) => chunk.len(),
        }
    }
}

// -----------------------------------------------------------------------------
impl Segment<char, String> {
    /// Returns the byte length of the string representation of `self`.
    ///
    /// For delimiters, this will first cast as a character, then return the
    /// UTF-8 length of the character. For chunks, the length of the chunk is
    /// used directly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased()
    ///     .into_owned();
    /// assert_eq!(segment.len(), 1);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased()
    ///     .into_owned();
    /// assert_eq!(segment.len(), 5);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    pub fn len(&self) -> usize {
        match self {
            Self::Delim(delim) => delim.len_utf8(),
            Self::Chunk(chunk) => chunk.len(),
        }
    }
}
