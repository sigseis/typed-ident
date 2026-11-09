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
const VALUES: &[AsciiPunctuation] = &[
    AsciiPunctuation::ExclamationMark,
    AsciiPunctuation::QuotationMark,
    AsciiPunctuation::NumberSign,
    AsciiPunctuation::DollarSign,
    AsciiPunctuation::PercentSign,
    AsciiPunctuation::Ampersand,
    AsciiPunctuation::Apostrophe,
    AsciiPunctuation::LeftParenthesis,
    AsciiPunctuation::RightParenthesis,
    AsciiPunctuation::Asterisk,
    AsciiPunctuation::PlusSign,
    AsciiPunctuation::Comma,
    AsciiPunctuation::HyphenMinus,
    AsciiPunctuation::FullStop,
    AsciiPunctuation::Solidus,
    AsciiPunctuation::Colon,
    AsciiPunctuation::Semicolon,
    AsciiPunctuation::LessThanSign,
    AsciiPunctuation::EqualsSign,
    AsciiPunctuation::GreaterThanSign,
    AsciiPunctuation::QuestionMark,
    AsciiPunctuation::CommercialAt,
    AsciiPunctuation::LeftSquareBracket,
    AsciiPunctuation::ReverseSolidus,
    AsciiPunctuation::RightSquareBracket,
    AsciiPunctuation::CircumflexAccent,
    AsciiPunctuation::LowLine,
    AsciiPunctuation::GraveAccent,
    AsciiPunctuation::LeftCurlyBracket,
    AsciiPunctuation::VerticalLine,
    AsciiPunctuation::RightCurlyBracket,
    AsciiPunctuation::Tilde,
];

// -----------------------------------------------------------------------------
#[cfg(feature = "serde")]
const INVALID: &[char] = &['A', ' ', 'z'];

// =============================================================================
// TESTS: IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod impls {
    use super::*;

    #[test]
    fn from_char() {
        assert_eq!(
            AsciiPunctuation::from_char('!'),
            Some(AsciiPunctuation::ExclamationMark)
        );
        assert_eq!(
            AsciiPunctuation::from_char('@'),
            Some(AsciiPunctuation::CommercialAt)
        );
        assert_eq!(
            AsciiPunctuation::from_char('_'),
            Some(AsciiPunctuation::LowLine)
        );
        assert_eq!(
            AsciiPunctuation::from_char('~'),
            Some(AsciiPunctuation::Tilde)
        );
        assert_eq!(AsciiPunctuation::from_char('a'), None);
        assert_eq!(AsciiPunctuation::from_char(' '), None);
    }

    #[test]
    fn is_delim() {
        assert!(AsciiPunctuation::is_delim('!'));
        assert!(AsciiPunctuation::is_delim('@'));
        assert!(AsciiPunctuation::is_delim('_'));
        assert!(AsciiPunctuation::is_delim('~'));
        assert!(!AsciiPunctuation::is_delim('a'));
        assert!(!AsciiPunctuation::is_delim(' '));
    }

    #[test]
    fn to_char() {
        assert_eq!(AsciiPunctuation::ExclamationMark.to_char(), '!');
        assert_eq!(AsciiPunctuation::CommercialAt.to_char(), '@');
        assert_eq!(AsciiPunctuation::LowLine.to_char(), '_');
        assert_eq!(AsciiPunctuation::Tilde.to_char(), '~');
    }

    #[test]
    fn to_str() {
        assert_eq!(AsciiPunctuation::ExclamationMark.to_str(), "!");
        assert_eq!(AsciiPunctuation::CommercialAt.to_str(), "@");
        assert_eq!(AsciiPunctuation::LowLine.to_str(), "_");
        assert_eq!(AsciiPunctuation::Tilde.to_str(), "~");
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
        assert_eq!(AsciiPunctuation::ExclamationMark.as_char(), '!');
        assert_eq!(AsciiPunctuation::Tilde.as_char(), '~');
    }

    #[test]
    fn from_char() {
        assert_eq!(
            AsciiPunctuation::from_char('!'),
            Some(AsciiPunctuation::ExclamationMark)
        );
        assert_eq!(AsciiPunctuation::from_char('a'), None);
    }

    #[test]
    fn from_ident_start() {
        assert_eq!(
            AsciiPunctuation::from_ident_start('!'),
            Some(AsciiPunctuation::ExclamationMark)
        );
        assert_eq!(AsciiPunctuation::from_ident_start('a'), None);
    }

    #[test]
    fn from_chunk_delim() {
        assert_eq!(
            AsciiPunctuation::from_chunk_delim('!'),
            Some(AsciiPunctuation::ExclamationMark)
        );
        assert_eq!(AsciiPunctuation::from_chunk_delim('a'), None);
    }

    #[test]
    fn is_delim() {
        assert!(AsciiPunctuation::is_delim('!'));
        assert!(!AsciiPunctuation::is_delim('a'));
    }

    #[test]
    fn is_ident_start_delim() {
        assert!(AsciiPunctuation::is_ident_start_delim('!'));
        assert!(!AsciiPunctuation::is_ident_start_delim('a'));
    }

    #[test]
    fn is_chunk_delim() {
        assert!(AsciiPunctuation::is_chunk_delim('!'));
        assert!(!AsciiPunctuation::is_chunk_delim('a'));
    }

    #[test]
    fn partial_eq_char() {
        assert!(AsciiPunctuation::ExclamationMark == '!');
        assert!('!' == AsciiPunctuation::ExclamationMark);
        assert!(AsciiPunctuation::ExclamationMark != '@');
    }

    #[test]
    fn partial_eq_delimiter() {
        assert!(AsciiPunctuation::ExclamationMark == AsciiPunctuation::ExclamationMark);
        assert!(AsciiPunctuation::ExclamationMark != AsciiPunctuation::CommercialAt);
        assert!(AsciiPunctuation::LowLine == LowLine);
        assert!(AsciiPunctuation::LowLine != HyphenMinus);
    }

    #[test]
    fn partial_ord_char() {
        assert!(AsciiPunctuation::ExclamationMark < '@');
        assert!(AsciiPunctuation::Tilde > '!');
        assert!('!' < AsciiPunctuation::Tilde);
    }

    #[test]
    fn partial_ord_delimiter() {
        assert!(AsciiPunctuation::ExclamationMark < AsciiPunctuation::CommercialAt);
        assert!(AsciiPunctuation::Tilde > AsciiPunctuation::ExclamationMark);
    }

    #[test]
    fn as_ref_str() {
        assert_eq!(AsciiPunctuation::ExclamationMark.as_ref(), "!");
        assert_eq!(AsciiPunctuation::Tilde.as_ref(), "~");
    }

    #[test]
    fn from_to_char() {
        assert_eq!(char::from(AsciiPunctuation::ExclamationMark), '!');
        assert_eq!(char::from(AsciiPunctuation::Tilde), '~');
    }

    #[test]
    fn from_ascii_flat_line() {
        assert_eq!(
            AsciiPunctuation::from(AsciiFlatLine::HyphenMinus),
            AsciiPunctuation::HyphenMinus
        );
        assert_eq!(
            AsciiPunctuation::from(AsciiFlatLine::LowLine),
            AsciiPunctuation::LowLine
        );
    }

    #[test]
    fn from_hyphen_minus() {
        assert_eq!(
            AsciiPunctuation::from(HyphenMinus),
            AsciiPunctuation::HyphenMinus
        );
    }

    #[test]
    fn from_low_line() {
        assert_eq!(AsciiPunctuation::from(LowLine), AsciiPunctuation::LowLine);
    }

    #[test]
    fn try_from_char() {
        assert_eq!(
            AsciiPunctuation::try_from('!').unwrap(),
            AsciiPunctuation::ExclamationMark
        );
        assert!(AsciiPunctuation::try_from('a').is_err());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn display() {
        use std_alloc::string::ToString;
        assert_eq!(AsciiPunctuation::ExclamationMark.to_string(), "!");
        assert_eq!(AsciiPunctuation::Tilde.to_string(), "~");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_round_trip_json() -> Result<(), serde_json::Error> {
        for value in VALUES {
            let ser = serde_json::to_string(value)?;
            let de: AsciiPunctuation = serde_json::from_str(&ser)?;
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
            assert_de_tokens_error::<AsciiPunctuation>(
                &[Token::Char(value)],
                "attempted to convert a non-delimiter character to a delimiter",
            );
        }
    }
}
