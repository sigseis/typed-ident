// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::{Boundary, CasedProfile, Delimiter, SubsetOf};
use crate::{Error, Ident, Identifier};
use std_alloc::boxed::Box;
use std_alloc::string::String;

// =============================================================================
// TRAITS
// =============================================================================

/// A type used to suggest a possible conversion from another type.
///
/// The value passed into this type is the value post-conversion. So the target
/// type must be something owned currently. Since this trait is not able to be
/// implemented by the end-user, this is probably okay.
pub trait FromIntermediate<I: Identifier + ?Sized>: Sized {
    fn from_intermediate(intermediate: String) -> Result<Self, Error>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> FromIntermediate<I> for Box<str> {
    #[inline(always)]
    fn from_intermediate(intermediate: String) -> Result<Self, Error> {
        Ok(intermediate.into_boxed_str())
    }
}

// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized> FromIntermediate<I> for String {
    #[inline(always)]
    fn from_intermediate(intermediate: String) -> Result<Self, Error> {
        Ok(intermediate)
    }
}

// -----------------------------------------------------------------------------
// This is a little less obvious than `String` (above).
//
// Basically this says that as long as this format delimiter is a subset of the
// target delimiter, and as long as the source character profile is a subset of
// the target character profile - we'll allow the conversion attempt.
//
// Technically it could be the case, even if these subsets aren't satisfied,
// that the conversion could be successful. But it at least would not be
// obviously intended from the types.
//
// For example: `let i: Box<LowerSnakeIdent> = i.to_lower_kebab()`
// -----------------------------------------------------------------------------
impl<I: Identifier + ?Sized, B: Boundary, D: Delimiter, P: CasedProfile> FromIntermediate<I>
    for Box<Ident<B, D, P>>
where
    I::Delimiter: SubsetOf<D>,
    I::Profile: SubsetOf<P>,
{
    #[inline(always)]
    fn from_intermediate(intermediate: String) -> Result<Self, Error> {
        Ident::new_boxed(intermediate)
    }
}
