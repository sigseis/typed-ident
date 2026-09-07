// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
use crate::Error;
use crate::syntax::*;

// =============================================================================
// CONFIGURATION
// =============================================================================

// -----------------------------------------------------------------------------
type ChunkUnderTest = Chunk<boundary::Standard, delimiter::LowLine, profile::Ascii>;

// =============================================================================
// TESTS: IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod impls {
    use super::*;

    #[test]
    fn empty() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("")?;
        let mut iter = chunk.words();
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);
        Ok(())
    }

    #[test]
    fn single_word() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        let mut iter = chunk.words();
        assert_eq!(iter.as_str(), "test");
        assert_eq!(iter.as_fragment(), "test");
        assert_eq!(iter.as_chunk(), "test");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("test")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("test")?));

        // Single word, no boundaries
        assert_eq!(iter.clone().type_erased().next(), Some("test"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("test")?));
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        Ok(())
    }

    #[test]
    fn multi_word() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("fooBarBaz")?;
        let mut iter = chunk.words();
        assert_eq!(iter.as_str(), "fooBarBaz");
        assert_eq!(iter.as_fragment(), "fooBarBaz");
        assert_eq!(iter.as_chunk(), "fooBarBaz");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some("foo"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.as_str(), "BarBaz");
        assert_eq!(iter.as_fragment(), "BarBaz");
        assert_eq!(iter.as_chunk(), "BarBaz");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next(), Some("Bar"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.as_str(), "Baz");
        assert_eq!(iter.as_fragment(), "Baz");
        assert_eq!(iter.as_chunk(), "Baz");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some("Baz"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.as_fragment(), "");
        assert_eq!(iter.as_chunk(), "");
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        Ok(())
    }

    #[test]
    fn multi_word_reverse() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("fooBarBaz")?;
        let mut iter = chunk.words();
        assert_eq!(iter.as_str(), "fooBarBaz");
        assert_eq!(iter.as_fragment(), "fooBarBaz");
        assert_eq!(iter.as_chunk(), "fooBarBaz");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Baz"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.as_str(), "fooBar");
        assert_eq!(iter.as_fragment(), "fooBar");
        assert_eq!(iter.as_chunk(), "fooBar");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Bar")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Bar"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.as_str(), "foo");
        assert_eq!(iter.as_fragment(), "foo");
        assert_eq!(iter.as_chunk(), "foo");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("foo")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next_back(), Some("foo"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.as_fragment(), "");
        assert_eq!(iter.as_chunk(), "");
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        Ok(())
    }

    #[test]
    fn multi_word_zig_zag() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("fooBarBaz")?;
        let mut iter = chunk.words();
        assert_eq!(iter.as_str(), "fooBarBaz");
        assert_eq!(iter.as_fragment(), "fooBarBaz");
        assert_eq!(iter.as_chunk(), "fooBarBaz");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some("foo"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.as_str(), "BarBaz");
        assert_eq!(iter.as_fragment(), "BarBaz");
        assert_eq!(iter.as_chunk(), "BarBaz");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Baz"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.as_str(), "Bar");
        assert_eq!(iter.as_fragment(), "Bar");
        assert_eq!(iter.as_chunk(), "Bar");
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Bar")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some("Bar"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.as_fragment(), "");
        assert_eq!(iter.as_chunk(), "");
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        Ok(())
    }
}
