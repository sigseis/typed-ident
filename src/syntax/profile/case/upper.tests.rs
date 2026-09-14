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
    fn is_chunk_char() {
        assert!(Upper::<Ascii>::is_chunk_char('A'));
        assert!(!Upper::<Ascii>::is_chunk_char('a'));
        assert!(Upper::<Ascii>::is_chunk_char('1'));
        assert!(!Upper::<Ascii>::is_chunk_char(' '));
    }

    #[test]
    fn is_chunk_continue() {
        assert!(Upper::<Ascii>::is_chunk_continue('A'));
        assert!(!Upper::<Ascii>::is_chunk_continue('a'));
        assert!(Upper::<Ascii>::is_chunk_continue('1'));
        assert!(!Upper::<Ascii>::is_chunk_continue(' '));
    }

    #[test]
    fn is_chunk_start() {
        assert!(Upper::<Ascii>::is_chunk_start('A'));
        assert!(!Upper::<Ascii>::is_chunk_start('a'));
        assert!(Upper::<Ascii>::is_chunk_start('1'));
        assert!(!Upper::<Ascii>::is_chunk_start(' '));
    }

    #[test]
    fn is_ident_start_char() {
        assert!(Upper::<Ascii>::is_ident_start_char('A'));
        assert!(!Upper::<Ascii>::is_ident_start_char('a'));
        assert!(!Upper::<Ascii>::is_ident_start_char('1'));
        assert!(!Upper::<Ascii>::is_ident_start_char(' '));
    }
}
