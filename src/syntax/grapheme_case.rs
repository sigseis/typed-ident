// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::CharCase;

// =============================================================================
// TYPES
// =============================================================================

/// Represents the case of a grapheme cluster of code points.
///
/// This is made to map quite directly to [`CharCase`], but with a new special
/// case variant (`Mixed`) for the case where two code points disagree on the
/// case of a single grapheme cluster.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum GraphemeCase {
    Uncased = 0,
    Lower = 1,
    Upper = 2,
    TitleNonGreek = 3,
    Digit = 4,
    Mixed = 5,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl GraphemeCase {
    /// Returns a grapheme case from a single character grapheme.
    #[inline]
    pub fn from_char(c: char) -> Self {
        Self::from_char_case(CharCase::new(c))
    }

    #[inline]
    fn from_char_case(case: CharCase) -> Self {
        match case {
            CharCase::Uncased => Self::Uncased,
            CharCase::Lower => Self::Lower,
            CharCase::Upper => Self::Upper,
            CharCase::TitleNonGreek => Self::TitleNonGreek,
            CharCase::Digit => Self::Digit,
        }
    }

    #[inline]
    fn from_chars(chars: impl Iterator<Item = char>) -> Self {
        let mut cases = chars
            .map(CharCase::new)
            .filter(|c| !matches!(c, CharCase::Uncased));
        let Some(case) = cases.next() else {
            return Self::Uncased;
        };
        if cases.any(|c| c != case) {
            return Self::Mixed;
        }
        Self::from_char_case(case)
    }

    /// Tests the case of each character within the grapheme, folding them into
    #[inline]
    pub fn new(grapheme: &str) -> Self {
        Self::from_chars(grapheme.chars())
    }

    /// First applies a lowercase transformation on the characters, then tests
    /// to see what the case of the resulting grapheme is.
    #[inline]
    pub fn new_lowercased(grapheme: &str) -> Self {
        Self::from_chars(grapheme.chars().flat_map(|c| c.to_lowercase()))
    }

    /// First applies an uppercase transformation on the characters, then tests
    /// to see what the case of the resulting grapheme is.
    #[inline]
    pub fn new_uppercased(grapheme: &str) -> Self {
        Self::from_chars(grapheme.chars().flat_map(|c| c.to_uppercase()))
    }
}
