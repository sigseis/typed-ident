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
use crate::syntax::profile::casing::Independent;
use crate::syntax::profile::{AppendClosed, Camel, CasedProfile, CharProfile, Mixed, Profile};
use crate::syntax::{CharCase, Delimiter, SubsetOf, SyntaxError};
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
/// The [`Strict`] profile disallows such characters on chunk start, which can
/// be used either directly or through the [`strict`] presets.
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
    type CharProfile = P;
    type Segmentation = P::Segmentation;

    #[inline(always)]
    fn is_chunk_char(c: char) -> bool {
        P::is_chunk_char(c)
    }
    #[inline]
    fn is_chunk_start(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_chunk_start(c)
    }
    #[inline(always)]
    fn is_chunk_continue(c: char) -> bool {
        P::is_chunk_continue(c)
    }
    #[inline]
    fn is_ident_start_char(c: char) -> bool {
        CharCase::is_lowercase_compatible(c) && P::is_ident_start_char(c)
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile> CasedProfile for LowerCamel<P> {
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
/// Proof: If `Subset` ⊆ `Superset`, then `LowerCamel<Subset>` ⊆ `Superset`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Superset> for LowerCamel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `LowerCamel<Subset>` ⊆ `Mixed<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Mixed<Superset>> for LowerCamel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `LowerCamel<Subset>` ⊆ `LowerCamel<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<LowerCamel<Superset>> for LowerCamel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}

// -----------------------------------------------------------------------------
/// Proof: If `Subset` ⊆ `Superset`, then `LowerCamel<Subset>` ⊆ `Camel<Superset>`
// -----------------------------------------------------------------------------
impl<Superset, Subset> SubsetOf<Camel<Superset>> for LowerCamel<Subset>
where
    Superset: CharProfile,
    Subset: CharProfile + SubsetOf<Superset>,
{
}
