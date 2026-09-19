// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::Identifier;
use crate::core::fmt::{
    AsLowerCamel, AsLowerHybrid, AsLowerKebab, AsLowerSnake, AsUpperCamel, AsUpperHybrid,
    AsUpperKebab, AsUpperSnake,
};

// =============================================================================
// TRAITS
// =============================================================================

/// A convenience trait for identifiers which support all preset formatting
/// operations (which are all valid identifiers).
///
/// This makes it easier to write generics that you intend on converting.
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// # use typed_ident::core::*;
/// fn convert_to_snake<I>(ident: &I) -> String
/// where
///     I: FormattableIdentifier + ?Sized,
/// {
///     ident.as_lower_snake().to_string()
/// }
/// ```
pub trait FormattableIdentifier:
    Identifier
    + AsLowerCamel
    + AsLowerHybrid
    + AsLowerKebab
    + AsLowerSnake
    + AsUpperCamel
    + AsUpperHybrid
    + AsUpperKebab
    + AsUpperSnake
{
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I> FormattableIdentifier for I where
    I: Identifier
        + AsLowerCamel
        + AsLowerHybrid
        + AsLowerKebab
        + AsLowerSnake
        + AsUpperCamel
        + AsUpperHybrid
        + AsUpperKebab
        + AsUpperSnake
        + ?Sized
{
}
