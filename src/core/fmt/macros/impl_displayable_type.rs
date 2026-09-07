/// Implements a new-type pattern for a provided formatter.
///
/// It's beneficial to dedupe the types for this, but I don't want people taking
/// a dependency on the fact that two separate identifier formats happen to
/// share the same formatting type.
///
/// So we wrap them in a new-type to avoid that.
macro_rules! impl_displayable_type {
    (
        name=$name:ident,
        over=$over:ident,
        upper=$upper:literal,
        docs=$docs:expr,
    ) => {
        #[doc = $docs]
        #[repr(transparent)]
        pub struct $name<'a>($over<'a, $upper>);

        // ---------------------------------------------------------------------
        impl<'a> core::fmt::Display for $name<'a> {
            #[inline(always)]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
