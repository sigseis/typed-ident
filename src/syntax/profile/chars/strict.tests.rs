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
        assert!(Strict::is_chunk_char('a'));
        assert!(Strict::is_chunk_char('1'));
        assert!(!Strict::is_chunk_char('_'));
    }

    #[test]
    fn is_chunk_continue() {
        assert!(Strict::is_chunk_continue('a'));
        assert!(Strict::is_chunk_continue('1'));
        assert!(!Strict::is_chunk_continue('_'));
    }

    #[test]
    fn is_chunk_start() {
        // U+0301 COMBINING ACUTE ACCENT is a NonspacingMark (Mn)
        assert!(!Strict::is_chunk_start('\u{0301}'));
        // Test a normal character
        assert!(Strict::is_chunk_start('a'));
    }

    #[test]
    fn is_ident_start_char() {
        assert!(Strict::is_ident_start_char('a'));
        assert!(!Strict::is_ident_start_char('1'));
    }
}
