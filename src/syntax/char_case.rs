// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::generated;

// =============================================================================
// TYPES
// =============================================================================

/// Represents the visual case of a single character code point.
///
/// # Titlecase Mapping
///
/// From the perspective of this crate, Greek titlecase characters are
/// equivalent to uppercase characters, and will be mapped to that value.
///
/// This isn't a random decision - it's based on how boundaries are defined in
/// [Unicode Technical Standard #55](https://www.unicode.org/reports/tr55/#Identifier-Chunks).
///
/// In that document, they make special note that some boundaries may form on
/// titlecase characters, specifically whether or not they are non-Greek. The
/// reason for this is because non-Greek characters have a specific visual
/// property about them.
///
/// * **Example Greek Titlecase:** ᾈ, ᾨ, ῌ, ᾚ
/// * **Example Non-Greek Titlecase:** ǅ, ǈ, ǋ, ǲ
///
/// Just looking at these characters, why such boundaries form becomes clear.
///
/// Visually, non-Greek titlecase characters are Latin characters which are a
/// single code point consist of a combination of one uppercase, followed by one
/// lowercase character.
///
/// As such, the decision made is:
///
/// * **Greek titlecase** is equivalent to a fully uppercase code point.
/// * **Non-Greek titlecase** is equivalent to a grapheme containing two code
///   points, the first of which is uppercase, and the remaining lower.
///
/// As such, non-Greek is *NOT* completely uppercase-compatible, and we need to
/// take this special property about such characters into consideration.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum CharCase {
    Uncased = 0,
    Lower = 1,
    Upper = 2,
    TitleNonGreek = 3,
    Digit = 4,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl CharCase {
    /// Whether or not a code point is compatible with something lowercase-ish.
    ///
    /// # Justification
    ///
    /// * **Uncased** - Obviously justifiable. Identifiers should not fail to
    ///   parse based on case formatting because an uncased character is used.
    /// * **Lower** - This is what we're testing for! :)
    /// * **Upper** - This is what we're testing against! >:(
    /// * **Title** - Titlecase consists of digraph characters containing one
    ///   uppercase, and maybe one lowercase character in the same code point.
    ///   The presence of the uppercase character makes these unsuitable for a
    ///   lowercase-ish test.
    #[inline]
    pub fn is_lowercase_compatible(c: char) -> bool {
        !c.is_uppercase() && !Self::is_titlecase(c)
    }

    /// Whether or not a code point is compatible with something uppercase-ish.
    ///
    /// # Justification
    ///
    /// * **Uncased** - Obviously justifiable. Identifiers should not fail to
    ///   parse based on case formatting because an uncased character is used.
    /// * **Lower** - This is what we're testing against! >:(
    /// * **Upper** - This is what we're testing for! :)
    /// * **Title** - Titlecase consists of digraph characters containing one
    ///   uppercase, and maybe one lowercase character in the same code point.
    ///   Not all titlecase characters will be completely uppercase-ish.
    #[inline]
    pub fn is_uppercase_compatible(c: char) -> bool {
        !c.is_lowercase() && !Self::is_non_greek_titlecase(c)
    }

    /// Whether or not a code point starts with an uppercase *at least*.
    ///
    /// # Justification
    ///
    /// * **Uncased** - Obviously justifiable. Identifiers should not fail to
    ///   parse based on case formatting because an uncased character is used.
    /// * **Lower** - This is what we're testing against! >:(
    /// * **Upper** - This is what we're testing for! :)
    /// * **Title** - Titlecase consists of digraph characters containing one
    ///   uppercase, and maybe one lowercase character in the same code point.
    #[inline]
    pub fn is_uppercase_start_compatible(c: char) -> bool {
        !c.is_lowercase()
    }

    /// Whether or not a code point is compatible with something uniform-ish.
    ///
    /// # Justification
    ///
    /// * **Uncased** - Obviously justifiable. Identifiers should not fail to
    ///   parse based on case formatting because an uncased character is used.
    /// * **Lower** - This could be considered a valid uniform character.
    /// * **Upper** - This could be considered a valid uniform character.
    /// * **Title** - Titlecase consists of digraph characters containing one
    ///   uppercase, and maybe one lowercase character in the same code point.
    ///   Not all titlecase characters will be completely uniform-ish.
    #[inline]
    pub fn is_uniform_compatible(c: char) -> bool {
        !Self::is_non_greek_titlecase(c)
    }

    /// Given a character code point, produce a case for that character.
    #[inline]
    pub fn new(c: char) -> Self {
        // TODO: Could be made slightly more efficient when `CharCase` is
        // implemented in the standard.
        //
        // See: https://github.com/rust-lang/rust/issues/153892
        if c.is_ascii_digit() {
            Self::Digit
        } else if c.is_uppercase() {
            Self::Upper
        } else if c.is_lowercase() {
            Self::Lower
        } else if Self::is_titlecase(c) {
            if generated::is_titlecase_greek_variant(c) {
                Self::Upper
            } else {
                Self::TitleNonGreek
            }
        } else {
            Self::Uncased
        }
    }

    /// Given a character code point, tell whether or not it's a cased code.
    #[inline]
    pub fn is_cased(c: char) -> bool {
        Self::new(c) != Self::Uncased
    }

    #[inline]
    fn is_non_greek_titlecase(c: char) -> bool {
        generated::is_titlecase(c) && !generated::is_titlecase_greek_variant(c)
    }

    /// Given a character code point, tell whether or not it's titlecase.
    #[inline(always)]
    pub fn is_titlecase(c: char) -> bool {
        // TODO: After `char::case` is stable, we won't need to provide this
        // function any more.
        //
        // see: https://github.com/rust-lang/rust/issues/153892
        generated::is_titlecase(c)
    }
}
