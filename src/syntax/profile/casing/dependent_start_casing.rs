// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::CharCase;
use crate::syntax::profile::casing::Casing;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum DependentStartCasing {
    #[default]
    Uncased = 0,
    Lower = 1,
    Upper = 2,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl DependentStartCasing {
    #[inline]
    fn new(c: char) -> Self {
        match CharCase::new(c) {
            CharCase::Uncased => Self::Uncased,
            CharCase::Lower => Self::Lower,
            CharCase::Upper => Self::Upper,
            CharCase::TitleNonGreek => Self::Upper,
            CharCase::Digit => Self::Uncased,
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Casing for DependentStartCasing {
    const UNIFORM: bool = false;

    #[inline]
    fn chunk_start_case(&mut self, c: char) -> bool {
        *self = match (*self, Self::new(c)) {
            (left, Self::Uncased) => left,
            (Self::Uncased, right) => right,
            (Self::Lower, Self::Lower) => Self::Lower,
            (Self::Upper, Self::Upper) => Self::Upper,
            _ => return false,
        };
        true
    }

    #[inline(always)]
    fn chunk_continue_case(&mut self, _: char) -> bool {
        true
    }
}
