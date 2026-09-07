macro_rules! impl_typed_slice_traits {
    (
        name=$name:ident,
        index_target=$index_target:ident,
    ) => {
        // ---------------------------------------------------------------------
        impl<'a, B1, B2, D1, D2, P1, P2> AsRef<crate::core::Fragment<B2, D2, P2>>
            for $name<B1, D1, P1>
        where
            D1: crate::syntax::Delimiter + crate::syntax::SubsetOf<D2>,
            P1: crate::syntax::Profile + crate::syntax::SubsetOf<P2>,
        {
            #[inline(always)]
            fn as_ref(&self) -> &crate::core::Fragment<B2, D2, P2> {
                self.cast()
            }
        }

        // ---------------------------------------------------------------------
        impl<B1, B2, D1, D2, P1, P2> core::cmp::PartialEq<$name<B1, D1, P1>> for $name<B2, D2, P2> {
            #[inline]
            fn eq(&self, rhs: &$name<B1, D1, P1>) -> bool {
                self.as_str().eq(rhs.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::Eq for $name<B, D, P> {}

        // ---------------------------------------------------------------------
        impl<B1, B2, D1, D2, P1, P2> core::cmp::PartialOrd<$name<B1, D1, P1>>
            for $name<B2, D2, P2>
        {
            #[inline]
            fn partial_cmp(&self, rhs: &$name<B1, D1, P1>) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(rhs.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::Ord for $name<B, D, P> {
            #[inline]
            fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
                self.as_str().cmp(rhs.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::hash::Hash for $name<B, D, P> {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: core::hash::Hasher,
            {
                self.as_str().hash(state)
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialEq<str> for $name<B, D, P> {
            #[inline]
            fn eq(&self, rhs: &str) -> bool {
                self.as_str() == rhs
            }
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> core::cmp::PartialEq<$name<B, D, P>> for str {
            #[inline]
            fn eq(&self, rhs: &$name<B, D, P>) -> bool {
                self == rhs.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<str> for $name<B, D, P> {
            #[inline]
            fn partial_cmp(&self, rhs: &str) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(rhs)
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<$name<B, D, P>> for str {
            #[inline]
            fn partial_cmp(&self, rhs: &$name<B, D, P>) -> Option<core::cmp::Ordering> {
                self.partial_cmp(rhs.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::convert::AsRef<str> for $name<B, D, P> {
            #[inline]
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::convert::AsRef<[u8]> for $name<B, D, P> {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> core::convert::From<&'a $name<B, D, P>> for &'a str {
            #[inline]
            fn from(orig: &'a $name<B, D, P>) -> &'a str {
                orig.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::fmt::Debug for $name<B, D, P> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&self.as_str())
                    .finish()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::fmt::Display for $name<B, D, P> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<I, B, D, P> core::ops::Index<I> for $name<B, D, P>
        where
            I: crate::core::SliceIndex<crate::core::$index_target<B, D, P>>,
        {
            type Output = crate::core::$index_target<B, D, P>;

            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {
                index.index(self)
            }
        }

        #[cfg(feature = "alloc")]
        mod alloc {
            use super::*;

            // -----------------------------------------------------------------
            impl<B, D, P> core::cmp::PartialEq<std_alloc::string::String> for $name<B, D, P> {
                #[inline]
                fn eq(&self, rhs: &std_alloc::string::String) -> bool {
                    self.as_str() == rhs.as_str()
                }
            }

            // -----------------------------------------------------------------
            impl<B, D, P> core::cmp::PartialOrd<$name<B, D, P>> for std_alloc::string::String {
                #[inline]
                fn partial_cmp(&self, rhs: &$name<B, D, P>) -> Option<core::cmp::Ordering> {
                    self.as_str().partial_cmp(rhs.as_str())
                }
            }

            // -----------------------------------------------------------------
            impl<B, D, P> core::cmp::PartialOrd<std_alloc::string::String> for $name<B, D, P> {
                #[inline]
                fn partial_cmp(
                    &self,
                    rhs: &std_alloc::string::String,
                ) -> Option<core::cmp::Ordering> {
                    self.as_str().partial_cmp(rhs.as_str())
                }
            }

            // -----------------------------------------------------------------
            impl<B, D, P> core::cmp::PartialEq<$name<B, D, P>> for std_alloc::string::String {
                #[inline]
                fn eq(&self, rhs: &$name<B, D, P>) -> bool {
                    self.as_str() == rhs.as_str()
                }
            }
        }
    };
}
