// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::convert::{
    ToLowerCamel, ToLowerHybrid, ToLowerKebab, ToLowerSnake, ToUpperCamel, ToUpperHybrid,
    ToUpperKebab, ToUpperSnake,
};
use crate::core::fmt::FormattableIdentifier;

// =============================================================================
// TRAITS
// =============================================================================

/// A convenience trait for identifiers which support all preset conversion
/// operations (which are all valid identifiers).
///
/// This makes it easier to write generics that you intend on converting.
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// # use typed_ident::alloc::*;
/// fn convert_to_snake<I>(ident: &I) -> String
/// where
///     I: ConvertibleIdentifier + ?Sized,
/// {
///     ident.to_lower_snake()
/// }
/// ```
pub trait ConvertibleIdentifier:
    FormattableIdentifier
    + ToLowerCamel
    + ToLowerHybrid
    + ToLowerKebab
    + ToLowerSnake
    + ToUpperCamel
    + ToUpperHybrid
    + ToUpperKebab
    + ToUpperSnake
{
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I> ConvertibleIdentifier for I where
    I: FormattableIdentifier
        + ToLowerCamel
        + ToLowerHybrid
        + ToLowerKebab
        + ToLowerSnake
        + ToUpperCamel
        + ToUpperHybrid
        + ToUpperKebab
        + ToUpperSnake
        + ?Sized
{
}
