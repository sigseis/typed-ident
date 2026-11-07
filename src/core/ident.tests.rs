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
const INVALID: &[&str] = &["", "01234", "日本語", "-test", "test-", "test-test"];

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
        for test in INVALID {
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
    fn serde_boxed_valid() {
        use std_alloc::boxed::Box;
        for test in VALID.iter().copied() {
            // It's a valid value, so we should be able to construct it.
            let value = IdentUnderTest::new_boxed(test).unwrap();

            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(&value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // Validate that it deserializes as expected.
            let deserialized: Box<IdentUnderTest> = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_boxed_invalid() {
        use std_alloc::boxed::Box;
        use std_alloc::string::ToString;
        for test in INVALID.iter().copied() {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<Box<IdentUnderTest>, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert!(
                match test {
                    "" => error == "empty identifier",
                    _s => error.starts_with("invalid format: at byte offset"),
                },
                "actual error: {error}"
            );
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_cow_valid() {
        use std_alloc::borrow::Cow;
        for test in VALID.iter().copied() {
            // It's a valid value, so we should be able to construct it.
            let value = Cow::Borrowed(IdentUnderTest::new(test).unwrap());

            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(&value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // Validate that it deserializes as expected.
            let deserialized: Cow<IdentUnderTest> = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_cow_invalid() {
        use std_alloc::borrow::Cow;
        use std_alloc::string::ToString;
        for test in INVALID.iter().copied() {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<Cow<IdentUnderTest>, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert!(
                match test {
                    "" => error == "empty identifier",
                    _s => error.starts_with("invalid format: at byte offset"),
                },
                "actual error: {error}"
            );
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_ref_valid() {
        for test in VALID.iter().copied() {
            // It's a valid value, so we should be able to construct it.
            let value = IdentUnderTest::new(test).unwrap();

            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // Validate that it deserializes as expected.
            let deserialized: &IdentUnderTest = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_ref_invalid() {
        use std_alloc::string::ToString;
        for test in INVALID.iter().copied() {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<&IdentUnderTest, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert!(
                match test {
                    "" => error == "empty identifier",
                    _s => error.starts_with("invalid format: at byte offset"),
                },
                "actual error: {error}"
            );
        }
    }
}
