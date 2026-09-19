// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::Independent;
use crate::syntax::profile::{AppendClosed, CasedProfile, CharProfile, Profile};
use crate::syntax::{Delimiter, SubsetOf, SyntaxError};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// A case adapter for a character profile which makes no further restrictions
/// based on casing.
pub struct Mixed<P>(PhantomData<P>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<P: CharProfile> Profile for Mixed<P> {
    const APPEND_CLOSED: AppendClosed = P::APPEND_CLOSED;
    type CharProfile = P;
    type Segmentation = P::Segmentation;

    #[inline]
    fn is_chunk_char(c: char) -> bool {
        P::is_chunk_char(c)
    }
    #[inline]
    fn is_chunk_continue(c: char) -> bool {
        P::is_chunk_continue(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        P::is_chunk_start(c)
    }
    #[inline]
    fn is_ident_start_char(c: char) -> bool {
        P::is_ident_start_char(c)
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile> CasedProfile for Mixed<P> {
    #[inline(always)]
    fn is_chunk<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        Independent::<D, Self>::is_chunk(s)
    }

    #[inline(always)]
    fn is_fragment<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        Independent::<D, Self>::is_fragment(s)
    }

    #[inline(always)]
    fn is_ident<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        Independent::<D, Self>::is_ident(s)
    }

    #[inline(always)]
    fn is_ident_fragment<D: Delimiter>(fragment: &str) -> Result<(), SyntaxError> {
        Independent::<D, Self>::is_ident_fragment(fragment)
    }
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Mixed<Subset>` ⊆ `Superset`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Superset> for Mixed<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Mixed<Subset>` ⊆ `Mixed<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Mixed<Superset>> for Mixed<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
