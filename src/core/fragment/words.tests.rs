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
type ChunkUnderTest = Chunk<boundary::Standard, delimiter::LowLine, profile::Mixed<profile::Ascii>>;
type FragmentUnderTest =
    Fragment<boundary::Standard, delimiter::LowLine, profile::Mixed<profile::Ascii>>;

// =============================================================================
// TESTS: IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod impls {
    use super::*;

    #[test]
    fn empty() -> Result<(), Error> {
        let chunk = FragmentUnderTest::new("")?;
        let mut iter = chunk.words();
        assert_eq!(iter.next(), None);
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);
        Ok(())
    }

    #[test]
    fn single_word() -> Result<(), Error> {
        let chunk = FragmentUnderTest::new("test")?;
        let mut iter = chunk.words();
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("test")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("test")?));

        // Single word, no boundaries
        assert_eq!(iter.clone().type_erased().next(), Some("test"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("test")?));
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        Ok(())
    }

    #[test]
    fn single_word_with_delims() -> Result<(), Error> {
        let chunk = FragmentUnderTest::new("__test__")?;
        let mut iter = chunk.words();
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("test")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("test")?));

        // Single word, no boundaries
        assert_eq!(iter.clone().type_erased().next(), Some("test"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("test")?));
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
        let chunk = FragmentUnderTest::new("fooBarBaz")?;
        let mut iter = chunk.words();
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some("foo"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next(), Some("Bar"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some("Baz"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        Ok(())
    }

    #[test]
    fn multi_word_with_delims() -> Result<(), Error> {
        let chunk = FragmentUnderTest::new("__fooBar_Baz__")?;
        let mut iter = chunk.words();
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some("foo"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next(), Some("Bar"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some("Baz"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Baz")?));
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
        let chunk = FragmentUnderTest::new("fooBarBaz")?;
        let mut iter = chunk.words();
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Baz"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Bar")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Bar"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("foo")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next_back(), Some("foo"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        Ok(())
    }

    #[test]
    fn multi_word_reverse_with_delims() -> Result<(), Error> {
        let chunk = FragmentUnderTest::new("__fooBar_Baz__")?;
        let mut iter = chunk.words();
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Baz"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Bar")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Bar"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("foo")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next_back(), Some("foo"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("foo")?));
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
        let chunk = FragmentUnderTest::new("fooBarBaz")?;
        let mut iter = chunk.words();
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some("foo"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Baz"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Bar")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some("Bar"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        Ok(())
    }

    #[test]
    fn multi_word_zig_zag_with_delims() -> Result<(), Error> {
        let chunk = FragmentUnderTest::new("__fooBar_Baz__")?;
        let mut iter = chunk.words();
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some("foo"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("foo")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Baz")?));

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some("Baz"));
        assert_eq!(iter.next_back(), Some(ChunkUnderTest::new("Baz")?));
        assert_eq!(iter.clone().last(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().next_back(), Some(ChunkUnderTest::new("Bar")?));

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some("Bar"));
        assert_eq!(iter.next(), Some(ChunkUnderTest::new("Bar")?));
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        Ok(())
    }
}
