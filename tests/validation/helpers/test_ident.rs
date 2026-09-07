// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::helpers::{Boundaries, Boundary, Format, SegmentRule};
use typed_ident::alloc::StringSegment;
use unicode_general_category::{GeneralCategory, get_general_category};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TestIdent {
    pub name: &'static str,
    pub format: Format,
    pub segments: &'static [SegmentRule],
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl TestIdent {
    pub fn build_string(&self) -> String {
        let mut string = String::new();
        for segment in self.segments {
            match segment {
                SegmentRule::Bound(_) => (),
                SegmentRule::Chunk(c) => string.push_str(c),
                SegmentRule::Delim(d) => string.push(*d),
            }
        }
        string
    }

    // Charset Compatibility
    pub fn is_ascii(&self) -> bool {
        self.segments
            .iter()
            .filter_map(|s| match s {
                SegmentRule::Chunk(c) => Some(c),
                _ => None,
            })
            .all(|c| c.is_ascii())
    }
    pub fn is_strict(&self) -> bool {
        let mut start_chunk_run = true;
        for segment in self.segments {
            match segment {
                SegmentRule::Chunk(chunk) if start_chunk_run => {
                    if chunk.chars().next().is_some_and(|c| {
                        matches!(
                            get_general_category(c),
                            GeneralCategory::EnclosingMark
                                | GeneralCategory::SpacingMark
                                | GeneralCategory::NonspacingMark,
                        )
                    }) {
                        return false;
                    }
                    start_chunk_run = false;
                }
                SegmentRule::Delim(_) => {
                    start_chunk_run = true;
                }
                _ => continue,
            }
        }
        true
    }
    pub fn is_unicode(&self) -> bool {
        true
    }

    // Identifier Formats
    pub fn is_camel(&self) -> bool {
        !self
            .segments
            .iter()
            .filter(|s| matches!(s, SegmentRule::Delim(_)))
            .any(|s| !matches!(s, SegmentRule::Delim('_')))
    }
    pub fn is_hybrid(&self) -> bool {
        !self
            .segments
            .iter()
            .filter(|s| matches!(s, SegmentRule::Delim(_)))
            .any(|s| !matches!(s, SegmentRule::Delim('_' | '-')))
    }
    pub fn is_kebab(&self) -> bool {
        !self
            .segments
            .iter()
            .filter(|s| matches!(s, SegmentRule::Delim(_)))
            .any(|s| !matches!(s, SegmentRule::Delim('-')))
    }
    pub fn is_snake(&self) -> bool {
        self.is_camel()
    }

    // Specific Identifier Formats
    pub fn is_lower_camel(&self) -> bool {
        self.is_camel() && matches!(self.format, Format::Lower | Format::LowerCamel)
    }
    pub fn is_upper_camel(&self) -> bool {
        self.is_camel() && matches!(self.format, Format::Upper | Format::UpperCamel)
    }
    pub fn is_lower_hybrid(&self) -> bool {
        self.is_hybrid() && matches!(self.format, Format::Lower | Format::LowerCamel)
    }
    pub fn is_upper_hybrid(&self) -> bool {
        self.is_hybrid() && matches!(self.format, Format::Upper | Format::UpperCamel)
    }
    pub fn is_lower_kebab(&self) -> bool {
        self.is_kebab() && matches!(self.format, Format::Lower)
    }
    pub fn is_upper_kebab(&self) -> bool {
        self.is_kebab() && matches!(self.format, Format::Upper)
    }
    pub fn is_lower_snake(&self) -> bool {
        self.is_snake() && matches!(self.format, Format::Lower)
    }
    pub fn is_upper_snake(&self) -> bool {
        self.is_snake() && matches!(self.format, Format::Upper)
    }

    pub fn chunked_segments(&self) -> Vec<StringSegment> {
        self.segments(&Boundaries::default())
    }
    pub fn segments(&self, boundaries: &Boundaries) -> Vec<StringSegment> {
        let mut segments = Vec::with_capacity(self.segments.len());
        let mut curr_chunk = String::new();
        for s in self.segments {
            match s {
                SegmentRule::Chunk(c) => curr_chunk.push_str(c),
                SegmentRule::Delim(c) => {
                    if !curr_chunk.is_empty() {
                        segments.push(StringSegment::Chunk(curr_chunk.clone()));
                        curr_chunk.clear();
                    }
                    segments.push(StringSegment::Delim(*c));
                }
                SegmentRule::Bound(b) if !curr_chunk.is_empty() => {
                    let split = match b {
                        Boundary::Camel => boundaries.camel,
                        Boundary::DigitToLower => boundaries.digit_to_lower,
                        Boundary::DigitToUpper => boundaries.digit_to_upper,
                        Boundary::Hat => boundaries.hat,
                        Boundary::LowerToDigit => boundaries.lower_to_digit,
                        Boundary::UpperToDigit => boundaries.upper_to_digit,
                    };
                    if split {
                        segments.push(StringSegment::Chunk(curr_chunk.clone()));
                        curr_chunk.clear();
                    }
                }
                _ => (),
            }
        }
        if !curr_chunk.is_empty() {
            segments.push(StringSegment::Chunk(curr_chunk));
        }
        segments
    }
    pub fn segment_indices(&self, boundaries: &Boundaries) -> Vec<(usize, StringSegment)> {
        let segments = self.segments(boundaries);
        let mut segment_indices = Vec::with_capacity(segments.len());
        let mut idx = 0;
        for segment in segments {
            let curr_idx = idx;
            idx += segment.len();
            segment_indices.push((curr_idx, segment));
        }
        segment_indices
    }
    pub fn chunked_segment_indices(&self) -> Vec<(usize, StringSegment)> {
        self.segment_indices(&Boundaries::default())
    }
}

impl std::fmt::Display for TestIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}
