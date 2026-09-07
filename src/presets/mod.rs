#![doc = include_str!("README.md")]

// =============================================================================
// MACRO
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! impl_type_aliases {
    (
        names={
            chunk=$chunk:ident,
            fragment=$fragment:ident,
            fragment_buf=$fragment_buf:ident,
            ident=$ident:ident,
            ident_buf=$ident_buf:ident,
            module=$module:ident,
            segment=$segment:ident,
        },
        options=$options:ident,
        case=$case:ident,
        delimiter=$delimiter:ident,
        doc=$doc:literal,
    ) => {
        #[allow(missing_docs)]
        pub mod $module {
            #[doc = concat!("The chunk type for [`", stringify!($ident) ,"`].")]
            pub type $chunk<P, O = crate::syntax::boundary::options::$options> = crate::core::Chunk<
                crate::syntax::boundary::Standard<O>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<P>,
            >;

            #[doc = concat!("The fragment type for [`", stringify!($ident) ,"`].")]
            pub type $fragment<P, O = crate::syntax::boundary::options::$options> = crate::core::Fragment<
                crate::syntax::boundary::Standard<O>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<P>,
            >;

            #[cfg(feature = "alloc")]
            #[doc = concat!("The fragment buffer type for [`", stringify!($ident) ,"`].")]
            pub type $fragment_buf<P, O = crate::syntax::boundary::options::$options> = crate::alloc::FragmentBuf<
                crate::syntax::boundary::Standard<O>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<P>,
            >;

            #[doc = concat!($doc, " identifier.")]
            pub type $ident<P, O = crate::syntax::boundary::options::$options> = crate::core::Ident<
                crate::syntax::boundary::Standard<O>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<P>,
            >;

            #[cfg(feature = "alloc")]
            #[doc = concat!($doc, " buffer.")]
            pub type $ident_buf<P, O = crate::syntax::boundary::options::$options> = crate::alloc::IdentBuf<
                crate::syntax::boundary::Standard<O>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<P>,
            >;

            #[doc = concat!("The segment type for [`", stringify!($ident) ,"`].")]
            pub type $segment<'a, P, O = crate::syntax::boundary::options::$options> = crate::core::Segment<
                crate::syntax::delimiter::$delimiter,
                &'a crate::core::Chunk<
                    crate::syntax::boundary::Standard<O>,
                    crate::syntax::delimiter::$delimiter,
                    crate::syntax::profile::case::$case<P>
                >,
            >;
        }
        #[doc(inline)]
        pub use $module::$ident;
        #[doc(inline)]
        #[cfg(feature = "alloc")]
        pub use $module::$ident_buf;
    };
    (
        profile=$profile:ident,
        names={
            chunk=$chunk:ident,
            fragment=$fragment:ident,
            fragment_buf=$fragment_buf:ident,
            ident=$ident:ident,
            ident_buf=$ident_buf:ident,
            module=$module:ident,
            segment=$segment:ident,
        },
        options=$options:ident,
        case=$case:ident,
        delimiter=$delimiter:ident,
        doc=$doc:literal,
    ) => {
        #[allow(missing_docs)]
        pub mod $module {
            #[doc = concat!("The word type for [`", stringify!($ident) ,"`].")]
            pub type $chunk = crate::core::Chunk<
                crate::syntax::boundary::Standard<crate::syntax::boundary::options::$options>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<crate::syntax::profile::$profile>,
            >;

            #[doc = concat!("The fragment type for [`", stringify!($ident) ,"`].")]
            pub type $fragment = crate::core::Fragment<
                crate::syntax::boundary::Standard<crate::syntax::boundary::options::$options>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<crate::syntax::profile::$profile>,
            >;

            #[cfg(feature = "alloc")]
            #[doc = concat!($doc, " buffer.")]
            pub type $fragment_buf = crate::alloc::FragmentBuf<
                crate::syntax::boundary::Standard<crate::syntax::boundary::options::$options>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<crate::syntax::profile::$profile>,
            >;

            #[doc = concat!($doc, " identifier.")]
            pub type $ident = crate::core::Ident<
                crate::syntax::boundary::Standard<crate::syntax::boundary::options::$options>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<crate::syntax::profile::$profile>,
            >;

            #[cfg(feature = "alloc")]
            #[doc = concat!($doc, " buffer.")]
            pub type $ident_buf = crate::alloc::IdentBuf<
                crate::syntax::boundary::Standard<crate::syntax::boundary::options::$options>,
                crate::syntax::delimiter::$delimiter,
                crate::syntax::profile::case::$case<crate::syntax::profile::$profile>
            >;

            #[doc = concat!("The segment type for [`", stringify!($ident) ,"`].")]
            pub type $segment<'a> = crate::core::Segment<
                crate::syntax::delimiter::$delimiter,
                &'a crate::core::Chunk<
                    crate::syntax::boundary::Standard<crate::syntax::boundary::options::$options>,
                    crate::syntax::delimiter::$delimiter,
                    crate::syntax::profile::case::$case<crate::syntax::profile::$profile>,
                >,
            >;
        }
        #[doc(inline)]
        pub use $module::$ident;
        #[doc(inline)]
        #[cfg(feature = "alloc")]
        pub use $module::$ident_buf;
    };
    (
        $($tt:tt)*
    ) => {
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=CamelChunk,
                fragment=CamelFragment,
                fragment_buf=CamelFragmentBuf,
                ident=CamelIdent,
                ident_buf=CamelIdentBuf,
                module=camel,
                segment=CamelSegment,
            },
            options=Default,
            case=Mixed,
            delimiter=LowLine,
            doc="An `UpperCamel` or `lowerCamel`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=UpperCamelChunk,
                fragment=UpperCamelFragment,
                fragment_buf=UpperCamelFragmentBuf,
                ident=UpperCamelIdent,
                ident_buf=UpperCamelIdentBuf,
                module=upper_camel,
                segment=UpperCamelSegment,
            },
            options=Default,
            case=UpperCamel,
            delimiter=LowLine,
            doc="An `UpperCamel`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=LowerCamelChunk,
                fragment=LowerCamelFragment,
                fragment_buf=LowerCamelFragmentBuf,
                ident=LowerCamelIdent,
                ident_buf=LowerCamelIdentBuf,
                module=lower_camel,
                segment=LowerCamelSegment,
            },
            options=Default,
            case=LowerCamel,
            delimiter=LowLine,
            doc="A `lowerCamel`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=SnakeChunk,
                fragment=SnakeFragment,
                fragment_buf=SnakeFragmentBuf,
                ident=SnakeIdent,
                ident_buf=SnakeIdentBuf,
                module=snake,
                segment=SnakeSegment,
            },
            options=Default,
            case=Mixed,
            delimiter=LowLine,
            doc="A mixed-cased `Snake_Delim`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=UpperSnakeChunk,
                fragment=UpperSnakeFragment,
                fragment_buf=UpperSnakeFragmentBuf,
                ident=UpperSnakeIdent,
                ident_buf=UpperSnakeIdentBuf,
                module=upper_snake,
                segment=UpperSnakeSegment,
            },
            options=NoBoundaries,
            case=Upper,
            delimiter=LowLine,
            doc="An `UPPER_SNAKE`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=LowerSnakeChunk,
                fragment=LowerSnakeFragment,
                fragment_buf=LowerSnakeFragmentBuf,
                ident=LowerSnakeIdent,
                ident_buf=LowerSnakeIdentBuf,
                module=lower_snake,
                segment=LowerSnakeSegment,
            },
            options=NoBoundaries,
            case=Lower,
            delimiter=LowLine,
            doc="A `lower_snake`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=KebabChunk,
                fragment=KebabFragment,
                fragment_buf=KebabFragmentBuf,
                ident=KebabIdent,
                ident_buf=KebabIdentBuf,
                module=kebab,
                segment=KebabSegment,
            },
            options=Default,
            case=Mixed,
            delimiter=HyphenMinus,
            doc="A mixed-case `Kebab-Delim`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=UpperKebabChunk,
                fragment=UpperKebabFragment,
                fragment_buf=UpperKebabFragmentBuf,
                ident=UpperKebabIdent,
                ident_buf=UpperKebabIdentBuf,
                module=upper_kebab,
                segment=UpperKebabSegment,
            },
            options=NoBoundaries,
            case=Upper,
            delimiter=HyphenMinus,
            doc="An `UPPER-KEBAB`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=LowerKebabChunk,
                fragment=LowerKebabFragment,
                fragment_buf=LowerKebabFragmentBuf,
                ident=LowerKebabIdent,
                ident_buf=LowerKebabIdentBuf,
                module=lower_kebab,
                segment=LowerKebabSegment,
            },
            options=NoBoundaries,
            case=Lower,
            delimiter=HyphenMinus,
            doc="A `lower-kebab`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=HybridChunk,
                fragment=HybridFragment,
                fragment_buf=HybridFragmentBuf,
                ident=HybridIdent,
                ident_buf=HybridIdentBuf,
                module=hybrid,
                segment=HybridSegment,
            },
            options=Default,
            case=Mixed,
            delimiter=AsciiFlatLine,
            doc="A mixed-case `mixed_Hybrid-ident`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=UpperHybridChunk,
                fragment=UpperHybridFragment,
                fragment_buf=UpperHybridFragmentBuf,
                ident=UpperHybridIdent,
                ident_buf=UpperHybridIdentBuf,
                module=upper_hybrid,
                segment=UpperHybridSegment,
            },
            options=Default,
            case=UpperCamel,
            delimiter=AsciiFlatLine,
            doc="An `Upper_Hybrid-Ident`",
        }
        impl_type_aliases! {
            $($tt)*
            names={
                chunk=LowerHybridChunk,
                fragment=LowerHybridFragment,
                fragment_buf=LowerHybridFragmentBuf,
                ident=LowerHybridIdent,
                ident_buf=LowerHybridIdentBuf,
                module=lower_hybrid,
                segment=LowerHybridSegment,
            },
            options=Default,
            case=LowerCamel,
            delimiter=AsciiFlatLine,
            doc="A `lower_hybrid-ident`",
        }
    };
}

// =============================================================================
// PRESETS
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "unicode")]
#[allow(missing_docs)]
pub mod generic {
    impl_type_aliases! {}
}

// =============================================================================
// PROFILE PRESETS
// =============================================================================

// -----------------------------------------------------------------------------
#[allow(missing_docs)]
pub mod ascii {
    impl_type_aliases! {
        profile=Ascii,
    }
}

// -----------------------------------------------------------------------------
#[cfg(feature = "unicode-strict")]
#[allow(missing_docs)]
pub mod strict {
    impl_type_aliases! {
        profile=Strict,
    }
}

// -----------------------------------------------------------------------------
#[cfg(feature = "unicode")]
#[allow(missing_docs)]
pub mod unicode {
    impl_type_aliases! {
        profile=Unicode,
    }
}
