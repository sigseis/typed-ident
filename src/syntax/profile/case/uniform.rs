// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::Dependent;
use crate::syntax::profile::{AppendClosed, Camel, CasedProfile, CharProfile, Mixed, Profile};
use crate::syntax::{CharCase, Delimiter, SubsetOf, SyntaxError};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// A case adapter for a character profile that accepts *either* [`Lower`] or
/// or [`Upper`] (but not a mixture of both, like [`Mixed`] would).
///
/// # Caution
///
/// Unlike many other profiles, this profile presents an interdependent casing
/// requirement (e.g. the case of the other restricted characters impact the
/// validity of each other).
///
/// Because of this, just plainly calling the functions of this [`Profile`] may
/// be more permissive than actually calling the [`is_ident`], [`is_fragment`],
/// or [`is_ident_fragment`] functions.
///
/// [`is_ident`]: CasedProfile::is_ident
/// [`is_fragment`]: CasedProfile::is_fragment
/// [`is_ident_fragment`]: CasedProfile::is_ident_fragment
///
/// [`Lower`]: crate::syntax::profile::case::Lower
/// [`Upper`]: crate::syntax::profile::case::Upper
pub struct Uniform<P>(PhantomData<P>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<P: CharProfile> Profile for Uniform<P> {
    const APPEND_CLOSED: AppendClosed = P::APPEND_CLOSED;
    type CharProfile = P;
    type Segmentation = P::Segmentation;

    #[inline]
    fn is_chunk_char(c: char) -> bool {
        CharCase::is_uniform_compatible(c) && P::is_chunk_char(c)
    }
    #[inline]
    fn is_chunk_continue(c: char) -> bool {
        CharCase::is_uniform_compatible(c) && P::is_chunk_continue(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        CharCase::is_uniform_compatible(c) && P::is_chunk_start(c)
    }
    #[inline]
    fn is_ident_start_char(c: char) -> bool {
        CharCase::is_uniform_compatible(c) && P::is_ident_start_char(c)
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile> CasedProfile for Uniform<P> {
    #[inline(always)]
    fn is_chunk<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        Dependent::<D, Self>::is_chunk(s)
    }

    #[inline(always)]
    fn is_fragment<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        Dependent::<D, Self>::is_fragment(s)
    }

    #[inline(always)]
    fn is_ident<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        Dependent::<D, Self>::is_ident(s)
    }

    #[inline(always)]
    fn is_ident_fragment<D: Delimiter>(fragment: &str) -> Result<(), SyntaxError> {
        Dependent::<D, Self>::is_ident_fragment(fragment)
    }
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Uniform<Subset>` ⊆ `Superset`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Superset> for Uniform<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Uniform<Subset>` ⊆ `Mixed<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Mixed<Superset>> for Uniform<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Uniform<Subset>` ⊆ `Uniform<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Uniform<Superset>> for Uniform<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Uniform<Subset>` ⊆ `Camel<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Camel<Superset>> for Uniform<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
