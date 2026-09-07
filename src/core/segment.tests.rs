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

// -----------------------------------------------------------------------------
type SegmentUnderTest<'a> = Segment<delimiter::LowLine, &'a ChunkUnderTest>;

// -----------------------------------------------------------------------------
type AltSegmentUnderTest<'a> = Segment<delimiter::HyphenMinus, &'a ChunkUnderTest>;

// =============================================================================
// TESTS: TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
mod traits {
    use super::*;

    #[test]
    fn eq_both_delim() -> Result<(), Error> {
        let seg1 = SegmentUnderTest::Delim(delimiter::LowLine);
        let seg2 = SegmentUnderTest::Delim(delimiter::LowLine);

        assert_eq!(seg1, seg2);
        Ok(())
    }

    #[test]
    fn eq_different_delims() -> Result<(), Error> {
        let seg1 = SegmentUnderTest::Delim(delimiter::LowLine);
        let seg2 = AltSegmentUnderTest::Delim(delimiter::HyphenMinus);

        assert_ne!(seg1, seg2);
        Ok(())
    }

    #[test]
    fn eq_both_chunk() -> Result<(), Error> {
        let chunk1 = ChunkUnderTest::new("hello")?;
        let chunk2 = ChunkUnderTest::new("hello")?;

        let seg1 = SegmentUnderTest::Chunk(chunk1);
        let seg2 = SegmentUnderTest::Chunk(chunk2);

        assert_eq!(seg1, seg2);
        Ok(())
    }

    #[test]
    fn eq_different_chunks() -> Result<(), Error> {
        let chunk1 = ChunkUnderTest::new("hello")?;
        let chunk2 = ChunkUnderTest::new("world")?;

        let seg1 = SegmentUnderTest::Chunk(chunk1);
        let seg2 = SegmentUnderTest::Chunk(chunk2);

        assert_ne!(seg1, seg2);
        Ok(())
    }

    #[test]
    fn eq_mixed() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("hello")?;

        let delim_seg = SegmentUnderTest::Delim(delimiter::LowLine);
        let chunk_seg = SegmentUnderTest::Chunk(chunk);

        assert_ne!(delim_seg, chunk_seg);
        Ok(())
    }

    #[test]
    fn partial_ord_both_delim() -> Result<(), Error> {
        let seg1 = AltSegmentUnderTest::Delim(delimiter::HyphenMinus);
        let seg2 = SegmentUnderTest::Delim(delimiter::LowLine);

        assert!(seg1 < seg2);
        Ok(())
    }

    #[test]
    fn partial_ord_both_chunk() -> Result<(), Error> {
        let chunk1 = ChunkUnderTest::new("abc")?;
        let chunk2 = ChunkUnderTest::new("def")?;

        let seg1 = SegmentUnderTest::Chunk(chunk1);
        let seg2 = SegmentUnderTest::Chunk(chunk2);

        assert!(seg1 < seg2);
        Ok(())
    }

    #[test]
    fn partial_ord_delim_vs_chunk() -> Result<(), Error> {
        let seg1 = SegmentUnderTest::Delim(delimiter::LowLine);
        let chunk = ChunkUnderTest::new("a")?;
        let seg2 = SegmentUnderTest::Chunk(chunk);

        // Delims are less than chunks
        assert!(seg1 < seg2);
        Ok(())
    }

    #[test]
    fn partial_ord_chunk_vs_delim() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("a")?;
        let seg1 = SegmentUnderTest::Chunk(chunk);
        let seg2 = SegmentUnderTest::Delim(delimiter::LowLine);

        // Chunks are greater than delims
        assert!(seg1 > seg2);
        Ok(())
    }

    #[test]
    fn partial_ord_different_types_delim_vs_chunk() -> Result<(), Error> {
        let seg1 = SegmentUnderTest::Delim(delimiter::LowLine);
        let chunk = ChunkUnderTest::new("a")?;
        let seg2 = AltSegmentUnderTest::Chunk(chunk);

        // Delims are less than chunks
        assert!(seg1 < seg2);
        Ok(())
    }

    #[test]
    fn partial_ord_different_types_chunk_vs_delim() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("a")?;
        let seg1 = SegmentUnderTest::Chunk(chunk);
        let seg2 = AltSegmentUnderTest::Delim(delimiter::HyphenMinus);

        // Chunks are greater than delims
        assert!(seg1 > seg2);
        Ok(())
    }

    #[test]
    fn as_ref_str() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("hello")?;
        let segment = SegmentUnderTest::Chunk(chunk);

        assert_eq!(segment.as_ref(), "hello");
        Ok(())
    }

    #[test]
    fn from_delimiter() -> Result<(), Error> {
        let segment: SegmentUnderTest = delimiter::LowLine.into();
        assert_eq!(segment.delim(), Some(delimiter::LowLine));
        Ok(())
    }

    #[test]
    fn from_chunk() -> Result<(), Error> {
        let chunk = ChunkUnderTest::new("hello")?;
        let segment: SegmentUnderTest = chunk.into();
        assert_eq!(segment.chunk(), Some(chunk));
        Ok(())
    }

    #[test]
    fn try_from_str_empty_chunk() -> Result<(), Error> {
        let segment = SegmentUnderTest::try_from("")?;
        assert!(segment.is_chunk());
        assert_eq!(segment.as_str(), "");
        Ok(())
    }

    #[test]
    fn try_from_str_valid_chunk() -> Result<(), Error> {
        let segment = SegmentUnderTest::try_from("hello")?;
        assert!(segment.is_chunk());
        assert_eq!(segment.as_str(), "hello");
        Ok(())
    }

    #[test]
    fn try_from_str_valid_delim() -> Result<(), Error> {
        let segment = SegmentUnderTest::try_from("_")?;
        assert!(segment.is_delim());
        assert_eq!(segment.as_str(), "_");
        Ok(())
    }

    #[test]
    fn try_from_str_invalid() -> Result<(), Error> {
        // Chunks aren't allowed to contain delimiters.
        assert!(SegmentUnderTest::try_from("_hello").is_err());
        assert!(SegmentUnderTest::try_from("hello_").is_err());
        assert!(SegmentUnderTest::try_from("he_llo").is_err());

        // Plus, you can't have more than one delimiter in a segment.
        assert!(SegmentUnderTest::try_from("__").is_err());
        Ok(())
    }
}
