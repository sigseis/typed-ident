// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
use crate::syntax::profile::Ascii;

// =============================================================================
// TESTS: TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
mod traits {
    use super::*;

    #[test]
    fn profile_is_ident_start() {
        assert!(Lower::<Ascii>::is_ident_start('a'));
        assert!(!Lower::<Ascii>::is_ident_start('A'));
        assert!(!Lower::<Ascii>::is_ident_start('1'));
        assert!(!Lower::<Ascii>::is_ident_start(' '));
    }

    #[test]
    fn profile_is_chunk_start() {
        assert!(Lower::<Ascii>::is_chunk_start('a'));
        assert!(!Lower::<Ascii>::is_chunk_start('A'));
        assert!(Lower::<Ascii>::is_chunk_start('1'));
        assert!(!Lower::<Ascii>::is_chunk_start(' '));
    }

    #[test]
    fn profile_in_profile() {
        assert!(Lower::<Ascii>::in_profile('a'));
        assert!(!Lower::<Ascii>::in_profile('A'));
        assert!(Lower::<Ascii>::in_profile('1'));
        assert!(!Lower::<Ascii>::in_profile(' '));
    }

    #[test]
    fn profile_is_chunk_continue() {
        assert!(Lower::<Ascii>::is_chunk_continue('a'));
        assert!(!Lower::<Ascii>::is_chunk_continue('A'));
        assert!(Lower::<Ascii>::is_chunk_continue('1'));
        assert!(!Lower::<Ascii>::is_chunk_continue(' '));
    }
}
