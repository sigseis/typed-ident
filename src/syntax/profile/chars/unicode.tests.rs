// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// TESTS: TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
mod traits {
    use super::*;

    #[test]
    fn profile_is_ident_start() {
        assert!(Unicode::is_ident_start('a'));
        assert!(!Unicode::is_ident_start('1'));
        assert!(!Unicode::is_ident_start('_'));
    }

    #[test]
    fn profile_is_chunk_start() {
        assert!(Unicode::is_chunk_start('a'));
        assert!(Unicode::is_chunk_start('1'));
        assert!(!Unicode::is_chunk_start('_'));
    }

    #[test]
    fn profile_in_profile() {
        assert!(Unicode::in_profile('a'));
        assert!(Unicode::in_profile('1'));
        assert!(!Unicode::in_profile('_'));
    }

    #[test]
    fn profile_is_chunk_continue() {
        assert!(Unicode::is_chunk_continue('a'));
        assert!(Unicode::is_chunk_continue('1'));
        assert!(!Unicode::is_chunk_continue('_'));
    }
}
