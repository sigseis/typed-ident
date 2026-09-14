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
        pub fn camel(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::CamelIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "CamelIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::CamelIdent>(
                    "CamelIdent",
                    crate::generate::independent,
                    &['_'],
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn cased_camel(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::CasedCamelIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "CasedCamelIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::CasedCamelIdent>(
                    "CasedCamelIdent",
                    crate::generate::dependent_start,
                    &['_'],
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
                    "LowerCamelIdent",
                    crate::generate::independent,
                    &['_'],
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
                    "UpperCamelIdent",
                    crate::generate::independent,
                    &['_'],
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn hybrid(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::HybridIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "HybridIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::HybridIdent>(
                    "HybridIdent",
                    crate::generate::independent,
                    &['_', '-'],
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn cased_hybrid(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::CasedHybridIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "CasedHybridIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::CasedHybridIdent>(
                    "CasedHybridIdent",
                    crate::generate::dependent_start,
                    &['_', '-'],
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
                    "LowerHybridIdent",
                    crate::generate::independent,
                    &['_', '-'],
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
                    "UpperHybridIdent",
                    crate::generate::independent,
                    &['_', '-'],
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn kebab(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::KebabIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "KebabIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::KebabIdent>(
                    "KebabIdent",
                    crate::generate::independent,
                    &['-'],
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn cased_kebab(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::CasedKebabIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "CasedKebabIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::CasedKebabIdent>(
                    "CasedKebabIdent",
                    crate::generate::dependent,
                    &['-'],
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
                    "LowerKebabIdent",
                    crate::generate::independent,
                    &['-'],
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
                    "UpperKebabIdent",
                    crate::generate::independent,
                    &['-'],
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn snake(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::SnakeIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "SnakeIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::SnakeIdent>(
                    "SnakeIdent",
                    crate::generate::independent,
                    &['_'],
                ),
            );
        }

        // ---------------------------------------------------------------------
        pub fn cased_snake(c: &mut Criterion) {
            crate::benches::bench::<typed_ident::presets::$presets::CasedSnakeIdent>(
                c,
                stringify!($charset),
                stringify!($presets),
                "CasedSnakeIdent",
                &crate::generate::$charset::<typed_ident::presets::$presets::CasedSnakeIdent>(
                    "CasedSnakeIdent",
                    crate::generate::dependent,
                    &['_'],
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
                    "LowerSnakeIdent",
                    crate::generate::independent,
                    &['_'],
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
                    "UpperSnakeIdent",
                    crate::generate::independent,
                    &['_'],
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
