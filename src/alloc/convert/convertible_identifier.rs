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

/// A marker trait that constrains to only identifiers which support preset
/// conversion (the `To*` traits; e.g. [`ToUpperCamel`], [`ToLowerKebab`], etc).
///
/// This makes it easier to write generics that you intend on converting.
///
/// [`ToUpperCamel`]: crate::alloc::convert::ToUpperCamel
/// [`ToLowerKebab`]: crate::alloc::convert::ToLowerKebab
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
