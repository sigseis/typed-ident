// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
use crate::syntax::*;

// =============================================================================
// CONSTANTS
// =============================================================================

// -----------------------------------------------------------------------------
const VALID: &[&str] = &["", "test", "01234", "_test", "test_", "test_test", "_"];

// -----------------------------------------------------------------------------
const INVALID: &[(&str, &str)] = &[
    ("日本語", "invalid format: at byte offset 0"),
    ("-test", "invalid format: at byte offset 0"),
    ("test-", "invalid format: at byte offset 4"),
    ("test-test", "invalid format: at byte offset 4"),
];

// =============================================================================
// CONFIGURATION
// =============================================================================

// -----------------------------------------------------------------------------
type FragmentBufUnderTest =
    FragmentBuf<boundary::Standard, delimiter::LowLine, profile::Mixed<profile::Ascii>>;

// =============================================================================
// TESTS: IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod impls {
    use super::*;

    #[test]
    fn new_valid() -> Result<(), Error> {
        for test in VALID {
            let fragment = FragmentBufUnderTest::from_str(test)?;
            assert_eq!(fragment.is_empty(), test.is_empty());
            assert_eq!(fragment, *test);
        }
        Ok(())
    }

    #[test]
    fn new_invalid() -> Result<(), Error> {
        for (test, _) in INVALID {
            assert!(FragmentBufUnderTest::from_str(test).is_err());
        }
        Ok(())
    }
}

// =============================================================================
// TESTS: TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
mod traits {
    use super::*;

    #[test]
    fn as_ref_str() -> Result<(), Error> {
        let fragment = FragmentBufUnderTest::from_str("test")?;
        let fragment: &str = fragment.as_ref();
        assert_eq!(fragment, "test");
        Ok(())
    }

    #[test]
    fn as_ref_u8_slice() -> Result<(), Error> {
        let fragment = FragmentBufUnderTest::from_str("test")?;
        let fragment: &[u8] = fragment.as_ref();
        assert_eq!(fragment, b"test");
        Ok(())
    }

    #[test]
    fn eq_str() -> Result<(), Error> {
        let fragment = FragmentBufUnderTest::from_str("test")?;
        assert_eq!(fragment, "test");
        assert_ne!(fragment, "other");
        Ok(())
    }

    #[test]
    fn display_fragment() -> Result<(), Error> {
        use std_alloc::string::ToString;
        let fragment = FragmentBufUnderTest::from_str("test")?;
        assert_eq!(fragment.to_string(), "test");
        Ok(())
    }

    #[test]
    fn default() -> Result<(), Error> {
        let fragment: FragmentBufUnderTest = Default::default();
        assert!(fragment.is_empty());
        assert_eq!(fragment.as_str(), "");
        Ok(())
    }

    #[test]
    fn str_eq_fragment() -> Result<(), Error> {
        let fragment = FragmentBufUnderTest::from_str("test")?;
        assert_eq!("test", fragment);
        assert_ne!("other", fragment);
        Ok(())
    }

    #[test]
    fn partial_ord_str() -> Result<(), Error> {
        let fragment = FragmentBufUnderTest::from_str("abc")?;
        assert!(fragment < "def");
        assert_eq!(fragment, "abc");
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_round_trip_json() -> Result<(), serde_json::Error> {
        for test in VALID.iter().copied() {
            let value = FragmentBufUnderTest::from_str(test).unwrap();
            let ser = serde_json::to_string(&value)?;
            let de: FragmentBufUnderTest = serde_json::from_str(&ser)?;
            assert_eq!(de, value);
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_tokens_valid() -> Result<(), Error> {
        use serde_test::{Token, assert_tokens};
        for test in VALID.iter().copied() {
            let value = FragmentBufUnderTest::from_str(test)?;
            assert_tokens(&value, &[Token::String(test)]);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_tokens_invalid() {
        use serde_test::{Token, assert_de_tokens_error};
        for (test, error) in INVALID.iter().copied() {
            assert_de_tokens_error::<FragmentBufUnderTest>(&[Token::String(test)], error);
        }
    }

    #[test]
    fn str_partial_ord_fragment() -> Result<(), Error> {
        let fragment = FragmentBufUnderTest::from_str("abc")?;
        assert!("abc" <= fragment);
        assert!("def" > fragment);
        Ok(())
    }

    #[test]
    fn try_from_str_valid() -> Result<(), Error> {
        use std_alloc::string::ToString;
        for test in VALID {
            let fragment: FragmentBufUnderTest = TryFrom::try_from(test.to_string())?;
            assert_eq!(fragment.is_empty(), test.is_empty());
            assert_eq!(fragment, *test);
        }
        Ok(())
    }

    #[test]
    fn try_from_str_invalid() -> Result<(), Error> {
        use std_alloc::string::ToString;
        for (test, _) in INVALID {
            let fragment: Result<FragmentBufUnderTest, Error> = TryFrom::try_from(test.to_string());
            assert!(fragment.is_err());
        }
        Ok(())
    }
}
