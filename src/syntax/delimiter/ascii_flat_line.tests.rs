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
    fn serde_valid() {
        for (value, string) in [
            (AsciiFlatLine::HyphenMinus, "-"),
            (AsciiFlatLine::LowLine, "_"),
        ] {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(string));

            // Validate that it deserializes as expected.
            let deserialized: AsciiFlatLine = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_invalid() {
        use std_alloc::string::ToString;
        for (test, string) in [('?', "?"), ('!', "!"), ('A', "A")] {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(string));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<AsciiFlatLine, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert_eq!(
                error, "attempted to convert a non-delimiter character to a delimiter",
                "actual error: {error}"
            );
        }
    }
}
