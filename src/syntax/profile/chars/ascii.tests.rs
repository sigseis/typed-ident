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
        assert!(Ascii::is_ident_start('a'));
        assert!(Ascii::is_ident_start('Z'));
        assert!(!Ascii::is_ident_start('1'));
        assert!(!Ascii::is_ident_start('_'));
    }

    #[test]
    fn profile_is_chunk_start() {
        assert!(Ascii::is_chunk_start('a'));
        assert!(Ascii::is_chunk_start('1'));
        assert!(!Ascii::is_chunk_start('_'));
    }

    #[test]
    fn profile_in_profile() {
        assert!(Ascii::in_profile('a'));
        assert!(Ascii::in_profile('1'));
        assert!(!Ascii::in_profile('_'));
    }

    #[test]
    fn profile_is_chunk_continue() {
        assert!(Ascii::is_chunk_continue('a'));
        assert!(Ascii::is_chunk_continue('1'));
        assert!(!Ascii::is_chunk_continue('_'));
    }
}
