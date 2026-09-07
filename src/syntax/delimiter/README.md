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

If the delimiter can only be represented by a single character, you should consider implementing the [`UnitDelimiter`] trait instead, which will allow your delimiter to be used in more operations (where we can infer that a valid minimal anonymous identifier would be a single character with a known delimiter).
