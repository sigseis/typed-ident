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
        assert!(UpperCamel::<Ascii>::is_ident_start('A'));
        assert!(!UpperCamel::<Ascii>::is_ident_start('a'));
        assert!(!UpperCamel::<Ascii>::is_ident_start('1'));
        assert!(!UpperCamel::<Ascii>::is_ident_start(' '));
    }

    #[test]
    fn profile_is_chunk_start() {
        assert!(UpperCamel::<Ascii>::is_chunk_start('A'));
        assert!(!UpperCamel::<Ascii>::is_chunk_start('a'));
        assert!(UpperCamel::<Ascii>::is_chunk_start('1'));
        assert!(!UpperCamel::<Ascii>::is_chunk_start(' '));
    }

    #[test]
    fn profile_in_profile() {
        assert!(UpperCamel::<Ascii>::in_profile('A'));
        assert!(UpperCamel::<Ascii>::in_profile('a'));
        assert!(UpperCamel::<Ascii>::in_profile('1'));
        assert!(!UpperCamel::<Ascii>::in_profile(' '));
    }

    #[test]
    fn profile_is_chunk_continue() {
        assert!(UpperCamel::<Ascii>::is_chunk_continue('A'));
        assert!(UpperCamel::<Ascii>::is_chunk_continue('a'));
        assert!(UpperCamel::<Ascii>::is_chunk_continue('1'));
        assert!(!UpperCamel::<Ascii>::is_chunk_continue(' '));
    }
}
