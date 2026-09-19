// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Segment;
use crate::core::fmt::delimited::Writer;
use crate::core::fragment::StrSegments;
use crate::syntax::{Boundary, Delimiter, Profile};

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
    fn format<B: Boundary, D: Delimiter, P: Profile>(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let mut writer = Writer::<UPPER>::new(f);
        writer.write_canonical::<P::CharProfile>(
            &mut StrSegments::<B, D, P::Segmentation>::new(self.ident).filter_map(|s| match s {
                Segment::Chunk(c) => Some(c),
                _ => None,
            }),
            self.default_delim,
            self.validate_start,
        )
    }
    pub(crate) fn new<B: Boundary, D: Delimiter, P: Profile>(
        ident: &'a str,
        default_delim: char,
        validate_start: bool,
    ) -> Self {
        Self {
            default_delim,
            format: Self::format::<B, D, P>,
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
