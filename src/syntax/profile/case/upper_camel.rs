// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "upper_camel.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::case::CasedProfile;
use crate::syntax::profile::{AppendClosed, CharProfile, Profile};
use crate::syntax::{CharCase, SubsetOf};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// A case adapter for a character profile that accepts uncased, uppercase, and
/// titlecase characters on the first character of a chunk or identifier.
///
/// # Note
///
/// If you're using the regular unicode profile, it's possible that you can
/// construct valid strings which appear to break the requirements of this type.
///
/// ```
/// use typed_ident::presets::unicode::UpperCamelIdent;
/// assert!(UpperCamelIdent::new("_a").is_err()); // Doesn't work (expected).
/// assert!(UpperCamelIdent::new("_゙a").is_ok()); // Works?
/// ```
///
/// The payload provided for that second case is:
///
/// * `_` (U+005F) LOW LINE
/// * ` ゙` (U+3099) COMBINING KATAKANA-HIRAGANA VOICED SOUND MARK
/// * `a` (U+0061) LATIN SMALL LETTER A
///
/// The way to prevent this is to disallow Mn/Mc/Me characters on chunk start.
///
/// You can get a profile which does this by enabling `unicode-strict`. This
/// will enable the [`Strict`] profile which can be used directly - or if you
/// have presets enabled - used indirectly through [`strict`] presets.
///
/// ```
/// use typed_ident::presets::strict::UpperCamelIdent;
/// assert!(UpperCamelIdent::new("_a").is_err()); // Doesn't work (expected).
/// assert!(UpperCamelIdent::new("_゙a").is_err()); // Now it doesn't work!
/// ```
///
/// See [`Strict`] for more details on when you should do this.
///
/// [`Strict`]: crate::syntax::profile::chars::Strict
/// [`strict`]: crate::presets::strict
pub struct UpperCamel<P>(PhantomData<P>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<P: CharProfile> Profile for UpperCamel<P> {
    /// `UpperCamel` cannot be append-closed for >=`Fragment`, because
    /// `is_chunk_start` is not identical to `is_chunk_continue`.
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
    fn is_ident_start(c: char) -> bool {
        CharCase::is_uppercase_start_compatible(c) && P::is_ident_start(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        CharCase::is_uppercase_start_compatible(c) && P::is_chunk_start(c)
    }
    #[inline(always)]
    fn in_profile(c: char) -> bool {
        P::in_profile(c)
    }
    #[inline(always)]
    fn is_chunk_continue(c: char) -> bool {
        P::is_chunk_continue(c)
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile> CasedProfile for UpperCamel<P> {}

/// Proof: If `Super` ⊆ `Subset`, then `Super` ⊆ `UpperCamel<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `UpperCamel<Ascii>: SubsetOf<Unicode>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, UpperCamel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<UpperCamel<Ascii>, Ascii>();
/// left_is_subset_of_right::<UpperCamel<Unicode>, Unicode>();
/// left_is_subset_of_right::<UpperCamel<Ascii>, Unicode>();
/// ```
impl<Superset, Subset> SubsetOf<Superset> for UpperCamel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `UpperCamel<Super>` ⊆ `UpperCamel<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `UpperCamel<Ascii>: SubsetOf<UpperCamel<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, UpperCamel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<UpperCamel<Ascii>, UpperCamel<Ascii>>();
/// left_is_subset_of_right::<UpperCamel<Unicode>, UpperCamel<Unicode>>();
/// left_is_subset_of_right::<UpperCamel<Ascii>, UpperCamel<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<UpperCamel<Superset>> for UpperCamel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
