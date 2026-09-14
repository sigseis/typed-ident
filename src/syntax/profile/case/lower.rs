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
use crate::syntax::profile::casing::Independent;
use crate::syntax::profile::{
    AppendClosed, Camel, CasedProfile, CharProfile, LowerCamel, Mixed, Profile, Uniform,
};
use crate::syntax::{CharCase, Delimiter, SubsetOf, SyntaxError};
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
    fn is_chunk_char(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_chunk_char(c)
    }
    #[inline]
    fn is_chunk_continue(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_chunk_continue(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_chunk_start(c)
    }
    #[inline]
    fn is_ident_start_char(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_ident_start_char(c)
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile> CasedProfile for Lower<P> {
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

    #[inline(always)]
    fn is_ident_split_boundary<D: Delimiter>(_ident: &str, _mid: usize) -> bool {
        true
    }
}

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

/// Proof: If `Super` ⊆ `Subset`, then `Mixed<Super>` ⊆ `Lower<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Lower<Ascii>: SubsetOf<Mixed<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Lower, Mixed, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Lower<Ascii>, Mixed<Ascii>>();
/// left_is_subset_of_right::<Lower<Unicode>, Mixed<Unicode>>();
/// left_is_subset_of_right::<Lower<Ascii>, Mixed<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Mixed<Superset>> for Lower<Subset>
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

/// Proof: If `Super` ⊆ `Subset`, then `Camel<Super>` ⊆ `Lower<Subset>`
/// (because `Camel` ⊆ `Lower`).
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Lower<Ascii>: SubsetOf<Camel<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Lower, Camel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Lower<Ascii>, Camel<Ascii>>();
/// left_is_subset_of_right::<Lower<Unicode>, Camel<Unicode>>();
/// left_is_subset_of_right::<Lower<Ascii>, Camel<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Camel<Superset>> for Lower<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `Uniform<Super>` ⊆ `Lower<Subset>`
/// (because `Uniform` ⊆ `Lower`).
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Lower<Ascii>: SubsetOf<Uniform<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Lower, Uniform, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Lower<Ascii>, Uniform<Ascii>>();
/// left_is_subset_of_right::<Lower<Unicode>, Uniform<Unicode>>();
/// left_is_subset_of_right::<Lower<Ascii>, Uniform<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Uniform<Superset>> for Lower<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
