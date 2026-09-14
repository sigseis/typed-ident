// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::Casing;

// =============================================================================
// TYPES
// =============================================================================

/// Represents the casing of a uniform identifier.
pub(super) trait UniformCasing: Casing {
    fn chunk_case(&mut self, c: char) -> bool;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<C: UniformCasing> Casing for C {
    const UNIFORM: bool = true;
    #[inline(always)]
    fn chunk_start_case(&mut self, c: char) -> bool {
        self.chunk_case(c)
    }
    #[inline(always)]
    fn chunk_continue_case(&mut self, c: char) -> bool {
        self.chunk_case(c)
    }
}
