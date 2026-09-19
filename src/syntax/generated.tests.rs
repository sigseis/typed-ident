// =============================================================================
// CONSTANTS
// =============================================================================

// -----------------------------------------------------------------------------
// It's possible that this test data could go stale, but the frequency in which
// new titlecase characters would be added should be quite rare.
// -----------------------------------------------------------------------------
const TITLECASE: &[char] = &[
    '\u{01C5}', '\u{01C8}', '\u{01CB}', '\u{01F2}', '\u{1F88}', '\u{1F89}', '\u{1F8A}', '\u{1F8B}',
    '\u{1F8C}', '\u{1F8D}', '\u{1F8E}', '\u{1F8F}', '\u{1F98}', '\u{1F99}', '\u{1F9A}', '\u{1F9B}',
    '\u{1F9C}', '\u{1F9D}', '\u{1F9E}', '\u{1F9F}', '\u{1FA8}', '\u{1FA9}', '\u{1FAA}', '\u{1FAB}',
    '\u{1FAC}', '\u{1FAD}', '\u{1FAE}', '\u{1FAF}', '\u{1FBC}', '\u{1FCC}', '\u{1FFC}',
];

// -----------------------------------------------------------------------------
// It's possible that this test data could go stale, but the frequency in which
// new Greek titlecase characters would be added should be quite rare.
// -----------------------------------------------------------------------------
const GREEK_TITLECASE: &[char] = &[
    '\u{1F88}', '\u{1F89}', '\u{1F8A}', '\u{1F8B}', '\u{1F8C}', '\u{1F8D}', '\u{1F8E}', '\u{1F8F}',
    '\u{1F98}', '\u{1F99}', '\u{1F9A}', '\u{1F9B}', '\u{1F9C}', '\u{1F9D}', '\u{1F9E}', '\u{1F9F}',
    '\u{1FA8}', '\u{1FA9}', '\u{1FAA}', '\u{1FAB}', '\u{1FAC}', '\u{1FAD}', '\u{1FAE}', '\u{1FAF}',
    '\u{1FBC}', '\u{1FCC}', '\u{1FFC}',
];

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
#[test]
fn is_titlecase() {
    for c in char::MIN..=char::MAX {
        assert_eq!(super::is_titlecase(c), TITLECASE.contains(&c));
    }
}

// -----------------------------------------------------------------------------
#[test]
fn is_titlecase_greek_variant() {
    for c in char::MIN..=char::MAX {
        if super::is_titlecase(c) {
            assert_eq!(
                super::is_titlecase_greek_variant(c),
                GREEK_TITLECASE.contains(&c)
            );
        }
    }
}

// -----------------------------------------------------------------------------
// If the above functions test well, then it's likely the simple, non-merged
// generated functions are valid. `is_combining_mark` is a merged set of tables,
// so it's more likely to be wrong - so we will check against non-merged tables.
// -----------------------------------------------------------------------------
#[test]
#[cfg(feature = "unicode")]
fn is_combining_mark() {
    for c in char::MIN..=char::MAX {
        assert_eq!(
            super::is_combining_mark(c),
            super::is_enclosing_mark(c)
                || super::is_nonspacing_mark(c)
                || super::is_spacing_mark(c)
        );
    }
}
