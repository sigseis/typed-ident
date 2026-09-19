// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "upper.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::Independent;
use crate::syntax::profile::{
    AppendClosed, Camel, CasedProfile, CharProfile, Mixed, Profile, Uniform, UpperCamel,
};
use crate::syntax::{CharCase, Delimiter, SubsetOf, SyntaxError};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// A case adapter for a character profile that accepts uncased, uppercase, and
/// titlecase characters.
pub struct Upper<P>(PhantomData<P>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<P: CharProfile> Profile for Upper<P> {
    const APPEND_CLOSED: AppendClosed = P::APPEND_CLOSED;
    type CharProfile = P;
    type Segmentation = P::Segmentation;

    #[inline]
    fn is_chunk_char(c: char) -> bool {
        CharCase::is_uppercase_compatible(c) && P::is_chunk_char(c)
    }
    #[inline]
    fn is_chunk_continue(c: char) -> bool {
        CharCase::is_uppercase_compatible(c) && P::is_chunk_continue(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        CharCase::is_uppercase_compatible(c) && P::is_chunk_start(c)
    }
    #[inline]
    fn is_ident_start_char(c: char) -> bool {
        CharCase::is_uppercase_compatible(c) && P::is_ident_start_char(c)
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile> CasedProfile for Upper<P> {
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
/// Proof: If `Subset` ⊆ `Superset`, then `Upper<Subset>` ⊆ `Superset`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Superset> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Upper<Subset>` ⊆ `Mixed<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Mixed<Superset>> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Upper<Subset>` ⊆ `Upper<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Upper<Superset>> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Upper<Subset>` ⊆ `Camel<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<UpperCamel<Superset>> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Upper<Subset>` ⊆ `UpperCamel<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Camel<Superset>> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Upper<Subset>` ⊆ `Uniform<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Uniform<Superset>> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
