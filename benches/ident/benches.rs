// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
use criterion::BenchmarkGroup;
use criterion::measurement::WallTime;

// =============================================================================
// HELPERS
// =============================================================================

// -----------------------------------------------------------------------------
fn new_group<'a>(
    c: &'a mut Criterion,
    type_prefix: &str,
    function_name: &str,
    identifiers: &[String],
) -> BenchmarkGroup<'a, WallTime> {
    c.benchmark_group(format!(
        "{type_prefix}::{function_name}/Points<{}>",
        identifiers.len()
    ))
}

// =============================================================================
// BENCHES
// =============================================================================

// -----------------------------------------------------------------------------
// FUNCTION: `Ident::new`
// -----------------------------------------------------------------------------
// VARIANTS:
// * `baseline`
//   * A simple comparison with a basic `is_xid_ident` function.
//     (Note: this does not enforce a specific format like `new`)
// * `typed`
//   * The actual call to `Ident::new` from `typed-ident`.
// -----------------------------------------------------------------------------
fn bench_new(
    c: &mut Criterion,
    type_prefix: &str,
    function_name: &str,
    identifiers: &[String],
    baseline: impl Fn(&str) -> bool,
    typed: impl Fn(&str) -> bool,
) {
    let mut group = new_group(c, type_prefix, function_name, identifiers);
    group.bench_function("baseline", |b| {
        b.iter(|| {
            for input in identifiers {
                let _ = black_box(baseline(input));
            }
        });
    });
    group.bench_function("typed", |b| {
        b.iter(|| {
            for input in identifiers {
                let _ = black_box(typed(input));
            }
        });
    });
    group.finish();
}

// -----------------------------------------------------------------------------
// FUNCTION: `Ident::to_*` conversions
// -----------------------------------------------------------------------------
// VARIANTS:
// * `ccase`
//   * The default formatting operation as implemented by `convert_case`.
// * `heck`
//   * The default formatting operation as implemented by `heck`.
// * `canonical`
//   * The canonical-form formatting operation as implemented by `typed-ident`.
//     (Note: This is the most equivalent to `ccase` and `heck`.)
// * `decorated`
//   * The decorated-form formatting operation as implemented by `typed-ident`.
// -----------------------------------------------------------------------------
#[allow(clippy::too_many_arguments)]
fn bench_convert(
    c: &mut Criterion,
    type_prefix: &str,
    function_name: &str,
    identifiers: &[String],
    ccase_format: impl Fn(&str) -> String,
    heck_format: impl Fn(&str) -> String,
    canonical_format: impl Fn(&str) -> String,
    decorated_format: impl Fn(&str) -> String,
) {
    let mut group = new_group(c, type_prefix, function_name, identifiers);
    group.bench_function("ccase", |b| {
        b.iter(|| {
            for input in identifiers {
                let _ = black_box(ccase_format(input));
            }
        });
    });
    group.bench_function("heck", |b| {
        b.iter(|| {
            for input in identifiers {
                let _ = black_box(heck_format(input));
            }
        });
    });
    group.bench_function("canonical", |b| {
        b.iter(|| {
            for input in identifiers {
                let _ = black_box(canonical_format(input));
            }
        });
    });
    group.bench_function("decorated", |b| {
        b.iter(|| {
            for input in identifiers {
                let _ = black_box(decorated_format(input));
            }
        });
    });
    group.finish();
}

// -----------------------------------------------------------------------------
// FUNCTION: `Ident::segments`
// -----------------------------------------------------------------------------
// VARIANTS:
// * `ccase`
//   * The default by-word split operation as implemented by `convert_case`.
// * `segments`
//   * The default segments operation as implemented by `typed-ident`.
// * `words`
//   * An equivalent by-word split operation as implemented by `typed-ident`.
//     (Note: This is the most equivalent to `ccase`.)
// -----------------------------------------------------------------------------
fn bench_segmentation(
    c: &mut Criterion,
    type_prefix: &str,
    function_name: &str,
    identifiers: &[String],
    ccase_words: impl Fn(&str),
    typed_segments: impl Fn(&str),
    typed_words: impl Fn(&str),
) {
    let mut group = new_group(c, type_prefix, function_name, identifiers);
    group.bench_function("ccase", |b| {
        b.iter(|| {
            for input in identifiers {
                #[allow(clippy::unit_arg)]
                black_box(ccase_words(input));
            }
        });
    });
    group.bench_function("segments", |b| {
        b.iter(|| {
            for input in identifiers {
                #[allow(clippy::unit_arg)]
                black_box(typed_segments(input));
            }
        });
    });
    group.bench_function("words", |b| {
        b.iter(|| {
            for input in identifiers {
                #[allow(clippy::unit_arg)]
                black_box(typed_words(input));
            }
        });
    });
    group.finish();
}

// =============================================================================
// BENCHES MAIN
// =============================================================================

// -----------------------------------------------------------------------------
pub fn bench<I>(
    c: &mut Criterion,
    charset: &str,
    presets: &str,
    typename: &str,
    identifiers: &[String],
) where
    I: ConvertibleIdentifier + FormattableIdentifier + ?Sized,
{
    let type_prefix = format!("//Input<{charset}>/Preset<{presets}>/{typename}");
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // New
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    bench_new(
        c,
        &type_prefix,
        "new",
        &identifiers[..POINTS_VALIDATION],
        /*baseline=*/
        crate::validate::baseline,
        /*typed=*/
        |s| I::new(s).is_ok(),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Segmentation
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    bench_segmentation(
        c,
        &type_prefix,
        "segments",
        &identifiers[..POINTS_SEGMENTATION],
        /*ccase_words=*/
        |s| {
            // BUG: This function doesn't allow for `T: ?Sized`.
            //
            // I'd like to just return the vector, but I can't because the
            // lifetimes won't be happy, since the str slice needs an additional
            // reference taken out over it.
            //
            // Until this is fixed upstream, I will just black-box it here for
            // all the types.
            let _ = black_box(convert_case::split(
                &s,
                // Roughly equivalent to what we do by default in typed-ident.
                &[
                    convert_case::Boundary::Acronym,
                    convert_case::Boundary::Hyphen,
                    convert_case::Boundary::LowerUpper,
                    convert_case::Boundary::Underscore,
                ],
            ));
        },
        /*typed_segments=*/
        |s| {
            let _: Vec<_> = black_box(I::new(s).unwrap().segments().type_erased().collect());
        },
        /*typed_words=*/
        |s| {
            let _: Vec<_> = black_box(
                I::new(s)
                    .unwrap()
                    .segments()
                    .type_erased()
                    .filter_map(|s| match s {
                        StrSegment::Chunk(c) => Some(c),
                        _ => None,
                    })
                    .collect(),
            );
        },
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Camel-Like Conversions
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    bench_convert(
        c,
        &type_prefix,
        "to_lower_camel",
        &identifiers[..POINTS_SEGMENTATION],
        /*ccase_format=*/ |s| ccase!(camel, s),
        /*heck_format=*/
        |s| {
            use heck::ToLowerCamelCase;
            s.to_lower_camel_case()
        },
        /*canonical_format=*/
        |s| I::new(s).unwrap().to_lower_camel_canonical(),
        /*decorated_format=*/
        |s| I::new(s).unwrap().to_lower_camel_decorated(),
    );
    bench_convert(
        c,
        &type_prefix,
        "to_upper_camel",
        &identifiers[..POINTS_SEGMENTATION],
        /*ccase_format=*/ |s| ccase!(upper_camel, s),
        /*heck_format=*/
        |s| {
            use heck::ToUpperCamelCase;
            s.to_upper_camel_case()
        },
        /*canonical_format=*/
        |s| I::new(s).unwrap().to_upper_camel_canonical(),
        /*decorated_format=*/
        |s| I::new(s).unwrap().to_upper_camel_decorated(),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Kebab-Like Conversions
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    bench_convert(
        c,
        &type_prefix,
        "to_lower_kebab",
        &identifiers[..POINTS_SEGMENTATION],
        /*ccase_format=*/ |s| ccase!(kebab, s),
        /*heck_format=*/
        |s| {
            use heck::ToKebabCase;
            s.to_kebab_case()
        },
        /*canonical_format=*/
        |s| I::new(s).unwrap().to_lower_kebab_canonical(),
        /*decorated_format=*/
        |s| I::new(s).unwrap().to_lower_kebab_decorated(),
    );
    bench_convert(
        c,
        &type_prefix,
        "to_upper_kebab",
        &identifiers[..POINTS_SEGMENTATION],
        /*ccase_format=*/ |s| ccase!(upper_kebab, s),
        /*heck_format=*/
        |s| {
            use heck::ToShoutyKebabCase;
            s.to_shouty_kebab_case()
        },
        /*canonical_format=*/
        |s| I::new(s).unwrap().to_upper_kebab_canonical(),
        /*decorated_format=*/
        |s| I::new(s).unwrap().to_upper_kebab_decorated(),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Snake-Like Conversions
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    bench_convert(
        c,
        &type_prefix,
        "to_lower_snake",
        &identifiers[..POINTS_SEGMENTATION],
        /*ccase_format=*/ |s| ccase!(snake, s),
        /*heck_format=*/
        |s| {
            use heck::ToSnakeCase;
            s.to_snake_case()
        },
        /*canonical_format=*/
        |s| I::new(s).unwrap().to_lower_snake_canonical(),
        /*decorated_format=*/
        |s| I::new(s).unwrap().to_lower_snake_decorated(),
    );
    bench_convert(
        c,
        &type_prefix,
        "to_upper_snake",
        &identifiers[..POINTS_SEGMENTATION],
        /*ccase_format=*/ |s| ccase!(upper_snake, s),
        /*heck_format=*/
        |s| {
            use heck::ToShoutySnakeCase;
            s.to_shouty_snake_case()
        },
        /*canonical_format=*/
        |s| I::new(s).unwrap().to_upper_snake_canonical(),
        /*decorated_format=*/
        |s| I::new(s).unwrap().to_upper_snake_decorated(),
    );
}
