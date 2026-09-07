// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "ascii_punctuation.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::SubsetOf;
use crate::syntax::delimiter::{
    AppendClosed, AsciiFlatLine, Delimiter, HyphenMinus, LowLine, TryFromCharError,
};

// =============================================================================
// TYPES
// =============================================================================

/// Represents an ASCII punctuation (non-whitespace, visible, non-letter/digit).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord)]
pub enum AsciiPunctuation {
    /// U+0021 EXCLAMATION MARK (`!`)
    ExclamationMark = 33,
    /// U+0022 QUOTATION MARK (`"`)
    QuotationMark = 34,
    /// U+0023 NUMBER SIGN (`#`)
    NumberSign = 35,
    /// U+0024 DOLLAR SIGN (`$`)
    DollarSign = 36,
    /// U+0025 PERCENT SIGN (`%`)
    PercentSign = 37,
    /// U+0026 AMPERSAND (`&`)
    Ampersand = 38,
    /// U+0027 APOSTROPHE (`'`)
    Apostrophe = 39,
    /// U+0028 LEFT PARENTHESIS (`(`)
    LeftParenthesis = 40,
    /// U+0029 RIGHT PARENTHESIS (`)`)
    RightParenthesis = 41,
    /// U+002A ASTERISK (`*`)
    Asterisk = 42,
    /// U+002B PLUS SIGN (`+`)
    PlusSign = 43,
    /// U+002C COMMA (`,`)
    Comma = 44,
    /// U+002D HYPHEN-MINUS (`-`)
    HyphenMinus = 45,
    /// U+002E FULL STOP (`.`)
    FullStop = 46,
    /// U+002F SOLIDUS (`/`)
    Solidus = 47,
    /// U+003A COLON (`:`)
    Colon = 58,
    /// U+003B SEMICOLON (`;`)
    Semicolon = 59,
    /// U+003C LESS-THAN SIGN (`<`)
    LessThanSign = 60,
    /// U+003D EQUALS SIGN (`=`)
    EqualsSign = 61,
    /// U+003E GREATER-THAN SIGN (`>`)
    GreaterThanSign = 62,
    /// U+003F QUESTION MARK (`?`)
    QuestionMark = 63,
    /// U+0040 COMMERCIAL AT (`@`)
    CommercialAt = 64,
    /// U+005B LEFT SQUARE BRACKET (`[`)
    LeftSquareBracket = 91,
    /// U+005C REVERSE SOLIDUS (`\`)
    ReverseSolidus = 92,
    /// U+005D RIGHT SQUARE BRACKET (`]`)
    RightSquareBracket = 93,
    /// U+005E CIRCUMFLEX ACCENT (`^`)
    CircumflexAccent = 94,
    /// U+005F LOW LINE (`_`)
    LowLine = 95,
    /// U+0060 GRAVE ACCENT (`` ` ``)
    GraveAccent = 96,
    /// U+007B LEFT CURLY BRACKET (`{`)
    LeftCurlyBracket = 123,
    /// U+007C VERTICAL LINE (`|`)
    VerticalLine = 124,
    /// U+007D RIGHT CURLY BRACKET (`}`)
    RightCurlyBracket = 125,
    /// U+007E TILDE (`~`)
    Tilde = 126,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl AsciiPunctuation {
    #[inline]
    const fn from_char(c: char) -> Option<Self> {
        match c {
            '\u{0021}' => Some(Self::ExclamationMark),
            '\u{0022}' => Some(Self::QuotationMark),
            '\u{0023}' => Some(Self::NumberSign),
            '\u{0024}' => Some(Self::DollarSign),
            '\u{0025}' => Some(Self::PercentSign),
            '\u{0026}' => Some(Self::Ampersand),
            '\u{0027}' => Some(Self::Apostrophe),
            '\u{0028}' => Some(Self::LeftParenthesis),
            '\u{0029}' => Some(Self::RightParenthesis),
            '\u{002A}' => Some(Self::Asterisk),
            '\u{002B}' => Some(Self::PlusSign),
            '\u{002C}' => Some(Self::Comma),
            '\u{002D}' => Some(Self::HyphenMinus),
            '\u{002E}' => Some(Self::FullStop),
            '\u{002F}' => Some(Self::Solidus),
            '\u{003A}' => Some(Self::Colon),
            '\u{003B}' => Some(Self::Semicolon),
            '\u{003C}' => Some(Self::LessThanSign),
            '\u{003D}' => Some(Self::EqualsSign),
            '\u{003E}' => Some(Self::GreaterThanSign),
            '\u{003F}' => Some(Self::QuestionMark),
            '\u{0040}' => Some(Self::CommercialAt),
            '\u{005B}' => Some(Self::LeftSquareBracket),
            '\u{005C}' => Some(Self::ReverseSolidus),
            '\u{005D}' => Some(Self::RightSquareBracket),
            '\u{005E}' => Some(Self::CircumflexAccent),
            '\u{005F}' => Some(Self::LowLine),
            '\u{0060}' => Some(Self::GraveAccent),
            '\u{007B}' => Some(Self::LeftCurlyBracket),
            '\u{007C}' => Some(Self::VerticalLine),
            '\u{007D}' => Some(Self::RightCurlyBracket),
            '\u{007E}' => Some(Self::Tilde),
            _ => None,
        }
    }
    #[inline]
    const fn is_delim(c: char) -> bool {
        matches!(
            c,
            '\u{0021}'
                | '\u{0022}'
                | '\u{0023}'
                | '\u{0024}'
                | '\u{0025}'
                | '\u{0026}'
                | '\u{0027}'
                | '\u{0028}'
                | '\u{0029}'
                | '\u{002A}'
                | '\u{002B}'
                | '\u{002C}'
                | '\u{002D}'
                | '\u{002E}'
                | '\u{002F}'
                | '\u{003A}'
                | '\u{003B}'
                | '\u{003C}'
                | '\u{003D}'
                | '\u{003E}'
                | '\u{003F}'
                | '\u{0040}'
                | '\u{005B}'
                | '\u{005C}'
                | '\u{005D}'
                | '\u{005E}'
                | '\u{005F}'
                | '\u{0060}'
                | '\u{007B}'
                | '\u{007C}'
                | '\u{007D}'
                | '\u{007E}'
        )
    }
    #[inline]
    const fn to_char(self) -> char {
        match self {
            Self::ExclamationMark => '\u{0021}',
            Self::QuotationMark => '\u{0022}',
            Self::NumberSign => '\u{0023}',
            Self::DollarSign => '\u{0024}',
            Self::PercentSign => '\u{0025}',
            Self::Ampersand => '\u{0026}',
            Self::Apostrophe => '\u{0027}',
            Self::LeftParenthesis => '\u{0028}',
            Self::RightParenthesis => '\u{0029}',
            Self::Asterisk => '\u{002A}',
            Self::PlusSign => '\u{002B}',
            Self::Comma => '\u{002C}',
            Self::HyphenMinus => '\u{002D}',
            Self::FullStop => '\u{002E}',
            Self::Solidus => '\u{002F}',
            Self::Colon => '\u{003A}',
            Self::Semicolon => '\u{003B}',
            Self::LessThanSign => '\u{003C}',
            Self::EqualsSign => '\u{003D}',
            Self::GreaterThanSign => '\u{003E}',
            Self::QuestionMark => '\u{003F}',
            Self::CommercialAt => '\u{0040}',
            Self::LeftSquareBracket => '\u{005B}',
            Self::ReverseSolidus => '\u{005C}',
            Self::RightSquareBracket => '\u{005D}',
            Self::CircumflexAccent => '\u{005E}',
            Self::LowLine => '\u{005F}',
            Self::GraveAccent => '\u{0060}',
            Self::LeftCurlyBracket => '\u{007B}',
            Self::VerticalLine => '\u{007C}',
            Self::RightCurlyBracket => '\u{007D}',
            Self::Tilde => '\u{007E}',
        }
    }
    #[inline]
    const fn to_str(self) -> &'static str {
        match self {
            Self::ExclamationMark => "\u{0021}",
            Self::QuotationMark => "\u{0022}",
            Self::NumberSign => "\u{0023}",
            Self::DollarSign => "\u{0024}",
            Self::PercentSign => "\u{0025}",
            Self::Ampersand => "\u{0026}",
            Self::Apostrophe => "\u{0027}",
            Self::LeftParenthesis => "\u{0028}",
            Self::RightParenthesis => "\u{0029}",
            Self::Asterisk => "\u{002A}",
            Self::PlusSign => "\u{002B}",
            Self::Comma => "\u{002C}",
            Self::HyphenMinus => "\u{002D}",
            Self::FullStop => "\u{002E}",
            Self::Solidus => "\u{002F}",
            Self::Colon => "\u{003A}",
            Self::Semicolon => "\u{003B}",
            Self::LessThanSign => "\u{003C}",
            Self::EqualsSign => "\u{003D}",
            Self::GreaterThanSign => "\u{003E}",
            Self::QuestionMark => "\u{003F}",
            Self::CommercialAt => "\u{0040}",
            Self::LeftSquareBracket => "\u{005B}",
            Self::ReverseSolidus => "\u{005C}",
            Self::RightSquareBracket => "\u{005D}",
            Self::CircumflexAccent => "\u{005E}",
            Self::LowLine => "\u{005F}",
            Self::GraveAccent => "\u{0060}",
            Self::LeftCurlyBracket => "\u{007B}",
            Self::VerticalLine => "\u{007C}",
            Self::RightCurlyBracket => "\u{007D}",
            Self::Tilde => "\u{007E}",
        }
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Delimiter for AsciiPunctuation {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Identifier;

    #[inline(always)]
    fn as_char(&self) -> char {
        self.to_char()
    }

    #[inline]
    fn find_delimiter(fragment: &str) -> Option<usize> {
        fragment.find(|c: char| c.is_ascii_punctuation())
    }

    #[inline]
    fn rfind_delimiter(fragment: &str) -> Option<usize> {
        fragment.rfind(|c: char| c.is_ascii_punctuation())
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

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl core::cmp::PartialEq<char> for AsciiPunctuation {
    fn eq(&self, rhs: &char) -> bool {
        self.to_char().eq(rhs)
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialEq<AsciiPunctuation> for char {
    fn eq(&self, rhs: &AsciiPunctuation) -> bool {
        *self == rhs.to_char()
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter> core::cmp::PartialEq<D> for AsciiPunctuation {
    fn eq(&self, rhs: &D) -> bool {
        self.to_char() == rhs.as_char()
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialOrd<char> for AsciiPunctuation {
    fn partial_cmp(&self, rhs: &char) -> Option<core::cmp::Ordering> {
        self.to_char().partial_cmp(rhs)
    }
}

// -----------------------------------------------------------------------------
impl core::cmp::PartialOrd<AsciiPunctuation> for char {
    fn partial_cmp(&self, rhs: &AsciiPunctuation) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&rhs.to_char())
    }
}

// -----------------------------------------------------------------------------
impl<D: Delimiter> core::cmp::PartialOrd<D> for AsciiPunctuation {
    fn partial_cmp(&self, rhs: &D) -> Option<core::cmp::Ordering> {
        self.to_char().partial_cmp(&rhs.as_char())
    }
}

// -----------------------------------------------------------------------------
impl core::convert::AsRef<str> for AsciiPunctuation {
    #[inline]
    fn as_ref(&self) -> &str {
        (*self).to_str()
    }
}

// -----------------------------------------------------------------------------
impl core::convert::From<AsciiPunctuation> for char {
    #[inline]
    fn from(orig: AsciiPunctuation) -> Self {
        orig.to_char()
    }
}

// -----------------------------------------------------------------------------
impl core::convert::From<AsciiFlatLine> for AsciiPunctuation {
    #[inline]
    fn from(orig: AsciiFlatLine) -> Self {
        match orig {
            AsciiFlatLine::HyphenMinus => Self::HyphenMinus,
            AsciiFlatLine::LowLine => Self::LowLine,
        }
    }
}

// -----------------------------------------------------------------------------
impl core::convert::From<HyphenMinus> for AsciiPunctuation {
    #[inline(always)]
    fn from(_: HyphenMinus) -> Self {
        Self::HyphenMinus
    }
}

// -----------------------------------------------------------------------------
impl core::convert::From<LowLine> for AsciiPunctuation {
    #[inline(always)]
    fn from(_: LowLine) -> Self {
        Self::LowLine
    }
}

// -----------------------------------------------------------------------------
impl TryFrom<char> for AsciiPunctuation {
    type Error = TryFromCharError;

    #[inline]
    fn try_from(orig: char) -> Result<Self, Self::Error> {
        Self::from_char(orig).ok_or(TryFromCharError(()))
    }
}

// -----------------------------------------------------------------------------
impl core::fmt::Display for AsciiPunctuation {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;
        f.write_char((*self).to_char())
    }
}

// -----------------------------------------------------------------------------
/// Proof: It's always safe to implement this against yourself.
// -----------------------------------------------------------------------------
impl SubsetOf<AsciiPunctuation> for AsciiPunctuation {}
