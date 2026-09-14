#![allow(clippy::module_inception)]
// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
pub mod benches;
pub mod configs;
pub mod generate;
pub mod validate;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use convert_case::ccase;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::distr::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};
use std::hint::black_box;
use std::time::Duration;
use typed_ident::alloc::convert::*;
use typed_ident::core::fmt::*;
use typed_ident::syntax::Profile;
use typed_ident::{Identifier, StrSegment};

// =============================================================================
// CONFIGURATION
// =============================================================================

// -----------------------------------------------------------------------------
// The minimum length of any identifier in the sample set (should be >= CHUNKS).
// -----------------------------------------------------------------------------
const MIN_IDENTIFIER_LENGTH: usize = 12;

// -----------------------------------------------------------------------------
// The maximum length of any identifier in the sample set (should be >= MIN).
// -----------------------------------------------------------------------------
const MAX_IDENTIFIER_LENGTH: usize = 128;

// -----------------------------------------------------------------------------
// The maximum number of chunks possible per identifier.
// -----------------------------------------------------------------------------
const MAX_IDENTIFIER_CHUNKS: usize = 6;

// -----------------------------------------------------------------------------
// The number of data points used for testing segmentation methods.
// -----------------------------------------------------------------------------
const POINTS_SEGMENTATION: usize = 2_500;

// -----------------------------------------------------------------------------
// The number of data points used for testing validation (`*::new`).
// -----------------------------------------------------------------------------
const POINTS_VALIDATION: usize = 100_000;

// -----------------------------------------------------------------------------
// The maximum number of data points needed for an individual bench.
// -----------------------------------------------------------------------------
const POINTS_MAX: usize = POINTS_VALIDATION;

// -----------------------------------------------------------------------------
fn configure_criterion() -> Criterion {
    // We're trying to pair large data sample sizes (longer sample runtimes)
    // with larger measurement & dataset sample sizes (number of times over all
    // data samples).
    //
    // We're trying to improve confidence in the results. I've found this to be
    // pretty reasonable, albeit a bit slow to run.
    Criterion::default()
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(15))
        .sample_size(200)
}

// =============================================================================
// BENCHMARK CONFIGURATION
// =============================================================================

// -----------------------------------------------------------------------------
criterion_group! {
    name = benches;
    config = configure_criterion();
    targets =
        configs::ascii::ascii::camel,
        configs::ascii::ascii::cased_camel,
        configs::ascii::ascii::lower_camel,
        configs::ascii::ascii::upper_camel,
        configs::ascii::ascii::hybrid,
        configs::ascii::ascii::cased_hybrid,
        configs::ascii::ascii::lower_hybrid,
        configs::ascii::ascii::upper_hybrid,
        configs::ascii::ascii::kebab,
        configs::ascii::ascii::cased_kebab,
        configs::ascii::ascii::lower_kebab,
        configs::ascii::ascii::upper_kebab,
        configs::ascii::ascii::snake,
        configs::ascii::ascii::cased_snake,
        configs::ascii::ascii::lower_snake,
        configs::ascii::ascii::upper_snake,

        configs::ascii::strict::camel,
        configs::ascii::strict::cased_camel,
        configs::ascii::strict::lower_camel,
        configs::ascii::strict::upper_camel,
        configs::ascii::strict::hybrid,
        configs::ascii::strict::cased_hybrid,
        configs::ascii::strict::lower_hybrid,
        configs::ascii::strict::upper_hybrid,
        configs::ascii::strict::kebab,
        configs::ascii::strict::cased_kebab,
        configs::ascii::strict::lower_kebab,
        configs::ascii::strict::upper_kebab,
        configs::ascii::strict::snake,
        configs::ascii::strict::cased_snake,
        configs::ascii::strict::lower_snake,
        configs::ascii::strict::upper_snake,

        configs::ascii::unicode::camel,
        configs::ascii::unicode::cased_camel,
        configs::ascii::unicode::lower_camel,
        configs::ascii::unicode::upper_camel,
        configs::ascii::unicode::hybrid,
        configs::ascii::unicode::cased_hybrid,
        configs::ascii::unicode::lower_hybrid,
        configs::ascii::unicode::upper_hybrid,
        configs::ascii::unicode::kebab,
        configs::ascii::unicode::cased_kebab,
        configs::ascii::unicode::lower_kebab,
        configs::ascii::unicode::upper_kebab,
        configs::ascii::unicode::snake,
        configs::ascii::unicode::cased_snake,
        configs::ascii::unicode::lower_snake,
        configs::ascii::unicode::upper_snake,

        configs::unicode::strict::camel,
        configs::unicode::strict::cased_camel,
        configs::unicode::strict::lower_camel,
        configs::unicode::strict::upper_camel,
        configs::unicode::strict::hybrid,
        configs::unicode::strict::cased_hybrid,
        configs::unicode::strict::lower_hybrid,
        configs::unicode::strict::upper_hybrid,
        configs::unicode::strict::kebab,
        configs::unicode::strict::cased_kebab,
        configs::unicode::strict::lower_kebab,
        configs::unicode::strict::upper_kebab,
        configs::unicode::strict::snake,
        configs::unicode::strict::cased_snake,
        configs::unicode::strict::lower_snake,
        configs::unicode::strict::upper_snake,

        configs::unicode::unicode::camel,
        configs::unicode::unicode::cased_camel,
        configs::unicode::unicode::lower_camel,
        configs::unicode::unicode::upper_camel,
        configs::unicode::unicode::hybrid,
        configs::unicode::unicode::cased_hybrid,
        configs::unicode::unicode::lower_hybrid,
        configs::unicode::unicode::upper_hybrid,
        configs::unicode::unicode::kebab,
        configs::unicode::unicode::cased_kebab,
        configs::unicode::unicode::lower_kebab,
        configs::unicode::unicode::upper_kebab,
        configs::unicode::unicode::snake,
        configs::unicode::unicode::cased_snake,
        configs::unicode::unicode::lower_snake,
        configs::unicode::unicode::upper_snake,
}
criterion_main!(benches);
