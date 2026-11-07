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
const INVALID: &[&str] = &["日本語", "-test", "test-", "test-test"];

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
        for test in INVALID {
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
    fn serde_valid() {
        for test in VALID.iter().copied() {
            // It's a valid value, so we should be able to construct it.
            let value = FragmentBufUnderTest::from_str(test).unwrap();

            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(&value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // Validate that it deserializes as expected.
            let deserialized: FragmentBufUnderTest = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_invalid() {
        use std_alloc::string::ToString;
        for test in INVALID.iter().copied() {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<FragmentBufUnderTest, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert!(
                error.starts_with("invalid format: at byte offset"),
                "actual error: {error}"
            );
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
        for test in INVALID {
            let fragment: Result<FragmentBufUnderTest, Error> = TryFrom::try_from(test.to_string());
            assert!(fragment.is_err());
        }
        Ok(())
    }
}
