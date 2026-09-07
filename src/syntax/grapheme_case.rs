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
    #[inline]
    pub fn from_char(c: char) -> Self {
        Self::from_char_case(CharCase::new(c))
    }

    #[inline]
    pub fn from_char_case(case: CharCase) -> Self {
        match case {
            CharCase::Uncased => Self::Uncased,
            CharCase::Lower => Self::Lower,
            CharCase::Upper => Self::Upper,
            CharCase::TitleNonGreek => Self::TitleNonGreek,
            CharCase::Digit => Self::Digit,
        }
    }

    #[inline]
    pub fn from_chars(chars: impl Iterator<Item = char>) -> Self {
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

    #[inline]
    pub fn new_lowercased(grapheme: &str) -> Self {
        Self::from_chars(grapheme.chars().flat_map(|c| c.to_lowercase()))
    }

    #[inline]
    pub fn new_uppercase(grapheme: &str) -> Self {
        Self::from_chars(grapheme.chars().flat_map(|c| c.to_uppercase()))
    }

    #[inline]
    pub fn new(grapheme: &str) -> Self {
        Self::from_chars(grapheme.chars())
    }
}
