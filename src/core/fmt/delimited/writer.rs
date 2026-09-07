// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Segment;
use crate::syntax::CharProfile;
use core::fmt::{Display, Result, Write};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub struct Writer<'a, 'b, const UPPER: bool> {
    formatter: &'a mut core::fmt::Formatter<'b>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, 'b, const UPPER: bool> Writer<'a, 'b, UPPER> {
    #[inline]
    pub fn new(formatter: &'a mut core::fmt::Formatter<'b>) -> Self {
        Self { formatter }
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result {
        self.formatter.write_char(c)
    }

    // Writes an entire string of graphemes in lowercase (if possible).
    #[inline]
    fn write_lowercase_word(&mut self, word: &str) -> Result {
        for c in word.chars() {
            c.to_lowercase().fmt(self.formatter)?;
        }
        Ok(())
    }

    // Writes an entire string of graphemes in uppercase (if possible).
    #[inline]
    fn write_uppercase_word(&mut self, word: &str) -> Result {
        for c in word.chars() {
            c.to_uppercase().fmt(self.formatter)?;
        }
        Ok(())
    }

    #[inline]
    pub fn write_cased_word(&mut self, word: &str) -> Result {
        match UPPER {
            true => self.write_uppercase_word(word),
            false => self.write_lowercase_word(word),
        }
    }

    #[inline]
    pub fn write_canonical<P: CharProfile>(
        &mut self,
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
        self.write_cased_word(first_word)?;

        // The remainder of the words are easy - it's repeatedly delim + chunk.
        for word in words {
            self.write_char(mapped_delim)?;
            self.write_cased_word(word)?;
        }
        Ok(())
    }

    #[inline]
    pub fn write_decorated(
        &mut self,
        segments: &mut dyn Iterator<Item = Segment<char, &str>>,
        mapped_delim: char,
    ) -> core::fmt::Result {
        let mut last_was_delim = true;
        for segment in segments {
            // For delimited identifiers, it's pretty simple - we always write
            // the delimiters (mapped to the target `mapped_delim`), then we
            // write the chunks, ensuring that there was a prior delimiter.
            let chunk = match segment {
                Segment::Chunk(chunk) => chunk,
                Segment::Delim(_delim) => {
                    self.write_char(mapped_delim)?;
                    last_was_delim = true;
                    continue;
                }
            };

            // If there was no prior delimiter, then just write one, and then
            // format the next chunk and append it onto the identifier.
            if !last_was_delim {
                self.write_char(mapped_delim)?;
            }
            self.write_cased_word(chunk)?;
            last_was_delim = false;
        }
        Ok(())
    }
}
