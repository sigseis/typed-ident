macro_rules! impl_typed_slice_cmp {
    (
        name=$name:ident,
        against=$against:ident,
    ) => {
        // -----------------------------------------------------------------------------
        impl<B1, B2, D1, D2, P1, P2> core::cmp::PartialEq<$against<B1, D1, P1>>
            for $name<B2, D2, P2>
        {
            #[inline]
            fn eq(&self, rhs: &$against<B1, D1, P1>) -> bool {
                self.as_str().eq(rhs.as_str())
            }
        }

        // -----------------------------------------------------------------------------
        impl<B1, B2, D1, D2, P1, P2> core::cmp::PartialEq<$name<B1, D1, P1>>
            for $against<B2, D2, P2>
        {
            #[inline]
            fn eq(&self, rhs: &$name<B1, D1, P1>) -> bool {
                self.as_str().eq(rhs.as_str())
            }
        }

        // -----------------------------------------------------------------------------
        impl<B1, B2, D1, D2, P1, P2> core::cmp::PartialOrd<$against<B1, D1, P1>>
            for $name<B2, D2, P2>
        {
            #[inline]
            fn partial_cmp(&self, rhs: &$against<B1, D1, P1>) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(rhs.as_str())
            }
        }

        // -----------------------------------------------------------------------------
        impl<B1, B2, D1, D2, P1, P2> core::cmp::PartialOrd<$name<B1, D1, P1>>
            for $against<B2, D2, P2>
        {
            #[inline]
            fn partial_cmp(&self, rhs: &$name<B1, D1, P1>) -> Option<core::cmp::Ordering> {
                self.as_str().partial_cmp(rhs.as_str())
            }
        }
    };
}
