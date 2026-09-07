// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::fragment::{
    CharIndices, Chars, ChunkedSegmentIndices, ChunkedSegments, MatchIndices, Matches,
    RMatchIndices, RMatches, SegmentIndices, Segments,
};
use crate::core::{Chunk, Error, Fragment, Ident, Segment};
use crate::syntax::{Boundary, Delimiter, Profile};

// =============================================================================
// TRAITS
// =============================================================================

/// A trait for dealing with identifiers in a generic way.
///
/// Sometimes it can be a bit annoying to be specific about the exact syntax
/// rules for an identifier. Because of that, this trait exists so that you can
/// just generically say that you want something that looks like an identifier.
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// # use typed_ident::Identifier;
/// # use typed_ident::presets::unicode::*;
/// fn print_segments<I: Identifier + ?Sized>(ident: &I) {
///     println!("segments:");
///     for s in ident.segments() {
///         println!("* {s:?}");
///     }
/// }
///
/// print_segments(LowerSnakeIdent::new("lower_snake")?);
/// print_segments(UpperCamelIdent::new("UpperCamel")?);
/// # Ok::<(), typed_ident::Error>(())
/// ```
#[allow(clippy::len_without_is_empty)] // Identifiers cannot be empty.
#[allow(clippy::type_complexity)] // Types can be complex because they're fully expanded here.
pub trait Identifier {
    /// The boundary configuration in-use by this identifier.
    type Boundary: Boundary;

    /// The valid delimiters that are allowed by this identifier.
    type Delimiter: Delimiter;

    /// The character profile that is allowed by this identifier.
    type Profile: Profile;

    /// Converts to an actual typed identifier.
    ///
    /// This is used to fill in the rest of the functionality on the
    /// `Identifier` trait using default functions. However, it can also be
    /// useful if you need to depend on some specific trait implementations on
    /// an actual identifier type, versus just abstractly working with an
    /// identifier through this trait.
    fn as_ident(&self) -> &Ident<Self::Boundary, Self::Delimiter, Self::Profile>;

    /// Converts a fragment to an identifier.
    ///
    /// See the [`Ident::from_fragment`] documentation for details.
    #[inline]
    fn from_fragment(
        fragment: &Fragment<Self::Boundary, Self::Delimiter, Self::Profile>,
    ) -> Result<&Ident<Self::Boundary, Self::Delimiter, Self::Profile>, Error> {
        Ident::from_fragment(fragment)
    }

    /// Converts a string slice to an identifier.
    ///
    /// See the [`Ident::new`] documentation for details.
    #[inline]
    fn new(s: &str) -> Result<&Ident<Self::Boundary, Self::Delimiter, Self::Profile>, Error> {
        Ident::new(s)
    }

    /// Converts an allocated string to a boxed identifier.
    ///
    /// See the [`Ident::new_boxed`] documentation for details.
    #[cfg(feature = "alloc")]
    #[inline]
    fn new_boxed(
        s: std_alloc::string::String,
    ) -> Result<std_alloc::boxed::Box<Ident<Self::Boundary, Self::Delimiter, Self::Profile>>, Error>
    {
        Ident::new_boxed(s)
    }

    /// Constructs a new ident buffer for this type.
    ///
    /// See the [`FragmentBuf`] documentation for details.
    ///
    /// [`FragmentBuf`]: crate::alloc::FragmentBuf
    #[cfg(feature = "alloc")]
    #[inline]
    fn new_fragment_buffer()
    -> crate::alloc::FragmentBuf<Self::Boundary, Self::Delimiter, Self::Profile> {
        crate::alloc::FragmentBuf::new()
    }

    /// Constructs a new ident buffer for this type.
    ///
    /// See the [`IdentBuf`] documentation for details.
    ///
    /// [`IdentBuf`]: crate::alloc::IdentBuf
    #[cfg(feature = "alloc")]
    #[inline]
    fn new_ident_buffer() -> crate::alloc::IdentBuf<Self::Boundary, Self::Delimiter, Self::Profile>
    {
        crate::alloc::IdentBuf::new()
    }

    /// Casts the identifier to a fragment.
    ///
    /// See the [`Ident::as_fragment`] documentation for details.
    #[must_use]
    #[inline]
    fn as_fragment(&self) -> &Fragment<Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_ident().as_fragment()
    }

    /// Returns the first segment from an identifier
    ///
    /// See the [`Ident::first_segment`] documentation for details.
    #[must_use]
    #[inline]
    fn first_segment(
        &self,
    ) -> Segment<Self::Delimiter, &Chunk<Self::Boundary, Self::Delimiter, Self::Profile>> {
        self.as_ident().first_segment()
    }

    /// Returns the last segment from an identifier
    ///
    /// See the [`Ident::last_segment`] documentation for details.
    #[must_use]
    #[inline]
    fn last_segment(
        &self,
    ) -> Segment<Self::Delimiter, &Chunk<Self::Boundary, Self::Delimiter, Self::Profile>> {
        self.as_ident().last_segment()
    }

    /// Splits an identifier into fragments at the given index.
    ///
    /// See the [`Ident::split_at`] documentation for details.
    ///
    /// # Panics
    ///
    /// Panics if `mid` is not on a UTF-8 code point boundary, or if it is past
    /// the end of the last code point of the identifier. For a non-panicking
    /// alternative see [`split_at_checked`].
    ///
    /// [`split_at_checked`]: Self::split_at_checked
    #[must_use]
    #[inline]
    fn split_at(
        &self,
        mid: usize,
    ) -> (
        Option<&Ident<Self::Boundary, Self::Delimiter, Self::Profile>>,
        &Fragment<Self::Boundary, Self::Delimiter, Self::Profile>,
    ) {
        self.as_ident().split_at(mid)
    }

    /// Splits an identifier into fragments at the given index (checked variant).
    ///
    /// See the [`Ident::split_at_checked`] documentation for details.
    #[must_use]
    #[inline]
    fn split_at_checked(
        &self,
        mid: usize,
    ) -> Option<(
        Option<&Ident<Self::Boundary, Self::Delimiter, Self::Profile>>,
        &Fragment<Self::Boundary, Self::Delimiter, Self::Profile>,
    )> {
        self.as_ident().split_at_checked(mid)
    }

    /// Trims the decorative (non-required) delimiters from the ends of an
    /// identifier.
    ///
    /// See the [`Ident::trim_decorative_delims`] documentation for details.
    #[must_use = "this returns the remaining sub-identifier as a new identifier, without modifying the original"]
    #[inline]
    fn trim_decorative_delims(&self) -> &Ident<Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_ident().trim_decorative_delims()
    }

    /// Trims the decorative (non-required) delimiters from the start of an
    /// identifier.
    ///
    /// See the [`Ident::trim_leading_decorative_delims`] documentation for details.
    #[must_use = "this returns the remaining sub-identifier as a new identifier, without modifying the original"]
    #[inline]
    fn trim_leading_decorative_delims(
        &self,
    ) -> &Ident<Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_ident().trim_leading_decorative_delims()
    }

    /// Trims the decorative (non-required) delimiters from the end of an
    /// identifier.
    ///
    /// See the [`Ident::trim_trailing_decorative_delims`] documentation for details.
    #[must_use = "this returns the remaining sub-identifier as a new identifier, without modifying the original"]
    #[inline]
    fn trim_trailing_decorative_delims(
        &self,
    ) -> &Ident<Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_ident().trim_trailing_decorative_delims()
    }

    /// Trims the delimiters from the ends of an identifier.
    ///
    /// This function can leave you with an invalid identifier, because it may
    /// remove required delimiters to make the identifier valid (or strip down
    /// to an empty identifier). As such, it returns a [`Fragment`].
    ///
    /// For a version that leaves you with a valid identifier, see
    /// [`trim_decorative_delims`].
    ///
    /// See the [`Fragment::trim_delims`] documentation for details.
    ///
    /// [`trim_decorative_delims`]: Self::trim_decorative_delims
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    fn trim_delims(&self) -> &Fragment<Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_ident().trim_delims()
    }

    /// Trims the delimiters from the start of an identifier.
    ///
    /// This function can leave you with an invalid identifier, because it may
    /// remove required delimiters to make the identifier valid (or strip down
    /// to an empty identifier). As such, it returns a [`Fragment`].
    ///
    /// For a version that leaves you with a valid identifier, see
    /// [`trim_leading_decorative_delims`].
    ///
    /// See the [`Fragment::trim_leading_delims`] documentation for details.
    ///
    /// [`trim_leading_decorative_delims`]: Self::trim_leading_decorative_delims
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    fn trim_leading_delims(&self) -> &Fragment<Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_ident().trim_leading_delims()
    }

    /// Trims the delimiters from the end of an identifier.
    ///
    /// This function can leave you with an invalid identifier, because it may
    /// strip down to an empty identifier. As such, it returns a [`Fragment`].
    ///
    /// For a version that leaves you with a valid identifier, see
    /// [`trim_trailing_decorative_delims`].
    ///
    /// See the [`Fragment::trim_trailing_delims`] documentation for details.
    ///
    /// [`trim_trailing_decorative_delims`]: Self::trim_trailing_decorative_delims
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    fn trim_trailing_delims(&self) -> &Fragment<Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_ident().trim_trailing_delims()
    }

    /// Returns whether the identifier has leading delimiters.
    ///
    /// See the [`Fragment::has_leading_delim`] documentation for details.
    #[must_use]
    #[inline]
    fn has_leading_delim(&self) -> bool {
        self.as_fragment().has_leading_delim()
    }

    /// Returns whether the identifier has trailing delimiters.
    ///
    /// See the [`Fragment::has_trailing_delim`] documentation for details.
    #[must_use]
    #[inline]
    fn has_trailing_delim(&self) -> bool {
        self.as_fragment().has_trailing_delim()
    }

    /// Returns whether the identifier consists of only delimiters.
    ///
    /// See the [`Fragment::is_anonymous`] documentation for details.
    #[must_use]
    #[inline]
    fn is_anonymous(&self) -> bool {
        self.as_fragment().is_anonymous()
    }

    /// Returns the string representation of an identifier.
    ///
    /// See the [`Fragment::as_str`] documentation for details.
    #[must_use]
    #[inline]
    fn as_str(&self) -> &str {
        self.as_fragment().as_str()
    }

    /// Returns whether the identifier contains a pattern.
    ///
    /// See the [`Fragment::contains`] documentation for details.
    #[must_use]
    #[inline]
    fn contains<M>(&self, pat: M) -> bool
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().contains(pat)
    }

    /// Returns whether the identifier ends with a pattern.
    ///
    /// See the [`Fragment::ends_with`] documentation for details.
    #[must_use]
    #[inline]
    fn ends_with<M>(&self, pat: M) -> bool
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().ends_with(pat)
    }

    /// Returns whether the identifier starts with a pattern.
    ///
    /// See the [`Fragment::starts_with`] documentation for details.
    #[must_use]
    #[inline]
    fn starts_with<M>(&self, pat: M) -> bool
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().starts_with(pat)
    }

    /// Returns the position where a pattern can first be found.
    ///
    /// See the [`Fragment::find`] documentation for details.
    #[must_use]
    #[inline]
    fn find<M>(&self, pat: M) -> Option<usize>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().find(pat)
    }

    /// Returns the position where a pattern can first be found from the end.
    ///
    /// See the [`Fragment::rfind`] documentation for details.
    #[must_use]
    #[inline]
    fn rfind<M>(&self, pat: M) -> Option<usize>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().rfind(pat)
    }

    /// Returns an iterator for chunked segments of an identifier.
    ///
    /// See the [`Fragment::chunked_segments`] documentation for details.
    #[must_use]
    #[inline]
    fn chunked_segments(
        &self,
    ) -> ChunkedSegments<'_, Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_fragment().chunked_segments()
    }

    /// Returns an iterator for chunked segments of an identifier with their
    /// positions.
    ///
    /// See the [`Fragment::chunked_segment_indices`] documentation for details.
    #[must_use]
    #[inline]
    fn chunked_segment_indices(
        &self,
    ) -> ChunkedSegmentIndices<'_, Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_fragment().chunked_segment_indices()
    }

    /// Returns an iterator for segments of an identifier.
    ///
    /// See the [`Fragment::segments`] documentation for details.
    #[must_use]
    #[inline]
    fn segments(&self) -> Segments<'_, Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_fragment().segments()
    }

    /// Returns an iterator for segments of an identifier with their positions.
    ///
    /// See the [`Fragment::segment_indices`] documentation for details.
    #[must_use]
    #[inline]
    fn segment_indices(
        &self,
    ) -> SegmentIndices<'_, Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_fragment().segment_indices()
    }

    /// Zero-cost casts an identifier to another compatible format.
    ///
    /// See the [`Ident::cast`] documentation for details.
    #[must_use]
    #[inline]
    fn cast<B2, D2, P2>(&self) -> &Ident<B2, D2, P2>
    where
        Self::Delimiter: crate::syntax::SubsetOf<D2>,
        Self::Profile: crate::syntax::SubsetOf<P2>,
    {
        self.as_ident().cast()
    }

    /// Returns an iterator over the characters of an identifier and their
    /// positions.
    ///
    /// See the [`Fragment::char_indices`] documentation for details.
    #[must_use]
    #[inline]
    fn char_indices(&self) -> CharIndices<'_, Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_fragment().char_indices()
    }

    /// Returns an iterator over the characters of an identifier.
    ///
    /// See the [`Fragment::chars`] documentation for details.
    #[must_use]
    #[inline]
    fn chars(&self) -> Chars<'_, Self::Boundary, Self::Delimiter, Self::Profile> {
        self.as_fragment().chars()
    }

    /// Returns an iterator over the matches of an identifier and their positions.
    ///
    /// See the [`Fragment::match_indices`] documentation for details.
    #[must_use]
    #[inline]
    fn match_indices<M>(
        &self,
        pat: M,
    ) -> MatchIndices<'_, Self::Boundary, Self::Delimiter, Self::Profile, M>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().match_indices(pat)
    }

    /// Returns an iterator over the matches of an identifier.
    ///
    /// See the [`Fragment::matches`] documentation for details.
    #[must_use]
    #[inline]
    fn matches<M>(&self, pat: M) -> Matches<'_, Self::Boundary, Self::Delimiter, Self::Profile, M>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().matches(pat)
    }

    /// Returns an iterator over the matches of an identifier from the end, and
    /// their positions.
    ///
    /// See the [`Fragment::rmatch_indices`] documentation for details.
    #[must_use]
    #[inline]
    fn rmatch_indices<M>(
        &self,
        pat: M,
    ) -> RMatchIndices<'_, Self::Boundary, Self::Delimiter, Self::Profile, M>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().rmatch_indices(pat)
    }

    /// Returns an iterator over the matches of an identifier from the end.
    ///
    /// See the [`Fragment::rmatches`] documentation for details.
    #[must_use]
    #[inline]
    fn rmatches<M>(&self, pat: M) -> RMatches<'_, Self::Boundary, Self::Delimiter, Self::Profile, M>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().rmatches(pat)
    }

    /// Trims the characters that match the provided pattern from the start.
    ///
    /// See the [`Fragment::trim_start_matches`] documentation for details.
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    fn trim_start_matches<M>(
        &self,
        pat: M,
    ) -> &Fragment<Self::Boundary, Self::Delimiter, Self::Profile>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().trim_start_matches(pat)
    }

    /// Trims the characters that match the provided pattern from the end.
    ///
    /// See the [`Fragment::trim_end_matches`] documentation for details.
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    fn trim_end_matches<M>(
        &self,
        pat: M,
    ) -> &Fragment<Self::Boundary, Self::Delimiter, Self::Profile>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().trim_end_matches(pat)
    }

    /// Returns a subslice of an identifier.
    ///
    /// This is the checked variant to just using the index operator. If you are
    /// anyways going to `unwrap` or `expect`, you should just use the index
    /// operator - as it will panic on failure.
    ///
    /// See the [`Fragment::get`] documentation for details.
    #[must_use]
    #[inline]
    fn get<I: crate::core::SliceIndex<Fragment<Self::Boundary, Self::Delimiter, Self::Profile>>>(
        &self,
        i: I,
    ) -> Option<&Fragment<Self::Boundary, Self::Delimiter, Self::Profile>> {
        self.as_fragment().get(i)
    }

    /// Returns an unchecked subslice of an identifier.
    ///
    /// See the [`Ident::get_unchecked`] documentation for details.
    ///
    /// # Safety
    ///
    /// Callers of this function are responsible that these preconditions
    /// are satisfied:
    ///
    /// * The starting index must not exceed the ending index;
    /// * Indexes must be within bounds of the original slice;
    /// * Indexes must lie on UTF-8 sequence boundaries.
    #[must_use]
    #[inline]
    unsafe fn get_unchecked<
        I: crate::core::SliceIndex<Fragment<Self::Boundary, Self::Delimiter, Self::Profile>>,
    >(
        &self,
        i: I,
    ) -> &Fragment<Self::Boundary, Self::Delimiter, Self::Profile> {
        unsafe { self.as_fragment().get_unchecked(i) }
    }

    /// Returns the length of the identifier.
    ///
    /// See the [`Fragment::len`] documentation for details.
    #[must_use]
    #[inline(always)]
    fn len(&self) -> usize {
        self.as_fragment().len()
    }

    /// Strips the circumfix off an identifier.
    ///
    /// See the [`Fragment::strip_circumfix`] documentation for details.
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    fn strip_circumfix<Prefix, Suffix>(
        &self,
        prefix: Prefix,
        suffix: Suffix,
    ) -> Option<&Fragment<Self::Boundary, Self::Delimiter, Self::Profile>>
    where
        Prefix: crate::core::pattern::Pattern,
        Suffix: crate::core::pattern::Pattern,
    {
        self.as_fragment().strip_circumfix(prefix, suffix)
    }

    /// Strips the prefix off an identifier.
    ///
    /// See the [`Fragment::strip_prefix`] documentation for details.
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    fn strip_prefix<M>(
        &self,
        prefix: M,
    ) -> Option<&Fragment<Self::Boundary, Self::Delimiter, Self::Profile>>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().strip_prefix(prefix)
    }

    /// Strips the suffix off an identifier.
    ///
    /// See the [`Fragment::strip_suffix`] documentation for details.
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    fn strip_suffix<M>(
        &self,
        suffix: M,
    ) -> Option<&Fragment<Self::Boundary, Self::Delimiter, Self::Profile>>
    where
        M: crate::core::pattern::Pattern,
    {
        self.as_fragment().strip_suffix(suffix)
    }
}
