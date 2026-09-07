// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "ascii_flat_line.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::SubsetOf;
use crate::syntax::delimiter::{
    AppendClosed, AsciiPunctuation, Delimiter, HyphenMinus, LowLine, TryFromCharError,
};

// =============================================================================
// TYPES
// =============================================================================

/// Represents an ASCII flat-line delimiter (`_` or `-`).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord)]
pub enum AsciiFlatLine {
    /// U+002D HYPHEN-MINUS (`-`)
    HyphenMinus = 45,
    /// U+005F LOW LINE (`_`)
    LowLine = 95,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl AsciiFlatLine {
    #[inline]
    const fn from_char(c: char) -> Option<Self> {
        match c {
            '\u{002D}' => Some(Self::HyphenMinus),
            '\u{005F}' => Some(Self::LowLine),
            _ => None,
        }
    }
    #[inline]
    const fn is_delim(c: char) -> bool {
        matches!(c, '\u{002D}' | '\u{005F}')
    }
    #[inline]
    const fn to_char(self) -> char {
        match self {
            Self::HyphenMinus => '\u{002D}',
            Self::LowLine => '\u{005F}',
        }
    }
    #[inline]
    const fn to_str(self) -> &'static str {
        match self {
            Self::HyphenMinus => "\u{002D}",
            Self::LowLine => "\u{005F}",
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Delimiter for AsciiFlatLine {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Identifier;

    #[inline(always)]
    fn as_char(&self) -> char {
        self.to_char()
    }

    #[inline]
    fn find_delimiter(fragment: &str) -> Option<usize> {
        fragment.find(['_', '-'])
    }

    #[inline]
    fn rfind_delimiter(fragment: &str) -> Option<usize> {
        fragment.rfind(['_', '-'])
    }

    #[inline(always)]
    fn from_char(c: char) -> Option<Self> {
        Self::from_char(c)
    }
    #[inline(always)]
    fn from_ident_start(c: char) -> Option<Self> {
        Self::from_char(c)
    }
    #[inline(always)]
    fn from_chunk_delim(c: char) -> Option<Self> {
        Self::from_char(c)
    }

    #[inline(always)]
    fn is_delim(c: char) -> bool {
        Self::is_delim(c)
    }
    #[inline(always)]
    fn is_ident_start(c: char) -> bool {
        Self::is_delim(c)
    }
    #[inline(always)]
    fn is_chunk_delim(c: char) -> bool {
        Self::is_delim(c)
    }
}

// -----------------------------------------------------------------------------
/// Proof: It's always safe to implement this against yourself.
// -----------------------------------------------------------------------------
impl SubsetOf<AsciiFlatLine> for AsciiFlatLine {}

// -----------------------------------------------------------------------------
/// Proof: AsciiPunctuation very obviously contains AsciiFlatLine ('_', '-').
// -----------------------------------------------------------------------------
impl SubsetOf<AsciiPunctuation> for AsciiFlatLine {}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl core::cmp::PartialEq<char> for AsciiFlatLine {
    fn eq(&self, rhs: &char) -> bool {
        self.as_char().eq(rhs)
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialEq<AsciiFlatLine> for char {
    fn eq(&self, rhs: &AsciiFlatLine) -> bool {
        *self == rhs.to_char()
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter> core::cmp::PartialEq<D> for AsciiFlatLine {
    fn eq(&self, rhs: &D) -> bool {
        self.to_char() == rhs.as_char()
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter> core::cmp::PartialOrd<D> for AsciiFlatLine {
    fn partial_cmp(&self, rhs: &D) -> Option<core::cmp::Ordering> {
        self.to_char().partial_cmp(&rhs.as_char())
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialOrd<char> for AsciiFlatLine {
    fn partial_cmp(&self, rhs: &char) -> Option<core::cmp::Ordering> {
        self.to_char().partial_cmp(rhs)
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialOrd<AsciiFlatLine> for char {
    fn partial_cmp(&self, rhs: &AsciiFlatLine) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&rhs.to_char())
    }
}

// -----------------------------------------------------------------------------
impl core::convert::AsRef<str> for AsciiFlatLine {
    #[inline]
    fn as_ref(&self) -> &str {
        (*self).to_str()
    }
}

// -----------------------------------------------------------------------------
impl core::convert::From<AsciiFlatLine> for char {
    #[inline]
    fn from(orig: AsciiFlatLine) -> char {
        orig.to_char()
    }
}

// -----------------------------------------------------------------------------
impl TryFrom<char> for AsciiFlatLine {
    type Error = TryFromCharError;

    #[inline]
    fn try_from(orig: char) -> Result<Self, Self::Error> {
        Self::from_char(orig).ok_or(TryFromCharError(()))
    }
}

// -----------------------------------------------------------------------------
impl core::convert::From<LowLine> for AsciiFlatLine {
    #[inline(always)]
    fn from(_: LowLine) -> Self {
        Self::LowLine
    }
}

// -----------------------------------------------------------------------------
impl core::convert::From<HyphenMinus> for AsciiFlatLine {
    #[inline(always)]
    fn from(_: HyphenMinus) -> Self {
        Self::HyphenMinus
    }
}

// -----------------------------------------------------------------------------
impl core::fmt::Display for AsciiFlatLine {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;
        f.write_char((*self).to_char())
    }
}
