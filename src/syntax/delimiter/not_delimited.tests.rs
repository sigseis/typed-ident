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
    fn from_char() {
        assert_eq!(NotDelimited::from_char('a'), None);
        assert_eq!(NotDelimited::from_char('_'), None);
    }

    #[test]
    fn from_ident_start() {
        assert_eq!(NotDelimited::from_ident_start('a'), None);
        assert_eq!(NotDelimited::from_ident_start('_'), None);
    }

    #[test]
    fn from_chunk_delim() {
        assert_eq!(NotDelimited::from_chunk_delim('a'), None);
        assert_eq!(NotDelimited::from_chunk_delim('_'), None);
    }

    #[test]
    fn is_delim() {
        assert!(!NotDelimited::is_delim('a'));
        assert!(!NotDelimited::is_delim('_'));
    }

    #[test]
    fn is_ident_start_delim() {
        assert!(!NotDelimited::is_ident_start_delim('a'));
        assert!(!NotDelimited::is_ident_start_delim('_'));
    }

    #[test]
    fn is_chunk_delim() {
        assert!(!NotDelimited::is_chunk_delim('a'));
        assert!(!NotDelimited::is_chunk_delim('_'));
    }

    #[test]
    fn try_from_char() {
        assert!(NotDelimited::try_from('a').is_err());
        assert!(NotDelimited::try_from('_').is_err());
    }
}
