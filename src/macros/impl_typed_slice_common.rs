macro_rules! impl_typed_slice_common {
    (
        name=$name:ident,
        name_lowercase=$name_lowercase:ident,
    ) => {
        // ---------------------------------------------------------------------
        impl<B, D, P> $name<B, D, P> {
            /// Zero-cost cast into the type-configured target.
            ///
            /// This function does not perform any checks that the format
            /// matches the expectations of the target type. The way it's able
            /// to be provided depends on implementation of the `SubsetOf`
            /// trait.
            ///
            /// # Casting Requirements
            ///
            /// This function will be able to be called, if:
            ///
            /// * `Source::D: SubsetOf<Target::D>`, *and...*
            /// * `Source::P: SubsetOf<Target::P>`
            ///
            /// If these invariants are not upheld, attempting to call this
            /// function will result in a compilation failure.
            ///
            /// # Pro-Tip
            ///
            /// If this type can perform a zero-cost cast, then it will also
            /// implement `AsRef` to the target type. Because of this, if you
            /// know the shape of target type that you want, but also want to
            /// accept the widest range of inputs, you can use an `AsRef` trait
            /// bounds.
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            /// # use typed_ident::presets::unicode::lower_snake::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!(r#"fn expect_hybrid<I: AsRef<Hybrid"#, stringify!($name), r#"> + ?Sized>(ident: &I) {}"#)]
            #[doc = concat!(r#"expect_hybrid(LowerSnake"#, stringify!($name), r#"::new("apple")?);"#)]
            #[doc = concat!(r#"expect_hybrid(UpperCamel"#, stringify!($name), r#"::new("Apple")?);"#)]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            ///
            /// # Examples
            ///
            /// Example traversing case profile boundary:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::lower_snake::*;
            /// # use typed_ident::presets::unicode::lower_camel::*;
            /// // Compilable Cast:
            #[doc = concat!("let original = LowerSnake", stringify!($name), r#"::new("apple")?;"#)]
            #[doc = concat!("let casted: &LowerCamel", stringify!($name), " = original.cast();")]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// ```compile_fail
            /// # use typed_ident::presets::unicode::lower_snake::*;
            /// # use typed_ident::presets::unicode::lower_camel::*;
            /// // Bad Cast (Fails Compilation):
            #[doc = concat!("let original = LowerCamel", stringify!($name), r#"::new("apple")?;"#)]
            #[doc = concat!("let casted: &LowerSnake", stringify!($name), " = original.cast();")]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// Example traversing character profile boundary:
            ///
            /// ```
            /// # use typed_ident::presets::ascii::lower_snake as ascii;
            /// # use typed_ident::presets::unicode::lower_snake as unicode;
            /// // Compilable Cast:
            #[doc = concat!("let original = ascii::LowerSnake", stringify!($name), r#"::new("apple")?;"#)]
            #[doc = concat!("let casted: &unicode::LowerSnake", stringify!($name), " = original.cast();")]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// ```compile_fail
            /// # use typed_ident::presets::ascii::lower_snake as ascii;
            /// # use typed_ident::presets::unicode::lower_snake as unicode;
            /// // Bad Cast (Fails Compilation):
            #[doc = concat!("let original = unicode::LowerSnake", stringify!($name), r#"::new("apple")?;"#)]
            #[doc = concat!("let casted: &ascii::LowerSnake", stringify!($name), " = original.cast();")]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// Example traversing delimiter boundary:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::lower_snake::*;
            /// # use typed_ident::presets::unicode::hybrid::*;
            /// // Compilable Cast:
            #[doc = concat!("let original = LowerSnake", stringify!($name), r#"::new("apple")?;"#)]
            #[doc = concat!("let casted: &Hybrid", stringify!($name), " = original.cast();")]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// ```compile_fail
            /// # use typed_ident::presets::unicode::lower_snake::*;
            /// # use typed_ident::presets::unicode::hybrid::*;
            /// // Bad Cast (Fails Compilation):
            #[doc = concat!("let original = Hybrid", stringify!($name), r#"::new("apple")?;"#)]
            #[doc = concat!("let casted: &LowerSnake", stringify!($name), " = original.cast();")]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline(always)]
            pub const fn cast<B2, D2, P2>(&self) -> &$name<B2, D2, P2>
            where
                D: crate::syntax::SubsetOf<D2>,
                P: crate::syntax::SubsetOf<P2>,
            {
                $name::new_unchecked(self.as_str())
            }

            /// Returns an iterator over the [`char`]s of the underlying string
            /// slice, and their positions.
            ///
            /// This is a special version of the standard-provided
            /// `CharIndices`. It has additional functions on it to allow you to
            /// cast the remainder of the string slice back to this type.
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let mut chars = slice.char_indices();
            /// assert_eq!(chars.next(), Some((0, 't')));
            /// assert_eq!(chars.next(), Some((1, 'e')));
            /// assert_eq!(chars.next(), Some((2, 's')));
            /// assert_eq!(chars.next(), Some((3, 't')));
            /// assert_eq!(chars.next(), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// If needed, you can cast the remainder back to this type:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let mut chars = slice.char_indices();
            /// assert_eq!(chars.next(), Some((0, 't')));
            /// assert_eq!(chars.next(), Some((1, 'e')));
            #[doc = concat!("let remainder: &Hybrid", stringify!($name), " = chars.as_", stringify!($name_lowercase), "();")]
            /// assert_eq!(remainder, "st");
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// If you don't need type information, you can drop it with the
            /// [`type_erased`] method:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let chars: std::str::CharIndices = slice.char_indices().type_erased();
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// [`type_erased`]: CharIndices::type_erased
            #[must_use]
            #[inline]
            pub fn char_indices(&self) -> CharIndices<'_, B, D, P> {
                CharIndices::new(self)
            }

            /// Returns an iterator over the [`char`]s of the underlying string
            /// slice.
            ///
            /// This is a special version of the standard-provided `Chars`. It
            /// has additional functions on it to allow you to cast the
            /// remainder of the string slice back to this type.
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let mut chars = slice.chars();
            /// assert_eq!(chars.next(), Some('t'));
            /// assert_eq!(chars.next(), Some('e'));
            /// assert_eq!(chars.next(), Some('s'));
            /// assert_eq!(chars.next(), Some('t'));
            /// assert_eq!(chars.next(), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// If needed, you can cast the remainder back to this type:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let mut chars = slice.chars();
            /// assert_eq!(chars.next(), Some('t'));
            /// assert_eq!(chars.next(), Some('e'));
            #[doc = concat!("let remainder: &Hybrid", stringify!($name), " = chars.as_", stringify!($name_lowercase), "();")]
            /// assert_eq!(remainder, "st");
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// If you don't need type information, you can drop it with the
            /// [`type_erased`] method:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let chars: std::str::Chars = slice.chars().type_erased();
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// [`type_erased`]: Chars::type_erased
            #[must_use]
            #[inline]
            pub fn chars(&self) -> Chars<'_, B, D, P> {
                Chars::new(self)
            }

            #[doc = concat!("Returns a subslice of a `", stringify!($name) ,"`")]
            ///
            /// This is the non-panicking alternative to using the index operator.
            /// Returns [`None`] whenever the equivalent indexing operation
            /// would panic.
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("こんにちは世界")?;"#)]
            ///
            /// // indices not on UTF-8 sequence boundaries
            /// assert!(slice.get(1..).is_none());
            /// assert!(slice.get(..20).is_none());
            ///
            /// // out of bounds
            /// assert!(slice.get(..42).is_none());
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub fn get<I: crate::core::SliceIndex<Self>>(&self, i: I) -> Option<&Self> {
                i.get(self)
            }

            #[doc = concat!("Returns an unchecked subslice of a `", stringify!($name) ,"`")]
            ///
            /// This is the unchecked alternative to using the index operator.
            ///
            /// # Safety
            ///
            /// Callers of this function are responsible that these preconditions
            /// are satisfied:
            ///
            /// * The starting index must not exceed the ending index;
            /// * Indexes must be within bounds of the original slice;
            /// * Indexes must lie on UTF-8 sequence boundaries.
            ///
            /// Failing that, the returned slice may reference invalid memory or
            #[doc = concat!("violate the invariants communicated by the `", stringify!($name) ,"` type.")]
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("こんにちは世界")?;"#)]
            /// unsafe {
            #[doc = concat!("    assert_eq!(slice.get_unchecked(0..15), Hybrid", stringify!($name), r#"::new("こんにちは")?);"#)]
            #[doc = concat!("    assert_eq!(slice.get_unchecked(15..21), Hybrid", stringify!($name), r#"::new("世界")?);"#)]
            /// }
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub unsafe fn get_unchecked<I: crate::core::SliceIndex<Self>>(&self, i: I) -> &Self {
                // SAFETY: the caller must uphold the safety contract for `get_unchecked`.
                // the slice is dereferenceable because `self` is a safe reference.
                // The returned pointer is safe because impls of `SliceIndex` operates over string slices.
                unsafe { &*i.get_unchecked(self) }
            }

            /// Returns `true` if `self` has a length of zero bytes.
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("")?;"#)]
            /// assert!(slice.is_empty());
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("content")?;"#)]
            /// assert!(!slice.is_empty());
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.as_str().is_empty()
            }

            /// Returns the length of `self`.
            ///
            /// This length is in bytes, not [`char`]s or graphemes. In other
            /// words, it might not be what a human considers the length of the
            /// subslice.
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("foo")?;"#)]
            /// let len = slice.len();
            /// assert_eq!(len, 3);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("ƒoo")?;"#)]
            /// assert_eq!(slice.len(), 4); // fancy f!
            /// assert_eq!(slice.chars().count(), 3);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub const fn len(&self) -> usize {
                self.as_str().len()
            }

            /// Returns an iterator over the disjoint matches of a pattern within
            /// the underlying string slice as well as the index that the match
            /// starts at.
            ///
            /// This is a special version of the standard-provided
            /// `MatchIndices`. Instead of returning regular string slices, it
            #[doc = concat!("returns [`", stringify!($name), "`] elements.")]
            ///
            /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or
            /// a function or closure that determines if a character matches.
            ///
            /// # Iterator behavior
            ///
            /// The returned iterator will be a [`DoubleEndedIterator`] if the
            /// pattern allows a reverse search and forward/reverse search
            /// yields the same elements. This is true for, e.g., [`char`], but
            /// not for `&str`.
            ///
            /// If the pattern allows a reverse search but its results might
            /// differ from a forward search, the [`rmatch_indices`] method can
            /// be used.
            ///
            /// [`rmatch_indices`]: Self::rmatch_indices
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("abcXXXabcYYYabc")?;"#)]
            /// let mut matches = slice.match_indices("abc");
            #[doc = concat!("assert_eq!(matches.next(), Some((0, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some((6, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some((12, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            /// assert_eq!(matches.next(), None);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("1abcabc2")?;"#)]
            /// let mut matches = slice.match_indices("abc");
            #[doc = concat!("assert_eq!(matches.next(), Some((1, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some((4, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            /// assert_eq!(matches.next(), None);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("ababa")?;"#)]
            /// let mut matches = slice.match_indices("aba");
            #[doc = concat!("assert_eq!(matches.next(), Some((0, Hybrid", stringify!($name), r#"::new("aba")?)));"#)]
            /// assert_eq!(matches.next(), None); // only the first `aba`
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// If you don't need type information, you can drop it with the
            /// [`type_erased`] method. This can be especially useful if you
            /// don't need the typed versions of the results.
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let mut matches: std::str::MatchIndices<char> = slice.match_indices('t').type_erased();
            /// assert_eq!(matches.next(), Some((0, "t")));
            /// assert_eq!(matches.next(), Some((3, "t")));
            /// assert_eq!(matches.next(), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// [`type_erased`]: MatchIndices::type_erased
            #[must_use]
            #[inline]
            pub fn match_indices<M>(&self, pat: M) -> MatchIndices<'_, B, D, P, M>
            where
                M: crate::core::pattern::Pattern,
            {
                MatchIndices::new(self, pat)
            }

            /// Returns an iterator over the disjoint matches of a pattern within
            /// the underlying string slice.
            ///
            /// This is a special version of the standard-provided
            /// `Matches`. Instead of returning regular string slices, it
            #[doc = concat!("returns [`", stringify!($name), "`] elements.")]
            ///
            /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or
            /// a function or closure that determines if a character matches.
            ///
            /// # Iterator behavior
            ///
            /// The returned iterator will be a [`DoubleEndedIterator`] if the
            /// pattern allows a reverse search and forward/reverse search
            /// yields the same elements. This is true for, e.g., [`char`], but
            /// not for `&str`.
            ///
            /// If the pattern allows a reverse search but its results might
            /// differ from a forward search, the [`rmatches`] method can
            /// be used.
            ///
            /// [`rmatches`]: Self::rmatches
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("abcXXXabcYYYabc")?;"#)]
            /// let mut matches = slice.matches("abc");
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            /// assert_eq!(matches.next(), None);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("1abcabc2")?;"#)]
            /// let mut matches = slice.matches("abc");
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            /// assert_eq!(matches.next(), None);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("ababa")?;"#)]
            /// let mut matches = slice.matches("aba");
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("aba")?));"#)]
            /// assert_eq!(matches.next(), None); // only the first `aba`
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// If you don't need type information, you can drop it with the
            /// [`type_erased`] method. This can be especially useful if you
            /// don't need the typed versions of the results.
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let mut matches: std::str::Matches<char> = slice.matches('t').type_erased();
            /// assert_eq!(matches.next(), Some("t"));
            /// assert_eq!(matches.next(), Some("t"));
            /// assert_eq!(matches.next(), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// [`type_erased`]: Matches::type_erased
            #[must_use]
            #[inline]
            pub fn matches<M>(&self, pat: M) -> Matches<'_, B, D, P, M>
            where
                M: crate::core::pattern::Pattern,
            {
                Matches::new(self, pat)
            }

            #[doc = concat!("Converts a string slice to a ", stringify!($name_lowercase))]
            /// without checking that the contents are a valid chunk.
            ///
            /// # Memory Safety
            ///
            /// This function is *not* memory-unsafe - it simply bypasses checks
            /// to see if the string layout matches the syntax. Getting this
            /// wrong can lead to logical inconsistencies, but not memory ones.
            ///
            /// You should almost always prefer the [`new`] function.
            ///
            /// [`new`]: Self::new
            ///
            /// # When To Use
            ///
            /// Needless to say, this is very difficult to deduce on your own.
            ///
            /// The easiest way to uphold this invariant is when taking a slice
            /// of a string from something you already know is this type - *or*
            /// if you have checked the fragment in a proc-macro during
            /// compilation.
            ///
            /// Those are the intended use-cases for this function.
            #[must_use]
            #[inline(always)]
            pub(crate) const fn new_unchecked(s: &str) -> &Self {
                // SAFETY: Transparent over str. Because of this, the layout,
                // alignment, metadata, and validity are identical.
                unsafe { core::mem::transmute::<&str, &Self>(s) }
            }

            /// Returns an iterator over the disjoint matches of a pattern within
            /// the underlying string slice yielded in reverse order, as well as
            /// the index that the match starts at
            ///
            /// This is a special version of the standard-provided
            /// `RMatchIndices`. Instead of returning regular string slices, it
            #[doc = concat!("returns [`", stringify!($name), "`] elements.")]
            ///
            /// For matches of `pat` within `self` that overlap, only the indices
            /// corresponding to the last match are returned.
            ///
            /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
            /// function or closure that determines if a character matches.
            ///
            /// # Iterator behavior
            ///
            /// The returned iterator requires that the pattern supports a
            /// reverse search, and it will be a [`DoubleEndedIterator`] if a
            /// forward/reverse search yields the same elements.
            ///
            /// For iterating from the front, the [`match_indices`] method can
            /// be used.
            ///
            /// [`match_indices`]: Self::match_indices
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("abcXXXabcYYYabc")?;"#)]
            /// let mut matches = slice.rmatch_indices("abc");
            #[doc = concat!("assert_eq!(matches.next(), Some((12, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some((6, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some((0, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            /// assert_eq!(matches.next(), None);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("1abcabc2")?;"#)]
            /// let mut matches = slice.rmatch_indices("abc");
            #[doc = concat!("assert_eq!(matches.next(), Some((4, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some((1, Hybrid", stringify!($name), r#"::new("abc")?)));"#)]
            /// assert_eq!(matches.next(), None);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("ababa")?;"#)]
            /// let mut matches = slice.rmatch_indices("aba");
            #[doc = concat!("assert_eq!(matches.next(), Some((2, Hybrid", stringify!($name), r#"::new("aba")?)));"#)]
            /// assert_eq!(matches.next(), None); // only the first `aba`
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// If you don't need type information, you can drop it with the
            /// [`type_erased`] method. This can be especially useful if you
            /// don't need the typed versions of the results.
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let mut matches: std::str::RMatchIndices<char> = slice.rmatch_indices('t').type_erased();
            /// assert_eq!(matches.next(), Some((3, "t")));
            /// assert_eq!(matches.next(), Some((0, "t")));
            /// assert_eq!(matches.next(), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// [`type_erased`]: RMatchIndices::type_erased
            #[must_use]
            #[inline]
            pub fn rmatch_indices<M>(&self, pat: M) -> RMatchIndices<'_, B, D, P, M>
            where
                M: crate::core::pattern::Pattern,
            {
                RMatchIndices::new(self, pat)
            }

            /// Returns an iterator over the disjoint matches of a pattern within
            /// the underlying string slice yielded in reverse order.
            ///
            /// This is a special version of the standard-provided
            /// `RMatches`. Instead of returning regular string slices, it
            #[doc = concat!("returns [`", stringify!($name), "`] elements.")]
            ///
            /// For matches of `pat` within `self` that overlap, only the indices
            /// corresponding to the last match are returned.
            ///
            /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
            /// function or closure that determines if a character matches.
            ///
            /// # Iterator behavior
            ///
            /// The returned iterator requires that the pattern supports a
            /// reverse search, and it will be a [`DoubleEndedIterator`] if a
            /// forward/reverse search yields the same elements.
            ///
            /// For iterating from the front, the [`matches`] method can
            /// be used.
            ///
            /// [`matches`]: Self::matches
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("abcXXXabcYYYabc")?;"#)]
            /// let mut matches = slice.rmatches("abc");
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            /// assert_eq!(matches.next(), None);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("1abcabc2")?;"#)]
            /// let mut matches = slice.rmatches("abc");
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("abc")?));"#)]
            /// assert_eq!(matches.next(), None);
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("ababa")?;"#)]
            /// let mut matches = slice.rmatches("aba");
            #[doc = concat!("assert_eq!(matches.next(), Some(Hybrid", stringify!($name), r#"::new("aba")?));"#)]
            /// assert_eq!(matches.next(), None); // only the first `aba`
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// If you don't need type information, you can drop it with the
            /// [`type_erased`] method. This can be especially useful if you
            /// don't need the typed versions of the results.
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("test")?;"#)]
            /// let mut matches: std::str::RMatches<char> = slice.rmatches('t').type_erased();
            /// assert_eq!(matches.next(), Some("t"));
            /// assert_eq!(matches.next(), Some("t"));
            /// assert_eq!(matches.next(), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// [`type_erased`]: RMatches::type_erased
            #[must_use]
            #[inline]
            pub fn rmatches<M>(&self, pat: M) -> RMatches<'_, B, D, P, M>
            where
                M: crate::core::pattern::Pattern,
            {
                RMatches::new(self, pat)
            }

            #[doc = concat!("Divides one ", stringify!($name_lowercase), " into two at an index.")]
            ///
            /// The argument, `mid`, should be a byte offset from the start of the
            #[doc = concat!(stringify!($name_lowercase), ".")]
            /// It must also be on the boundary of a UTF-8 code point.
            ///
            /// The two slices returned go from the start of the
            #[doc = stringify!($name_lowercase)]
            /// to `mid`, and from `mid` to the end of the
            #[doc = concat!(stringify!($name_lowercase), ".")]
            ///
            /// # Panics
            ///
            /// Panics if `mid` is not on a UTF-8 code point boundary, or if it
            /// is past the end of the last code point of the
            #[doc = concat!(stringify!($name_lowercase), ".")]
            /// For a non-panicking alternative see [`split_at_checked`].
            ///
            /// [`split_at_checked`]: Self::split_at_checked
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("こんにちは世界")?;"#)]
            ///
            /// let (first, last) = slice.split_at(15);
            #[doc = concat!("assert_eq!(first, Hybrid", stringify!($name), r#"::new("こんにちは")?);"#)]
            #[doc = concat!("assert_eq!(last, Hybrid", stringify!($name), r#"::new("世界")?);"#)]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use = concat!("this returns the left and right sub-", stringify!($name_lowercase), "s as a new slices, without modifying the original")]
            #[inline]
            pub const fn split_at(&self, mid: usize) -> (&Self, &Self) {
                let (left, right) = self.as_str().split_at(mid);
                (Self::new_unchecked(left), Self::new_unchecked(right))
            }

            #[doc = concat!("Divides one ", stringify!($name_lowercase), " into two at an index.")]
            ///
            /// The argument, `mid`, should be a byte offset from the start of the
            #[doc = concat!(stringify!($name_lowercase), ".")]
            /// It must also be on the boundary of a UTF-8 code point. The method
            /// returns `None` if that's not the case.
            ///
            /// The two slices returned go from the start of the
            #[doc = stringify!($name_lowercase)]
            /// to `mid`, and from `mid` to the end of the
            #[doc = concat!(stringify!($name_lowercase), ".")]
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("こんにちは世界")?;"#)]
            ///
            /// let (first, last) = slice.split_at_checked(15).unwrap();
            #[doc = concat!("assert_eq!(first, Hybrid", stringify!($name), r#"::new("こんにちは")?);"#)]
            #[doc = concat!("assert_eq!(last, Hybrid", stringify!($name), r#"::new("世界")?);"#)]
            ///
            /// assert!(slice.split_at_checked(16).is_none()); // Inside "世"
            /// assert!(slice.split_at_checked(42).is_none()); // Beyond the length
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use = concat!("this returns the left and right sub-", stringify!($name_lowercase), "s as a new slices, without modifying the original")]
            #[inline]
            pub const fn split_at_checked(&self, mid: usize) -> Option<(&Self, &Self)> {
                match self.as_str().split_at_checked(mid) {
                    Some((l, r)) => {
                        Some((Self::new_unchecked(l), Self::new_unchecked(r)))
                    }
                    None => None,
                }
            }

            #[doc = concat!("Returns a ", stringify!($name_lowercase), " with the prefix and suffix removed.")]
            ///
            /// If the
            #[doc = stringify!($name_lowercase)]
            /// starts with the pattern `prefix` and ends with the pattern
            /// `suffix`, and the prefix and suffix don't overlap, returns the
            #[doc = concat!("sub-", stringify!($name_lowercase))]
            /// after the prefix and before the suffix, wrapped in `Some`.
            /// Unlike [`trim_start_matches`] and [`trim_end_matches`], this
            /// method removes both the prefix and suffix exactly once.
            ///
            /// If the
            #[doc = stringify!($name_lowercase)]
            /// does not start with `prefix`, does not end with `suffix`, or the
            /// prefix and suffix overlap, returns `None`.
            ///
            /// Each pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
            /// function or closure that determines if a character matches.
            ///
            /// [`trim_start_matches`]: Self::trim_start_matches
            /// [`trim_end_matches`]: Self::trim_end_matches
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("FooHelloWorldBar")?;"#)]
            #[doc = concat!(r#"assert_eq!(slice.strip_circumfix("Foo", "Bar"), Some(Hybrid"#, stringify!($name), r#"::new("HelloWorld")?));"#)]
            #[doc = concat!(r#"assert_eq!(slice.strip_circumfix("FooHello", "WorldBar"), Some(Hybrid"#, stringify!($name), r#"::new("")?));"#)]
            /// assert_eq!(slice.strip_circumfix("Foo", "Foo"), None);
            /// assert_eq!(slice.strip_circumfix("Bar", "Bar"), None);
            /// assert_eq!(slice.strip_circumfix("FooHello", "oWorldBar"), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use = concat!("this returns the remaining sub-", stringify!($name_lowercase), " as a new ", stringify!($name_lowercase), ", without modifying the original")]
            #[inline]
            pub fn strip_circumfix<Prefix, Suffix>(
                &self,
                prefix: Prefix,
                suffix: Suffix,
            ) -> Option<&Self>
            where
                Prefix: crate::core::pattern::Pattern,
                Suffix: crate::core::pattern::Pattern,
            {
                self.strip_prefix(prefix)?.strip_suffix(suffix)
            }

            #[doc = concat!("Returns a ", stringify!($name_lowercase), " with the prefix removed.")]
            ///
            /// If the
            #[doc = stringify!($name_lowercase)]
            /// starts with the pattern `prefix`, returns the
            #[doc = concat!("sub-", stringify!($name_lowercase))]
            /// after the prefix, wrapped in `Some`. Unlike
            /// [`trim_start_matches`], this method removes the prefix exactly
            /// once.
            ///
            /// If the
            #[doc = stringify!($name_lowercase)]
            /// does not start with `prefix`, returns `None`.
            ///
            /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
            /// function or closure that determines if a character matches.
            ///
            /// [`trim_start_matches`]: Self::trim_start_matches
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("HelloWorld")?;"#)]
            #[doc = concat!(r#"assert_eq!(slice.strip_prefix("Hello"), Some(Hybrid"#, stringify!($name), r#"::new("World")?));"#)]
            #[doc = concat!(r#"assert_eq!(slice.strip_prefix("HelloWorld"), Some(Hybrid"#, stringify!($name), r#"::new("")?));"#)]
            /// assert_eq!(slice.strip_prefix("Goodbye"), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use = concat!("this returns the remaining sub-", stringify!($name_lowercase), " as a new ", stringify!($name_lowercase), ", without modifying the original")]
            #[inline]
            pub fn strip_prefix<M>(&self, prefix: M) -> Option<&Self>
            where
                M: crate::core::pattern::Pattern,
            {
                prefix
                    .strip_prefix(self.as_str())
                    .map(Self::new_unchecked)
            }

            #[doc = concat!("Returns a ", stringify!($name_lowercase), " with the suffix removed.")]
            ///
            /// If the
            #[doc = stringify!($name_lowercase)]
            /// ends with the pattern `suffix`, returns the
            #[doc = concat!("sub-", stringify!($name_lowercase))]
            /// before the suffix, wrapped in `Some`. Unlike
            /// [`trim_end_matches`], this method removes the suffix exactly
            /// once.
            ///
            /// If the
            #[doc = stringify!($name_lowercase)]
            /// does not end with `suffix`, returns `None`.
            ///
            /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
            /// function or closure that determines if a character matches.
            ///
            /// [`trim_end_matches`]: Self::trim_end_matches
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("HelloWorld")?;"#)]
            #[doc = concat!(r#"assert_eq!(slice.strip_suffix("World"), Some(Hybrid"#, stringify!($name), r#"::new("Hello")?));"#)]
            #[doc = concat!(r#"assert_eq!(slice.strip_suffix("HelloWorld"), Some(Hybrid"#, stringify!($name), r#"::new("")?));"#)]
            /// assert_eq!(slice.strip_suffix("Computer"), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use = concat!("this returns the remaining sub-", stringify!($name_lowercase), " as a new ", stringify!($name_lowercase), ", without modifying the original")]
            #[inline]
            pub fn strip_suffix<M>(&self, suffix: M) -> Option<&Self>
            where
                M: crate::core::pattern::Pattern,
            {
                suffix
                    .strip_suffix(self.as_str())
                    .map(Self::new_unchecked)
            }

            #[doc = concat!("Returns a ", stringify!($name_lowercase), " with all prefixes that match a pattern repeatedly removed.")]
            ///
            /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
            /// function or closure that determines if a character matches.
            ///
            /// # Text Directionality
            ///
            /// A
            #[doc = stringify!($name_lowercase)]
            /// is a sequence of bytes. `start` in this context means the first
            /// position of that byte string; for a left-to-right language like
            /// English or Russian, this will be left side, and for right-to-left
            /// languages like Arabic or Hebrew, this will be the right side.
            ///
            /// # Examples
            ///
            /// Simple examples:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("11foo1bar11")?;"#)]
            #[doc = concat!("assert_eq!(slice.trim_start_matches('1'), Hybrid", stringify!($name), r#"::new("foo1bar11")?);"#)]
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("123foo1bar123")?;"#)]
            #[doc = concat!("assert_eq!(slice.trim_start_matches(char::is_numeric), Hybrid", stringify!($name), r#"::new("foo1bar123")?);"#)]
            ///
            /// let x: &[_] = &['1', '2'];
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("12foo1bar12")?;"#)]
            #[doc = concat!("assert_eq!(slice.trim_start_matches(x), Hybrid", stringify!($name), r#"::new("foo1bar12")?);"#)]
            ///
            /// // Example with a right-to-left language
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("שלוםעולם")?;"#)]
            #[doc = concat!(r#"assert_eq!(slice.trim_start_matches("שלום"), Hybrid"#, stringify!($name), r#"::new("עולם")?);"#)]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// A more complex pattern, using a closure:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("1fooX")?;"#)]
            #[doc = concat!("assert_eq!(slice.trim_start_matches(|c| c == '1' || c == 'X'), Hybrid", stringify!($name), r#"::new("fooX")?);"#)]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use = concat!("this returns the remaining sub-", stringify!($name_lowercase), " as a new ", stringify!($name_lowercase), ", without modifying the original")]
            #[inline]
            pub fn trim_start_matches<M>(&self, pat: M) -> &Self
            where
                M: crate::core::pattern::Pattern,
            {
                Self::new_unchecked(pat.trim_start_matches(self.as_str()))
            }

            #[doc = concat!("Returns a ", stringify!($name_lowercase), " with all suffixes that match a pattern repeatedly removed.")]
            ///
            /// The pattern can be a `&str`, [`char`], a slice of [`char`]s, or a
            /// function or closure that determines if a character matches.
            ///
            /// # Text Directionality
            ///
            /// A
            #[doc = stringify!($name_lowercase)]
            /// is a sequence of bytes. `end` in this context means the last
            /// position of that byte string; for a left-to-right language like
            /// English or Russian, this will be right side, and for right-to-left
            /// languages like Arabic or Hebrew, this will be the left side.
            ///
            /// # Examples
            ///
            /// Simple examples:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("11foo1bar11")?;"#)]
            #[doc = concat!("assert_eq!(slice.trim_end_matches('1'), Hybrid", stringify!($name), r#"::new("11foo1bar")?);"#)]
            ///
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("123foo1bar123")?;"#)]
            #[doc = concat!("assert_eq!(slice.trim_end_matches(char::is_numeric), Hybrid", stringify!($name), r#"::new("123foo1bar")?);"#)]
            ///
            /// let x: &[_] = &['1', '2'];
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("12foo1bar12")?;"#)]
            #[doc = concat!("assert_eq!(slice.trim_end_matches(x), Hybrid", stringify!($name), r#"::new("12foo1bar")?);"#)]
            ///
            /// // Example with a right-to-left language
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("שלוםעולם")?;"#)]
            #[doc = concat!(r#"assert_eq!(slice.trim_end_matches("עולם"), Hybrid"#, stringify!($name), r#"::new("שלום")?);"#)]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            ///
            /// A more complex pattern, using a closure:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let slice = Hybrid", stringify!($name), r#"::new("1fooX")?;"#)]
            #[doc = concat!("assert_eq!(slice.trim_end_matches(|c| c == '1' || c == 'X'), Hybrid", stringify!($name), r#"::new("1foo")?);"#)]
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use = concat!("this returns the remaining sub-", stringify!($name_lowercase), " as a new ", stringify!($name_lowercase), ", without modifying the original")]
            #[inline]
            pub fn trim_end_matches<M>(&self, pat: M) -> &Self
            where
                M: crate::core::pattern::Pattern,
            {
                Self::new_unchecked(pat.trim_end_matches(self.as_str()))
            }

            /// Attempts a fallible cast into the type-configured target.
            ///
            /// You should first attempt to call [`cast`] on a type, if that
            /// compiles it is preferred to this function (and you will not need
            /// to call this), because it is truly zero-cost.
            ///
            /// This is equivalent to just calling [`new`] on the target type
            /// with the current type's string contents. This function is
            /// provided for ergonomic convenience.
            ///
            /// [`cast`]: Self::cast
            /// [`new`]: Self::new
            #[inline]
            pub fn try_cast<B2, D2, P2>(&self) -> Result<&$name<B2, D2, P2>, Error>
            where
                B2: Boundary,
                D2: Delimiter,
                P2: Profile,
            {
                $name::new(self.as_str())
            }

            #[allow(unused)] // Not all implementors use this function, but some do.
            #[must_use]
            #[inline]
            pub(crate) fn type_erased(&self) -> &str {
                self.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl_typed_slice_traits! {
            name=$name,
            index_target=$name,
        }
    };
}
