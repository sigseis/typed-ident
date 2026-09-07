// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! define_benchmarks_functions {
    (
        charset=$charset:ident,
        presets=$presets:ident,
    ) => {
        // ---------------------------------------------------------------------
        pub fn mixed_camel(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::CamelIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "CamelIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::CamelIdent>(
                    &['_'],
                    crate::validate::mixed_camel,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn lower_camel(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::LowerCamelIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "LowerCamelIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::LowerCamelIdent>(
                    &['_'],
                    crate::validate::lower_camel,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn upper_camel(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::UpperCamelIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "UpperCamelIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::UpperCamelIdent>(
                    &['_'],
                    crate::validate::upper_camel,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn mixed_hybrid(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::HybridIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "HybridIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::HybridIdent>(
                    &['_', '-'],
                    crate::validate::mixed_hybrid,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn lower_hybrid(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::LowerHybridIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "LowerHybridIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::LowerHybridIdent>(
                    &['_', '-'],
                    crate::validate::lower_hybrid,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn upper_hybrid(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::UpperHybridIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "UpperHybridIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::UpperHybridIdent>(
                    &['_', '-'],
                    crate::validate::upper_hybrid,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn mixed_kebab(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::KebabIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "KebabIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::KebabIdent>(
                    &['-'],
                    crate::validate::mixed_kebab,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn lower_kebab(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::LowerKebabIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "LowerKebabIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::LowerKebabIdent>(
                    &['-'],
                    crate::validate::lower_kebab,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn upper_kebab(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::UpperKebabIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "UpperKebabIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::UpperKebabIdent>(
                    &['-'],
                    crate::validate::upper_kebab,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn mixed_snake(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::SnakeIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "SnakeIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::SnakeIdent>(
                    &['_'],
                    crate::validate::mixed_snake,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn lower_snake(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::LowerSnakeIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "LowerSnakeIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::LowerSnakeIdent>(
                    &['_'],
                    crate::validate::lower_snake,
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn upper_snake(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::UpperSnakeIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "UpperSnakeIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::UpperSnakeIdent>(
                    &['_'],
                    crate::validate::upper_snake,
                ),
            );
        }
    };
}

// =============================================================================
// BENCH CONFIGURATIONS
// =============================================================================

// -----------------------------------------------------------------------------
pub mod ascii {
    use super::*;

    // -------------------------------------------------------------------------
    pub mod ascii {
        use super::*;

        define_benchmarks_functions! {
            charset=ascii,
            presets=ascii,
        }
    }

    // -------------------------------------------------------------------------
    pub mod strict {
        use super::*;

        define_benchmarks_functions! {
            charset=ascii,
            presets=strict,
        }
    }

    // -------------------------------------------------------------------------
    pub mod unicode {
        use super::*;

        define_benchmarks_functions! {
            charset=ascii,
            presets=unicode,
        }
    }
}

// -----------------------------------------------------------------------------
pub mod unicode {
    use super::*;

    // -------------------------------------------------------------------------
    pub mod strict {
        use super::*;

        define_benchmarks_functions! {
            charset=unicode,
            presets=strict,
        }
    }

    // -------------------------------------------------------------------------
    pub mod unicode {
        use super::*;

        define_benchmarks_functions! {
            charset=unicode,
            presets=unicode,
        }
    }
}
