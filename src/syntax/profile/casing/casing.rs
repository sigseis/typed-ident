// =============================================================================
// TYPES
// =============================================================================

/// Represents the casing checks for an identifier.
pub(super) trait Casing: Default {
    const UNIFORM: bool;
    fn chunk_start_case(&mut self, c: char) -> bool;
    fn chunk_continue_case(&mut self, c: char) -> bool;
}
