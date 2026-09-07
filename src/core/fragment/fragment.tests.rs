// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
use crate::syntax::*;

// =============================================================================
// NOTICE
// -----------------------------------------------------------------------------
// This type is already quite thoroughly tested in several other parts of this
// project, and in addition with benchmark, doc, and integration tests. This file
// will only test things that are lacking coverage from those other tests.
// =============================================================================

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
type FragmentUnderTest = Fragment<boundary::Standard, delimiter::LowLine, profile::Ascii>;

// =============================================================================
// TESTS: IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod impls {
    use super::*;

    #[test]
    fn new_valid() -> Result<(), Error> {
        for test in VALID {
            let fragment = FragmentUnderTest::new(test)?;
            assert_eq!(fragment.is_empty(), test.is_empty());
            assert_eq!(fragment, *test);
        }
        Ok(())
    }

    #[test]
    fn new_invalid() -> Result<(), Error> {
        for test in INVALID {
            assert!(FragmentUnderTest::new(test).is_err());
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
    #[cfg(feature = "unicode-strict")]
    fn as_ref_fragment() -> Result<(), Error> {
        test_as_ref_comprehensive!(Fragment as Fragment);
        Ok(())
    }

    #[test]
    fn as_ref_str() -> Result<(), Error> {
        let fragment = FragmentUnderTest::new("test")?;
        let fragment: &str = fragment.as_ref();
        assert_eq!(fragment, "test");
        Ok(())
    }

    #[test]
    fn as_ref_u8_slice() -> Result<(), Error> {
        let fragment = FragmentUnderTest::new("test")?;
        let fragment: &[u8] = fragment.as_ref();
        assert_eq!(fragment, b"test");
        Ok(())
    }

    #[test]
    fn eq_str() -> Result<(), Error> {
        let fragment = FragmentUnderTest::new("test")?;
        assert_eq!(fragment, "test");
        assert_ne!(fragment, "other");
        Ok(())
    }

    #[test]
    fn str_eq_fragment() -> Result<(), Error> {
        let fragment = FragmentUnderTest::new("test")?;
        assert_eq!("test", fragment);
        assert_ne!("other", fragment);
        Ok(())
    }

    #[test]
    fn partial_ord_str() -> Result<(), Error> {
        let fragment = FragmentUnderTest::new("abc")?;
        assert!(fragment < "def");
        assert_eq!(fragment, "abc");
        Ok(())
    }

    #[test]
    fn str_partial_ord_fragment() -> Result<(), Error> {
        let fragment = FragmentUnderTest::new("abc")?;
        assert!("abc" <= fragment);
        assert!("def" > fragment);
        Ok(())
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn display_fragment() -> Result<(), Error> {
        use std_alloc::string::ToString;

        let fragment = FragmentUnderTest::new("test")?;
        assert_eq!(fragment.to_string(), "test");
        Ok(())
    }

    #[test]
    fn default() -> Result<(), Error> {
        let fragment: &FragmentUnderTest = Default::default();
        assert!(fragment.is_empty());
        assert_eq!(fragment.as_str(), "");
        Ok(())
    }

    #[test]
    fn try_from_str_valid() -> Result<(), Error> {
        for test in VALID {
            let fragment: &FragmentUnderTest = TryFrom::try_from(*test)?;
            assert_eq!(fragment.is_empty(), test.is_empty());
            assert_eq!(fragment, *test);
        }
        Ok(())
    }

    #[test]
    fn try_from_str_invalid() -> Result<(), Error> {
        for test in INVALID {
            let fragment: Result<&FragmentUnderTest, Error> = TryFrom::try_from(*test);
            assert!(fragment.is_err());
        }
        Ok(())
    }
}
