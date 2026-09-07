// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "segment.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::error::{Error, ErrorKind};
use crate::core::{Chunk, StrSegment};
use crate::syntax::{Boundary, Delimiter, Profile};

// =============================================================================
// TYPES
// =============================================================================

/// A part of an identifier returned during segmentation.
///
/// The default type contents of a segment are:
///
/// * `D`: The delimiter from the fragment/identifier.
/// * `C`: A chunk formed from the type parameters ([`&Chunk<B, D, P>`]).
///
/// If you don't need strong typing, you can use the [`type_erased`] method to
/// drop the type information (producing `char` and `&str`). There is a typedef
/// associated with this type for convenience ([`StrSegment`]).
///
/// You have two main things you can control about how segmentation happens;
/// whether or not a contiguous chunk is broken into it's largest "words" (runs
/// of characters with no natural boundaries as defined by `B`), and whether or
/// not you are returned the byte offset to the segment from the original data.
///
/// | **Words \\ Indices** |        **No**        |           **Yes**           |
/// |----------------------|----------------------|-----------------------------|
/// | **No**               | [`chunked_segments`] | [`chunked_segment_indices`] |
/// | **Yes**              | [`segments`]         | [`segment_indices`]         |
///
/// Normally, you'll want chunks broken into their largest words. So we've named
/// the variant that breaks chunks into their largest words simply [`segments`].
///
/// See the respective functions for more details.
///
/// [`&Chunk<B, D, P>`]: crate::core::Chunk
/// [`chunked_segment_indices`]: crate::core::fragment::Fragment::chunked_segment_indices
/// [`chunked_segments`]: crate::core::fragment::Fragment::chunked_segments
/// [`segment_indices`]: crate::core::fragment::Fragment::segment_indices
/// [`segments`]: crate::core::fragment::Fragment::segments
/// [`type_erased`]: Self::type_erased
#[derive(Copy, Clone, Debug, Eq, Hash, Ord)]
pub enum Segment<D, C> {
    /// A chunk (run of non-delimiter characters) of an identifier.
    Chunk(C),
    /// A single delimiter.
    Delim(D),
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D: Delimiter, P> Segment<D, &'a Chunk<B, D, P>> {
    /// A helper type that converts a string slice segment to a typed segment
    /// without checking the underlying data.
    #[inline]
    pub(crate) fn from_unchecked(orig: StrSegment<'a>) -> Self {
        match orig {
            Segment::Delim(delim) => Self::Delim(D::from_char(delim).unwrap()),
            Segment::Chunk(chunk) => Self::Chunk(Chunk::new_unchecked(chunk)),
        }
    }

    /// Returns `true` if the contents of this segment is an empty chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine);
    /// assert_eq!(segment.is_empty(), false);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?);
    /// assert_eq!(segment.is_empty(), false);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("")?);
    /// assert_eq!(segment.is_empty(), true);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Delim(_) => false,
            Self::Chunk(chunk) => chunk.is_empty(),
        }
    }

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
    /// let segment = CamelSegment::Delim(LowLine);
    /// assert_eq!(segment.len(), 1);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?);
    /// assert_eq!(segment.len(), 5);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Delim(delim) => delim.as_char().len_utf8(),
            Self::Chunk(chunk) => (*chunk).as_str().len(),
        }
    }

    /// Converts the segment to a type-erased representation.
    ///
    /// The `Delim` variant will be converted to a `char`, and the `Chunk`
    /// variant will be converted to a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::assert_matches;
    /// # use typed_ident::core::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine).type_erased();
    /// assert!(segment.is_delim_and(|c| *c == '_'));
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?).type_erased();
    /// assert!(segment.is_chunk_and(|c| *c == "Upper"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn type_erased(self) -> Segment<char, &'a str> {
        match self {
            Self::Delim(delim) => Segment::Delim(delim.as_char()),
            Self::Chunk(chunk) => Segment::Chunk(chunk.as_str()),
        }
    }

    /// Converts a segment's delimiter to a type-erased representation.
    ///
    /// The `Delim` variant will be converted to a `char`, but the chunk
    /// will remain strongly-typed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::assert_matches;
    /// # use typed_ident::core::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine).type_erased_delim();
    /// assert!(segment.is_delim_and(|c| *c == '_'));
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?).type_erased_delim();
    /// assert!(segment.is_chunk_and(|c| *c == "Upper"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn type_erased_delim(self) -> Segment<char, &'a Chunk<B, D, P>> {
        match self {
            Self::Delim(delim) => Segment::Delim(delim.as_char()),
            Self::Chunk(chunk) => Segment::Chunk(chunk),
        }
    }

    /// Converts a segment's chunk to a type-erased representation.
    ///
    /// The `Chunk` variant will be converted to a `&str`, but the delimiter
    /// will remain strongly-typed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::assert_matches;
    /// # use typed_ident::core::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine).type_erased_chunk();
    /// assert!(segment.is_delim_and(|c| *c == '_'));
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?).type_erased_chunk();
    /// assert!(segment.is_chunk_and(|c| *c == "Upper"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn type_erased_chunk(self) -> Segment<D, &'a str> {
        match self {
            Self::Delim(delim) => Segment::Delim(delim),
            Self::Chunk(chunk) => Segment::Chunk(chunk.as_str()),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, D: Delimiter> Segment<D, &'a str> {
    /// Returns `true` if the contents of this segment is an empty chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased_chunk();
    /// assert_eq!(segment.is_empty(), false);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased_chunk();
    /// assert_eq!(segment.is_empty(), false);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("")?)
    ///     .type_erased_chunk();
    /// assert_eq!(segment.is_empty(), true);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Delim(_) => false,
            Self::Chunk(chunk) => chunk.is_empty(),
        }
    }

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
    ///     .type_erased_chunk();
    /// assert_eq!(segment.len(), 1);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased_chunk();
    /// assert_eq!(segment.len(), 5);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Delim(delim) => delim.as_char().len_utf8(),
            Self::Chunk(chunk) => chunk.len(),
        }
    }

    /// Converts the segment to a type-erased representation.
    ///
    /// The `Delim` variant will be converted to a `char`, and the `Chunk`
    /// variant will be converted to a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::assert_matches;
    /// # use typed_ident::core::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased_chunk().type_erased();
    /// assert!(segment.is_delim_and(|c| *c == '_'));
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased_chunk().type_erased();
    /// assert!(segment.is_chunk_and(|c| *c == "Upper"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn type_erased(self) -> Segment<char, &'a str> {
        match self {
            Self::Delim(delim) => Segment::Delim(delim.as_char()),
            Self::Chunk(chunk) => Segment::Chunk(chunk),
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B, D, P> Segment<char, &'a Chunk<B, D, P>> {
    /// Returns `true` if the contents of this segment is an empty chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased_delim();
    /// assert_eq!(segment.is_empty(), false);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased_delim();
    /// assert_eq!(segment.is_empty(), false);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("")?)
    ///     .type_erased_delim();
    /// assert_eq!(segment.is_empty(), true);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Delim(_) => false,
            Self::Chunk(chunk) => chunk.is_empty(),
        }
    }

    /// Returns the byte length of the string representation of `self`.
    ///
    /// For character delimiters, this will return the UTF-8 length of the
    /// character. For chunks, the length of the chunk is used directly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased_delim();
    /// assert_eq!(segment.len(), 1);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased_delim();
    /// assert_eq!(segment.len(), 5);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Delim(delim) => delim.len_utf8(),
            Self::Chunk(chunk) => (*chunk).as_str().len(),
        }
    }

    /// Converts the segment to a type-erased representation.
    ///
    /// The `Chunk` variant will be converted to a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::assert_matches;
    /// # use typed_ident::core::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased_delim().type_erased();
    /// assert!(segment.is_delim_and(|c| *c == '_'));
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased_delim().type_erased();
    /// assert!(segment.is_chunk_and(|c| *c == "Upper"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn type_erased(self) -> Segment<char, &'a str> {
        match self {
            Self::Delim(delim) => Segment::Delim(delim),
            Self::Chunk(chunk) => Segment::Chunk(chunk.as_str()),
        }
    }
}

// -----------------------------------------------------------------------------
impl Segment<char, &str> {
    /// Returns `true` if the contents of this segment is an empty chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased();
    /// assert_eq!(segment.is_empty(), false);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased();
    /// assert_eq!(segment.is_empty(), false);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("")?)
    ///     .type_erased();
    /// assert_eq!(segment.is_empty(), true);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Delim(_) => false,
            Self::Chunk(chunk) => chunk.is_empty(),
        }
    }

    /// Returns the byte length of the string representation of `self`.
    ///
    /// For character delimiters, this will return the UTF-8 length of the
    /// character. For chunks, the length of the chunk is used directly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine)
    ///     .type_erased();
    /// assert_eq!(segment.len(), 1);
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?)
    ///     .type_erased();
    /// assert_eq!(segment.len(), 5);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Delim(delim) => delim.len_utf8(),
            Self::Chunk(chunk) => chunk.len(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<D, C> Segment<D, C> {
    /// Returns the original string representation of this segment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert_eq!(delim.as_str(), "_");
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert_eq!(chunk.as_str(), "chunk");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str
    where
        D: AsRef<str>,
        C: AsRef<str>,
    {
        match self {
            Self::Chunk(chunk) => chunk.as_ref(),
            Self::Delim(delim) => delim.as_ref(),
        }
    }

    /// Returns `true` if the segment contains a delimiter, `false` if not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert!(delim.is_delim());
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert!(!chunk.is_delim());
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub const fn is_delim(&self) -> bool {
        matches!(self, Self::Delim(_))
    }

    /// Returns `true` if the segment contains a delimiter, *and* if the
    /// provided predicate passes. Returns `false` if not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert!(delim.is_delim_and(|d| *d == '_'));
    /// assert!(!delim.is_delim_and(|d| *d == '-'));
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert!(!chunk.is_delim_and(|d| *d == '_'));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn is_delim_and(&self, pred: impl FnOnce(&D) -> bool) -> bool {
        match self {
            Self::Delim(delim) => pred(delim),
            _ => false,
        }
    }

    /// Returns `true` if the segment contains a delimiter, *or* if the segment
    /// contained a chunk, and provided predicate passes. Returns `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert!(delim.is_delim_or(|c| *c == "chunk"));
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert!(chunk.is_delim_or(|c| *c == "chunk"));
    /// assert!(!chunk.is_delim_or(|c| *c == "text"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn is_delim_or(&self, pred: impl FnOnce(&C) -> bool) -> bool {
        match self {
            Self::Chunk(chunk) => pred(chunk),
            _ => true,
        }
    }

    /// Returns `true` if the segment contains a chunk, `false` if not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert!(!delim.is_chunk());
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert!(chunk.is_chunk());
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub const fn is_chunk(&self) -> bool {
        matches!(self, Self::Chunk(_))
    }

    /// Returns `true` if the segment contains a chunk, *and* if the provided
    /// predicate passes. Returns `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert!(!delim.is_chunk_and(|c| *c == "chunk"));
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert!(chunk.is_chunk_and(|c| *c == "chunk"));
    /// assert!(!chunk.is_chunk_and(|c| *c == "text"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn is_chunk_and(&self, pred: impl FnOnce(&C) -> bool) -> bool {
        match self {
            Self::Chunk(chunk) => pred(chunk),
            _ => false,
        }
    }

    /// Returns `true` if the segment contains a chunk, *or* if the segment
    /// contained a delimiter, and provided predicate passes. Returns `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert!(delim.is_chunk_or(|d| *d == '_'));
    /// assert!(!delim.is_chunk_or(|d| *d == '-'));
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert!(chunk.is_chunk_or(|d| *d == '_'));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn is_chunk_or(&self, pred: impl FnOnce(&D) -> bool) -> bool {
        match self {
            Self::Delim(delim) => pred(delim),
            _ => true,
        }
    }

    /// Returns `Some` if the segment is a delim, otherwise returns `None`.
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert_eq!(delim.delim(), Some(LowLine));
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert_eq!(chunk.delim(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn delim(self) -> Option<D> {
        match self {
            Self::Delim(delim) => Some(delim),
            _ => None,
        }
    }

    /// Returns `Some` if the segment is a chunk, otherwise returns `None`.
    ///
    /// ```
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let delim = CamelSegment::Delim(LowLine);
    /// assert_eq!(delim.chunk(), None);
    ///
    /// let chunk = CamelSegment::Chunk(CamelChunk::new("chunk")?);
    /// assert_eq!(chunk.chunk(), Some(CamelChunk::new("chunk")?));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn chunk(self) -> Option<C> {
        match self {
            Self::Chunk(chunk) => Some(chunk),
            _ => None,
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D1, D2: PartialEq<D1>, C1, C2: PartialEq<C1>> core::cmp::PartialEq<Segment<D1, C1>>
    for Segment<D2, C2>
{
    #[inline]
    fn eq(&self, rhs: &Segment<D1, C1>) -> bool {
        match (self, rhs) {
            (Segment::Delim(lhs), Segment::Delim(rhs)) => lhs.eq(rhs),
            (Segment::Chunk(lhs), Segment::Chunk(rhs)) => lhs.eq(rhs),
            _ => false,
        }
    }
}

// -----------------------------------------------------------------------------
impl<D1, D2: core::cmp::PartialOrd<D1>, C1, C2: core::cmp::PartialOrd<C1>>
    core::cmp::PartialOrd<Segment<D1, C1>> for Segment<D2, C2>
{
    #[inline]
    fn partial_cmp(&self, rhs: &Segment<D1, C1>) -> Option<core::cmp::Ordering> {
        match (self, rhs) {
            (Segment::Delim(lhs), Segment::Delim(rhs)) => lhs.partial_cmp(rhs),
            (Segment::Chunk(lhs), Segment::Chunk(rhs)) => lhs.partial_cmp(rhs),
            (Segment::Delim(_), Segment::Chunk(_)) => Some(core::cmp::Ordering::Less),
            (Segment::Chunk(_), Segment::Delim(_)) => Some(core::cmp::Ordering::Greater),
        }
    }
}

// -----------------------------------------------------------------------------
impl<D: AsRef<str>, C: AsRef<str>> core::convert::AsRef<str> for Segment<D, C> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter, C> core::convert::From<D> for Segment<D, C> {
    #[inline]
    fn from(orig: D) -> Self {
        Self::Delim(orig)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B, D, P> core::convert::From<&'a Chunk<B, D, P>> for Segment<D, &'a Chunk<B, D, P>> {
    #[inline]
    fn from(orig: &'a Chunk<B, D, P>) -> Self {
        Self::Chunk(orig)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> core::convert::TryFrom<&'a str>
    for Segment<D, &'a Chunk<B, D, P>>
{
    type Error = Error;

    #[inline]
    fn try_from(orig: &'a str) -> Result<Self, Error> {
        let mut chars = orig.chars();
        let Some(first) = chars.next() else {
            return Ok(Segment::Chunk(Chunk::new("").unwrap()));
        };
        Ok(match D::from_char(first) {
            Some(delim) => match chars.as_str().is_empty() {
                true => Segment::Delim(delim),
                false => {
                    return Err(Error::new(ErrorKind::InvalidDelimiter)
                        .with_byte_offset(delim.as_char().len_utf8()));
                }
            },
            None => Segment::Chunk(Chunk::new(orig)?),
        })
    }
}

// -----------------------------------------------------------------------------
impl<D: core::fmt::Display, C: core::fmt::Display> core::fmt::Display for Segment<D, C> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Delim(delim) => delim.fmt(f),
            Self::Chunk(chunk) => chunk.fmt(f),
        }
    }
}
