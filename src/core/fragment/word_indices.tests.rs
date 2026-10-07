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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.next(), None);
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);
        Ok(())
    }

    #[test]
    fn single_word() -> Result<(), Error> {
        let chunk = FragmentUnderTest::new("test")?;
        let mut iter = chunk.word_indices();
        assert_eq!(iter.clone().last(), Some((0, ChunkUnderTest::new("test")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((0, ChunkUnderTest::new("test")?))
        );

        // Single word, no boundaries
        assert_eq!(iter.clone().type_erased().next(), Some((0, "test")));
        assert_eq!(iter.next(), Some((0, ChunkUnderTest::new("test")?)));
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.clone().last(), Some((2, ChunkUnderTest::new("test")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((2, ChunkUnderTest::new("test")?))
        );

        // Single word, no boundaries
        assert_eq!(iter.clone().type_erased().next(), Some((2, "test")));
        assert_eq!(iter.next(), Some((2, ChunkUnderTest::new("test")?)));
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some((0, "foo")));
        assert_eq!(iter.next(), Some((0, ChunkUnderTest::new("foo")?)));
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next(), Some((3, "Bar")));
        assert_eq!(iter.next(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some((6, "Baz")));
        assert_eq!(iter.next(), Some((6, ChunkUnderTest::new("Baz")?)));
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.clone().last(), Some((9, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((9, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some((2, "foo")));
        assert_eq!(iter.next(), Some((2, ChunkUnderTest::new("foo")?)));
        assert_eq!(iter.clone().last(), Some((9, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((9, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next(), Some((5, "Bar")));
        assert_eq!(iter.next(), Some((5, ChunkUnderTest::new("Bar")?)));
        assert_eq!(iter.clone().last(), Some((9, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((9, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some((9, "Baz")));
        assert_eq!(iter.next(), Some((9, ChunkUnderTest::new("Baz")?)));
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next_back(), Some((6, "Baz")));
        assert_eq!(iter.next_back(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(iter.clone().last(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((3, ChunkUnderTest::new("Bar")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some((3, "Bar")));
        assert_eq!(iter.next_back(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(iter.clone().last(), Some((0, ChunkUnderTest::new("foo")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((0, ChunkUnderTest::new("foo")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next_back(), Some((0, "foo")));
        assert_eq!(iter.next_back(), Some((0, ChunkUnderTest::new("foo")?)));
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.clone().last(), Some((9, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((9, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next_back(), Some((9, "Baz")));
        assert_eq!(iter.next_back(), Some((9, ChunkUnderTest::new("Baz")?)));
        assert_eq!(iter.clone().last(), Some((5, ChunkUnderTest::new("Bar")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((5, ChunkUnderTest::new("Bar")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some((5, "Bar")));
        assert_eq!(iter.next_back(), Some((5, ChunkUnderTest::new("Bar")?)));
        assert_eq!(iter.clone().last(), Some((2, ChunkUnderTest::new("foo")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((2, ChunkUnderTest::new("foo")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next_back(), Some((2, "foo")));
        assert_eq!(iter.next_back(), Some((2, ChunkUnderTest::new("foo")?)));
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some((0, "foo")));
        assert_eq!(iter.next(), Some((0, ChunkUnderTest::new("foo")?)));
        assert_eq!(iter.clone().last(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((6, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some((6, "Baz")));
        assert_eq!(iter.next_back(), Some((6, ChunkUnderTest::new("Baz")?)));
        assert_eq!(iter.clone().last(), Some((3, ChunkUnderTest::new("Bar")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((3, ChunkUnderTest::new("Bar")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some((3, "Bar")));
        assert_eq!(iter.next(), Some((3, ChunkUnderTest::new("Bar")?)));
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
        let mut iter = chunk.word_indices();
        assert_eq!(iter.clone().last(), Some((9, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((9, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: First call
        assert_eq!(iter.clone().type_erased().next(), Some((2, "foo")));
        assert_eq!(iter.next(), Some((2, ChunkUnderTest::new("foo")?)));
        assert_eq!(iter.clone().last(), Some((9, ChunkUnderTest::new("Baz")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((9, ChunkUnderTest::new("Baz")?))
        );

        // Multi-word, boundaries: Second call
        assert_eq!(iter.clone().type_erased().next_back(), Some((9, "Baz")));
        assert_eq!(iter.next_back(), Some((9, ChunkUnderTest::new("Baz")?)));
        assert_eq!(iter.clone().last(), Some((5, ChunkUnderTest::new("Bar")?)));
        assert_eq!(
            iter.clone().next_back(),
            Some((5, ChunkUnderTest::new("Bar")?))
        );

        // Multi-word, boundaries: Third call
        assert_eq!(iter.clone().type_erased().next(), Some((5, "Bar")));
        assert_eq!(iter.next(), Some((5, ChunkUnderTest::new("Bar")?)));
        assert_eq!(iter.clone().last(), None);
        assert_eq!(iter.clone().next_back(), None);

        // Fused Iterator
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
        Ok(())
    }
}
