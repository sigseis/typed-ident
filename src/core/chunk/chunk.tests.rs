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
const VALID: &[&str] = &["", "test", "01234"];

// -----------------------------------------------------------------------------
const FRAGMENT_VALID: &[&str] = &["_test", "test_", "test_test", "_"];

// -----------------------------------------------------------------------------
const INVALID: &[&str] = &["日本語"]; // We are testing ASCII

// =============================================================================
// CONFIGURATION
// =============================================================================

// -----------------------------------------------------------------------------
type ChunkUnderTest = Chunk<boundary::Standard, delimiter::LowLine, profile::Mixed<profile::Ascii>>;

// -----------------------------------------------------------------------------
type FragmentUnderTest =
    Fragment<boundary::Standard, delimiter::LowLine, profile::Mixed<profile::Ascii>>;

// =============================================================================
// TESTS: IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod impls {
    use super::*;

    #[test]
    fn new_valid() -> Result<(), Error> {
        for test in VALID {
            let chunk = ChunkUnderTest::new(test)?;
            assert_eq!(chunk.is_empty(), test.is_empty());
            assert_eq!(chunk, *test);
        }
        Ok(())
    }

    #[test]
    fn new_invalid() -> Result<(), Error> {
        for test in FRAGMENT_VALID.iter().chain(INVALID) {
            assert!(ChunkUnderTest::new(test).is_err());
        }
        Ok(())
    }

    #[test]
    fn from_fragment_valid() -> Result<(), Error> {
        for test in VALID {
            let fragment = FragmentUnderTest::new(test)?;
            let chunk = ChunkUnderTest::from_fragment(fragment)?;
            assert_eq!(chunk, *test);
        }
        Ok(())
    }

    #[test]
    fn from_fragment_invalid() -> Result<(), Error> {
        for test in FRAGMENT_VALID {
            let fragment = FragmentUnderTest::new(test)?;
            assert!(ChunkUnderTest::from_fragment(fragment).is_err());
        }
        Ok(())
    }

    #[test]
    fn as_fragment() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        let fragment = chunk.as_fragment();
        assert_eq!(fragment.as_str(), "test");
        Ok(())
    }

    #[test]
    fn as_str() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        assert_eq!(chunk.as_str(), "test");
        Ok(())
    }

    #[test]
    fn word_indices_single_word() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        let mut words = chunk.word_indices().type_erased();

        // Single word with no boundaries
        assert_eq!(words.next().unwrap(), (0, "test"));
        assert_eq!(words.next(), None);
        Ok(())
    }

    #[test]
    fn word_indices_multiple_words() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("UpperCamelCase")?;
        let mut words = chunk.word_indices().type_erased();

        // Multiple words with boundaries
        assert_eq!(words.next().unwrap(), (0, "Upper"));
        assert_eq!(words.next().unwrap(), (5, "Camel"));
        assert_eq!(words.next().unwrap(), (10, "Case"));
        assert_eq!(words.next(), None);
        Ok(())
    }

    #[test]
    fn words_single_word() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        let mut words = chunk.words();

        assert_eq!(words.next().unwrap(), "test");
        Ok(())
    }

    #[test]
    fn words_multiple_words() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("UpperCamelCase")?;
        let mut words = chunk.words().type_erased();

        assert_eq!(words.next().unwrap(), "Upper");
        assert_eq!(words.next().unwrap(), "Camel");
        assert_eq!(words.next().unwrap(), "Case");
        Ok(())
    }
}

// =============================================================================
// TESTS: traits
// =============================================================================

// -----------------------------------------------------------------------------
mod traits {
    use super::*;

    #[test]
    #[cfg(feature = "unicode")]
    fn as_ref_chunk() -> Result<(), Error> {
        crate::macros::test_as_ref! {
            from=Chunk,
            into=Chunk,
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "unicode")]
    fn as_ref_fragment() -> Result<(), Error> {
        crate::macros::test_as_ref! {
            from=Chunk,
            into=Fragment,
        }
        Ok(())
    }

    #[test]
    fn deref() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        let _fragment: &FragmentUnderTest = chunk; // Auto-deref
        Ok(())
    }

    #[test]
    fn eq_fragment() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        let fragment = FragmentUnderTest::new("test")?;

        assert_eq!(chunk, fragment);
        Ok(())
    }

    #[test]
    fn fragment_eq_chunk() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        let fragment = FragmentUnderTest::new("test")?;

        assert_eq!(fragment, chunk);
        Ok(())
    }

    #[test]
    fn partial_ord_fragment() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("abc")?;
        let fragment = FragmentUnderTest::new("def")?;

        assert!(chunk < fragment);
        Ok(())
    }

    #[test]
    fn partial_ord_chunk() -> Result<(), Error> {
        let chunk1 = ChunkUnderTest::new("abc")?;
        let chunk2 = ChunkUnderTest::new("def")?;

        assert!(chunk1 < chunk2);
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_boxed_valid() {
        use std_alloc::boxed::Box;
        for test in VALID.iter().copied() {
            // It's a valid value, so we should be able to construct it.
            let value = ChunkUnderTest::new_boxed(test).unwrap();

            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(&value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // Validate that it deserializes as expected.
            let deserialized: Box<ChunkUnderTest> = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_boxed_invalid() {
        use std_alloc::boxed::Box;
        use std_alloc::string::ToString;
        for test in FRAGMENT_VALID.iter().chain(INVALID).copied() {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<Box<ChunkUnderTest>, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert!(
                error.starts_with("invalid format: at byte offset"),
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
            let value = Cow::Borrowed(ChunkUnderTest::new(test).unwrap());

            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(&value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // Validate that it deserializes as expected.
            let deserialized: Cow<ChunkUnderTest> = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_cow_invalid() {
        use std_alloc::borrow::Cow;
        use std_alloc::string::ToString;
        for test in FRAGMENT_VALID.iter().chain(INVALID).copied() {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<Cow<ChunkUnderTest>, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert!(
                error.starts_with("invalid format: at byte offset"),
                "actual error: {error}"
            );
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_ref_valid() {
        for test in VALID.iter().copied() {
            // It's a valid value, so we should be able to construct it.
            let value = ChunkUnderTest::new(test).unwrap();

            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(value).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // Validate that it deserializes as expected.
            let deserialized: &ChunkUnderTest = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_ref_invalid() {
        use std_alloc::string::ToString;
        for test in FRAGMENT_VALID.iter().chain(INVALID).copied() {
            // Validate that it serializes as expected.
            let json_value = serde_json::to_value(test).unwrap();
            let serialized = serde_json::to_string(&json_value).unwrap();
            assert_eq!(json_value.as_str(), Some(test));

            // This is an invalid value, so it should fail deserialization.
            let deserialized: Result<&ChunkUnderTest, serde_json::Error> =
                serde_json::from_str(&serialized);
            let error = deserialized.unwrap_err().to_string();
            assert!(
                error.starts_with("invalid format: at byte offset"),
                "actual error: {error}"
            );
        }
    }

    #[test]
    fn str_eq_chunk() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        assert_eq!("test", chunk);
        assert_ne!("other", chunk);
        Ok(())
    }

    #[test]
    fn chunk_str_partial_ord() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("abc")?;
        assert!("abc" <= chunk);
        assert!("def" > chunk);
        Ok(())
    }

    #[test]
    fn default() -> Result<(), Error> {
        let chunk: &ChunkUnderTest = Default::default();
        assert!(chunk.is_empty());
        assert_eq!(chunk, "");
        Ok(())
    }

    #[test]
    fn from_fragment_try_from() -> Result<(), Error> {
        for test in VALID {
            let fragment = FragmentUnderTest::new(test)?;
            let chunk: &ChunkUnderTest = TryFrom::try_from(fragment)?;
            assert_eq!(chunk.as_str(), *test);
        }
        for test in FRAGMENT_VALID {
            let fragment = FragmentUnderTest::new(test)?;
            let chunk: Result<&ChunkUnderTest, Error> = TryFrom::try_from(fragment);
            assert!(chunk.is_err());
        }
        Ok(())
    }

    #[test]
    fn from_str_try_from() -> Result<(), Error> {
        for test in VALID {
            let chunk: &ChunkUnderTest = TryFrom::try_from(*test)?;
            assert_eq!(chunk.as_str(), *test);
        }
        for test in FRAGMENT_VALID.iter().chain(INVALID) {
            let chunk: Result<&ChunkUnderTest, Error> = TryFrom::try_from(*test);
            assert!(chunk.is_err());
        }
        Ok(())
    }
}
