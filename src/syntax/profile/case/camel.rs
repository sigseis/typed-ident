// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::DependentStart;
use crate::syntax::profile::{AppendClosed, CasedProfile, CharProfile, Mixed, Profile};
use crate::syntax::{Delimiter, SubsetOf, SyntaxError};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// A case adapter for a character profile that accepts *either* [`LowerCamel`]
/// or [`UpperCamel`] (but not a mixture of both, like [`Mixed`] would).
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
/// [`LowerCamel`]: crate::syntax::profile::case::LowerCamel
/// [`UpperCamel`]: crate::syntax::profile::case::UpperCamel
pub struct Camel<P>(PhantomData<P>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<P: CharProfile> Profile for Camel<P> {
    /// `Camel` cannot be append-closed for >=`Fragment`.
    ///
    /// While it seems like it could be, this cased profile has additional
    /// checks on the input to ensure the start char of every chunk agrees on a
    /// single casing (or is uncased).
    ///
    /// This means that, though unobvious from this implementation,
    /// `is_chunk_start` is *NOT* the same as `is_chunk_continue`, which makes
    /// this type *NOT* `Fragment` append-closed or higher.
    ///
    /// See [`Profile::APPEND_CLOSED`] for details.
    ///
    /// [`Profile::APPEND_CLOSED`]: crate::syntax::profile::Profile::APPEND_CLOSED
    const APPEND_CLOSED: AppendClosed = match P::APPEND_CLOSED.at_least_chunk() {
        true => AppendClosed::Chunk,
        false => AppendClosed::Empty,
    };
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
impl<P: CharProfile> CasedProfile for Camel<P> {
    #[inline(always)]
    fn is_chunk<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        DependentStart::<D, Self>::is_chunk(s)
    }

    #[inline(always)]
    fn is_fragment<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        DependentStart::<D, Self>::is_fragment(s)
    }

    #[inline(always)]
    fn is_ident<D: Delimiter>(s: &str) -> Result<(), SyntaxError> {
        DependentStart::<D, Self>::is_ident(s)
    }

    #[inline(always)]
    fn is_ident_fragment<D: Delimiter>(fragment: &str) -> Result<(), SyntaxError> {
        DependentStart::<D, Self>::is_ident_fragment(fragment)
    }
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Camel<Subset>` ⊆ `Superset`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Superset> for Camel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Camel<Subset>` ⊆ `Mixed<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Mixed<Superset>> for Camel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `Camel<Subset>` ⊆ `Camel<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Camel<Superset>> for Camel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
