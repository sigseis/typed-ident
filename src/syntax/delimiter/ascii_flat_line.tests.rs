// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// CONSTANTS
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "serde")]
const VALUES: &[AsciiFlatLine] = &[AsciiFlatLine::HyphenMinus, AsciiFlatLine::LowLine];

// -----------------------------------------------------------------------------
#[cfg(feature = "serde")]
const INVALID: &[char] = &['?', '!', 'A', ' '];

// =============================================================================
// TESTS: IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod impls {
    use super::*;

    #[test]
    fn from_char_valid() {
        assert_eq!(
            AsciiFlatLine::from_char('-'),
            Some(AsciiFlatLine::HyphenMinus)
        );
        assert_eq!(AsciiFlatLine::from_char('_'), Some(AsciiFlatLine::LowLine));
        assert_eq!(AsciiFlatLine::from_char('a'), None);
    }

    #[test]
    fn is_delim() {
        assert!(AsciiFlatLine::is_delim('-'));
        assert!(AsciiFlatLine::is_delim('_'));
        assert!(!AsciiFlatLine::is_delim('a'));
    }

    #[test]
    fn to_char() {
        assert_eq!(AsciiFlatLine::HyphenMinus.to_char(), '-');
        assert_eq!(AsciiFlatLine::LowLine.to_char(), '_');
    }

    #[test]
    fn to_str() {
        assert_eq!(AsciiFlatLine::HyphenMinus.to_str(), "-");
        assert_eq!(AsciiFlatLine::LowLine.to_str(), "_");
    }
}

// =============================================================================
// TESTS: TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
mod traits {
    use super::*;

    #[test]
    fn as_char() {
        assert_eq!(AsciiFlatLine::HyphenMinus.as_char(), '-');
        assert_eq!(AsciiFlatLine::LowLine.as_char(), '_');
    }

    #[test]
    fn from_char() {
        assert_eq!(
            AsciiFlatLine::from_char('-'),
            Some(AsciiFlatLine::HyphenMinus)
        );
        assert_eq!(AsciiFlatLine::from_char('_'), Some(AsciiFlatLine::LowLine));
        assert_eq!(AsciiFlatLine::from_char('a'), None);
    }

    #[test]
    fn from_ident_start() {
        assert_eq!(
            AsciiFlatLine::from_ident_start('-'),
            Some(AsciiFlatLine::HyphenMinus)
        );
        assert_eq!(
            AsciiFlatLine::from_ident_start('_'),
            Some(AsciiFlatLine::LowLine)
        );
        assert_eq!(AsciiFlatLine::from_ident_start('a'), None);
    }

    #[test]
    fn from_chunk_delim() {
        assert_eq!(
            AsciiFlatLine::from_chunk_delim('-'),
            Some(AsciiFlatLine::HyphenMinus)
        );
        assert_eq!(
            AsciiFlatLine::from_chunk_delim('_'),
            Some(AsciiFlatLine::LowLine)
        );
        assert_eq!(AsciiFlatLine::from_chunk_delim('a'), None);
    }

    #[test]
    fn is_delim() {
        assert!(AsciiFlatLine::is_delim('-'));
        assert!(AsciiFlatLine::is_delim('_'));
        assert!(!AsciiFlatLine::is_delim('a'));
    }

    #[test]
    fn is_ident_start_delim() {
        assert!(AsciiFlatLine::is_ident_start_delim('-'));
        assert!(AsciiFlatLine::is_ident_start_delim('_'));
        assert!(!AsciiFlatLine::is_ident_start_delim('a'));
    }

    #[test]
    fn is_chunk_delim() {
        assert!(AsciiFlatLine::is_chunk_delim('-'));
        assert!(AsciiFlatLine::is_chunk_delim('_'));
        assert!(!AsciiFlatLine::is_chunk_delim('a'));
    }

    #[test]
    fn partial_eq_char() {
        assert!(AsciiFlatLine::HyphenMinus == '-');
        assert!(AsciiFlatLine::LowLine == '_');
        assert!('-' == AsciiFlatLine::HyphenMinus);
        assert!('_' == AsciiFlatLine::LowLine);
        assert!(AsciiFlatLine::HyphenMinus != '_');
    }

    #[test]
    fn partial_eq_delimiter() {
        assert!(AsciiFlatLine::HyphenMinus == HyphenMinus);
        assert!(AsciiFlatLine::LowLine == LowLine);
        assert!(AsciiFlatLine::HyphenMinus != LowLine);
    }

    #[test]
    fn partial_ord_char() {
        assert!(AsciiFlatLine::HyphenMinus < '_');
        assert!(AsciiFlatLine::LowLine > '-');
        assert!('-' < AsciiFlatLine::LowLine);
    }

    #[test]
    fn partial_ord_delimiter() {
        assert!(AsciiFlatLine::HyphenMinus < LowLine);
        assert!(AsciiFlatLine::LowLine > HyphenMinus);
    }

    #[test]
    fn as_ref_str() {
        assert_eq!(AsciiFlatLine::HyphenMinus.as_ref(), "-");
        assert_eq!(AsciiFlatLine::LowLine.as_ref(), "_");
    }

    #[test]
    fn from_to_char() {
        assert_eq!(char::from(AsciiFlatLine::HyphenMinus), '-');
        assert_eq!(char::from(AsciiFlatLine::LowLine), '_');
    }

    #[test]
    fn try_from_char() {
        assert_eq!(
            AsciiFlatLine::try_from('-').unwrap(),
            AsciiFlatLine::HyphenMinus
        );
        assert_eq!(
            AsciiFlatLine::try_from('_').unwrap(),
            AsciiFlatLine::LowLine
        );
        assert!(AsciiFlatLine::try_from('a').is_err());
    }

    #[test]
    fn from_low_line() {
        assert_eq!(AsciiFlatLine::from(LowLine), AsciiFlatLine::LowLine);
    }

    #[test]
    fn from_hyphen_minus() {
        assert_eq!(AsciiFlatLine::from(HyphenMinus), AsciiFlatLine::HyphenMinus);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn display() {
        use std_alloc::string::ToString;
        assert_eq!(AsciiFlatLine::HyphenMinus.to_string(), "-");
        assert_eq!(AsciiFlatLine::LowLine.to_string(), "_");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_round_trip_json() -> Result<(), serde_json::Error> {
        for value in VALUES {
            let ser = serde_json::to_string(value)?;
            let de: AsciiFlatLine = serde_json::from_str(&ser)?;
            assert_eq!(de, *value);
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_tokens_valid() {
        use serde_test::{Token, assert_tokens};
        for value in VALUES {
            assert_tokens(value, &[Token::Char(value.as_char())]);
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_tokens_invalid() {
        use serde_test::{Token, assert_de_tokens_error};
        for value in INVALID.iter().copied() {
            assert_de_tokens_error::<AsciiFlatLine>(
                &[Token::Char(value)],
                "attempted to convert a non-delimiter character to a delimiter",
            );
        }
    }
}
