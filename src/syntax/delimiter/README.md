Syntax definitions for identifier chunk delimiters.

The provided implementations are:

* [`AsciiFlatLine`]: Any "flat-line" looking ASCII symbol (`_` or `-`).
* [`AsciiPunctuation`]: Any ASCII punctuation (whitespace and non-visual characters excluded).
* [`HyphenMinus`]: Explicitly the ASCII hyphen-minus symbol (`-`).
* [`LowLine`]: Explicitly the ASCII low line symbol (`_`).
* [`NotDelimited`]: No delimiter is allowed (it's all one big chunk).

None of the provided implementations delimit on anything other than the ASCII code points.

## Customizing Delimiters

The primary way to customize a delimiter is to create your own type (usually an enum), and then to implement the [`Delimiter`] trait on it.

If the delimiter can only be represented by a single value, you should consider implementing the [`UnitDelimiter`] trait instead, which will allow your delimiter to be used in more operations (and easier to implement).

### Implementing a `UnitDelimiter`

If you only want to consider a single value a valid delimiter for your identifier syntax, you can implement the [`UnitDelimiter`] trait. This trait should be preferred to the regular [`Delimiter`] trait where possible.

```rust
use typed_ident::*;
use typed_ident::syntax::*;
use typed_ident::syntax::delimiter::*;

// Define a custom delimiter for tilde (~).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Tilde;
impl UnitDelimiter for Tilde {
    const CHAR: char = '~';
    const STR: &'static str = "~";
}
impl SubsetOf<Tilde> for Tilde {}

// Use the delimiter with a custom combination of syntax rules.
type CustomIdent = Ident<
    boundary::Standard,
    Tilde,
    profile::Mixed<profile::Unicode>,
>;
```

### Implementing a `Delimiter`

If there are more than one valid delimiter values, or if some delimiter values are valid in some positions, but others in other positions (like for Bash), then you must implement the [`Delimiter`] trait.

Minimally, you need to implement the following functions:

* [`Delimiter::as_char`] returns the delimiter as its character value
* [`Delimiter::from_char`] returns an instance of the delimiter from a character value

But depending on how complicated the delimiter is, you may wish to provide a custom implementation of the other provided functions.

```rust
use typed_ident::syntax::*;
use typed_ident::syntax::delimiter::*;

// Define a custom delimiter for parentheses.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Parenthesis {
    Open,
    Close,
}
impl Delimiter for Parenthesis {
    fn as_char(&self) -> char {
        match self {
            Self::Open => '(',
            Self::Close => ')',
        }
    }
    fn from_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::Open),
            ')' => Some(Self::Close),
            _ => None,
        }
    }
}
impl SubsetOf<Parenthesis> for Parenthesis {}

// Use the delimiter with a custom combination of syntax rules.
use typed_ident::*;
type CustomIdent = Ident<
    boundary::Standard,
    Parenthesis,
    profile::Mixed<profile::Unicode>,
>;
```
