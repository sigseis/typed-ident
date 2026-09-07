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
use rand::SeedableRng;
use rand::distr::{Distribution, Uniform};
use rand::rngs::SmallRng;
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
        configs::ascii::ascii::mixed_camel,
        configs::ascii::ascii::lower_camel,
        configs::ascii::ascii::upper_camel,
        configs::ascii::ascii::mixed_hybrid,
        configs::ascii::ascii::lower_hybrid,
        configs::ascii::ascii::upper_hybrid,
        configs::ascii::ascii::mixed_kebab,
        configs::ascii::ascii::lower_kebab,
        configs::ascii::ascii::upper_kebab,
        configs::ascii::ascii::mixed_snake,
        configs::ascii::ascii::lower_snake,
        configs::ascii::ascii::upper_snake,

        configs::ascii::strict::mixed_camel,
        configs::ascii::strict::lower_camel,
        configs::ascii::strict::upper_camel,
        configs::ascii::strict::mixed_hybrid,
        configs::ascii::strict::lower_hybrid,
        configs::ascii::strict::upper_hybrid,
        configs::ascii::strict::mixed_kebab,
        configs::ascii::strict::lower_kebab,
        configs::ascii::strict::upper_kebab,
        configs::ascii::strict::mixed_snake,
        configs::ascii::strict::lower_snake,
        configs::ascii::strict::upper_snake,

        configs::ascii::unicode::mixed_camel,
        configs::ascii::unicode::lower_camel,
        configs::ascii::unicode::upper_camel,
        configs::ascii::unicode::mixed_hybrid,
        configs::ascii::unicode::lower_hybrid,
        configs::ascii::unicode::upper_hybrid,
        configs::ascii::unicode::mixed_kebab,
        configs::ascii::unicode::lower_kebab,
        configs::ascii::unicode::upper_kebab,
        configs::ascii::unicode::mixed_snake,
        configs::ascii::unicode::lower_snake,
        configs::ascii::unicode::upper_snake,

        configs::unicode::strict::mixed_camel,
        configs::unicode::strict::lower_camel,
        configs::unicode::strict::upper_camel,
        configs::unicode::strict::mixed_hybrid,
        configs::unicode::strict::lower_hybrid,
        configs::unicode::strict::upper_hybrid,
        configs::unicode::strict::mixed_kebab,
        configs::unicode::strict::lower_kebab,
        configs::unicode::strict::upper_kebab,
        configs::unicode::strict::mixed_snake,
        configs::unicode::strict::lower_snake,
        configs::unicode::strict::upper_snake,

        configs::unicode::unicode::mixed_camel,
        configs::unicode::unicode::lower_camel,
        configs::unicode::unicode::upper_camel,
        configs::unicode::unicode::mixed_hybrid,
        configs::unicode::unicode::lower_hybrid,
        configs::unicode::unicode::upper_hybrid,
        configs::unicode::unicode::mixed_kebab,
        configs::unicode::unicode::lower_kebab,
        configs::unicode::unicode::upper_kebab,
        configs::unicode::unicode::mixed_snake,
        configs::unicode::unicode::lower_snake,
        configs::unicode::unicode::upper_snake,
}
criterion_main!(benches);
