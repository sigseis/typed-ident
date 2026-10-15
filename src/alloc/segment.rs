// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Segment;
use std_alloc::borrow::ToOwned;

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D: ToOwned, C: ToOwned + ?Sized> Segment<D, &C> {
    /// Converts a segment into an owned representation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::camel::*;
    /// let segment = CamelSegment::Delim(LowLine).to_owned();
    /// assert_eq!(segment, OwnedCamelSegment::Delim(LowLine));
    ///
    /// let segment = CamelSegment::Chunk(CamelChunk::new("Upper")?).to_owned();
    /// assert_eq!(segment, OwnedCamelSegment::Chunk(CamelChunk::new_boxed("Upper")?));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn to_owned(&self) -> Segment<D::Owned, C::Owned> {
        match self {
            Segment::Chunk(chunk) => Segment::Chunk((*chunk).to_owned()),
            Segment::Delim(delim) => Segment::Delim(delim.to_owned()),
        }
    }
}
