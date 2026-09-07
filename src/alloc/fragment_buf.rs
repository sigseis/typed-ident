// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::error::{Error, ErrorKind};
use crate::core::{Chunk, Fragment, Ident};
use crate::syntax::{Boundary, Delimiter, Profile};
use core::marker::PhantomData;
use core::ops::{Bound, RangeBounds};
use std_alloc::collections::TryReserveError;
use std_alloc::string::String;

// =============================================================================
// TYPES
// =============================================================================

/// A dynamic, growable fragment.
///
/// This allows you to build a fragment dynamically, instead of having to get a
/// fragment from an identifier slice.
pub struct FragmentBuf<B, D, P> {
    config: PhantomData<(B, D, P)>,
    inner: String,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> FragmentBuf<B, D, P> {
    #[must_use]
    #[inline]
    pub(crate) fn can_push_fragment_char(left: &Fragment<B, D, P>, right: char) -> bool {
        // Append-closed means that taking any fragment and appending it on the
        // end of any other fragment, is a valid activity. Here, we're looking
        // at a single `char`, verified to be a valid part of a fragment.
        match D::APPEND_CLOSED.at_least_fragment() && P::APPEND_CLOSED.at_least_fragment() {
            true => true,
            false => {
                // If the first character to the right of the join is a chunk
                // delimiter, then it's always safe to join.
                //
                // Otherwise, we need to inspect the last character to the left
                // of the join - if it's a delimiter we need a chunk start, and
                // if it's not a delimiter we're in a chunk, so we need a chunk
                // continue character next.
                D::is_chunk_delim(right)
                    || match left.chars().last() {
                        None => true,
                        // If the profile is append-closed, then we don't need
                        // to check if it's a delimiter, because any character
                        // from chunk-continue is valid after a delim or char.
                        Some(left) => {
                            match !P::APPEND_CLOSED.at_least_fragment() && D::is_delim(left) {
                                true => P::is_chunk_start(right),
                                false => P::is_chunk_continue(right),
                            }
                        }
                    }
            }
        }
    }

    #[must_use]
    #[inline]
    pub(crate) fn can_join(left: &Fragment<B, D, P>, right: &Fragment<B, D, P>) -> bool {
        right
            .chars()
            .next()
            .is_none_or(|c| Self::can_push_fragment_char(left, c))
    }

    /// Attempts to represent the current fragment buffer as an [`Ident`].
    ///
    /// This may fail - a fragment isn't obviously a valid identifier, plus the
    /// fragment could be empty (which is never a valid identifier).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::hybrid::HybridFragmentBuf as UnicodeFragmentBuf;
    /// let mut buffer = UnicodeFragmentBuf::new();
    /// assert!(buffer.as_ident().is_err()); // Empty
    /// buffer.push('2')?;
    /// assert!(buffer.as_ident().is_err()); // Invalid start character
    ///
    /// buffer.clear();
    /// buffer.push('a')?;
    /// assert!(buffer.as_ident().is_ok()); // Valid!
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn as_ident(&self) -> Result<&Ident<B, D, P>, Error> {
        Ident::from_fragment(self.as_fragment())
    }

    /// Constructs a fragment buffer, initializing the contents to a provided
    /// string slice (attempting first to convert the string slice to a
    /// fragment).
    ///
    /// This is equivalent to `FragmentBuf::from_fragment(Fragment::new(s)?)`.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// assert!(UpperCamelFragmentBuf::from_str("").is_ok());
    /// assert!(UpperCamelFragmentBuf::from_str("ValidUpperCamel").is_ok());
    /// assert!(UpperCamelFragmentBuf::from_str("continuingUpperCamel").is_ok());
    /// assert!(UpperCamelFragmentBuf::from_str("not_validUpperCamel").is_err());
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    #[allow(clippy::should_implement_trait)] // It *does* implement the trait.
    pub fn from_str(s: &str) -> Result<Self, Error> {
        core::str::FromStr::from_str(s)
    }

    /// Constructs a fragment buffer, initializing the contents to a provided
    /// buffered string (checking first that the string is a valid fragment).
    ///
    /// This is similar to [`from_str`], except that it will not allocate a
    /// separate string. It will use the provided string, if it's valid.
    ///
    /// [`from_str`]: Self::from_str
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// assert!(UpperCamelFragmentBuf::from_string(String::from("")).is_ok());
    /// assert!(UpperCamelFragmentBuf::from_string(String::from("ValidUpperCamel")).is_ok());
    /// assert!(UpperCamelFragmentBuf::from_string(String::from("continuingUpperCamel")).is_ok());
    /// assert!(UpperCamelFragmentBuf::from_string(String::from("not_validUpperCamel")).is_err());
    /// # Ok::<(), Error>(())
    #[inline]
    pub fn from_string(s: String) -> Result<Self, Error> {
        let _ = Fragment::<B, D, P>::new(&s)?;
        Ok(Self::from_string_unchecked(s))
    }

    #[doc = include_str!("docs/methods/insert_fragment.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    #[doc = include_str!("docs/sections/errors.md")]
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::UpperCamelFragment;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_fragment(
    ///     0,
    ///     UpperCamelFragment::new("HAT")?,
    /// ).is_ok());
    /// assert_eq!(example, "HATUpperCamel");
    ///
    /// // Inserting at the end is ~append.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_fragment(
    ///     example.len(),
    ///     UpperCamelFragment::new("lower")?,
    /// ).is_ok());
    /// assert!(example.insert_fragment(
    ///     example.len(),
    ///     UpperCamelFragment::new("Camel")?,
    /// ).is_ok());
    /// assert_eq!(example, "UpperCamellowerCamel");
    ///
    /// // Inserting in the middle can be tricky, as your insertions
    /// // may invalidate the buffer's invariants in surprising ways.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_fragment(
    ///     2,
    ///     UpperCamelFragment::new("HAT_")?,
    /// ).is_err()); // "UpHAT_perCamel" != UpperCamel casing
    /// assert!(example.insert_fragment(
    ///     5,
    ///     UpperCamelFragment::new("HAT")?,
    /// ).is_ok());
    /// assert_eq!(example, "UpperHATCamel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_fragment(
        &mut self,
        idx: usize,
        fragment: &Fragment<B, D, P>,
    ) -> Result<(), Error> {
        let (left, right) = self.split_at(idx);
        if !Self::can_join(left, fragment) {
            return Err(Error::new(ErrorKind::FailedJoinLeft));
        }
        if !Self::can_join(fragment, right) {
            return Err(Error::new(ErrorKind::FailedJoinRight));
        }
        self.inner.insert_str(idx, fragment.as_str());
        Ok(())
    }

    #[doc = include_str!("docs/methods/insert_bounded_fragment.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    #[doc = include_str!("docs/sections/errors.md")]
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_fragment_with(
    ///     0,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok()); // Bounded because of `HAT` rules.
    /// assert!(example.insert_bounded_fragment_with(
    ///     0,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok()); // But another would not be.
    /// assert_eq!(example, "HAT_HATUpperCamel");
    ///
    /// // Inserting at the end is ~append.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_fragment_with(
    ///     example.len(),
    ///     UpperCamelFragment::new("lower")?,
    ///     LowLine,
    /// ).is_err()); // "UpperCamel_lower" != UpperCamel casing
    /// assert!(example.insert_bounded_fragment_with(
    ///     example.len(),
    ///     UpperCamelFragment::new("Camel")?,
    ///     LowLine,
    /// ).is_ok()); // Because of `CAMEL` boundary.
    /// assert_eq!(example, "UpperCamelCamel");
    ///
    /// // Inserting in the middle can be tricky, as your insertions
    /// // may invalidate the buffer's invariants in surprising ways.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_bounded_fragment_with(
    ///     2,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_err()); // "UpHAT_perCamel" != UpperCamel casing
    /// assert!(example.insert_bounded_fragment_with(
    ///     5,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok()); // Surprisingly a `CAMEL` & `HAT` boundary.
    /// assert_eq!(example, "UpperHATCamel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_bounded_fragment_with(
        &mut self,
        idx: usize,
        fragment: &Fragment<B, D, P>,
        delim: D,
    ) -> Result<(), Error> {
        // Edge-Case: The boundary definition requires a delimiter.
        //
        // In this case, we know we can't form a natural chunk boundary. So we
        // should just redirect the call to `insert_delimited_fragment_with`.
        //
        // We hope that the compiler will then drop the rest of this function.
        if !B::CAN_FIND_BOUNDARIES {
            return self.insert_delimited_fragment_with(idx, fragment, delim);
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

        // Since we know this *doesn't* require a delimiter, we can just try to
        // insert the fragment and see if boundaries are preserved.
        //
        // This obviously can fail, but interestingly, because we defined the
        // profile to have chunk_continue characters be a superset of chunk and
        // ident start characters, it can only fail in ways that we would not
        // anyways be able to remedy by simply adding a delimiter (which, for
        // this function, is our only possible remediation).
        //
        // So, thankfully, we can start with an insert, then check the sides.
        self.insert_fragment(idx, fragment)?;

        // Let's keep track of the complete inserted range of characters.
        // This way, if we need to undo, we can by simply removing the range.
        let left_idx = idx;
        let mut right_idx = idx + fragment.len();

        // Identify if there was already a delimiter on either end in the input.
        // This will help us determine if we need to perform boundary checks.
        let mut left_has_delimiter = self[..left_idx].chars().last().is_none_or(D::is_delim)
            || self[left_idx..].chars().next().is_none_or(D::is_delim);
        let mut right_has_delimiter = self[..right_idx].chars().last().is_none_or(D::is_delim)
            || self[right_idx..].chars().next().is_none_or(D::is_delim);

        // We need to loop at most twice - fixing one side may break the other.
        // (e.g. UpperCamel -> UUpperCamel -> UU_pperCamel -> U_U_pperCamel)
        let delim_len = delim.as_char().len_utf8();
        for _ in 0..2 {
            // Check if the left and right sides require a delimiter.
            let left_has_boundary = left_has_delimiter
                || B::has_boundary_at::<P::Segmentation>(self.as_str(), left_idx);
            let right_has_boundary = right_has_delimiter
                || B::has_boundary_at::<P::Segmentation>(self.as_str(), right_idx);
            if left_has_boundary && right_has_boundary {
                break;
            }

            // Try to fix-up the left-hand side first - on failure, undo the insert.
            if !left_has_boundary {
                let result = self.insert_delim_with(left_idx, delim);
                if result.is_err() {
                    // Don't need to check it again, if it was valid before, it's
                    // still valid after we "undo".
                    self.inner.replace_range(left_idx..right_idx, "");
                    return Err(Error::new(ErrorKind::FailedJoinLeft));
                }
                left_has_delimiter = true;
                right_idx += delim_len;
            }

            // Try to fix-up the right-hand side next - on failure, undo the insert.
            if !right_has_boundary {
                let result = self.insert_delim_with(right_idx, delim);
                if result.is_err() {
                    // Don't need to check it again, if it was valid before, it's
                    // still valid after we "undo".
                    self.inner.replace_range(left_idx..right_idx, "");
                    return Err(Error::new(ErrorKind::FailedJoinRight));
                }
                right_has_delimiter = true;
                right_idx += delim_len;
            }
        }

        Ok(())
    }

    #[doc = include_str!("docs/methods/insert_delimited_fragment.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    #[doc = include_str!("docs/sections/errors.md")]
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    ///
    /// // Inserting at the beginning is ~prepend.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_fragment_with(
    ///     0,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok());
    /// assert_eq!(example, "HAT_UpperCamel");
    ///
    /// // Inserting at the end is ~append.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_fragment_with(
    ///     example.len(),
    ///     UpperCamelFragment::new("lower")?,
    ///     LowLine,
    /// ).is_err()); // "UpperCamel_lower" != UpperCamel casing
    /// assert!(example.insert_delimited_fragment_with(
    ///     example.len(),
    ///     UpperCamelFragment::new("Camel")?,
    ///     LowLine,
    /// ).is_ok());
    /// assert_eq!(example, "UpperCamel_Camel");
    ///
    /// // Inserting in the middle can be tricky, as your insertions
    /// // may invalidate the buffer's invariants in surprising ways.
    /// let mut example = buffer.clone();
    /// assert!(example.insert_delimited_fragment_with(
    ///     2,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_err()); // "Up_HAT_perCamel" != UpperCamel casing
    /// assert!(example.insert_delimited_fragment_with(
    ///     5,
    ///     UpperCamelFragment::new("HAT")?,
    ///     LowLine,
    /// ).is_ok());
    /// assert_eq!(example, "Upper_HAT_Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn insert_delimited_fragment_with(
        &mut self,
        mut idx: usize,
        fragment: &Fragment<B, D, P>,
        delim: D,
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

        // Since we know this *requires* delimiters, we can check the inserting
        // fragment to see if we need delimiters on either end.
        let left_has_delim = self[..idx].chars().last().is_none_or(D::is_delim)
            || fragment.chars().next().is_none_or(D::is_delim);
        let right_has_delim = self[idx..].chars().next().is_none_or(D::is_delim)
            || fragment.chars().last().is_none_or(D::is_delim);

        // Let's keep track of the complete inserted range of characters.
        // This way, if we need to undo, we can by simply removing the range.
        let left_idx = idx;
        let mut right_idx = idx;
        let delim_len = delim.as_char().len_utf8();

        // If either side will be missing a delim after insert, we will need to
        // insert some delimiters.
        if !left_has_delim {
            self.insert_delim_with(idx, delim)?;
            idx += delim_len;
            right_idx += delim_len;
        }
        if !right_has_delim {
            let result = self.insert_delim_with(idx, delim);
            if result.is_err() {
                // Don't need to check it again, if it was valid before, it's
                // still valid after we "undo".
                self.inner.replace_range(left_idx..right_idx, "");
                return result;
            }
            right_idx += delim_len;
        }

        // Finally, we can simply insert the fragment, and it will either work
        // or not (nothing we can do but return an error if not).
        let result = self.insert_fragment(idx, fragment);
        if result.is_err() {
            // Don't need to check it again, if it was valid before, it's
            // still valid after we "undo".
            self.inner.replace_range(left_idx..right_idx, "");
        }
        result
    }

    #[doc = include_str!("docs/methods/push_delim.md")]
    #[doc = include_str!("docs/methods/push_delim.errors.md")]
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use syntax::delimiter::LowLine;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// let mut buffer = UpperCamelFragmentBuf::new();
    ///
    /// // For all preset and provided delimiters, you can push them anywhere in
    /// // an identifier. Unless you have a custom delimiter, it's always safe to push.
    /// assert!(buffer.push_delim_with(LowLine).is_ok());
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// Example Failure:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::*;
    /// # use syntax::delimiter::LowLine;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// struct DollarStart;
    ///
    /// impl Delimiter for DollarStart {
    ///     fn as_char(&self) -> char {
    ///         '$'
    ///     }
    ///     fn from_ident_start(c: char) -> Option<Self> {
    ///         match c {
    ///             '$' => Some(Self),
    ///             _ => None,
    ///         }
    ///     }
    ///     fn from_chunk_delim(c: char) -> Option<Self> {
    ///         None
    ///     }
    /// }
    ///
    /// type DollarStartFragmentBuf = FragmentBuf<
    ///     boundary::Standard,
    ///     DollarStart,
    ///     profile::Unicode,
    /// >;
    ///
    /// let mut buffer = DollarStartFragmentBuf::new();
    ///
    /// // Okay to push one `$` in, because it may be the start fragment.
    /// assert!(buffer.push_delim_with(DollarStart).is_ok());
    ///
    /// // But you definitely cannot push another in - that's invalid.
    /// assert!(buffer.push_delim_with(DollarStart).is_err());
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn push_delim_with(&mut self, delim: D) -> Result<(), Error> {
        let delim = delim.as_char();
        if !D::APPEND_CLOSED.at_least_fragment()
            && !self.inner.is_empty()
            && !D::is_chunk_delim(delim)
        {
            return Err(Error::new(ErrorKind::FailedJoinLeft));
        }
        self.inner.push(delim);
        Ok(())
    }

    #[doc = include_str!("docs/methods/remove.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    ///
    /// # Errors
    ///
    /// If the removal of the character at the provided index would lead to an
    /// invalid buffer, then the character will not be remove and instead the
    /// error `FailedRemove` will be returned.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("Upper_Camel")?;
    ///
    /// // This would be valid, because it might be a continuation fragment.
    /// assert!(buffer.remove(0).is_ok());
    /// assert_eq!(buffer, "pper_Camel");
    ///
    /// // However, attempting to remove `C` would fail for `UpperCamel`.
    /// assert!(buffer.remove(5).is_err());
    /// assert_eq!(buffer, "pper_Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn remove(&mut self, idx: usize) -> Result<(), Error> {
        let (left, right) = self.split_at(idx);
        let mut chars = right.chars();
        let _ = chars.next();
        let right = chars.as_fragment();
        if !Self::can_join(left, right) {
            return Err(Error::new(ErrorKind::FailedRemove).with_byte_offset(idx));
        }
        self.remove_unchecked(idx);
        Ok(())
    }

    #[doc = include_str!("docs/methods/replace_range.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    ///
    /// # Errors
    ///
    /// Returns `Err` if the fragment formed from the combination of `self` and
    /// `to` is invalid at any replacement index. If invalid, an [`Error`] is
    /// returned with the [`error_kind`] set to `FailedReplaceLeft` or
    /// `FailedReplaceRight` (if the replacement succeeded, but the
    /// remainder could not be appended).
    ///
    /// The value [`byte_offset`] *WILL* be set from this function, and it will
    /// be set to the index that caused the failure from the original fragment
    /// (`self`).
    ///
    /// [`Error`]: crate::Error
    /// [`error_kind`]: crate::Error::error_kind
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::*;
    /// let buffer = UpperCamelFragmentBuf::from_str("Upper_Camel")?;
    ///
    /// // Examples replacing various ranges.
    /// let replacement = UpperCamelFragment::new("R")?;
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(4..7, replacement).is_ok());
    /// assert_eq!(example, "UppeRamel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(4..=7, replacement).is_ok());
    /// assert_eq!(example, "UppeRmel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(..7, replacement).is_ok());
    /// assert_eq!(example, "Ramel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(..=7, replacement).is_ok());
    /// assert_eq!(example, "Rmel");
    ///
    /// let mut example = buffer.clone();
    /// assert!(example.replace_range_fragment(4.., replacement).is_ok());
    /// assert_eq!(example, "UppeR");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn replace_range_fragment<R>(
        &mut self,
        range: R,
        replace_with: &Fragment<B, D, P>,
    ) -> Result<(), Error>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(idx) => *idx,
            Bound::Excluded(idx) => *idx + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(idx) => *idx + 1,
            Bound::Excluded(idx) => *idx,
            Bound::Unbounded => self.len(),
        };
        let left = &self[..start];
        let right = &self[end..];
        if !Self::can_join(left, replace_with) {
            return Err(Error::new(ErrorKind::FailedReplaceLeft).with_byte_offset(start));
        }
        if !Self::can_join(replace_with, right) {
            return Err(Error::new(ErrorKind::FailedReplaceRight).with_byte_offset(end));
        }
        self.inner.replace_range(range, replace_with.as_str());
        Ok(())
    }

    #[doc = include_str!("docs/methods/split_off.md")]
    #[doc = include_str!("docs/sections/panics.md")]
    ///
    /// # Errors
    ///
    /// If the replacement of the range provided with the given fragment would
    /// lead to an invalid buffer, then the range will not be remove and instead
    /// the error `InvalidReplace` will be returned.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use typed_ident::syntax::delimiter::*;
    /// # use presets::unicode::upper_camel::*;
    /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
    /// let split = buffer.split_off(5);
    /// assert_eq!(buffer, "Upper");
    /// assert_eq!(split, "Camel");
    /// # Ok::<(), Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn split_off(&mut self, idx: usize) -> FragmentBuf<B, D, P> {
        Self::from_string_unchecked(self.inner.split_off(idx))
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> FragmentBuf<B, D, P> {
    /// Converts a fragment slice into a fragment buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::*;
    /// # use presets::unicode::upper_camel::UpperCamelFragment;
    /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
    /// let fragment = UpperCamelFragment::new("example")?;
    /// let mut buffer = UpperCamelFragmentBuf::from_fragment(fragment);
    /// assert_eq!(buffer, "example");
    /// # Ok::<(), Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn from_fragment(fragment: &Fragment<B, D, P>) -> Self {
        Self::from_string_unchecked(String::from(fragment.as_str()))
    }

    /// Converts an allocated string into a fragment buffer, without checking if
    /// the allocated string is a valid fragment or not.
    ///
    /// # Safety
    ///
    /// You can only call this if the input string is from a valid [`Fragment`]
    /// over the same generic parameters, or if you have ensured the string
    /// *would* have been valid.
    ///
    /// Needless to say, this is difficult to know unless you are taking a slice
    /// of an existing fragment/ident/etc, or if you are testing this at compile
    /// time.
    ///
    /// [`Fragment`]: crate::core::Fragment
    #[must_use]
    #[inline]
    pub(crate) fn from_string_unchecked(orig: String) -> Self {
        Self {
            config: PhantomData,
            inner: orig,
        }
    }

    /// Convert the buffer into an owned string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragmentBuf::from_str("example")?;
    /// let string: String = fragment.into_string();
    /// assert_eq!(string, "example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn into_string(self) -> String {
        self.inner
    }

    /// Leaks the fragment so that it lives for the rest of the execution of the
    /// program.
    ///
    /// This is a typed wrapper over the [`String::leak`] method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragmentBuf::from_str("example")?;
    /// let string: &'static UpperCamelFragment = fragment.leak();
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn leak<'a>(self) -> &'a Fragment<B, D, P> {
        Fragment::new_unchecked(self.inner.leak())
    }

    #[inline]
    pub(crate) fn remove_unchecked(&mut self, idx: usize) {
        self.inner.remove(idx);
    }

    /// Constructs an empty fragment buffer with an initial capacity.
    ///
    /// This has the same properties as [`String::with_capacity`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let buffer = UpperCamelFragmentBuf::with_capacity(10);
    /// assert!(buffer.capacity() >= 10);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            config: PhantomData,
            inner: String::with_capacity(capacity),
        }
    }

    /// Constructs a fragment with enough space to hold the provided fragment,
    /// as well as `additional` bytes, then initializes the contents of this
    /// buffer to `fragment`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("example")?;
    /// let buffer = UpperCamelFragmentBuf::with_overhead(fragment, 20);
    /// assert!(buffer.capacity() >= 27);
    /// assert_eq!(buffer, "example");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn with_overhead(fragment: &Fragment<B, D, P>, additional: usize) -> Self {
        let mut inner = String::with_capacity(fragment.len() + additional);
        inner.push_str(fragment.as_str());
        Self {
            config: PhantomData,
            inner,
        }
    }
}

// -----------------------------------------------------------------------------
impl_buffer_methods! {
    name=FragmentBuf,
}

// =============================================================================
// TRAIT IMPL
// =============================================================================

// -----------------------------------------------------------------------------
impl<'a, B, D, P> core::convert::From<&'a Fragment<B, D, P>> for FragmentBuf<B, D, P> {
    #[inline]
    fn from(orig: &'a Fragment<B, D, P>) -> Self {
        Self::from_fragment(orig)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Clone for FragmentBuf<B, D, P> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            config: PhantomData,
            inner: self.inner.clone(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> std_alloc::borrow::Borrow<Fragment<B, D, P>> for FragmentBuf<B, D, P> {
    #[inline]
    fn borrow(&self) -> &Fragment<B, D, P> {
        self.as_fragment()
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> std_alloc::borrow::ToOwned for Fragment<B, D, P> {
    type Owned = FragmentBuf<B, D, P>;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.to_fragment_buf()
    }
}

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> core::str::FromStr for FragmentBuf<B, D, P> {
    type Err = Error;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_fragment(Fragment::new(s)?))
    }
}

// -----------------------------------------------------------------------------
impl_buffer_traits! {
    name=FragmentBuf,
}

// -----------------------------------------------------------------------------
impl_typed_slice_traits! {
    name=FragmentBuf,
    index_target=Fragment,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=FragmentBuf,
    against=Chunk,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=FragmentBuf,
    against=Fragment,
}

// -----------------------------------------------------------------------------
impl_typed_slice_cmp! {
    name=FragmentBuf,
    against=Ident,
}
