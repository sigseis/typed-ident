// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::CharCase;
use crate::syntax::profile::casing::UniformCasing;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum DependentCasing {
    #[default]
    Uncased = 0,
    Lower = 1,
    Upper = 2,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl DependentCasing {
    #[inline]
    fn new(c: char) -> Self {
        // Because the case profile protects us from non-greek titlecase, this
        // function cannot be passed such a character. So assume it's not in the
        // set of values that could be presented to us.
        if c.is_lowercase() {
            Self::Lower
        } else if c.is_uppercase() || CharCase::is_titlecase_any(c) {
            Self::Upper
        } else {
            Self::Uncased
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl UniformCasing for DependentCasing {
    #[inline]
    fn chunk_case(&mut self, c: char) -> bool {
        *self = match (*self, Self::new(c)) {
            (left, Self::Uncased) => left,
            (Self::Uncased, right) => right,
            (Self::Lower, Self::Lower) => Self::Lower,
            (Self::Upper, Self::Upper) => Self::Upper,
            _ => return false,
        };
        true
    }
}
