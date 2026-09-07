// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "fragment.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::core::fragment::{
    CharIndices, Chars, ChunkedSegmentIndices, ChunkedSegments, MatchIndices, Matches,
    RMatchIndices, RMatches, SegmentIndices, Segments,
};
use crate::core::{Error, ErrorKind};
use crate::syntax::{Boundary, Delimiter, Profile};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An immutable UTF-8 encoded slice of an [`Ident`].
///
/// This type can be returned during sub-slicing operations on a fragment or
/// identifier (such as [`get`], [`matches`], [`split_at`], etc).
///
/// [`get`]: Fragment::get
/// [`matches`]: Fragment::matches
/// [`split_at`]: Fragment::split_at
///
/// # Type Parameters
///
/// The type parameters used on this type are:
///
/// * `B`: [`Boundary`] (a boundary definition; usually [`Standard`])
/// * `D`: [`Delimiter`] (a delimiter type; [`HyphenMinus`], [`LowLine`], etc.)
/// * `P`: [`Profile`] (a character profile; [`Ascii`], [`Unicode`], etc.)
///
/// # Character Requirements
///
/// The underlying string must either be empty, *or* satisfy the following:
///
/// 1. Starts with a valid starting sub-fragment, which is *either*:
///    1. A delimiter character that is [`D::is_delim`].
///    2. A chunk character that is [`P::in_profile`], followed by 0 or more
///       chunk characters which are [`P::is_chunk_continue`], up until the
///       next [`D::is_delim`] character.
/// 2. Followed by 0 or more continuing sub-fragments, which are *either*:
///    1. A delimiter character that is [`D::is_chunk_delim`].
///    2. A chunk character that is [`P::is_chunk_start`], followed by 0 or
///       more chunk characters which are [`P::is_chunk_continue`], up until
///       the next [`D::is_delim`] character.
///
/// This is a very technical way of saying that any valid fragment must be a
/// possible sub-slice of some theoretically-valid identifier with the same
/// syntax type parameters.
///
/// [`D::is_chunk_delim`]: crate::syntax::delimiter::Delimiter::is_chunk_delim
/// [`D::is_delim`]: crate::syntax::delimiter::Delimiter::is_delim
/// [`P::in_profile`]: crate::syntax::profile::Profile::in_profile
/// [`P::is_chunk_continue`]: crate::syntax::profile::Profile::is_chunk_continue
/// [`P::is_chunk_start`]: crate::syntax::profile::Profile::is_chunk_start
///
/// # Useful Properties
///
/// Some useful properties to be aware of when dealing with fragments:
///
/// * An empty string slice is always a valid fragment.
/// * A slice of any fragment is itself a fragment over the same generics.
///   * e.g. as long as we don't change the type parameters, you can slice a
///     fragment and get another valid fragment over the same types.
/// * You can trivially [`cast`] one fragment to another as long as the
///   fragment's generic types are [`SubsetOf`] the target fragment's generics.
///   * e.g. as long as we are casting to a more broad format, it's trivial and
///     we do not need to check the format again (enforced by the trait system).
///
/// # Examples
///
/// It is recommended that you configure a type alias to work with fragments, so
/// that you don't need to provide the type parameters everywhere (or use one of
/// the provided [`presets`]).
///
/// ```
/// // Custom Fragment Example
/// use typed_ident::core::Fragment;
/// use typed_ident::syntax::{boundary, delimiter, profile};
/// type CustomFragment = Fragment<
///     boundary::Standard,
///     delimiter::LowLine,
///     profile::Lower<profile::Unicode>,
/// >;
/// assert!(CustomFragment::new("only_accepts_lowercase").is_ok());
///
/// // Preset Fragment Example
/// use typed_ident::presets::unicode::upper_camel::UpperCamelFragment;
/// assert!(UpperCamelFragment::new("AcceptsUppercase_Camel").is_ok());
/// ```
///
/// [`Ascii`]: crate::syntax::profile::Ascii
/// [`Boundary`]: crate::syntax::boundary::Boundary
/// [`Delimiter`]: crate::syntax::delimiter::Delimiter
/// [`Standard`]: crate::syntax::boundary::Standard
/// [`HyphenMinus`]: crate::syntax::delimiter::HyphenMinus
/// [`LowLine`]: crate::syntax::delimiter::LowLine
/// [`Profile`]: crate::syntax::profile::Profile
/// [`SubsetOf`]: crate::syntax::SubsetOf
/// [`Unicode`]: crate::syntax::profile::Unicode
/// [`cast`]: Self::cast
/// [`presets`]: crate::presets
/// [`Ident`]: crate::core::Ident
#[repr(transparent)]
pub struct Fragment<B, D, P> {
    config: PhantomData<(B, D, P)>,
    inner: str,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<B: Boundary, D: Delimiter, P: Profile> Fragment<B, D, P> {
    /// Converts a string slice to a fragment.
    ///
    /// A fragment is made of a string slice ([`&str`]), this function converts
    /// between the two. Not all string slices are valid fragments, however. A
    /// fragment requires that the characters it is comprised of satisfy certain
    /// [requirements].
    ///
    /// `new` checks to ensure these are satisfied before the conversion.
    ///
    /// [requirements]: Self#character-requirements
    ///
    /// # Errors
    ///
    /// Returns `Err` if the string slice does not satisfy the character
    /// requirements. If an invalid character is found then an [`Error`] is
    /// returned, with [`byte_offset`] set to the byte index for the first
    /// invalid character.
    ///
    /// [`Error`]: crate::Error
    /// [`byte_offset`]: crate::Error::byte_offset
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("AnUpperCamel_Fragment")?;
    /// assert_eq!(fragment, "AnUpperCamel_Fragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[inline]
    pub fn new(s: &str) -> Result<&Self, Error> {
        // A specific validation strategy is selected based on the configuration.
        //
        // Most reasonable identifiers are append-closed (for delim and profile),
        // and that is actually a really helpful strategy for `Fragment`, because
        // it means we can greatly simplify the validation logic.
        //
        // It's worth it - validated with `cargo asm`:
        //
        // 1. Mark this function as `#[inline(never)]`
        // 2. `clear && cargo asm -p typed-ident --all-features --example util new`
        // 3. Don't forget to restore this function back to `#[inline]`
        //
        // I've hidden a command that uses preset profiles with `APPEND_CLOSED`
        // set to `Empty` (~open). For Strict, this is no different, as it is
        // already ~open. But for ASCII and Unicode, comparing to the `*Open`
        // profile variants can be enlightening.
        //
        // Looking at a basic configuration (unit delimiter, no casing rules):
        // * ASCII -> 193 lines (closed), 339 lines (open)
        // * Unicode -> 273 lines (closed), 465 lines (open)
        //
        // # Note
        //
        // The Rust compiler is pretty good at optimizing. This logic doesn't
        // actually use boundary information, so it will dedupe functions that
        // have identical `D` and `P` params, ignoring the `B` parameter.
        match D::APPEND_CLOSED.at_least_fragment() && P::APPEND_CLOSED.at_least_fragment() {
            true => Self::new_append_closed(s),
            false => Self::new_append_open(s),
        }
    }

    #[inline(always)]
    fn new_append_open(s: &str) -> Result<&Self, Error> {
        let mut chars = s.char_indices();
        if let Some((_, c)) = chars.next() {
            // First character must be any kind of delim or in-profile char.
            let mut last_is_delim = match D::is_delim(c) {
                true => true,
                false if P::in_profile(c) => false,
                false => return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(0)),
            };

            // Next character must be either:
            //
            // 1. Any kind of chunk delimiter (non-start delim), *or...*
            // 2. An in-profile char, depending on if the prior was a delim.
            //    a. If prior was a delim, next char must be `is_chunk_start`.
            //    a. Otherwise, next char must be `is_chunk_continue`.
            for (idx, c) in chars {
                last_is_delim = match D::is_chunk_delim(c) {
                    true => true,
                    false => {
                        // Maybe it's append-open because the delimiter is
                        // append-open. If that's the case we can at least
                        // optimize how we approach parsing a chunk.
                        //
                        // This is super unlikely, but it's really easy to
                        // account for anyways, so might as well do it.
                        let valid = match P::APPEND_CLOSED.at_least_fragment() {
                            true => P::is_chunk_continue(c),
                            false => match last_is_delim {
                                true => P::is_chunk_start(c),
                                false => P::is_chunk_continue(c),
                            },
                        };
                        if !valid {
                            return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(idx));
                        }
                        false
                    }
                };
            }
        }
        Ok(Self::new_unchecked(s))
    }

    #[inline(always)]
    fn new_append_closed(s: &str) -> Result<&Self, Error> {
        // Greatly simplified if both `D` and `P` are fragment append-closed.
        //
        // Recall what `APPEND_CLOSED` means for `D` and `P`:
        // * For delimiters, it means `is_chunk_delim` is a superset of
        //   `is_ident_start`.
        //   * This means that `is_delim` = `is_chunk_delim`, and so for a
        //     fragment you only need to use `is_delim` everywhere.
        // * For profiles, it means `is_chunk_continue` is a superset of
        //   `is_ident_start` and `is_chunk_start`, *and* that `is_chunk_start`
        //   is identical to `is_chunk_continue`.
        //   * This means that `in_profile` = `is_chunk_continue`, since it must
        //     be the superset, and so for a fragment you only need to use
        //     `in_profile` everywhere.
        //
        // Basically this translates to:
        //
        //   As long as every character is either `D::is_delim` or
        //   `P::in_profile`, then it's a valid fragment!
        for (idx, c) in s.char_indices() {
            if !D::is_delim(c) && !P::in_profile(c) {
                return Err(Error::new(ErrorKind::InvalidFormat).with_byte_offset(idx));
            }
        }
        Ok(Self::new_unchecked(s))
    }

    /// Produces an iterator over the [`Segment`]s of a fragment, joining chunks
    /// together into one chunk instead of separating based on boundary logic.
    ///
    /// * The `Delimiter` variant is of type `D`.
    /// * The `Chunk` variant is of type [`Chunk<'_, B, D, P>`].
    ///
    /// Usually, when breaking into segments, you want to also break chunk
    /// boundaries. However, this iterator *will not* do that. It simply breaks
    /// into broad segments and chunks.
    ///
    /// If you want chunk boundaries to be broken, you should instead use the
    /// [`segments`] function.
    ///
    /// [`Segment`]: crate::core::Segment
    /// [`Chunk<'_, B, D, P>`]: crate::core::Chunk
    /// [`segments`]: Self::segments
    ///
    /// # Type Erasure
    ///
    /// Because segments contain type information specific to the fragment, it
    /// can be a little hard to use them in generic situations (where maybe you
    /// don't care about the type information, and just want to see general data
    /// about the segments).
    ///
    /// In these cases, you should call [`type_erased`] to drop type information,
    /// mapping to a `Segment</*Delimiter=*/char, /*Chunk=*/&str>` (you can call
    /// this on the returned iterator, or on an individual segment).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::Segment;
    /// # use typed_ident::presets::unicode::CamelIdent;
    /// let flat_ident = CamelIdent::new("HelloWorld_GoodbyeWorld")?;
    /// let mut segments = flat_ident.chunked_segments().type_erased();
    /// assert_eq!(segments.next(), Some(Segment::Chunk("HelloWorld")));
    /// assert_eq!(segments.next(), Some(Segment::Delim('_')));
    /// assert_eq!(segments.next(), Some(Segment::Chunk("GoodbyeWorld")));
    /// assert_eq!(segments.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This also works in reverse:
    ///
    /// ```
    /// # use typed_ident::Segment;
    /// # use typed_ident::presets::unicode::CamelIdent;
    /// let flat_ident = CamelIdent::new("HelloWorld_GoodbyeWorld")?;
    /// let mut segments = flat_ident.chunked_segments().type_erased();
    /// assert_eq!(segments.next_back(), Some(Segment::Chunk("GoodbyeWorld")));
    /// assert_eq!(segments.next_back(), Some(Segment::Delim('_')));
    /// assert_eq!(segments.next_back(), Some(Segment::Chunk("HelloWorld")));
    /// assert_eq!(segments.next_back(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// [`type_erased`]: crate::core::fragment::ChunkedSegments::type_erased
    #[must_use]
    #[inline]
    pub fn chunked_segments(&self) -> ChunkedSegments<'_, B, D, P> {
        ChunkedSegments::new(self)
    }

    /// Produces an iterator over the [`Segment`]s of a fragment, and their
    /// positions, joining chunks together into one chunk instead of separating
    /// based on boundary logic.
    ///
    /// * The `Delimiter` variant is of type `D`.
    /// * The `Chunk` variant is of type [`Chunk<'_, B, D, P>`].
    ///
    /// Usually, when breaking into segments, you want to also break chunk
    /// boundaries. However, this iterator *will not* do that. It simply breaks
    /// into broad segments and chunks.
    ///
    /// If you want chunk boundaries to be broken, you should instead use the
    /// [`segment_indices`] function.
    ///
    /// [`Segment`]: crate::core::Segment
    /// [`Chunk<'_, B, D, P>`]: crate::core::Chunk
    /// [`segment_indices`]: Self::segments
    ///
    /// # Type Erasure
    ///
    /// Because segments contain type information specific to the fragment, it
    /// can be a little hard to use them in generic situations (where maybe you
    /// don't care about the type information, and just want to see general data
    /// about the segments).
    ///
    /// In these cases, you should call [`type_erased`] to drop type information,
    /// mapping to a `Segment</*Delimiter=*/char, /*Chunk=*/&str>` (you can call
    /// this on the returned iterator, or on an individual segment).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::Segment;
    /// # use typed_ident::presets::unicode::CamelIdent;
    /// let flat_ident = CamelIdent::new("HelloWorld_GoodbyeWorld")?;
    /// let mut segments = flat_ident.chunked_segment_indices().type_erased();
    /// assert_eq!(segments.next(), Some((0, Segment::Chunk("HelloWorld"))));
    /// assert_eq!(segments.next(), Some((10, Segment::Delim('_'))));
    /// assert_eq!(segments.next(), Some((11, Segment::Chunk("GoodbyeWorld"))));
    /// assert_eq!(segments.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// This also works in reverse:
    ///
    /// ```
    /// # use typed_ident::Segment;
    /// # use typed_ident::presets::unicode::CamelIdent;
    /// let flat_ident = CamelIdent::new("HelloWorld_GoodbyeWorld")?;
    /// let mut segments = flat_ident.chunked_segment_indices().type_erased();
    /// assert_eq!(segments.next_back(), Some((11, Segment::Chunk("GoodbyeWorld"))));
    /// assert_eq!(segments.next_back(), Some((10, Segment::Delim('_'))));
    /// assert_eq!(segments.next_back(), Some((0, Segment::Chunk("HelloWorld"))));
    /// assert_eq!(segments.next_back(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// [`type_erased`]: crate::core::fragment::ChunkedSegmentIndices::type_erased
    #[must_use]
    #[inline]
    pub fn chunked_segment_indices(&self) -> ChunkedSegmentIndices<'_, B, D, P> {
        ChunkedSegmentIndices::new(self)
    }

    /// Produces an iterator over the [`Segment`]s of a fragment.
    ///
    /// * The `Delimiter` variant is of type `D`.
    /// * The `Chunk` variant is of type [`Chunk<'_, B, D, P>`].
    ///
    /// This is similar to [`chunked_segments`], except that it will also break
    /// chunks based on the configured [`Boundary`] type parameter.
    ///
    /// [`Segment`]: crate::core::Segment
    /// [`Chunk<'_, B, D, P>`]: crate::core::Chunk
    /// [`chunked_segments`]: Self::chunked_segments
    /// [`Boundary`]: crate::syntax::boundary::Boundary
    ///
    /// # Type Erased
    ///
    /// Because segments contain type information specific to the fragment, it
    /// can be a little hard to use them in generic situations (where maybe you
    /// don't care about the type information, and just want to see general data
    /// about the segments).
    ///
    /// In these cases, you should call [`type_erased`] to drop type information,
    /// mapping to a `Segment</*Delimiter=*/char, /*Chunk=*/&str>` (you can call
    /// this on the returned iterator, or on an individual segment).
    ///
    /// [`type_erased`]: crate::core::fragment::Segments::type_erased
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::Segment;
    /// # use typed_ident::presets::unicode::CamelIdent;
    /// let flat_ident = CamelIdent::new("HelloWorld_GoodbyeWorld")?;
    /// let mut segments = flat_ident.segments().type_erased();
    /// assert_eq!(segments.next(), Some(Segment::Chunk("Hello")));
    /// assert_eq!(segments.next(), Some(Segment::Chunk("World")));
    /// assert_eq!(segments.next(), Some(Segment::Delim('_')));
    /// assert_eq!(segments.next(), Some(Segment::Chunk("Goodbye")));
    /// assert_eq!(segments.next(), Some(Segment::Chunk("World")));
    /// assert_eq!(segments.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// These work in reverse as well:
    ///
    /// ```
    /// # use typed_ident::Segment;
    /// # use typed_ident::presets::unicode::CamelIdent;
    /// let flat_ident = CamelIdent::new("HelloWorld_GoodbyeWorld")?;
    /// let mut segments = flat_ident.segments().type_erased();
    /// assert_eq!(segments.next_back(), Some(Segment::Chunk("World")));
    /// assert_eq!(segments.next_back(), Some(Segment::Chunk("Goodbye")));
    /// assert_eq!(segments.next_back(), Some(Segment::Delim('_')));
    /// assert_eq!(segments.next_back(), Some(Segment::Chunk("World")));
    /// assert_eq!(segments.next_back(), Some(Segment::Chunk("Hello")));
    /// assert_eq!(segments.next_back(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn segments(&self) -> Segments<'_, B, D, P> {
        Segments::new(self)
    }

    /// Produces an iterator over the [`Segment`]s of a fragment, and their
    /// positions.
    ///
    /// This is similar to [`chunked_segment_indices`], except that it will also
    /// break chunks based on the configured [`Boundary`] type parameter.
    ///
    /// [`Segment`]: crate::core::Segment
    /// [`chunked_segment_indices`]: Self::chunked_segment_indices
    /// [`Boundary`]: crate::syntax::boundary::Boundary
    ///
    /// # Type Erased
    ///
    /// Because segments contain type information specific to the fragment, it
    /// can be a little hard to use them in generic situations (where maybe you
    /// don't care about the type information, and just want to see general data
    /// about the segments).
    ///
    /// In these cases, you should call [`type_erased`] to drop type information,
    /// mapping to a `Segment</*Delimiter=*/char, /*Chunk=*/&str>` (you can call
    /// this on the returned iterator, or on an individual segment).
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// # use typed_ident::Segment;
    /// # use typed_ident::presets::unicode::CamelIdent;
    /// let flat_ident = CamelIdent::new("HelloWorld_GoodbyeWorld")?;
    /// let mut segments = flat_ident.segment_indices().type_erased();
    /// assert_eq!(segments.next(), Some((0, Segment::Chunk("Hello"))));
    /// assert_eq!(segments.next(), Some((5, Segment::Chunk("World"))));
    /// assert_eq!(segments.next(), Some((10, Segment::Delim('_'))));
    /// assert_eq!(segments.next(), Some((11, Segment::Chunk("Goodbye"))));
    /// assert_eq!(segments.next(), Some((18, Segment::Chunk("World"))));
    /// assert_eq!(segments.next(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// These work in reverse as well:
    ///
    /// ```
    /// # use typed_ident::Segment;
    /// # use typed_ident::presets::unicode::CamelIdent;
    /// let flat_ident = CamelIdent::new("HelloWorld_GoodbyeWorld")?;
    /// let mut segments = flat_ident.segment_indices().type_erased();
    /// assert_eq!(segments.next_back(), Some((18, Segment::Chunk("World"))));
    /// assert_eq!(segments.next_back(), Some((11, Segment::Chunk("Goodbye"))));
    /// assert_eq!(segments.next_back(), Some((10, Segment::Delim('_'))));
    /// assert_eq!(segments.next_back(), Some((5, Segment::Chunk("World"))));
    /// assert_eq!(segments.next_back(), Some((0, Segment::Chunk("Hello"))));
    /// assert_eq!(segments.next_back(), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// [`type_erased`]: crate::core::fragment::Segments::type_erased
    /// [`chunked_segment_indices`]: Self::chunked_segment_indices
    /// [`Boundary`]: crate::syntax::boundary::Boundary
    #[must_use]
    #[inline]
    pub fn segment_indices(&self) -> SegmentIndices<'_, B, D, P> {
        SegmentIndices::new(self)
    }
}

// -----------------------------------------------------------------------------
impl<B, D: Delimiter, P> Fragment<B, D, P> {
    /// Returns `true` if the fragment has a leading delimiter, `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("__LeadingDelim")?;
    /// assert!(fragment.has_leading_delim());
    ///
    /// let fragment = UpperCamelFragment::new("NoLeadingDelim")?;
    /// assert!(!fragment.has_leading_delim());
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn has_leading_delim(&self) -> bool {
        self.chars().next().is_some_and(D::is_delim)
    }

    /// Returns `true` if the fragment has a trailing delimiter, `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("TrailingDelim__")?;
    /// assert!(fragment.has_trailing_delim());
    ///
    /// let fragment = UpperCamelFragment::new("NoTrailingDelim")?;
    /// assert!(!fragment.has_trailing_delim());
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn has_trailing_delim(&self) -> bool {
        self.chars().next_back().is_some_and(D::is_delim)
    }

    /// Returns `true` if the fragment is comprised solely of delimiters,
    /// `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("__NotAnonymous__")?;
    /// assert!(!fragment.is_anonymous());
    ///
    /// let fragment = UpperCamelFragment::new("___")?;
    /// assert!(fragment.is_anonymous());
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn is_anonymous(&self) -> bool {
        let mut chars = self.chars();
        chars.next().is_some_and(D::is_delim) && chars.all(D::is_chunk_delim)
    }

    /// Trims the leading and trailing delimiters from a fragment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("__SurroundingDelim__")?;
    /// assert_eq!(fragment.trim_delims().as_str(), "SurroundingDelim");
    ///
    /// // Note that this can leave you with an empty fragment.
    /// let fragment = UpperCamelFragment::new("____")?;
    /// assert_eq!(fragment.trim_delims().as_str(), "");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    pub fn trim_delims(&self) -> &Self {
        self.trim_leading_delims().trim_trailing_delims()
    }

    /// Trims the leading delimiters from a fragment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("__SurroundingDelim__")?;
    /// assert_eq!(fragment.trim_leading_delims().as_str(), "SurroundingDelim__");
    ///
    /// // Note that this can leave you with an empty fragment.
    /// let fragment = UpperCamelFragment::new("____")?;
    /// assert_eq!(fragment.trim_leading_delims().as_str(), "");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    pub fn trim_leading_delims(&self) -> &Self {
        Self::new_unchecked(self.as_str().trim_start_matches(D::is_delim))
    }

    /// Trims the trailing delimiters from a fragment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("__SurroundingDelim__")?;
    /// assert_eq!(fragment.trim_trailing_delims().as_str(), "__SurroundingDelim");
    ///
    /// // Note that this can leave you with an empty fragment.
    /// let fragment = UpperCamelFragment::new("____")?;
    /// assert_eq!(fragment.trim_trailing_delims().as_str(), "");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use = "this returns the remaining sub-fragment as a new fragment, without modifying the original"]
    #[inline]
    pub fn trim_trailing_delims(&self) -> &Self {
        Self::new_unchecked(self.as_str().trim_end_matches(D::is_delim))
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Fragment<B, D, P> {
    /// Returns a string slice representation of the fragment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("ExampleFragment")?;
    /// assert_eq!(fragment.as_str(), "ExampleFragment");
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.inner
    }

    /// Returns `true` if the given pattern matches a sub-fragment of this
    /// fragment.
    ///
    /// Returns `false` if it does not.
    ///
    /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("bananas")?;
    ///
    /// assert!(fragment.contains("nana"));
    /// assert!(!fragment.contains("apples"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn contains<M>(&self, pat: M) -> bool
    where
        M: crate::core::pattern::Pattern,
    {
        pat.contains(self.as_str())
    }

    /// Returns `true` if the given pattern matches a suffix of this fragment.
    ///
    /// Returns `false` if it does not.
    ///
    /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("bananas")?;
    ///
    /// assert!(fragment.ends_with("anas"));
    /// assert!(!fragment.ends_with("nana"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn ends_with<M>(&self, pat: M) -> bool
    where
        M: crate::core::pattern::Pattern,
    {
        pat.ends_with(self.as_str())
    }

    /// Returns `true` if the given pattern matches a prefix of this fragment.
    ///
    /// Returns `false` if it does not.
    ///
    /// The pattern can be a `&str`, in which case this function will return
    /// true if the `&str` is a prefix of this string slice.
    ///
    /// The pattern can also be a [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    /// These will only be checked against the first character of this fragment.
    /// Look at the second example below regarding behavior for slices of
    /// [`char`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("bananas")?;
    ///
    /// assert!(fragment.starts_with("bana"));
    /// assert!(!fragment.starts_with("nana"));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("bananas")?;
    ///
    /// // Note that both of these assert successfully.
    /// assert!(fragment.starts_with(&['b', 'a', 'n', 'a']));
    /// assert!(fragment.starts_with(&['a', 'b', 'c', 'd']));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn starts_with<M>(&self, pat: M) -> bool
    where
        M: crate::core::pattern::Pattern,
    {
        pat.starts_with(self.as_str())
    }

    /// Returns the byte index of the first character of this fragment that
    /// matches the pattern.
    ///
    /// Returns [`None`] if the pattern doesn't match.
    ///
    /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// # Examples
    ///
    /// Simple patterns:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("こんにちはWorld")?;
    ///
    /// assert_eq!(fragment.find('こ'), Some(0));
    /// assert_eq!(fragment.find('W'), Some(15));
    /// assert_eq!(fragment.find("orld"), Some(16));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// More complex patterns using point-free style and closures:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("こんにちはWorld")?;
    ///
    /// assert_eq!(fragment.find(char::is_alphabetic), Some(0));
    /// assert_eq!(fragment.find(char::is_lowercase), Some(16));
    /// assert_eq!(fragment.find(|c: char| c == 'W' || c == 'w'), Some(15));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Not finding the pattern:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("こんにちはWorld")?;
    /// let x: &[_] = &['1', '2'];
    ///
    /// assert_eq!(fragment.find(x), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn find<M>(&self, pat: M) -> Option<usize>
    where
        M: crate::core::pattern::Pattern,
    {
        pat.find(self.as_str())
    }

    /// Returns the byte index of the first character of this fragment that
    /// matches the pattern.
    ///
    /// Returns [`None`] if the pattern doesn't match.
    ///
    /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// # Examples
    ///
    /// Simple patterns:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("HelloWorld")?;
    ///
    /// assert_eq!(fragment.rfind('o'), Some(6));
    /// assert_eq!(fragment.rfind('H'), Some(0));
    /// assert_eq!(fragment.rfind("lo"), Some(3));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// More complex patterns using point-free style and closures:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("HelloWorld")?;
    ///
    /// assert_eq!(fragment.rfind(char::is_uppercase), Some(5));
    /// assert_eq!(fragment.rfind(char::is_lowercase), Some(9));
    /// assert_eq!(fragment.rfind(|c: char| c == 'o' || c == 'e'), Some(6));
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    ///
    /// Not finding the pattern:
    ///
    /// ```
    /// # use typed_ident::presets::unicode::upper_camel::*;
    /// let fragment = UpperCamelFragment::new("HelloWorld")?;
    /// let x: &[_] = &['1', '2'];
    ///
    /// assert_eq!(fragment.rfind(x), None);
    /// # Ok::<(), typed_ident::Error>(())
    /// ```
    #[must_use]
    #[inline]
    pub fn rfind<M>(&self, pat: M) -> Option<usize>
    where
        M: crate::core::pattern::Pattern,
    {
        pat.rfind(self.as_str())
    }
}

// -----------------------------------------------------------------------------
impl<'a, B: Boundary, D: Delimiter, P: Profile> core::convert::TryFrom<&'a str>
    for &'a Fragment<B, D, P>
{
    type Error = Error;

    #[inline(always)]
    fn try_from(orig: &'a str) -> Result<Self, Self::Error> {
        Fragment::new(orig)
    }
}

// -----------------------------------------------------------------------------
impl<B, D, P> Default for &Fragment<B, D, P> {
    #[inline(always)]
    fn default() -> Self {
        Fragment::new_unchecked("")
    }
}

// -----------------------------------------------------------------------------
impl_typed_slice_common! {
    name=Fragment,
    name_lowercase=fragment,
}
