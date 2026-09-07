#![doc = include_str!("README.md")]
// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod append_closed;
mod ascii_flat_line;
mod ascii_punctuation;
mod delimiter;
mod not_delimited;
mod try_from_char_error;
mod unit_delimiter;

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub use append_closed::*;
pub use ascii_flat_line::*;
pub use ascii_punctuation::*;
pub use delimiter::*;
pub use not_delimited::*;
pub use try_from_char_error::*;
pub use unit_delimiter::*;

// =============================================================================
// COMMON UNIT DELIMITERS
// =============================================================================

// -----------------------------------------------------------------------------
impl_ascii_flat_line_delimiter! {
    name=LowLine,
    char='_',
    docs="A delimiter which is the ASCII character LOW LINE (`_`).",
}

// -----------------------------------------------------------------------------
impl_ascii_flat_line_delimiter! {
    name=HyphenMinus,
    char='-',
    docs="A delimiter which is the ASCII character HYPHEN-MINUS (`-`).",
}
