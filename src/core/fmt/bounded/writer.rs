// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Segment;
use crate::syntax::segmentation::GraphemesIterator;
use crate::syntax::{CharCase, CharProfile, GraphemeCase, Segmentation, TrivialBoundary};
use core::fmt::{Display, Result, Write};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone)]
enum SegmentRun {
    Chunks,
    Delim(char),
    Decoration,
}

// -----------------------------------------------------------------------------
pub(super) struct Writer<'a, 'b, const UPPER: bool> {
    formatter: &'a mut core::fmt::Formatter<'b>,
    prior_case: Option<GraphemeCase>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, 'b, const UPPER: bool> Writer<'a, 'b, UPPER> {
    #[inline]
    pub fn new(formatter: &'a mut core::fmt::Formatter<'b>) -> Self {
        Self {
            formatter,
            prior_case: None,
        }
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result {
        self.formatter.write_char(c)
    }

    // Writes an entire string of graphemes in lowercase (if possible).
    #[inline]
    fn write_lowercase_word<S: Segmentation>(&mut self, word: &str) -> Result {
        let Some(last_grapheme) = S::Graphemes::new(word).next_back() else {
            return Ok(());
        };
        self.prior_case = Some(GraphemeCase::new_lowercased(last_grapheme));
        for c in word.chars() {
            c.to_lowercase().fmt(self.formatter)?;
        }
        Ok(())
    }

    // Writes an entire string, capitalizing the first grapheme.
    #[inline]
    fn write_capitalized_word<B: TrivialBoundary, S: Segmentation>(
        &mut self,
        word: &str,
        delim: char,
    ) -> Result {
        let mut graphemes = S::Graphemes::new(word);
        let Some(first_grapheme) = graphemes.next() else {
            return Ok(());
        };

        // See if we ought to introduce a boundary first via a delimiter.
        let curr_case = GraphemeCase::new_uppercase(first_grapheme);
        if let Some(prior_case) = self.prior_case {
            let next = graphemes.clone().next().map(GraphemeCase::new_lowercased);
            if !B::is_boundary(prior_case, curr_case, next) {
                self.write_char(delim)?;
                if !UPPER {
                    return self.write_lowercase_word::<S>(word);
                }
            }
        }

        // Capitalize the first grapheme - this is distinct from uppercase.
        {
            self.prior_case = Some(curr_case); // Start with initial guess.
            let mut chars = first_grapheme.chars();
            for c in chars.by_ref() {
                if CharCase::is_cased(c) {
                    let mut upper = c.to_uppercase();
                    self.write_char(upper.next().unwrap())?;
                    for c in upper {
                        for c in c.to_lowercase() {
                            self.write_char(c)?;
                            // This transformation is 1 grapheme -> many graphemes.
                            //
                            // Because of that, our prior "guess" was inaccurate.
                            // So we will update the guess to the last-written char.
                            //
                            // This assumes that the case mapping doesn't do something
                            // funky like have ZWJ or anything like that in the
                            // output - but it really should not.
                            //
                            // It *doesn't* account for weird combinations of code points
                            // like a prefix joiner + a 2-char uppercase mapped expansion.
                            // I refuse to account for that unless a good reason is given.
                            //
                            // You're on your own if you're constructing nightmares like that.
                            self.prior_case = Some(GraphemeCase::from_char(c));
                        }
                    }
                    break;
                }
                self.write_char(c)?;
            }
            self.write_lowercase_word::<S>(chars.as_str())?;
        }

        // Everything else is lowercase.
        self.write_lowercase_word::<S>(graphemes.as_str())
    }

    #[inline]
    pub fn write_canonical<B: TrivialBoundary, P: CharProfile>(
        mut self,
        words: &mut dyn Iterator<Item = &str>,
        mapped_delim: char,
    ) -> core::fmt::Result {
        // There must be at least one word to write as a canonical identifier.
        // Otherwise, there's nothing but delimiters - in which case, we should
        // write a single delimiter - as that is the canonical form.
        let Some(first_word) = words.next() else {
            return self.write_char(mapped_delim);
        };

        // We have to see if the first word would invalidate the target profile.
        // If it would, then we will need to start with a delimiter first.
        if first_word
            .chars()
            .next()
            .is_none_or(|c| !P::is_ident_start(c))
        {
            self.write_char(mapped_delim)?;
        }

        // The first word is always written based on the casing rules.
        match UPPER {
            true => self.write_capitalized_word::<B, P::Segmentation>(first_word, mapped_delim)?,
            false => self.write_lowercase_word::<P::Segmentation>(first_word)?,
        }

        // Remaining words are all always written using capitalization rules.
        for word in words {
            self.write_capitalized_word::<B, P::Segmentation>(word, mapped_delim)?;
        }

        Ok(())
    }
    #[inline]
    pub fn write_decorated<B: TrivialBoundary, P: CharProfile>(
        mut self,
        segments: &mut dyn Iterator<Item = Segment<char, &str>>,
        mapped_delim: char,
        alternative_delim: Option<char>,
    ) -> core::fmt::Result {
        let mut run = SegmentRun::Decoration;
        for segment in segments {
            let chunk = match segment {
                // Handle delimiter runs, and consuming the non-essential
                // delimiter. We will check later when we get to formatting the
                // next word if the delimiter was actually necessary or not.
                Segment::Chunk(chunk) => chunk,
                Segment::Delim(delim) => {
                    let delim = match alternative_delim.is_some_and(|c| c == delim) {
                        true => delim,
                        false => mapped_delim,
                    };
                    run = match run {
                        SegmentRun::Delim(prior) => {
                            self.write_char(prior)?;
                            self.write_char(delim)?;
                            self.prior_case = None;
                            SegmentRun::Decoration
                        }
                        SegmentRun::Decoration => {
                            self.write_char(delim)?;
                            SegmentRun::Decoration
                        }
                        _ => SegmentRun::Delim(delim),
                    };
                    continue;
                }
            };

            // Capitalize on chunk runs (including if there was an unnecessary delim).
            let should_capitalize = matches!(run, SegmentRun::Chunks | SegmentRun::Delim(_));
            match UPPER || should_capitalize {
                true => self.write_capitalized_word::<B, P::Segmentation>(
                    chunk,
                    match run {
                        SegmentRun::Delim(delim) => delim,
                        _ => mapped_delim,
                    },
                )?,
                false => self.write_lowercase_word::<P::Segmentation>(chunk)?,
            }

            run = SegmentRun::Chunks;
        }

        // If there was a final stray delimiter, then it was decorative.
        if let SegmentRun::Delim(delim) = run {
            self.write_char(delim)?;
        }

        Ok(())
    }
    #[inline]
    pub fn write_delimited<B: TrivialBoundary, P: CharProfile>(
        mut self,
        segments: &mut dyn Iterator<Item = Segment<char, &str>>,
        mapped_delim: char,
        alternative_delim: Option<char>,
    ) -> core::fmt::Result {
        let mut continue_chunk = false;
        for segment in segments {
            let chunk = match segment {
                // Handle delimiter runs, and consuming the non-essential
                // delimiter. We will check later when we get to formatting the
                // next word if the delimiter was actually necessary or not.
                Segment::Chunk(chunk) => chunk,
                Segment::Delim(delim) => {
                    let delim = match alternative_delim.is_some_and(|c| c == delim) {
                        true => delim,
                        false => mapped_delim,
                    };
                    self.write_char(delim)?;
                    self.prior_case = None;
                    continue_chunk = false;
                    continue;
                }
            };

            match UPPER || continue_chunk {
                true => self.write_capitalized_word::<B, P::Segmentation>(chunk, mapped_delim)?,
                false => self.write_lowercase_word::<P::Segmentation>(chunk)?,
            }

            continue_chunk = true;
        }

        Ok(())
    }
}
