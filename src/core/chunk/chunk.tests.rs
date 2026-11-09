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
const FRAGMENT_VALID: &[(&str, &str)] = &[
    ("_test", "invalid format: at byte offset 0"),
    ("test_", "invalid format: at byte offset 4"),
    ("test_test", "invalid format: at byte offset 4"),
    ("_", "invalid format: at byte offset 0"),
];

// -----------------------------------------------------------------------------
const INVALID: &[(&str, &str)] = &[
    // Since the chunk being tested is ASCII only.
    ("日本語", "invalid format: at byte offset 0"),
];

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
        for (test, _) in FRAGMENT_VALID.iter().chain(INVALID) {
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
        for (test, _) in FRAGMENT_VALID {
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
    fn serde_boxed_round_trip_json() -> Result<(), serde_json::Error> {
        use std_alloc::boxed::Box;
        for test in VALID.iter().copied() {
            let value = ChunkUnderTest::new_boxed(test).unwrap();
            let ser = serde_json::to_string(&value)?;
            let de: Box<ChunkUnderTest> = serde_json::from_str(&ser)?;
            assert_eq!(de, value);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_boxed_tokens_valid() -> Result<(), Error> {
        use serde_test::{Token, assert_tokens};
        for test in VALID.iter().copied() {
            let value = ChunkUnderTest::new_boxed(test)?;
            assert_tokens(&value, &[Token::String(test)]);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_boxed_tokens_invalid() {
        use serde_test::{Token, assert_de_tokens_error};
        use std_alloc::boxed::Box;
        for (test, error) in FRAGMENT_VALID.iter().chain(INVALID) {
            assert_de_tokens_error::<Box<ChunkUnderTest>>(&[Token::String(test)], error);
        }
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_cow_round_trip_json() -> Result<(), serde_json::Error> {
        use std_alloc::borrow::Cow;
        for test in VALID.iter().copied() {
            let value = Cow::Borrowed(ChunkUnderTest::new(test).unwrap());
            let ser = serde_json::to_string(&value)?;
            let de: Cow<ChunkUnderTest> = serde_json::from_str(&ser)?;
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
            let value = Cow::Borrowed(ChunkUnderTest::new(test)?);
            assert_tokens(&value, &[Token::String(test)]);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_cow_tokens_invalid() {
        use serde_test::{Token, assert_de_tokens_error};
        use std_alloc::borrow::Cow;
        for (test, error) in FRAGMENT_VALID.iter().chain(INVALID) {
            assert_de_tokens_error::<Cow<ChunkUnderTest>>(&[Token::String(test)], error);
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_ref_round_trip_json() -> Result<(), serde_json::Error> {
        for test in VALID.iter().copied() {
            let value = ChunkUnderTest::new(test).unwrap();
            let ser = serde_json::to_string(&value)?;
            let de: &ChunkUnderTest = serde_json::from_str(&ser)?;
            assert_eq!(de, value);
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_ref_tokens_valid() -> Result<(), Error> {
        use serde_test::{Token, assert_tokens};
        for test in VALID.iter().copied() {
            let value = ChunkUnderTest::new(test)?;
            assert_tokens(&value, &[Token::BorrowedStr(test)]);
        }
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "alloc", feature = "serde"))]
    fn serde_ref_tokens_invalid() {
        use serde_test::{Token, assert_de_tokens_error};
        for (test, error) in FRAGMENT_VALID.iter().chain(INVALID) {
            assert_de_tokens_error::<&ChunkUnderTest>(&[Token::BorrowedStr(test)], error);
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
        for (test, _) in FRAGMENT_VALID {
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
        for (test, _) in FRAGMENT_VALID.iter().chain(INVALID) {
            let chunk: Result<&ChunkUnderTest, Error> = TryFrom::try_from(*test);
            assert!(chunk.is_err());
        }
        Ok(())
    }
}
