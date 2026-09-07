// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::fmt::delimited::Writer;
use crate::core::fragment::StrSegments;
use crate::syntax::{Boundary, Delimiter, Profile};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) struct Decorated<'a, const UPPER: bool> {
    default_delim: char,
    format: fn(&Self, &mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    ident: &'a str,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, const UPPER: bool> Decorated<'a, UPPER> {
    fn format<B: Boundary, D: Delimiter, P: Profile>(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let mut writer = Writer::<UPPER>::new(f);
        writer.write_decorated(
            &mut StrSegments::<B, D, P::Segmentation>::new(self.ident),
            self.default_delim,
        )
    }
    pub(crate) fn new<B: Boundary, D: Delimiter, P: Profile>(
        ident: &'a str,
        default_delim: char,
    ) -> Self {
        Self {
            default_delim,
            format: Self::format::<B, D, P>,
            ident,
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, const UPPER: bool> core::fmt::Display for Decorated<'a, UPPER> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (self.format)(self, f)
    }
}
