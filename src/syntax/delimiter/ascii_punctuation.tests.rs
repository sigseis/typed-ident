// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

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
    fn serde_valid() {
        for (value, string) in [
            (AsciiPunctuation::HyphenMinus, "-"),
            (AsciiPunctuation::LowLine, "_"),
            (AsciiPunctuation::Ampersand, "&"),
        ] {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(string));

            // Validate that it deserializes as expected.
            let deserialized: AsciiPunctuation = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_invalid() {
        use std_alloc::string::ToString;
        for (test, string) in [('A', "A"), (' ', " "), ('z', "z")] {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(string));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<AsciiPunctuation, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert_eq!(
                error, "attempted to convert a non-delimiter character to a delimiter",
                "actual error: {error}"
            );
        }
    }
}
