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
        assert!(Upper::<Ascii>::is_ident_start('A'));
        assert!(!Upper::<Ascii>::is_ident_start('a'));
        assert!(!Upper::<Ascii>::is_ident_start('1'));
        assert!(!Upper::<Ascii>::is_ident_start(' '));
    }

    #[test]
    fn profile_is_chunk_start() {
        assert!(Upper::<Ascii>::is_chunk_start('A'));
        assert!(!Upper::<Ascii>::is_chunk_start('a'));
        assert!(Upper::<Ascii>::is_chunk_start('1'));
        assert!(!Upper::<Ascii>::is_chunk_start(' '));
    }

    #[test]
    fn profile_in_profile() {
        assert!(Upper::<Ascii>::in_profile('A'));
        assert!(!Upper::<Ascii>::in_profile('a'));
        assert!(Upper::<Ascii>::in_profile('1'));
        assert!(!Upper::<Ascii>::in_profile(' '));
    }

    #[test]
    fn profile_is_chunk_continue() {
        assert!(Upper::<Ascii>::is_chunk_continue('A'));
        assert!(!Upper::<Ascii>::is_chunk_continue('a'));
        assert!(Upper::<Ascii>::is_chunk_continue('1'));
        assert!(!Upper::<Ascii>::is_chunk_continue(' '));
    }
}
