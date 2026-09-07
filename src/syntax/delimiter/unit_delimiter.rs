// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::delimiter::{AppendClosed, Delimiter};

// =============================================================================
// TYPES
// =============================================================================

/// A trait that should be implemented when a delimiter has exactly one value,
/// and that value is valid at any position of the identifier.
pub trait UnitDelimiter: Delimiter + Default {
    /// The character value of the delimiter.
    const CHAR: char;

    /// A string slice representation of the delimiter.
    const STR: &'static str;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D: UnitDelimiter> Delimiter for D {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Identifier;

    #[inline(always)]
    fn as_char(&self) -> char {
        D::CHAR
    }

    #[inline]
    fn find_delimiter(fragment: &str) -> Option<usize> {
        // JUSTIFICATION: Bench-tested, and scanning bytes is an improvement.
        //
        // Note this is safe, actually! Even in the presence of UTF-8 data.
        // Because scanning for ASCII codes cannot match with a part of a UTF-8
        // sequence which is not itself just a raw ASCII code point.
        if D::CHAR.is_ascii() {
            fragment.as_bytes().iter().position(|c| *c == D::CHAR as u8)
        } else {
            fragment.find(D::CHAR)
        }
    }

    #[inline]
    fn rfind_delimiter(fragment: &str) -> Option<usize> {
        // JUSTIFICATION: Bench-tested, and scanning bytes is an improvement.
        //
        // Note this is safe, actually! Even in the presence of UTF-8 data.
        // Because scanning for ASCII codes cannot match with a part of a UTF-8
        // sequence which is not itself just a raw ASCII code point.
        if D::CHAR.is_ascii() {
            fragment
                .as_bytes()
                .iter()
                .rposition(|c| *c == D::CHAR as u8)
        } else {
            fragment.rfind(D::CHAR)
        }
    }

    #[inline(always)]
    fn from_char(c: char) -> Option<Self> {
        Self::from_chunk_delim(c)
    }
    #[inline(always)]
    fn from_ident_start(c: char) -> Option<Self> {
        Self::from_chunk_delim(c)
    }
    #[inline]
    fn from_chunk_delim(c: char) -> Option<Self> {
        if c == D::CHAR {
            Some(Self::default())
        } else {
            None
        }
    }

    #[inline(always)]
    fn is_delim(c: char) -> bool {
        Self::is_chunk_delim(c)
    }
    #[inline(always)]
    fn is_ident_start(c: char) -> bool {
        Self::is_chunk_delim(c)
    }
    #[inline(always)]
    fn is_chunk_delim(c: char) -> bool {
        c == D::CHAR
    }
}
