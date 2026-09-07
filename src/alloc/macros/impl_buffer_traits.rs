macro_rules! impl_buffer_traits {
    (
        name=$name:ident,
    ) => {
        // ---------------------------------------------------------------------
        impl<B, D, P> Default for $name<B, D, P> {
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> From<$name<B, D, P>> for String {
            #[inline]
            fn from(orig: $name<B, D, P>) -> Self {
                orig.into_string()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> From<$name<B, D, P>> for std_alloc::boxed::Box<Fragment<B, D, P>> {
            #[inline]
            fn from(orig: $name<B, D, P>) -> Self {
                orig.into_boxed_fragment()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> From<$name<B, D, P>> for std_alloc::boxed::Box<str> {
            #[inline]
            fn from(orig: $name<B, D, P>) -> Self {
                orig.into_boxed_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::ops::Deref for $name<B, D, P> {
            type Target = Fragment<B, D, P>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                Fragment::new_unchecked(self.inner.as_str())
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialEq<&str> for $name<B, D, P> {
            #[inline]
            fn eq(&self, rhs: &&str) -> bool {
                self.as_str() == *rhs
            }
        }

        // ---------------------------------------------------------------------
        impl<'a, B, D, P> core::cmp::PartialEq<$name<B, D, P>> for &str {
            #[inline]
            fn eq(&self, rhs: &$name<B, D, P>) -> bool {
                *self == rhs.as_str()
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<&str> for $name<B, D, P> {
            #[inline]
            fn partial_cmp(&self, rhs: &&str) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(*rhs)
            }
        }

        // ---------------------------------------------------------------------
        impl<B, D, P> core::cmp::PartialOrd<$name<B, D, P>> for &str {
            #[inline]
            fn partial_cmp(&self, rhs: &$name<B, D, P>) -> Option<core::cmp::Ordering> {
                (*self).partial_cmp(rhs.as_str())
            }
        }
    };
}
