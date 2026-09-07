macro_rules! impl_char_indices_iterator {
    (
        name=$name:ident,
        over=$over:ident,
        function=$function:ident,
    ) => {
        impl_chars_iterator! {
            name=$name,
            over=$over,
            function=$function,
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> $name<'a, B, D, P> {
            /// Returns the byte position of the next character, or the total
            /// number of bytes that have been returned via [`next()`](Self::next).
            ///
            /// This means that, when the iterator has not been fully consumed,
            /// the returned value will match the index that will be returned
            /// by the next call to [`next()`](Self::next).
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::core::*;
            /// # use typed_ident::syntax::*;
            #[doc = concat!("# type Unicode", stringify!($over), " = ", stringify!($over), "<boundary::Standard, delimiter::LowLine, profile::Unicode>;")]
            #[doc = concat!("let slice = Unicode", stringify!($over), r#"::new("a楽")?;"#)]
            /// let mut chars = slice.char_indices();
            ///
            /// // `next()` has not been called yet, so `offset()` returns the byte
            /// // index of the first character of the slice, which is always 0.
            /// assert_eq!(chars.offset(), 0);
            /// // As expected, the first call to `next()` also returns 0 as index.
            /// assert_eq!(chars.next(), Some((0, 'a')));
            ///
            /// // `next()` has been called once, so `offset()` returns the byte index
            /// // of the second character ...
            /// assert_eq!(chars.offset(), 1);
            /// // ... which matches the index returned by the next call to `next()`.
            /// assert_eq!(chars.next(), Some((1, '楽')));
            ///
            /// // Once the iterator has been consumed, `offset()` returns the
            /// // length in bytes returned via `next`.
            /// assert_eq!(chars.offset(), 4);
            /// assert_eq!(chars.next(), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub fn offset(&self) -> usize {
                self.iter.offset()
            }
        }
    };
}
