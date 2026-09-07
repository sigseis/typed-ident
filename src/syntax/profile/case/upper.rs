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
use crate::syntax::profile::case::CasedProfile;
use crate::syntax::profile::{AppendClosed, CharProfile, Profile, UpperCamel};
use crate::syntax::{CharCase, SubsetOf};
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
    type BaseProfile = P;
    type Segmentation = P::Segmentation;

    #[inline]
    fn is_ident_start(c: char) -> bool {
        CharCase::is_uppercase_compatible(c) && P::is_ident_start(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        CharCase::is_uppercase_compatible(c) && P::is_chunk_start(c)
    }
    #[inline]
    fn in_profile(c: char) -> bool {
        CharCase::is_uppercase_compatible(c) && P::in_profile(c)
    }
    #[inline]
    fn is_chunk_continue(c: char) -> bool {
        CharCase::is_uppercase_compatible(c) && P::is_chunk_continue(c)
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile> CasedProfile for Upper<P> {}

/// Proof: If `Super` ⊆ `Subset`, then `Super` ⊆ `Upper<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Upper<Ascii>: SubsetOf<Unicode>`
///
/// # Examples
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Upper, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Upper<Ascii>, Ascii>();
/// left_is_subset_of_right::<Upper<Unicode>, Unicode>();
/// left_is_subset_of_right::<Upper<Ascii>, Unicode>();
/// ```
impl<Superset, Subset> SubsetOf<Superset> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: if `Super` ⊆ `Subset`, then `Upper<Super>` ⊆ `Upper<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Upper<Ascii>: SubsetOf<Upper<Unicode>>`
///
/// # Examples
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Upper, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Upper<Ascii>, Upper<Ascii>>();
/// left_is_subset_of_right::<Upper<Unicode>, Upper<Unicode>>();
/// left_is_subset_of_right::<Upper<Ascii>, Upper<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Upper<Superset>> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `UpperCamel<Super>` ⊆ `Upper<Subset>`
/// (because `UpperCamel` ⊆ `Upper`).
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Upper<Ascii>: SubsetOf<UpperCamel<Unicode>>`
///
/// # Examples
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Upper, UpperCamel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Upper<Ascii>, UpperCamel<Ascii>>();
/// left_is_subset_of_right::<Upper<Unicode>, UpperCamel<Unicode>>();
/// left_is_subset_of_right::<Upper<Ascii>, UpperCamel<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<UpperCamel<Superset>> for Upper<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
