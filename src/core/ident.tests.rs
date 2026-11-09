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
const VALID: &[&str] = &["test", "_test", "test_", "test_test", "_"];

// -----------------------------------------------------------------------------
const INVALID: &[(&str, &str)] = &[
    ("", "empty identifier"),
    ("01234", "invalid format: at byte offset 0"),
    ("日本語", "invalid format: at byte offset 0"),
    ("-test", "invalid format: at byte offset 0"),
    ("test-", "invalid format: at byte offset 4"),
    ("test-test", "invalid format: at byte offset 4"),
];

// =============================================================================
// CONFIGURATION
// =============================================================================

// -----------------------------------------------------------------------------
type IdentUnderTest = Ident<boundary::Standard, delimiter::LowLine, profile::Mixed<profile::Ascii>>;

// =============================================================================
// TESTS: IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod impls {
    use super::*;

    #[test]
    fn new_valid() -> Result<(), Error> {
        for test in VALID {
            let fragment = IdentUnderTest::new(test)?;
            assert_eq!(fragment, *test);
        }
        Ok(())
    }

    #[test]
    fn new_invalid() -> Result<(), Error> {
        for (test, _) in INVALID {
            assert!(IdentUnderTest::new(test).is_err());
        }
        Ok(())
    }
}

// =============================================================================
// TESTS: TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "unicode")]
mod traits {
    use super::*;

    #[test]
    fn as_ref_fragment() -> Result<(), Error> {
        crate::macros::test_as_ref! {
            from=Ident,
            into=Fragment,
        }
        Ok(())
    }

    #[test]
    fn as_ref_ident() -> Result<(), Error> {
        crate::macros::test_as_ref! {
            from=Ident,
            into=Ident,
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_boxed_round_trip_json() -> Result<(), serde_json::Error> {
        use std_alloc::boxed::Box;
        for test in VALID.iter().copied() {
            let value = IdentUnderTest::new_boxed(test).unwrap();
            let ser = serde_json::to_string(&value)?;
            let de: Box<IdentUnderTest> = serde_json::from_str(&ser)?;
            assert_eq!(de, value);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_boxed_tokens_valid() -> Result<(), Error> {
        use serde_test::{Token, assert_tokens};
        for test in VALID.iter().copied() {
            let value = IdentUnderTest::new_boxed(test)?;
            assert_tokens(&value, &[Token::String(test)]);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_boxed_tokens_invalid() {
        use serde_test::{Token, assert_de_tokens_error};
        use std_alloc::boxed::Box;
        for (test, error) in INVALID.iter().copied() {
            assert_de_tokens_error::<Box<IdentUnderTest>>(&[Token::String(test)], error);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_cow_round_trip_json() -> Result<(), serde_json::Error> {
        use std_alloc::borrow::Cow;
        for test in VALID.iter().copied() {
            let value = Cow::Borrowed(IdentUnderTest::new(test).unwrap());
            let ser = serde_json::to_string(&value)?;
            let de: Cow<IdentUnderTest> = serde_json::from_str(&ser)?;
            assert_eq!(de, value);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_cow_tokens_valid() -> Result<(), Error> {
        use serde_test::{Token, assert_tokens};
        use std_alloc::borrow::Cow;
        for test in VALID.iter().copied() {
            let value = Cow::Borrowed(IdentUnderTest::new(test)?);
            assert_tokens(&value, &[Token::String(test)]);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_cow_tokens_invalid() {
        use serde_test::{Token, assert_de_tokens_error};
        use std_alloc::borrow::Cow;
        for (test, error) in INVALID.iter().copied() {
            assert_de_tokens_error::<Cow<IdentUnderTest>>(&[Token::String(test)], error);
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_ref_round_trip_json() -> Result<(), serde_json::Error> {
        for test in VALID.iter().copied() {
            let value = IdentUnderTest::new(test).unwrap();
            let ser = serde_json::to_string(&value)?;
            let de: &IdentUnderTest = serde_json::from_str(&ser)?;
            assert_eq!(de, value);
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_ref_tokens_valid() -> Result<(), Error> {
        use serde_test::{Token, assert_tokens};
        for test in VALID.iter().copied() {
            let value = IdentUnderTest::new(test)?;
            assert_tokens(&value, &[Token::BorrowedStr(test)]);
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_ref_tokens_invalid() {
        use serde_test::{Token, assert_de_tokens_error};
        for (test, error) in INVALID.iter().copied() {
            assert_de_tokens_error::<&IdentUnderTest>(&[Token::BorrowedStr(test)], error);
        }
    }
}
