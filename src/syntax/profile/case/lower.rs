// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "lower.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::case::CasedProfile;
use crate::syntax::profile::{AppendClosed, CharProfile, LowerCamel, Profile};
use crate::syntax::{CharCase, SubsetOf};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// A case adapter for a character profile that accepts uncased and lowercase
/// characters.
pub struct Lower<P>(PhantomData<P>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<P: CharProfile> Profile for Lower<P> {
    const APPEND_CLOSED: AppendClosed = P::APPEND_CLOSED;
    type BaseProfile = P;
    type Segmentation = P::Segmentation;

    #[inline]
    fn is_ident_start(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_ident_start(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_chunk_start(c)
    }
    #[inline]
    fn in_profile(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::in_profile(c)
    }
    #[inline]
    fn is_chunk_continue(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_chunk_continue(c)
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile> CasedProfile for Lower<P> {}

/// Proof: If `Super` ⊆ `Subset`, then `Super` ⊆ `Lower<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Lower<Ascii>: SubsetOf<Unicode>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Lower, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Lower<Ascii>, Ascii>();
/// left_is_subset_of_right::<Lower<Unicode>, Unicode>();
/// left_is_subset_of_right::<Lower<Ascii>, Unicode>();
/// ```
impl<Superset, Subset> SubsetOf<Superset> for Lower<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `Lower<Super>` ⊆ `Lower<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Lower<Ascii>: SubsetOf<Lower<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Lower, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Lower<Ascii>, Lower<Ascii>>();
/// left_is_subset_of_right::<Lower<Unicode>, Lower<Unicode>>();
/// left_is_subset_of_right::<Lower<Ascii>, Lower<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Lower<Superset>> for Lower<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `LowerCamel<Super>` ⊆ `Lower<Subset>`
/// (because `LowerCamel` ⊆ `Lower`).
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Lower<Ascii>: SubsetOf<LowerCamel<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Lower, LowerCamel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Lower<Ascii>, LowerCamel<Ascii>>();
/// left_is_subset_of_right::<Lower<Unicode>, LowerCamel<Unicode>>();
/// left_is_subset_of_right::<Lower<Ascii>, LowerCamel<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<LowerCamel<Superset>> for Lower<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
