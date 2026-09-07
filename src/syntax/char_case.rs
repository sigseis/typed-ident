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
        !c.is_uppercase() && !Self::is_titlecase_any(c)
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

    /// Given a character code point, produce a case for that character.
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
        } else {
            match Self::is_titlecase(c) {
                Some(true) => Self::Upper,
                Some(false) => Self::TitleNonGreek,
                None => Self::Uncased,
            }
        }
    }

    /// Given a character code point, tell whether or not it's a cased code.
    #[inline]
    pub fn is_cased(c: char) -> bool {
        Self::new(c) != Self::Uncased
    }

    // TODO: Possibly helpful: https://github.com/rust-lang/rust/issues/153892
    //
    // I tried using a crate for this, but the crate was implemented incorrectly,
    // and returned `true` for some obviously not titlecase characters. I don't
    // really want to chase down fixing some other crate when we're going to have
    // this function in std soon anyways.
    //
    // TODO: Maybe we should bake this instead? - just feels wrong to include
    // huge dependencies just to tell if a titlecase character is Greek or not.
    //
    // For now, I will manually inline - post 0.0.1 effort to automate.
    #[inline]
    fn is_titlecase(c: char) -> Option<bool> {
        match c {
            '\u{01C5}' | '\u{01C8}' | '\u{01CB}' | '\u{01F2}' => Some(false),
            '\u{1F88}' | '\u{1F89}' | '\u{1F8A}' | '\u{1F8B}' | '\u{1F8C}' | '\u{1F8D}'
            | '\u{1F8E}' | '\u{1F8F}' | '\u{1F98}' | '\u{1F99}' | '\u{1F9A}' | '\u{1F9B}'
            | '\u{1F9C}' | '\u{1F9D}' | '\u{1F9E}' | '\u{1F9F}' | '\u{1FA8}' | '\u{1FA9}'
            | '\u{1FAA}' | '\u{1FAB}' | '\u{1FAC}' | '\u{1FAD}' | '\u{1FAE}' | '\u{1FAF}'
            | '\u{1FBC}' | '\u{1FCC}' | '\u{1FFC}' => Some(true),
            _ => None,
        }
    }
    #[inline]
    fn is_titlecase_any(c: char) -> bool {
        Self::is_titlecase(c).is_some()
    }
    #[inline]
    fn is_non_greek_titlecase(c: char) -> bool {
        Self::is_titlecase(c).is_some_and(|g| !g)
    }
}
