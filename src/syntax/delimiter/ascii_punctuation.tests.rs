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
    fn delimiter_as_char() {
        assert_eq!(AsciiPunctuation::ExclamationMark.as_char(), '!');
        assert_eq!(AsciiPunctuation::Tilde.as_char(), '~');
    }

    #[test]
    fn delimiter_from_char() {
        assert_eq!(
            AsciiPunctuation::from_char('!'),
            Some(AsciiPunctuation::ExclamationMark)
        );
        assert_eq!(AsciiPunctuation::from_char('a'), None);
    }

    #[test]
    fn delimiter_from_ident_start() {
        assert_eq!(
            AsciiPunctuation::from_ident_start('!'),
            Some(AsciiPunctuation::ExclamationMark)
        );
        assert_eq!(AsciiPunctuation::from_ident_start('a'), None);
    }

    #[test]
    fn delimiter_from_chunk_delim() {
        assert_eq!(
            AsciiPunctuation::from_chunk_delim('!'),
            Some(AsciiPunctuation::ExclamationMark)
        );
        assert_eq!(AsciiPunctuation::from_chunk_delim('a'), None);
    }

    #[test]
    fn delimiter_is_delim() {
        assert!(AsciiPunctuation::is_delim('!'));
        assert!(!AsciiPunctuation::is_delim('a'));
    }

    #[test]
    fn delimiter_is_ident_start() {
        assert!(AsciiPunctuation::is_ident_start('!'));
        assert!(!AsciiPunctuation::is_ident_start('a'));
    }

    #[test]
    fn delimiter_is_chunk_delim() {
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
}
