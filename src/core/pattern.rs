// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use core::fmt::Debug;
use core::iter::{FusedIterator, Iterator};
use core::str::{MatchIndices, Matches, RMatchIndices, RMatches};

// =============================================================================
// TYPES
// =============================================================================

/// A string pattern for an identifier, fragment, or chunk.
///
/// This is equivalent to the standard-provided `Pattern` type, except that we
/// have remade it so that we can make use of it in this crate safely.
pub trait Pattern: private::Sealed {
    type Matches<'a>: Iterator<Item = &'a str> + Debug + FusedIterator;
    type MatchIndices<'a>: Iterator<Item = (usize, &'a str)> + Debug + FusedIterator;
    type RMatches<'a>: Iterator<Item = &'a str> + Debug + FusedIterator;
    type RMatchIndices<'a>: Iterator<Item = (usize, &'a str)> + Debug + FusedIterator;

    fn contains(self, s: &str) -> bool;
    fn ends_with(self, s: &str) -> bool;
    fn find(self, s: &str) -> Option<usize>;
    fn match_indices(self, s: &str) -> Self::MatchIndices<'_>;
    fn matches(self, s: &str) -> Self::Matches<'_>;
    fn rfind(self, s: &str) -> Option<usize>;
    fn rmatch_indices(self, s: &str) -> Self::RMatchIndices<'_>;
    fn rmatches(self, s: &str) -> Self::RMatches<'_>;
    fn starts_with(self, s: &str) -> bool;
    fn strip_prefix(self, s: &str) -> Option<&str>;
    fn strip_suffix(self, s: &str) -> Option<&str>;
    fn trim_end_matches(self, s: &str) -> &str;
    fn trim_start_matches(self, s: &str) -> &str;
}

// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! impl_pattern {
    (
        $($impl_prefix:tt)*
    ) => {
        $($impl_prefix)* {
            type Matches<'a> = Matches<'a, Self>;
            type MatchIndices<'a> = MatchIndices<'a, Self>;
            type RMatches<'a> = RMatches<'a, Self>;
            type RMatchIndices<'a> = RMatchIndices<'a, Self>;

            #[inline(always)]
            fn contains(self, s: &str) -> bool {
                s.contains(self)
            }
            #[inline(always)]
            fn ends_with(self, s: &str) -> bool {
                s.ends_with(self)
            }
            #[inline(always)]
            fn find(self, s: &str) -> Option<usize> {
                s.find(self)
            }
            #[inline(always)]
            fn match_indices(self, s: &str) -> Self::MatchIndices<'_> {
                s.match_indices(self)
            }
            #[inline(always)]
            fn matches(self, s: &str) -> Self::Matches<'_> {
                s.matches(self)
            }
            #[inline(always)]
            fn rfind(self, s: &str) -> Option<usize> {
                s.rfind(self)
            }
            #[inline(always)]
            fn rmatch_indices(self, s: &str) -> Self::RMatchIndices<'_> {
                s.rmatch_indices(self)
            }
            #[inline(always)]
            fn rmatches(self, s: &str) -> Self::RMatches<'_> {
                s.rmatches(self)
            }
            #[inline(always)]
            fn starts_with(self, s: &str) -> bool {
                s.starts_with(self)
            }
            #[inline(always)]
            fn strip_prefix(self, s: &str) -> Option<&str> {
                s.strip_prefix(self)
            }
            #[inline(always)]
            fn strip_suffix(self, s: &str) -> Option<&str> {
                s.strip_suffix(self)
            }
            #[inline(always)]
            fn trim_end_matches(self, s: &str) -> &str {
                s.trim_end_matches(self)
            }
            #[inline(always)]
            fn trim_start_matches(self, s: &str) -> &str {
                s.trim_start_matches(self)
            }
        }
    };
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
mod private {
    pub trait Sealed {}
    impl Sealed for char {}
    impl Sealed for &str {}
    #[cfg(feature = "alloc")]
    impl Sealed for &std_alloc::string::String {}
    impl Sealed for &[char] {}
    impl Sealed for &&str {}
    impl<const N: usize> Sealed for &[char; N] {}
    impl<const N: usize> Sealed for [char; N] {}
    impl<F: FnMut(char) -> bool> Sealed for F {}
}

// -----------------------------------------------------------------------------
impl_pattern!(impl Pattern for char);
impl_pattern!(impl<'b> Pattern for &'b str);
#[cfg(feature = "alloc")]
impl_pattern!(impl<'b> Pattern for &'b std_alloc::string::String);
impl_pattern!(impl<'b> Pattern for &'b [char]);
impl_pattern!(impl<'b, 'c> Pattern for &'c &'b str);
impl_pattern!(impl<'b, const N: usize> Pattern for &'b [char; N]);
impl_pattern!(impl<const N: usize> Pattern for [char; N]);
impl_pattern!(impl<F: FnMut(char) -> bool> Pattern for F);
