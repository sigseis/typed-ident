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
/// [`Lower`]: crate::syntax::profile::case::Lower
/// [`Upper`]: crate::syntax::profile::case::Upper
pub struct Uniform<P>(PhantomData<P>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<P: CharProfile> Profile for Uniform<P> {
    const APPEND_CLOSED: AppendClosed = P::APPEND_CLOSED;
    type BaseProfile = P;
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

/// Proof: If `Super` ⊆ `Subset`, then `Super` ⊆ `Uniform<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Uniform<Ascii>: SubsetOf<Unicode>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Uniform, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Uniform<Ascii>, Ascii>();
/// left_is_subset_of_right::<Uniform<Unicode>, Unicode>();
/// left_is_subset_of_right::<Uniform<Ascii>, Unicode>();
/// ```
impl<Superset, Subset> SubsetOf<Superset> for Uniform<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `Mixed<Super>` ⊆ `Uniform<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Uniform<Ascii>: SubsetOf<Unicode>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Mixed, Uniform, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Uniform<Ascii>, Mixed<Ascii>>();
/// left_is_subset_of_right::<Uniform<Unicode>, Mixed<Unicode>>();
/// left_is_subset_of_right::<Uniform<Ascii>, Mixed<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Mixed<Superset>> for Uniform<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `Uniform<Super>` ⊆ `Uniform<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Uniform<Ascii>: SubsetOf<Uniform<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Uniform, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Uniform<Ascii>, Uniform<Ascii>>();
/// left_is_subset_of_right::<Uniform<Unicode>, Uniform<Unicode>>();
/// left_is_subset_of_right::<Uniform<Ascii>, Uniform<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Uniform<Superset>> for Uniform<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `Camel<Super>` ⊆ `Uniform<Subset>`
/// (because `Camel` ⊆ `Uniform`).
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Uniform<Ascii>: SubsetOf<Camel<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Uniform, Camel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Uniform<Ascii>, Camel<Ascii>>();
/// left_is_subset_of_right::<Uniform<Unicode>, Camel<Unicode>>();
/// left_is_subset_of_right::<Uniform<Ascii>, Camel<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Camel<Superset>> for Uniform<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
