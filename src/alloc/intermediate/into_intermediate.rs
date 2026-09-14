// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::{Chunk, Error, Fragment, Ident};
use crate::syntax::{Boundary, CasedProfile, Delimiter};
use core::fmt::Display;
use std_alloc::string::String;

// =============================================================================
// TYPES
// =============================================================================

/// A type that is able to be converted into some intermediate fragment type.
///
/// These are used to move from some non-representative format (like `char`)
/// into a format that can be used to represent a fragment (for the case of a
/// char, something like a byte buffer and a length).
pub trait IntoIntermediate<B, D, P> {
    /// The type that we need to convert into in order to represent as `T`.
    type Intermediate: AsRef<Fragment<B, D, P>> + AsRef<str> + Display;

    /// The conversion function to move from `Self` to `Intermediate`.
    fn into_intermediate(self) -> Result<Self::Intermediate, Error>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B: Boundary + 'a, D: Delimiter + 'a, P: CasedProfile + 'a> IntoIntermediate<B, D, P>
    for &'a str
{
    type Intermediate = &'a Fragment<B, D, P>;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        Fragment::new(self)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary + 'a, D: Delimiter + 'a, P: CasedProfile + 'a> IntoIntermediate<B, D, P>
    for &'a String
{
    type Intermediate = &'a Fragment<B, D, P>;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        Fragment::new(self)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: 'a, D: Delimiter + 'a, P: CasedProfile + 'a> IntoIntermediate<B, D, P>
    for &'a Fragment<B, D, P>
{
    type Intermediate = &'a Fragment<B, D, P>;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        Ok(self)
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: 'a, D: Delimiter + 'a, P: CasedProfile + 'a> IntoIntermediate<B, D, P>
    for &'a Chunk<B, D, P>
{
    type Intermediate = &'a Fragment<B, D, P>;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        Ok(self.as_fragment())
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: 'a, D: Delimiter + 'a, P: CasedProfile + 'a> IntoIntermediate<B, D, P>
    for &'a Ident<B, D, P>
{
    type Intermediate = &'a Fragment<B, D, P>;

    #[inline(always)]
    fn into_intermediate(self) -> Result<Self::Intermediate, Error> {
        Ok(self.as_fragment())
    }
}
