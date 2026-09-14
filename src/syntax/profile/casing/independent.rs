// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::{IndependentCasing, Validator};
use crate::syntax::{CasedProfile, Delimiter, SyntaxError};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An identifier profile where the validity of the a character is measured by
/// independent of other characters in the identifier.
///
/// For example, if the only measure of validity is whether or not the
/// characters are in-profile in their respective valid positions, then it's
/// said to be an independent profile.
///
/// All specifically-cased identifiers are independent; `UPPER_SNAKE`,
/// `lower-kebab`, `UpperCamel`, `lowerHybrid`, etc. This is because the
/// profiles in question simply encode the casing restrictions directly within
/// the character profile.
pub(crate) struct Independent<D, P>(PhantomData<(D, P)>);

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D: Delimiter, P: CasedProfile> Independent<D, P> {
    #[inline(always)]
    pub fn is_chunk(s: &str) -> Result<(), SyntaxError> {
        Validator::<IndependentCasing, D, P>::is_chunk(s)
    }
    #[inline(always)]
    pub fn is_fragment(s: &str) -> Result<(), SyntaxError> {
        Validator::<IndependentCasing, D, P>::is_fragment(s)
    }
    #[inline(always)]
    pub fn is_ident(s: &str) -> Result<(), SyntaxError> {
        Validator::<IndependentCasing, D, P>::is_ident(s)
    }
    #[inline(always)]
    pub fn is_ident_fragment(fragment: &str) -> Result<(), SyntaxError> {
        Validator::<IndependentCasing, D, P>::is_ident_fragment(fragment)
    }
}
