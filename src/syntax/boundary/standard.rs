// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "standard.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::boundary::{Boundary, Options, TrivialBoundary, options};
use crate::syntax::segmentation::{GraphemeIndicesIterator, GraphemesIterator};
use crate::syntax::{GraphemeCase, Segmentation};
use core::marker::PhantomData;
use core::num::NonZero;

// =============================================================================
// TYPES
// =============================================================================

/// Defines a standard boundary implementation.
///
/// This type is configurable over [`Options`] types, but it assumes the
/// [`Default`] options when another options type is not provided.
///
/// For a description of how boundaries are formed in this implementation, see
/// the [`boundary`](crate::syntax::boundary#options) module documentation.
///
/// [`Default`]: crate::syntax::boundary::options::Default
/// [`Options`]: crate::syntax::boundary::options::Options
pub struct Standard<O: Options = options::Default>(PhantomData<O>);

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<O: Options> Boundary for Standard<O> {
    const CAN_FIND_BOUNDARIES: bool = O::CAMEL
        || O::HAT
        || O::DIGIT_TO_LOWER
        || O::DIGIT_TO_UPPER
        || O::LOWER_TO_DIGIT
        || O::UPPER_TO_DIGIT;

    #[inline]
    fn find_boundary<S: Segmentation>(chunk: &str) -> Option<NonZero<usize>> {
        match Self::CAN_FIND_BOUNDARIES {
            true => {
                let mut iter =
                    S::GraphemeIndices::new(chunk).map(|(i, s)| (i, GraphemeCase::new(s)));
                let mut prev = iter.next().map(|(_, c)| c)?; // Must be at least one grapheme.
                let mut iter = iter.peekable();
                while let Some((idx, curr)) = iter.next() {
                    let next = iter.peek().copied().map(|(_, c)| c);
                    if Self::is_boundary(prev, curr, next) {
                        return NonZero::new(idx);
                    }
                    prev = curr;
                }
                None
            }
            false => None,
        }
    }

    #[inline]
    fn rfind_boundary<S: Segmentation>(chunk: &str) -> Option<NonZero<usize>> {
        match Self::CAN_FIND_BOUNDARIES {
            true => {
                let iter = S::GraphemeIndices::new(chunk)
                    .rev()
                    .map(|(i, s)| (i, GraphemeCase::new(s)));
                let mut next = None; // Recall this is from the right-side!
                let mut iter = iter.peekable();
                while let Some((idx, curr)) = iter.next() {
                    let prev = iter.peek().copied().map(|(_, c)| c)?;
                    if Self::is_boundary(prev, curr, next) {
                        return NonZero::new(idx);
                    }
                    next = Some(curr);
                }
                None
            }
            false => None,
        }
    }

    #[inline]
    fn has_boundary_at<S: Segmentation>(chunk: &str, idx: usize) -> bool {
        match Self::CAN_FIND_BOUNDARIES {
            true => {
                let (left, right) = chunk.split_at(idx);
                let Some(prev) = S::Graphemes::new(left).next_back() else {
                    return false;
                };
                let mut right = S::Graphemes::new(right);
                let Some(curr) = right.next() else {
                    return false;
                };
                Self::is_boundary_str(prev, curr, right.next())
            }
            false => false,
        }
    }
}

// -----------------------------------------------------------------------------
impl<O: Options> TrivialBoundary for Standard<O> {
    #[inline]
    fn is_boundary(prev: GraphemeCase, curr: GraphemeCase, next: Option<GraphemeCase>) -> bool {
        match curr {
            GraphemeCase::Digit => match prev {
                GraphemeCase::Lower => O::LOWER_TO_DIGIT,
                GraphemeCase::TitleNonGreek => O::LOWER_TO_DIGIT,
                GraphemeCase::Upper => O::UPPER_TO_DIGIT,
                _ => false,
            },
            GraphemeCase::Lower => match prev {
                GraphemeCase::Digit => O::DIGIT_TO_LOWER,
                _ => false,
            },
            GraphemeCase::TitleNonGreek => {
                let boundary = match prev {
                    GraphemeCase::Digit => O::DIGIT_TO_UPPER,
                    GraphemeCase::Lower => O::CAMEL,
                    GraphemeCase::TitleNonGreek => O::CAMEL,
                    _ => false,
                };
                boundary || O::HAT
            }
            GraphemeCase::Upper => {
                let boundary = match prev {
                    GraphemeCase::Digit => O::DIGIT_TO_UPPER,
                    GraphemeCase::Lower => O::CAMEL,
                    GraphemeCase::TitleNonGreek => O::CAMEL,
                    _ => false,
                };
                boundary || (O::HAT && next.is_some_and(|k| k == GraphemeCase::Lower))
            }
            _ => false,
        }
    }
}
