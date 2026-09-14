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
    fn is_chunk_char() {
        assert!(Unicode::is_chunk_char('a'));
        assert!(Unicode::is_chunk_char('1'));
        assert!(!Unicode::is_chunk_char('_'));
    }

    #[test]
    fn is_chunk_start() {
        assert!(Unicode::is_chunk_start('a'));
        assert!(Unicode::is_chunk_start('1'));
        assert!(!Unicode::is_chunk_start('_'));
    }

    #[test]
    fn is_chunk_continue() {
        assert!(Unicode::is_chunk_continue('a'));
        assert!(Unicode::is_chunk_continue('1'));
        assert!(!Unicode::is_chunk_continue('_'));
    }

    #[test]
    fn is_ident_start_char() {
        assert!(Unicode::is_ident_start_char('a'));
        assert!(!Unicode::is_ident_start_char('1'));
        assert!(!Unicode::is_ident_start_char('_'));
    }
}
