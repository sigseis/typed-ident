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
    type BaseProfile = P;
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

    #[inline(always)]
    fn is_ident_split_boundary<D: Delimiter>(_ident: &str, _mid: usize) -> bool {
        true
    }
}

/// Proof: If `Super` ⊆ `Subset`, then `Super` ⊆ `Camel<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Camel<Ascii>: SubsetOf<Unicode>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Camel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Camel<Ascii>, Ascii>();
/// left_is_subset_of_right::<Camel<Unicode>, Unicode>();
/// left_is_subset_of_right::<Camel<Ascii>, Unicode>();
/// ```
impl<Superset, Subset> SubsetOf<Superset> for Camel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `Mixed<Super>` ⊆ `Camel<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Camel<Ascii>: SubsetOf<Mixed<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Camel, Mixed, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Camel<Ascii>, Mixed<Ascii>>();
/// left_is_subset_of_right::<Camel<Unicode>, Mixed<Unicode>>();
/// left_is_subset_of_right::<Camel<Ascii>, Mixed<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Mixed<Superset>> for Camel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `Camel<Super>` ⊆ `Camel<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `Camel<Ascii>: SubsetOf<Camel<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, Camel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<Camel<Ascii>, Camel<Ascii>>();
/// left_is_subset_of_right::<Camel<Unicode>, Camel<Unicode>>();
/// left_is_subset_of_right::<Camel<Ascii>, Camel<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<Camel<Superset>> for Camel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
