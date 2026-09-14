// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::{DependentCasing, Validator};
use crate::syntax::{CasedProfile, Delimiter, SyntaxError};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An identifier profile where the validity of the profile is measured by
/// characters whose characters are dependent on one-another.
///
/// For example, if all characters must have the same case, then it's said to be
/// a dependent profile (this is used by `kebab-case` and `snake_case`).
pub(crate) struct Dependent<D, P>(PhantomData<(D, P)>);

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D: Delimiter, P: CasedProfile> Dependent<D, P> {
    #[inline(always)]
    pub fn is_chunk(s: &str) -> Result<(), SyntaxError> {
        Validator::<DependentCasing, D, P>::is_chunk(s)
    }
    #[inline(always)]
    pub fn is_fragment(s: &str) -> Result<(), SyntaxError> {
        Validator::<DependentCasing, D, P>::is_fragment(s)
    }
    #[inline(always)]
    pub fn is_ident(s: &str) -> Result<(), SyntaxError> {
        Validator::<DependentCasing, D, P>::is_ident(s)
    }
    #[inline(always)]
    pub fn is_ident_fragment(fragment: &str) -> Result<(), SyntaxError> {
        Validator::<DependentCasing, D, P>::is_ident_fragment(fragment)
    }
}
