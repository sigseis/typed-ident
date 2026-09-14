// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::{DependentStartCasing, Validator};
use crate::syntax::{CasedProfile, Delimiter, SyntaxError};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An identifier profile where the validity of the profile is measured by
/// characters whose chunk-start characters are dependent on one-another.
///
/// For example, if all chunk start characters must have the same case, then
/// it's said to be a dependent-start identifier (this is used by `camelCase`
/// and `hybridCase`).
pub(crate) struct DependentStart<D, P>(PhantomData<(D, P)>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D: Delimiter, P: CasedProfile> DependentStart<D, P> {
    #[inline(always)]
    pub fn is_chunk(s: &str) -> Result<(), SyntaxError> {
        Validator::<DependentStartCasing, D, P>::is_chunk(s)
    }
    #[inline(always)]
    pub fn is_fragment(s: &str) -> Result<(), SyntaxError> {
        Validator::<DependentStartCasing, D, P>::is_fragment(s)
    }
    #[inline(always)]
    pub fn is_ident(s: &str) -> Result<(), SyntaxError> {
        Validator::<DependentStartCasing, D, P>::is_ident(s)
    }
    #[inline(always)]
    pub fn is_ident_fragment(fragment: &str) -> Result<(), SyntaxError> {
        Validator::<DependentStartCasing, D, P>::is_ident_fragment(fragment)
    }
}
