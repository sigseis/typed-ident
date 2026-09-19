// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Segment;
use crate::core::fmt::bounded::Writer;
use crate::core::fragment::StrSegments;
use crate::syntax::{Boundary, Delimiter, Profile, TrivialBoundary};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) struct Canonical<'a, const UPPER: bool> {
    default_delim: char,
    format: fn(&Self, &mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    ident: &'a str,
    validate_start: bool,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, const UPPER: bool> Canonical<'a, UPPER> {
    fn format<B: Boundary, D: Delimiter, P: Profile, T: TrivialBoundary>(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let writer = Writer::<UPPER>::new(f);
        writer.write_canonical::<T, P::CharProfile>(
            &mut StrSegments::<B, D, P::Segmentation>::new(self.ident).filter_map(|s| match s {
                Segment::Chunk(c) => Some(c),
                _ => None,
            }),
            self.default_delim,
            self.validate_start,
        )
    }
    pub(crate) fn new<B: Boundary, D: Delimiter, P: Profile, T: TrivialBoundary>(
        ident: &'a str,
        default_delim: char,
        validate_start: bool,
    ) -> Self {
        Self {
            default_delim,
            format: Self::format::<B, D, P, T>,
            ident,
            validate_start,
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, const UPPER: bool> core::fmt::Display for Canonical<'a, UPPER> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (self.format)(self, f)
    }
}
