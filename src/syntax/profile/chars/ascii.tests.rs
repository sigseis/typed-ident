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
        assert!(Ascii::is_chunk_char('a'));
        assert!(Ascii::is_chunk_char('1'));
        assert!(!Ascii::is_chunk_char('_'));
    }

    #[test]
    fn is_chunk_continue() {
        assert!(Ascii::is_chunk_continue('a'));
        assert!(Ascii::is_chunk_continue('1'));
        assert!(!Ascii::is_chunk_continue('_'));
    }

    #[test]
    fn is_chunk_start() {
        assert!(Ascii::is_chunk_start('a'));
        assert!(Ascii::is_chunk_start('1'));
        assert!(!Ascii::is_chunk_start('_'));
    }

    #[test]
    fn is_ident_start_char() {
        assert!(Ascii::is_ident_start_char('a'));
        assert!(Ascii::is_ident_start_char('Z'));
        assert!(!Ascii::is_ident_start_char('1'));
        assert!(!Ascii::is_ident_start_char('_'));
    }
}
