// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::helpers::{
    Boundaries, Format, OptionsExt, SpecificOptions, TestIdent, TestResult, TypeLabel,
};
use typed_ident::presets::generic::*;
use typed_ident::syntax::boundary::{Options, options};
use typed_ident::syntax::profile::{Ascii, CharProfile, Strict, Unicode};
use typed_ident::{Identifier, Segment};

// =============================================================================
// MACRO
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! test_assertion {
    ($expr:expr) => {{
        if matches!($expr, TestResult::Fail) {
            return TestResult::Fail;
        }
    }};
}

// =============================================================================
// HELPERS
// =============================================================================

// -----------------------------------------------------------------------------
fn test_ident_pass<I: Identifier + TypeLabel + ?Sized>(
    test: &TestIdent,
    boundaries: &Boundaries,
) -> TestResult {
    let type_label = I::type_label();
    let ident = match I::new(test.name) {
        Ok(ident) => {
            log_for!(
                test,
                "✅ {test}: {type_label}: succeeded to parse as expected",
            );
            ident
        }
        Err(err) => {
            eprintln!("⛔ {test}: {type_label}: unexpectedly unable to parse");
            eprintln!("  with error: {err}");
            return TestResult::Fail;
        }
    };

    // Test segmentation
    {
        let expected_segments = test.segments(boundaries);
        let forward_segments: Vec<_> = ident.segments().type_erased().collect();
        let reverse_segments: Vec<_> = ident.segments().type_erased().rev().collect();
        let mut checked_segments = reverse_segments.clone();
        checked_segments.reverse();
        if forward_segments != checked_segments {
            eprintln!("⛔ {test}: {type_label}: forward and reverse segmentation don't match");
            eprintln!("  forward: {forward_segments:?}");
            eprintln!("  reverse: {reverse_segments:?}");
            return TestResult::Fail;
        }
        if forward_segments != expected_segments {
            eprintln!("⛔ {test}: {type_label}: unexpected segmentation values");
            eprintln!("  expected: {expected_segments:?}");
            eprintln!("    actual: {forward_segments:?}");
            return TestResult::Fail;
        }
        log_for!(
            test,
            "✅ {test}: {type_label}: checked segmentation: {forward_segments:?}"
        );
    }

    // Test segmentation indices
    {
        let expected_segments = test.segment_indices(boundaries);
        let forward_segments: Vec<_> = ident
            .segment_indices()
            .type_erased()
            .map(|(i, s)| (i, s.into_owned()))
            .collect();
        let reverse_segments: Vec<_> = ident
            .segment_indices()
            .type_erased()
            .map(|(i, s)| (i, s.into_owned()))
            .rev()
            .collect();
        let mut checked_segments = reverse_segments.clone();
        checked_segments.reverse();
        if forward_segments != checked_segments {
            eprintln!(
                "⛔ {test}: {type_label}: forward and reverse segmentation indices don't match"
            );
            eprintln!("  forward: {forward_segments:?}");
            eprintln!("  reverse: {reverse_segments:?}");
            return TestResult::Fail;
        }
        if forward_segments != expected_segments {
            eprintln!("⛔ {test}: {type_label}: unexpected segmentation indices values");
            eprintln!("  expected: {expected_segments:?}");
            eprintln!("    actual: {forward_segments:?}");
            return TestResult::Fail;
        }
        log_for!(
            test,
            "✅ {test}: {type_label}: checked segmentation indices: {forward_segments:?}"
        );
    }

    // Test chunked segmentation
    {
        let expected_segments = test.chunked_segments();
        let forward_segments: Vec<_> = ident.chunked_segments().type_erased().collect();
        let reverse_segments: Vec<_> = ident.chunked_segments().type_erased().rev().collect();
        let mut checked_segments = reverse_segments.clone();
        checked_segments.reverse();
        if forward_segments != checked_segments {
            eprintln!(
                "⛔ {test}: {type_label}: forward and reverse chunk segmentation don't match"
            );
            eprintln!("  forward: {forward_segments:?}");
            eprintln!("  reverse: {reverse_segments:?}");
            return TestResult::Fail;
        }
        if forward_segments != expected_segments {
            eprintln!("⛔ {test}: {type_label}: unexpected chunk segmentation values");
            eprintln!("  expected: {expected_segments:?}");
            eprintln!("    actual: {forward_segments:?}");
            return TestResult::Fail;
        }
        log_for!(
            test,
            "✅ {test}: {type_label}: checked chunk segmentation: {forward_segments:?}"
        );
    }

    // Test chunked segmentation indices
    {
        let expected_segments = test.chunked_segment_indices();
        let forward_segments: Vec<_> = ident
            .chunked_segment_indices()
            .type_erased()
            .map(|(i, s)| (i, s.into_owned()))
            .collect();
        let reverse_segments: Vec<_> = ident
            .chunked_segment_indices()
            .type_erased()
            .map(|(i, s)| (i, s.into_owned()))
            .rev()
            .collect();
        let mut checked_segments = reverse_segments.clone();
        checked_segments.reverse();
        if forward_segments != checked_segments {
            eprintln!(
                "⛔ {test}: {type_label}: forward and reverse chunk segmentation indices don't match"
            );
            eprintln!("  forward: {forward_segments:?}");
            eprintln!("  reverse: {reverse_segments:?}");
            return TestResult::Fail;
        }
        if forward_segments != expected_segments {
            eprintln!("⛔ {test}: {type_label}: unexpected chunk segmentation indices values");
            eprintln!("  expected: {expected_segments:?}");
            eprintln!("    actual: {forward_segments:?}");
            return TestResult::Fail;
        }
        log_for!(
            test,
            "✅ {test}: {type_label}: checked chunk segmentation indices: {forward_segments:?}"
        );
    }

    // Test building an ident buffer from segments
    {
        let mut buffer = I::new_ident_buffer();
        buffer.reserve(test.name.len());
        for segment in test.segments(boundaries) {
            let result = match &segment {
                Segment::Chunk(chunk) => buffer.push_str(chunk),
                Segment::Delim(delim) => buffer.push(*delim),
            };
            if let Err(error) = result {
                eprintln!(
                    "⛔ {test}: {type_label}: unable to re-construct ident buffer from segments"
                );
                eprintln!("  buffer: {buffer:?}");
                eprintln!("  failed: {segment:?}");
                eprintln!("   error: {error}");
                return TestResult::Fail;
            }
        }
        if buffer != test.name {
            eprintln!(
                "⛔ {test}: {type_label}: re-constructed ident buffer doesn't match original"
            );
            eprintln!("  expected: {test}");
            eprintln!("    actual: {buffer:?}");
            return TestResult::Fail;
        }
    }

    // Test building a fragment buffer from segments
    {
        let mut buffer = I::new_fragment_buffer();
        buffer.reserve(test.name.len());
        for segment in test.segments(boundaries) {
            let result = match &segment {
                Segment::Chunk(chunk) => buffer.push_str(chunk),
                Segment::Delim(delim) => buffer.push(*delim),
            };
            if let Err(error) = result {
                eprintln!(
                    "⛔ {test}: {type_label}: unable to re-construct fragment buffer from segments"
                );
                eprintln!("  buffer: {buffer:?}");
                eprintln!("  failed: {segment:?}");
                eprintln!("   error: {error}");
                return TestResult::Fail;
            }
        }
        if buffer != test.name {
            eprintln!(
                "⛔ {test}: {type_label}: re-constructed fragment buffer doesn't match original"
            );
            eprintln!("  expected: {test:?}");
            eprintln!("    actual: {buffer:?}");
            return TestResult::Fail;
        }
    }

    TestResult::Pass
}

// -----------------------------------------------------------------------------
fn test_ident_fail<I: Identifier + TypeLabel + ?Sized>(test: &TestIdent) -> TestResult {
    let type_label = I::type_label();
    match I::new(test.name) {
        Ok(_ident) => {
            eprintln!("⛔ {test}: {type_label}: unexpectedly able to parse");
            TestResult::Fail
        }
        Err(error) => {
            log_for!(test, "✅ {test}: {type_label}: failed to parse as expected");
            log_for!(test, "  with error: {error}");
            TestResult::Pass
        }
    }
}

// -----------------------------------------------------------------------------
fn test_profile_pass_opt<P: CharProfile + TypeLabel, O: OptionsExt>(
    test: &TestIdent,
) -> TestResult {
    let boundaries = O::boundaries();

    // Camel Identifiers
    if test.is_camel() {
        test_assertion!(test_ident_pass::<CamelIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<CamelIdent<P, O>>(test));
    }
    if test.is_lower_camel() {
        test_assertion!(test_ident_pass::<LowerCamelIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<LowerCamelIdent<P, O>>(test));
    }
    if test.is_upper_camel() {
        test_assertion!(test_ident_pass::<UpperCamelIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<UpperCamelIdent<P, O>>(test));
    }

    // Hybrid Identifiers
    if test.is_hybrid() {
        test_assertion!(test_ident_pass::<HybridIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<HybridIdent<P, O>>(test));
    }
    if test.is_lower_hybrid() {
        test_assertion!(test_ident_pass::<LowerHybridIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<LowerHybridIdent<P, O>>(test));
    }
    if test.is_upper_hybrid() {
        test_assertion!(test_ident_pass::<UpperHybridIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<UpperHybridIdent<P, O>>(test));
    }

    // Kebab Identifiers
    if test.is_kebab() {
        test_assertion!(test_ident_pass::<KebabIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<KebabIdent<P, O>>(test));
    }
    if test.is_lower_kebab() {
        test_assertion!(test_ident_pass::<LowerKebabIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<LowerKebabIdent<P, O>>(test));
    }
    if test.is_upper_kebab() {
        test_assertion!(test_ident_pass::<UpperKebabIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<UpperKebabIdent<P, O>>(test));
    }

    // Snake Identifiers
    if test.is_snake() {
        test_assertion!(test_ident_pass::<SnakeIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<SnakeIdent<P, O>>(test));
    }
    if test.is_lower_snake() {
        test_assertion!(test_ident_pass::<LowerSnakeIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<LowerSnakeIdent<P, O>>(test));
    }
    if test.is_upper_snake() {
        test_assertion!(test_ident_pass::<UpperSnakeIdent<P, O>>(test, &boundaries));
    } else {
        test_assertion!(test_ident_fail::<UpperSnakeIdent<P, O>>(test));
    }

    TestResult::Pass
}

// -----------------------------------------------------------------------------
fn test_profile_fail_opt<P: CharProfile + TypeLabel, O: Options>(test: &TestIdent) -> TestResult {
    test_assertion!(test_ident_fail::<CamelIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<LowerCamelIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<UpperCamelIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<HybridIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<LowerHybridIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<UpperHybridIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<KebabIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<LowerKebabIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<UpperKebabIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<SnakeIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<LowerSnakeIdent<P, O>>(test));
    test_assertion!(test_ident_fail::<UpperSnakeIdent<P, O>>(test));
    TestResult::Pass
}

// -----------------------------------------------------------------------------
fn test_profile_pass<P: CharProfile + TypeLabel>(test: &TestIdent) -> TestResult {
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, false, false, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, false, false, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, false, false, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, false, false, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, true, false, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, true, false, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, true, false, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, true, false, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, false, true, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, false, true, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, false, true, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, false, true, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, true, true, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, true, true, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, true, true, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, true, true, false, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, false, false, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, false, false, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, false, false, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, false, false, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, true, false, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, true, false, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, true, false, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, true, false, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, false, true, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, false, true, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, false, true, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, false, true, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, true, true, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, true, true, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, true, true, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, true, true, true, false>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, false, false, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, false, false, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, false, false, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, false, false, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, true, false, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, true, false, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, true, false, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, true, false, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, false, true, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, false, true, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, false, true, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, false, true, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, true, true, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, true, true, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, true, true, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, true, true, false, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, false, false, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, false, false, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, false, false, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, false, false, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, true, false, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, true, false, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, true, false, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, true, false, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, false, true, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, false, true, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, false, true, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, false, true, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, false, true, true, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, false, true, true, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<false, true, true, true, true, true>,
    >(test));
    test_assertion!(test_profile_pass_opt::<
        P,
        SpecificOptions<true, true, true, true, true, true>,
    >(test));
    TestResult::Pass
}

// -----------------------------------------------------------------------------
fn test_profile_fail<P: CharProfile + TypeLabel>(test: &TestIdent) -> TestResult {
    // Which options we select really doesn't matter here - it should fail.
    test_profile_fail_opt::<P, options::Default>(test)
}

// =============================================================================
// PUBLIC FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) fn test_valid(test: &TestIdent) -> TestResult {
    log_for!(
        test,
        "-------------------------------------------------------------------"
    );
    log_for!(
        test,
        r#"Running tests for "{}" (ascii={}, strict={}, unicode={})"#,
        test.name,
        test.is_ascii(),
        test.is_strict(),
        test.is_unicode()
    );
    if test.is_ascii() {
        test_assertion!(test_profile_pass::<Ascii>(test));
    } else {
        test_assertion!(test_profile_fail::<Ascii>(test));
    }
    if test.is_strict() {
        test_assertion!(test_profile_pass::<Strict>(test));
    } else {
        test_assertion!(test_profile_fail::<Strict>(test));
    }
    if test.is_unicode() {
        test_assertion!(test_profile_pass::<Unicode>(test));
    } else {
        test_assertion!(test_profile_fail::<Unicode>(test));
    }
    TestResult::Pass
}

// -----------------------------------------------------------------------------
pub(super) fn test_invalid(test: &'static str) -> TestResult {
    let test = TestIdent {
        name: test,
        format: Format::Mixed,
        segments: &[],
    };
    test_assertion!(test_profile_fail::<Ascii>(&test));
    test_assertion!(test_profile_fail::<Strict>(&test));
    test_assertion!(test_profile_fail::<Unicode>(&test));
    TestResult::Pass
}
