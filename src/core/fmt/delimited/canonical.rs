// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Identifier;
use crate::fmt::delimited::Writer;
use crate::fragment::StrWords;
use crate::syntax::{Boundary, CharProfile, Delimiter, Profile};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) struct Canonical<'a, const UPPER: bool> {
    default_delim: char,
    format: fn(&Self, &mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    ident: &'a str,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, const UPPER: bool> Canonical<'a, UPPER> {
    fn format<B: Boundary, D: Delimiter, P: CharProfile>(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let mut writer = Writer::<UPPER>::new(f);
        writer.write_canonical::<P>(
            &mut StrWords::<B, D, P::Segmentation>::new(self.ident),
            self.default_delim,
        )
    }
    pub(crate) fn new<I: Identifier + ?Sized>(ident: &'a str, default_delim: char) -> Self {
        Self {
            default_delim,
            format: Self::format::<I::Boundary, I::Delimiter, <I::Profile as Profile>::CharProfile>,
            ident,
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
