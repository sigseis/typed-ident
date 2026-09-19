// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::alloc::FragmentBuf;
use crate::core::error::{Error, ErrorKind};
use crate::syntax::{Boundary, CasedProfile, Delimiter, SyntaxError};
use core::marker::PhantomData;
use core::ops::RangeBounds;
use std_alloc::string::String;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
enum Undo {
    Overwrite(String),
    Truncate(usize),
}

/// Performs untyped modifications on the underlying buffer, and converts back
/// to the typed `FragmentBuf` representation at the end of a valid operation.
///
/// This exists to keep the implementation of `FragmentBuf` clean, and easily
/// statically analyzed to ensure correctness.
pub(super) struct FragmentBufOp<'a, B, D, P> {
    buffer: &'a mut String,
    undo: Undo,
    phantom: PhantomData<(B, D, P)>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D: Delimiter, P: CasedProfile> FragmentBufOp<'a, B, D, P> {
    #[inline]
    pub fn new(buffer: &'a mut FragmentBuf<B, D, P>) -> Self {
        Self {
            undo: Undo::Truncate(buffer.len()),
            buffer: &mut buffer.inner,
            phantom: PhantomData,
        }
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: CasedProfile> FragmentBufOp<'a, B, D, P> {
    #[inline]
    pub fn check_str(s: &str) -> Result<(), SyntaxError> {
        P::is_fragment::<D>(s)
    }

    #[inline]
    pub fn prepare_for_insert(&mut self, idx: usize) {
        if idx != self.buffer.len() {
            self.undo = Undo::Overwrite(self.buffer.clone());
        }
    }

    #[inline]
    pub fn prepare_for_removal(&mut self) {
        self.undo = Undo::Overwrite(self.buffer.clone());
    }

    #[inline]
    fn finish(self, kind: ErrorKind) -> Result<(), Error> {
        match Self::check_str(self.buffer.as_str()) {
            Ok(()) => Ok(()),
            Err(SyntaxError::Empty) => Ok(()),
            Err(SyntaxError::Format(_idx)) => {
                match self.undo {
                    Undo::Overwrite(other) => *self.buffer = other,
                    Undo::Truncate(idx) => self.buffer.truncate(idx),
                }
                Err(Error::new(kind))
            }
        }
    }

    #[inline]
    pub fn insert_bounded_str(
        mut self,
        idx: usize,
        fragment: &str,
        delim: char,
    ) -> Result<(), Error> {
        // Edge-Case: The boundary definition requires a delimiter.
        //
        // In this case, we know we can't form a natural chunk boundary. So we
        // should just redirect the call to `insert_delimited_fragment`.
        //
        // We hope that the compiler will then drop the rest of this function.
        if !B::CAN_FIND_BOUNDARIES {
            return self.insert_delimited_str(idx, fragment, delim);
        }

        // Edge-Case: There's no fragment data.
        //
        // The invariants of this function is that chunk boundaries are
        // maintained. An empty fragment cannot disrupt any chunk boundaries.
        //
        // Just return.
        if fragment.is_empty() {
            return Ok(());
        };

        self.prepare_for_insert(idx);

        // Since we know this *doesn't* require a delimiter, we can just try to
        // insert the fragment and see if boundaries are preserved.
        self.buffer.insert_str(idx, fragment);

        // Let's keep track of the complete inserted range of characters.
        // The value for the right side changes, so we need to keep tabs on it.
        let left_idx = idx;
        let mut right_idx = idx + fragment.len();

        // Identify if there was already a delimiter on either end in the input.
        // This will help us determine if we need to perform boundary checks.
        let mut left_has_delimiter = self.buffer[..left_idx]
            .chars()
            .last()
            .is_none_or(D::is_delim)
            || self.buffer[left_idx..]
                .chars()
                .next()
                .is_none_or(D::is_delim);
        let mut right_has_delimiter = self.buffer[..right_idx]
            .chars()
            .last()
            .is_none_or(D::is_delim)
            || self.buffer[right_idx..]
                .chars()
                .next()
                .is_none_or(D::is_delim);

        // We need to loop at most twice - fixing one side may break the other.
        // (e.g. UpperCamel -> UUpperCamel -> UU_pperCamel -> U_U_pperCamel)
        let delim_len = delim.len_utf8();
        for _ in 0..2 {
            // Check if the left and right sides require a delimiter.
            let left_has_boundary = left_has_delimiter
                || B::has_boundary_at::<P::Segmentation>(self.buffer.as_str(), left_idx);
            let right_has_boundary = right_has_delimiter
                || B::has_boundary_at::<P::Segmentation>(self.buffer.as_str(), right_idx);
            if left_has_boundary && right_has_boundary {
                break;
            }

            // Try to fix-up the left-hand side first - on failure, undo the insert.
            if !left_has_boundary {
                self.buffer.insert(left_idx, delim);
                left_has_delimiter = true;
                right_idx += delim_len;
            }

            // Try to fix-up the right-hand side next - on failure, undo the insert.
            if !right_has_boundary {
                self.buffer.insert(right_idx, delim);
                right_has_delimiter = true;
                right_idx += delim_len;
            }
        }

        self.finish(ErrorKind::FailedInsert)
    }

    #[inline]
    pub fn insert_delimited_str(
        mut self,
        mut idx: usize,
        fragment: &str,
        delim: char,
    ) -> Result<(), Error> {
        // Edge-Case: There's no fragment data.
        //
        // The invariants of this function is that chunk boundaries are
        // maintained. An empty fragment cannot disrupt any chunk boundaries.
        //
        // Just return.
        if fragment.is_empty() {
            return Ok(());
        };

        self.prepare_for_insert(idx);

        // Since we know this *requires* delimiters, we can check the inserting
        // fragment to see if we need delimiters on either end.
        let left_has_delim = self.buffer[..idx].chars().last().is_none_or(D::is_delim)
            || fragment.chars().next().is_none_or(D::is_delim);
        let right_has_delim = self.buffer[idx..].chars().next().is_none_or(D::is_delim)
            || fragment.chars().last().is_none_or(D::is_delim);

        // If either side will be missing a delim after insert, we will need to
        // insert some delimiters.
        if !left_has_delim {
            self.buffer.insert(idx, delim);
            idx += delim.len_utf8();
        }
        if !right_has_delim {
            self.buffer.insert(idx, delim);
        }
        self.buffer.insert_str(idx, fragment);
        self.finish(ErrorKind::FailedInsert)
    }

    #[inline]
    pub fn insert_str(mut self, idx: usize, fragment: &str) -> Result<(), Error> {
        self.prepare_for_insert(idx);
        self.buffer.insert_str(idx, fragment);
        self.finish(ErrorKind::FailedInsert)
    }

    #[inline]
    pub fn remove(mut self, idx: usize) -> Result<(), Error> {
        self.prepare_for_removal();
        self.buffer.remove(idx);
        self.finish(ErrorKind::FailedRemove)
    }

    #[inline]
    pub(crate) fn replace_range_str<R>(mut self, range: R, replace_with: &str) -> Result<(), Error>
    where
        R: RangeBounds<usize>,
    {
        self.prepare_for_removal();
        self.buffer.replace_range(range, replace_with);
        self.finish(ErrorKind::FailedReplace)
    }
}
