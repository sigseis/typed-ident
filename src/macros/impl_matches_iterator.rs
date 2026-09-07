macro_rules! impl_matches_iterator {
    (
        name=$name:ident,
        over=$over:ident,
        function=$function:ident,
        docs=$docs:expr,
    ) => {
        #[doc = $docs]
        pub struct $name<'a, B, D, P, M: crate::core::pattern::Pattern> {
            config: core::marker::PhantomData<&'a $over<B, D, P>>,
            iter: M::$name<'a>,
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P, M: crate::core::pattern::Pattern> $name<'a, B, D, P, M> {
            fn new(orig: &'a $over<B, D, P>, pat: M) -> Self {
                Self {
                    config: core::marker::PhantomData,
                    iter: pat.$function(orig.as_str()),
                }
            }

            /// Drops the syntax type information associated with this iterator.
            ///
            /// This will return the internal core iterator that this type is
            /// wrapping. It will not construct a new iterator.
            #[must_use]
            #[inline]
            pub fn type_erased(self) -> M::$name<'a> {
                self.iter
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P, M: crate::core::pattern::Pattern> core::fmt::Debug for $name<'_, B, D, P, M> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.iter).finish()
            }
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P, M: crate::core::pattern::Pattern> Clone for $name<'a, B, D, P, M>
        where
            M::$name<'a>: Clone,
        {
            #[inline(always)]
            fn clone(&self) -> Self {
                Self {
                    config: core::marker::PhantomData,
                    iter: self.iter.clone(),
                }
            }
        }

        // -----------------------------------------------------------------------------
        impl<'a, B, D, P, M: crate::core::pattern::Pattern> core::iter::Iterator
            for $name<'a, B, D, P, M>
        {
            type Item = &'a $over<B, D, P>;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next().map($over::new_unchecked)
            }
        }

        // -----------------------------------------------------------------------------
        impl<'a, B, D, P, M: crate::core::pattern::Pattern> core::iter::DoubleEndedIterator
            for $name<'a, B, D, P, M>
        where
            M::$name<'a>: core::iter::DoubleEndedIterator,
        {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.iter.next_back().map($over::new_unchecked)
            }
        }

        // -----------------------------------------------------------------------------
        impl<'a, B, D, P, M: crate::core::pattern::Pattern> core::iter::FusedIterator
            for $name<'a, B, D, P, M>
        {
        }
    };
}
