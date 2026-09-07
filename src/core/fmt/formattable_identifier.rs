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

/// A marker trait that constrains to only identifiers which support preset
/// conversion (the `As*` traits; e.g. [`AsUpperCamel`], [`AsLowerKebab`], etc).
///
/// This makes it easier to write generics that you intend on converting.
///
/// [`AsUpperCamel`]: crate::core::fmt::AsUpperCamel
/// [`AsLowerKebab`]: crate::core::fmt::AsLowerKebab
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
