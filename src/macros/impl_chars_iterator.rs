macro_rules! impl_chars_iterator {
    (
        @common
        name=$name:ident,
        over=$over:ident,
        function=$function:ident,
    ) => {
        #[doc = concat!("A wrapper over the core [`", stringify!($name), "`](core::str::", stringify!($name),"),")]
        #[doc = concat!("except that it is over a [`", stringify!($over), "`], and so it can take that representation.")]
        #[doc = concat!("You can construct this by calling [`", stringify!($function), "`](", stringify!($over), "::", stringify!($function) ,").")]
        #[repr(transparent)]
        pub struct $name<'a, B, D, P> {
            config: core::marker::PhantomData<&'a crate::core::$over<B, D, P>>,
            iter: core::str::$name<'a>,
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> $name<'a, B, D, P> {
            /// Views the underlying data as a subslice of the original data.
            ///
            /// This has the same lifetime as the original slice, and so the
            /// iterator can continue to be used while this exists.
            #[inline(always)]
            pub fn as_fragment(&self) -> &'a crate::core::Fragment<B, D, P> {
                crate::core::Fragment::new_unchecked(self.iter.as_str())
            }

            /// Views the underlying data as a subslice of the original data.
            ///
            /// This has the same lifetime as the original slice, and so the
            /// iterator can continue to be used while this exists.
            #[inline(always)]
            pub fn as_str(&self) -> &'a str {
                self.iter.as_str()
            }

            #[inline(always)]
            fn new(slice: &'a crate::core::$over<B, D, P>) -> Self {
                Self {
                    config: core::marker::PhantomData,
                    iter: slice.as_str().$function(),
                }
            }

            /// Drops the syntax type information associated with this iterator.
            ///
            /// This will return the internal core iterator that this type is
            /// wrapping. It will not construct a new iterator.
            #[inline(always)]
            pub fn type_erased(self) -> core::str::$name<'a> {
                self.iter
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::fmt::Debug for $name<'_, B, D, P> {
            #[inline(always)]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.iter.fmt(f)
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> Clone for $name<'_, B, D, P> {
            #[inline(always)]
            fn clone(&self) -> Self {
                Self {
                    config: core::marker::PhantomData,
                    iter: self.iter.clone(),
                }
            }
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> core::iter::Iterator for $name<'a, B, D, P> {
            type Item = <core::str::$name<'a> as Iterator>::Item;

            #[inline(always)]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }

            // TODO: https://github.com/rust-lang/rust/issues/77404
            // #[inline(always)]
            // fn advance_by(
            //     &mut self,
            //     mut remainder: usize,
            // ) -> Result<(), core::num::NonZero<usize>> {
            //     self.iter.advance_by(remainder)
            // }

            #[inline(always)]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iter.size_hint()
            }

            #[inline(always)]
            fn last(self) -> Option<Self::Item> {
                self.iter.last()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::iter::DoubleEndedIterator for $name<'_, B, D, P> {
            #[inline(always)]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.iter.next_back()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::iter::FusedIterator for $name<'_, B, D, P> {}
    };
    (
        name=$name:ident,
        over=Chunk,
        function=$function:ident,
    ) => {
        // ---------------------------------------------------------------------
        impl<'a, B, D, P> $name<'a, B, D, P> {
            /// Views the underlying data as a subslice of the original data.
            ///
            /// This has the same lifetime as the original slice, and so the
            /// iterator can continue to be used while this exists.
            #[must_use]
            #[inline]
            pub fn as_chunk(&self) -> &'a crate::core::Chunk<B, D, P> {
                crate::core::Chunk::new_unchecked(self.iter.as_str())
            }
        }

        impl_chars_iterator! {
            @common
            name=$name,
            over=Chunk,
            function=$function,
        }
    };
    (
        name=$name:ident,
        over=Fragment,
        function=$function:ident,
    ) => {
        impl_chars_iterator! {
            @common
            name=$name,
            over=Fragment,
            function=$function,
        }
    };
}
