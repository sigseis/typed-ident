// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::intermediate::IntoIntermediate;
use crate::core::{Error, Fragment, Segment};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub enum SegmentIntermediate<D, C> {
    Chunk(C),
    Delim(D),
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B, D, P, SC, SD> AsRef<Fragment<B, D, P>> for SegmentIntermediate<SD, SC>
where
    SC: AsRef<Fragment<B, D, P>>,
    SD: AsRef<Fragment<B, D, P>>,
{
    #[inline(always)]
    fn as_ref(&self) -> &Fragment<B, D, P> {
        match self {
            Self::Chunk(chunk) => chunk.as_ref(),
            Self::Delim(delim) => delim.as_ref(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<SC, SD> AsRef<str> for SegmentIntermediate<SD, SC>
where
    SC: AsRef<str>,
    SD: AsRef<str>,
{
    #[inline(always)]
    fn as_ref(&self) -> &str {
        match self {
            Self::Chunk(chunk) => chunk.as_ref(),
            Self::Delim(delim) => delim.as_ref(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<SC, SD> core::fmt::Display for SegmentIntermediate<SD, SC>
where
    SC: AsRef<str>,
    SD: AsRef<str>,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s: &str = self.as_ref();
        s.fmt(f)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P, SC, SD> IntoIntermediate<B, D, P> for Segment<SD, SC>
where
    SC: IntoIntermediate<B, D, P>,
    SD: IntoIntermediate<B, D, P>,
{
    type Intermediate = SegmentIntermediate<SD::Intermediate, SC::Intermediate>;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        Ok(match self {
            Self::Chunk(chunk) => SegmentIntermediate::Chunk(chunk.into_intermediate()?),
            Self::Delim(delim) => SegmentIntermediate::Delim(delim.into_intermediate()?),
        })
    }
}

// -----------------------------------------------------------------------------
impl<'a, B, D, P, SC, SD> IntoIntermediate<B, D, P> for &'a Segment<SD, SC>
where
    &'a SC: IntoIntermediate<B, D, P>,
    SD: IntoIntermediate<B, D, P> + Copy,
{
    type Intermediate =
        SegmentIntermediate<SD::Intermediate, <&'a SC as IntoIntermediate<B, D, P>>::Intermediate>;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        Ok(match self {
            Segment::Chunk(chunk) => SegmentIntermediate::Chunk(chunk.into_intermediate()?),
            Segment::Delim(delim) => SegmentIntermediate::Delim(delim.into_intermediate()?),
        })
    }
}
