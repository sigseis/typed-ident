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
        assert!(Strict::is_ident_start('a'));
        assert!(!Strict::is_ident_start('1'));
    }

    #[test]
    fn profile_is_chunk_start() {
        // U+0301 COMBINING ACUTE ACCENT is a NonspacingMark (Mn)
        assert!(!Strict::is_chunk_start('\u{0301}'));
        // Test a normal character
        assert!(Strict::is_chunk_start('a'));
    }

    #[test]
    fn profile_in_profile() {
        assert!(Strict::in_profile('a'));
        assert!(Strict::in_profile('1'));
        assert!(!Strict::in_profile('_'));
    }

    #[test]
    fn profile_is_chunk_continue() {
        assert!(Strict::is_chunk_continue('a'));
        assert!(Strict::is_chunk_continue('1'));
        assert!(!Strict::is_chunk_continue('_'));
    }
}
