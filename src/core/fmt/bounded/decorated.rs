// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::fmt::bounded::Writer;
use crate::core::fragment::StrSegments;
use crate::syntax::{Boundary, Delimiter, Profile, TrivialBoundary};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) struct Decorated<'a, const UPPER: bool> {
    alternative_delim: Option<char>,
    default_delim: char,
    format: fn(&Self, &mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    ident: &'a str,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, const UPPER: bool> Decorated<'a, UPPER> {
    fn format<B: Boundary, D: Delimiter, P: Profile, T: TrivialBoundary>(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let writer = Writer::<UPPER>::new(f);
        writer.write_decorated::<T, P::BaseProfile>(
            &mut StrSegments::<B, D, P::Segmentation>::new(self.ident),
            self.default_delim,
            self.alternative_delim,
        )
    }
    pub(crate) fn new<B: Boundary, D: Delimiter, P: Profile, T: TrivialBoundary>(
        ident: &'a str,
        default_delim: char,
        alternative_delim: Option<char>,
    ) -> Self {
        Self {
            alternative_delim,
            default_delim,
            format: Self::format::<B, D, P, T>,
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
