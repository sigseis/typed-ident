// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "lower_camel.tests.rs"]
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

/// A case adapter for a character profile that accepts uncased and lowercase
/// characters on the first character of a chunk or identifier.
///
/// # Note
///
/// If you're using the regular unicode profile, it's possible that you can
/// construct valid strings which appear to break the requirements of this type.
///
/// ```
/// use typed_ident::presets::unicode::LowerCamelIdent;
/// assert!(LowerCamelIdent::new("_A").is_err()); // Doesn't work (expected).
/// assert!(LowerCamelIdent::new("_゙A").is_ok()); // Works?
/// ```
///
/// The payload provided for that second case is:
///
/// * `_` (U+005F) LOW LINE
/// * ` ゙` (U+3099) COMBINING KATAKANA-HIRAGANA VOICED SOUND MARK
/// * `A` (U+0041) LATIN CAPITAL LETTER A
///
/// The way to prevent this is to disallow Mn/Mc/Me characters on chunk start.
///
/// You can get a profile which does this by enabling `unicode-strict`. This
/// will enable the [`Strict`] profile which can be used directly - or if you
/// have presets enabled - used indirectly through [`strict`] presets.
///
/// ```
/// use typed_ident::presets::strict::LowerCamelIdent;
/// assert!(LowerCamelIdent::new("_A").is_err()); // Doesn't work (expected).
/// assert!(LowerCamelIdent::new("_゙A").is_err()); // Now it doesn't work!
/// ```
///
/// See [`Strict`] for more details on when you should do this.
///
/// [`Strict`]: crate::syntax::profile::chars::Strict
/// [`strict`]: crate::presets::strict
pub struct LowerCamel<P>(PhantomData<P>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<P: CharProfile> Profile for LowerCamel<P> {
    /// `LowerCamel` cannot be append-closed for >=`Fragment`, because
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
        CharCase::is_lowercase_compatible(c) && P::is_ident_start(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_chunk_start(c)
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
impl<P: CharProfile> CasedProfile for LowerCamel<P> {}

/// Proof: If `Super` ⊆ `Subset`, then `Super` ⊆ `LowerCamel<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `LowerCamel<Ascii>: SubsetOf<Unicode>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, LowerCamel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<LowerCamel<Ascii>, Ascii>();
/// left_is_subset_of_right::<LowerCamel<Unicode>, Unicode>();
/// left_is_subset_of_right::<LowerCamel<Ascii>, Unicode>();
/// ```
impl<Superset, Subset> SubsetOf<Superset> for LowerCamel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

/// Proof: If `Super` ⊆ `Subset`, then `LowerCamel<Super>` ⊆ `LowerCamel<Subset>`.
///
/// Let's pretend that `Superset=Unicode`, and `Subset=Ascii`.
///
/// If `Ascii: SubsetOf<Unicode>` (true), then this implies the following:
///
/// * `LowerCamel<Ascii>: SubsetOf<LowerCamel<Unicode>>`
///
/// # Proof
///
/// ```
/// # use typed_ident::syntax::SubsetOf;
/// # use typed_ident::syntax::profile::{Ascii, LowerCamel, Unicode};
/// # fn left_is_subset_of_right<Sub, Super>() where Sub: SubsetOf<Super> {}
/// left_is_subset_of_right::<LowerCamel<Ascii>, LowerCamel<Ascii>>();
/// left_is_subset_of_right::<LowerCamel<Unicode>, LowerCamel<Unicode>>();
/// left_is_subset_of_right::<LowerCamel<Ascii>, LowerCamel<Unicode>>();
/// ```
impl<Superset, Subset> SubsetOf<LowerCamel<Superset>> for LowerCamel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
