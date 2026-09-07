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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.offset(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);
        Ok(())
    }

    #[test]
    fn single_word() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("test")?;
        let mut iter = chunk.word_indices();
        assert_eq!(iter.as_str(), "test");
        assert_eq!(iter.as_fragment(), "test");
        assert_eq!(iter.as_chunk(), "test");
        assert_eq!(iter.offset(), 0);
        assert_eq!(iter.clone().last(), Some((0, ChunkUnderTest::new("test")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((0, ChunkUnderTest::new("test")?))
        );

        // Single word, no boundaries
        assert_eq!(iter.clone().type_erased().next(), Some((0, "test")));
        assert_eq!(iter.next(), Some((0, ChunkUnderTest::new("test")?)));
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.offset(), 4);
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.as_str(), "fooBarBaz");
        assert_eq!(iter.as_fragment(), "fooBarBaz");
        assert_eq!(iter.as_chunk(), "fooBarBaz");
        assert_eq!(iter.offset(), 0);
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some((0, "foo")));
        assert_eq!(iter.next(), Some((0, ChunkUnderTest::new("foo")?)));
        assert_eq!(iter.as_str(), "BarBaz");
        assert_eq!(iter.as_fragment(), "BarBaz");
        assert_eq!(iter.as_chunk(), "BarBaz");
        assert_eq!(iter.offset(), 3);
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next(), Some((3, "Bar")));
        assert_eq!(iter.next(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(iter.as_str(), "Baz");
        assert_eq!(iter.as_fragment(), "Baz");
        assert_eq!(iter.as_chunk(), "Baz");
        assert_eq!(iter.offset(), 6);
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some((6, "Baz")));
        assert_eq!(iter.next(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.as_fragment(), "");
        assert_eq!(iter.as_chunk(), "");
        assert_eq!(iter.offset(), 9);
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.as_str(), "fooBarBaz");
        assert_eq!(iter.as_fragment(), "fooBarBaz");
        assert_eq!(iter.as_chunk(), "fooBarBaz");
        assert_eq!(iter.offset(), 0);
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next_back(), Some((6, "Baz")));
        assert_eq!(iter.next_back(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(iter.as_str(), "fooBar");
        assert_eq!(iter.as_fragment(), "fooBar");
        assert_eq!(iter.as_chunk(), "fooBar");
        assert_eq!(iter.offset(), 0);
        assert_eq!(iter.clone().last(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((3, ChunkUnderTest::new("Bar")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some((3, "Bar")));
        assert_eq!(iter.next_back(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(iter.as_str(), "foo");
        assert_eq!(iter.as_fragment(), "foo");
        assert_eq!(iter.as_chunk(), "foo");
        assert_eq!(iter.offset(), 0);
        assert_eq!(iter.clone().last(), Some((0, ChunkUnderTest::new("foo")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((0, ChunkUnderTest::new("foo")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next_back(), Some((0, "foo")));
        assert_eq!(iter.next_back(), Some((0, ChunkUnderTest::new("foo")?)));
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.as_fragment(), "");
        assert_eq!(iter.as_chunk(), "");
        assert_eq!(iter.offset(), 0);
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.as_str(), "fooBarBaz");
        assert_eq!(iter.as_fragment(), "fooBarBaz");
        assert_eq!(iter.as_chunk(), "fooBarBaz");
        assert_eq!(iter.offset(), 0);
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some((0, "foo")));
        assert_eq!(iter.next(), Some((0, ChunkUnderTest::new("foo")?)));
        assert_eq!(iter.as_str(), "BarBaz");
        assert_eq!(iter.as_fragment(), "BarBaz");
        assert_eq!(iter.as_chunk(), "BarBaz");
        assert_eq!(iter.offset(), 3);
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some((6, "Baz")));
        assert_eq!(iter.next_back(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(iter.as_str(), "Bar");
        assert_eq!(iter.as_fragment(), "Bar");
        assert_eq!(iter.as_chunk(), "Bar");
        assert_eq!(iter.offset(), 3);
        assert_eq!(iter.clone().last(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((3, ChunkUnderTest::new("Bar")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some((3, "Bar")));
        assert_eq!(iter.next(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(iter.as_str(), "");
        assert_eq!(iter.as_fragment(), "");
        assert_eq!(iter.as_chunk(), "");
        assert_eq!(iter.offset(), 6);
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        Ok(())
    }
}
