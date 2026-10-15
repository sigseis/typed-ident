macro_rules! impl_boxed_slice_traits {
    (
        name=$name:ident,
        index_target=$index_target:ident,
    ) => {
        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialEq<str> for Box<$name<B, D, P>> {
            #[inline]
            fn eq(&self, rhs: &str) -> bool {
                self.as_str() == rhs
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialEq<&str> for Box<$name<B, D, P>> {
            #[inline]
            fn eq(&self, rhs: &&str) -> bool {
                self.as_str() == *rhs
            }
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> core::cmp::PartialEq<Box<$name<B, D, P>>> for &str {
            #[inline]
            fn eq(&self, rhs: &Box<$name<B, D, P>>) -> bool {
                *self == rhs.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> core::cmp::PartialEq<Box<$name<B, D, P>>> for str {
            #[inline]
            fn eq(&self, rhs: &Box<$name<B, D, P>>) -> bool {
                self == rhs.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<str> for Box<$name<B, D, P>> {
            #[inline]
            fn partial_cmp(&self, rhs: &str) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(rhs)
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<&str> for Box<$name<B, D, P>> {
            #[inline]
            fn partial_cmp(&self, rhs: &&str) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(*rhs)
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<Box<$name<B, D, P>>> for str {
            #[inline]
            fn partial_cmp(&self, rhs: &Box<$name<B, D, P>>) -> Option<core::cmp::Ordering> {
                self.partial_cmp(rhs.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<Box<$name<B, D, P>>> for &str {
            #[inline]
            fn partial_cmp(&self, rhs: &Box<$name<B, D, P>>) -> Option<core::cmp::Ordering> {
                (*self).partial_cmp(rhs.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::convert::AsRef<str> for Box<$name<B, D, P>> {
            #[inline]
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::convert::AsRef<[u8]> for Box<$name<B, D, P>> {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> core::convert::From<&'a Box<$name<B, D, P>>> for &'a str {
            #[inline]
            fn from(orig: &'a Box<$name<B, D, P>>) -> &'a str {
                orig.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<I, B, D, P> core::ops::Index<I> for Box<$name<B, D, P>>
        where
            I: crate::slice_index::SliceIndex<crate::$index_target<B, D, P>>,
        {
            type Output = crate::$index_target<B, D, P>;

            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {
                index.index(self)
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialEq<std_alloc::string::String> for Box<$name<B, D, P>> {
            #[inline]
            fn eq(&self, rhs: &std_alloc::string::String) -> bool {
                self.as_str() == rhs.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<Box<$name<B, D, P>>> for std_alloc::string::String {
            #[inline]
            fn partial_cmp(&self, rhs: &Box<$name<B, D, P>>) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(rhs.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<std_alloc::string::String> for Box<$name<B, D, P>> {
            #[inline]
            fn partial_cmp(&self, rhs: &std_alloc::string::String) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(rhs.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialEq<Box<$name<B, D, P>>> for std_alloc::string::String {
            #[inline]
            fn eq(&self, rhs: &Box<$name<B, D, P>>) -> bool {
                self.as_str() == rhs.as_str()
            }
        }
    };
}
