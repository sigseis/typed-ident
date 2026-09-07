// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::{Chunk, Fragment};
use core::ops::Index;
use core::{ops, range};

// =============================================================================
// TRAITS
// =============================================================================

/// The valid index types for slicing fragments, chunks, and identifiers.
///
/// This is implemented over the various supported indexing operations, and can
/// be used as a trait bounds for the client writing their own new-type
/// identifiers, who wish to expose slicing operations.
///
/// # Safety
///
/// Unlike the core `SliceIndex`, this trait is safe, because it operates on
/// the string slicing operations that the standard provides. So this trait need
/// not be marked as `unsafe`, since it simply uses safe abstractions.
pub trait SliceIndex<T: ?Sized>: private::Sealed {
    /// Returns a slice of type `T`, using `Self` as the index.
    ///
    /// This is the checked variant, that is allowed to fail. On failure, this
    /// type should return `None`.
    fn get(self, slice: &T) -> Option<&T>;

    /// Returns a slice of type `T`, using `Self` as the index.
    ///
    /// This is done without checking that the bounds are appropriate, and that
    /// the slice happens on a unicode sequence boundary (both requirements of
    /// calling this function).
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    ///
    /// * The starting index must not exceed the ending index;
    /// * Indexes must be within bounds of the original slice;
    /// * Indexes must lie on UTF-8 sequence boundaries.
    ///
    /// Failure to uphold these promises can lead to undefined behavior.
    unsafe fn get_unchecked(self, slice: &T) -> &T;

    /// A checked version of the slicing operation.
    ///
    /// # Panics
    ///
    /// If the requirements for taking a slice are not upheld, this function
    /// will panic and abort the application.
    fn index(self, slice: &T) -> &T;
}

// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! impl_index {
    ($type:ident, $op:ty) => {
        impl<B, D, P> SliceIndex<$type<B, D, P>> for $op {
            #[inline]
            fn get(self, slice: &$type<B, D, P>) -> Option<&$type<B, D, P>> {
                slice.as_str().get(self).map($type::new_unchecked)
            }
            #[inline]
            unsafe fn get_unchecked(self, slice: &$type<B, D, P>) -> &$type<B, D, P> {
                // SAFETY: the caller must uphold the safety contract for `get_unchecked`.
                // the slice is dereferenceable because `self` is a safe reference.
                // The returned pointer is safe because impls of `SliceIndex` have to guarantee that it is.
                $type::new_unchecked(unsafe { slice.as_str().get_unchecked(self) })
            }
            #[inline]
            fn index(self, slice: &$type<B, D, P>) -> &$type<B, D, P> {
                $type::new_unchecked(slice.as_str().index(self))
            }
        }
    };
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod private {
    use core::{ops, range};
    pub trait Sealed {}
    impl Sealed for ops::Range<usize> {}
    impl Sealed for ops::RangeTo<usize> {}
    impl Sealed for ops::RangeFrom<usize> {}
    impl Sealed for ops::RangeFull {}
    impl Sealed for ops::RangeInclusive<usize> {}
    impl Sealed for ops::RangeToInclusive<usize> {}
    impl Sealed for (ops::Bound<usize>, ops::Bound<usize>) {}
    impl Sealed for range::Range<usize> {}
    impl Sealed for range::RangeInclusive<usize> {}
    impl Sealed for range::RangeToInclusive<usize> {}
    impl Sealed for range::RangeFrom<usize> {}
}

// -----------------------------------------------------------------------------
impl_index!(Chunk, ops::Range<usize>);
impl_index!(Chunk, ops::RangeTo<usize>);
impl_index!(Chunk, ops::RangeFrom<usize>);
impl_index!(Chunk, ops::RangeFull);
impl_index!(Chunk, ops::RangeInclusive<usize>);
impl_index!(Chunk, ops::RangeToInclusive<usize>);
impl_index!(Chunk, (ops::Bound<usize>, ops::Bound<usize>));
impl_index!(Chunk, range::Range<usize>);
impl_index!(Chunk, range::RangeInclusive<usize>);
impl_index!(Chunk, range::RangeToInclusive<usize>);
impl_index!(Chunk, range::RangeFrom<usize>);

// -----------------------------------------------------------------------------
impl_index!(Fragment, ops::Range<usize>);
impl_index!(Fragment, ops::RangeTo<usize>);
impl_index!(Fragment, ops::RangeFrom<usize>);
impl_index!(Fragment, ops::RangeFull);
impl_index!(Fragment, ops::RangeInclusive<usize>);
impl_index!(Fragment, ops::RangeToInclusive<usize>);
impl_index!(Fragment, (ops::Bound<usize>, ops::Bound<usize>));
impl_index!(Fragment, range::Range<usize>);
impl_index!(Fragment, range::RangeInclusive<usize>);
impl_index!(Fragment, range::RangeToInclusive<usize>);
impl_index!(Fragment, range::RangeFrom<usize>);
