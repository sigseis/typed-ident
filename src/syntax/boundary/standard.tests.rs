// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
use crate::syntax::boundary::options::*;

// =============================================================================
// COMMON CONFIGURATIONS
// =============================================================================

// -----------------------------------------------------------------------------
struct CamelOnly;
impl Options for CamelOnly {
    const CAMEL: bool = true;
}

// -----------------------------------------------------------------------------
struct HatOnly;
impl super::Options for HatOnly {
    const HAT: bool = true;
}

// -----------------------------------------------------------------------------
struct UpperDigitOnly;
impl super::Options for UpperDigitOnly {
    const DIGIT_TO_UPPER: bool = true;
    const UPPER_TO_DIGIT: bool = true;
}

// -----------------------------------------------------------------------------
struct LowerDigitOnly;
impl super::Options for LowerDigitOnly {
    const DIGIT_TO_LOWER: bool = true;
    const LOWER_TO_DIGIT: bool = true;
}

// =============================================================================
// CHAR SEGMENTATION TESTS
// =============================================================================

// -----------------------------------------------------------------------------
mod r#char {
    use super::*;
    use crate::syntax::segmentation::Char;

    // -------------------------------------------------------------------------
    mod none {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<NoBoundaries>;

        #[test]
        fn find_boundary() {
            let chunk = "abcDef";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "abcDef";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "abcDef";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 4));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));
        }
    }

    // -------------------------------------------------------------------------
    mod camel {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<CamelOnly>;

        #[test]
        fn find_boundary() {
            let chunk = "abcDef";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "ABCDef";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abǅDef"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(2)); // Lower with titlecase after
            let res = Standard::find_boundary::<Char>(&chunk[2..]);
            assert_eq!(res, NonZero::new(2)); // Upper with non-greek titlecase before

            let chunk = "abᾈDef"; // Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(2)); // Lower with titlecase after
            let res = Standard::find_boundary::<Char>(&chunk[2..]);
            assert_eq!(res, None); // Upper, but greek titlecase before
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "abcDef";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "ABCDef";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abǅDef"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(4)); // Upper with non-greek titlecase before
            let res = Standard::rfind_boundary::<Char>(&chunk[..4]);
            assert_eq!(res, NonZero::new(2)); // Lower with titlecase after

            let chunk = "abᾈDef"; // Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(2)); // Lower with titlecase after
            let res = Standard::rfind_boundary::<Char>(&chunk[..2]);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "abcDef";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(Standard::has_boundary_at::<Char>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 4));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));

            let chunk = "abǅDef";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(Standard::has_boundary_at::<Char>(chunk, 4));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));

            let chunk = "abᾈDef";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 7));
        }
    }

    // -------------------------------------------------------------------------
    mod hat {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<HatOnly>;

        #[test]
        fn find_boundary() {
            let chunk = "abcDef";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "ABCDef";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "abǅDef"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(2)); // Before a non-greek titlecase
            let res = Standard::find_boundary::<Char>(&chunk[2..]);
            assert_eq!(res, NonZero::new(2)); // Uppercase followed by lowercase

            let chunk = "abᾈDef"; // Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(5)); // Uppercase followed by lowercase
            let res = Standard::find_boundary::<Char>(&chunk[5..]);
            assert_eq!(res, None);
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "abcDef";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "ABCDef";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "abǅDef"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(4)); // Uppercase followed by lowercase
            let res = Standard::rfind_boundary::<Char>(&chunk[..4]);
            assert_eq!(res, NonZero::new(2)); // Before a non-greek titlecase

            let chunk = "abᾈDef"; // Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(5)); // Uppercase followed by lowercase
            let res = Standard::rfind_boundary::<Char>(&chunk[..5]);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "abcDef";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(Standard::has_boundary_at::<Char>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 4));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));

            let chunk = "abǅDef";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(Standard::has_boundary_at::<Char>(chunk, 4));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));

            let chunk = "abᾈDef";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 7));
        }
    }

    // -------------------------------------------------------------------------
    mod lower_digits {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<LowerDigitOnly>;

        #[test]
        fn find_boundary() {
            let chunk = "123abc";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "123Abc";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abc123";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "abC123";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abǅ123"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(4)); // non-greek titlecase to digit
            let res = Standard::find_boundary::<Char>(&chunk[4..]);
            assert_eq!(res, None);

            let chunk = "abᾈ123"; // Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "123ǅbc"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "123ᾈbc"; // Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "123abc";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "123Abc";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abc123";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "abC123";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abǅ123"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(4)); // non-greek titlecase to digit
            let res = Standard::rfind_boundary::<Char>(&chunk[..4]);
            assert_eq!(res, None);

            let chunk = "abᾈ123"; // Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "123ǅbc"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "123ᾈbc"; // Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "123abc";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(Standard::has_boundary_at::<Char>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 4));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));
        }
    }

    // -------------------------------------------------------------------------
    mod upper_digits {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<UpperDigitOnly>;

        #[test]
        fn find_boundary() {
            let chunk = "123abc";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "123Abc";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "abc123";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abC123";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "abǅ123"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abᾈ123"; // Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(5)); // greek titlecase to digit
            let res = Standard::find_boundary::<Char>(&chunk[5..]);
            assert_eq!(res, None);

            let chunk = "123ǅbc"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "123ᾈbc"; // Greek titlecase
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "123abc";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "123ABC";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "abc123";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "ABC123";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "abǅ123"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abᾈ123"; // Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(5)); // greek titlecase to digit
            let res = Standard::rfind_boundary::<Char>(&chunk[..5]);
            assert_eq!(res, None);

            let chunk = "123ǅbc"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "123ᾈbc"; // Greek titlecase
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "123Abc";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(Standard::has_boundary_at::<Char>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 4));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));
        }
    }

    // -------------------------------------------------------------------------
    mod default {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<Default>;

        #[test]
        fn find_boundary() {
            let chunk = "abcDef";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "ABCDef";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "123abc";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "123ABC";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abc123";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "ABC123";
            let res = Standard::find_boundary::<Char>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "abcDef";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "ABCDef";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, NonZero::new(3));

            let chunk = "123abc";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "123ABC";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "abc123";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);

            let chunk = "ABC123";
            let res = Standard::rfind_boundary::<Char>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "abcDef";
            assert!(!Standard::has_boundary_at::<Char>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 1));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 2));
            assert!(Standard::has_boundary_at::<Char>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 4));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Char>(chunk, 6));
        }
    }
}

// =============================================================================
// GRAPHEME SEGMENTATION TESTS
// -----------------------------------------------------------------------------
// These tests start with some basic string ("abcDef"), then mix in a combining
// code points that turns it into a grapheme which would not be a single code
// point (e.g. "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}" is 18
// code points, but only 6 graphemes).
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "unicode")]
mod grapheme {
    use super::*;
    use crate::syntax::segmentation::Grapheme;

    // -------------------------------------------------------------------------
    mod none {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<NoBoundaries>;

        #[test]
        fn find_boundary() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 6));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 9));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 10));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));
        }
    }

    // -------------------------------------------------------------------------
    mod camel {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<CamelOnly>;

        #[test]
        fn find_boundary() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));
            let res = Standard::find_boundary::<Grapheme>(&chunk[9..]);
            assert_eq!(res, None);

            let chunk = "A\u{0301}B\u{0301}C\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}D\u{0301}e\u{0301}f\u{0301}"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(6)); // Lower with titlecase after
            let res = Standard::find_boundary::<Grapheme>(&chunk[6..]);
            assert_eq!(res, NonZero::new(4)); // Upper with non-greek titlecase before

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}D\u{0301}e\u{0301}f\u{0301}"; // Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(6)); // Lower with titlecase after
            let res = Standard::find_boundary::<Grapheme>(&chunk[6..]);
            assert_eq!(res, None); // Upper, but greek titlecase before
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "A\u{0301}B\u{0301}C\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}D\u{0301}e\u{0301}f\u{0301}"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(10)); // Upper with non-greek titlecase before
            let res = Standard::rfind_boundary::<Grapheme>(&chunk[..10]);
            assert_eq!(res, NonZero::new(6)); // Lower with titlecase after

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}D\u{0301}e\u{0301}f\u{0301}"; // Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(6)); // Lower with titlecase after
            let res = Standard::rfind_boundary::<Grapheme>(&chunk[..6]);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 6));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 9));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 10));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 6));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 9));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 10));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 14));

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 6));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 9));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 10));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 14));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 15));
        }
    }

    // -------------------------------------------------------------------------
    mod hat {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<HatOnly>;

        #[test]
        fn find_boundary() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "A\u{0301}B\u{0301}C\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}D\u{0301}e\u{0301}f\u{0301}"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(6)); // Before a non-greek titlecase
            let res = Standard::find_boundary::<Grapheme>(&chunk[6..]);
            assert_eq!(res, NonZero::new(4)); // Uppercase followed by lowercase

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}D\u{0301}e\u{0301}f\u{0301}"; // Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(11)); // Uppercase followed by lowercase
            let res = Standard::find_boundary::<Grapheme>(&chunk[11..]);
            assert_eq!(res, None);
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "A\u{0301}B\u{0301}C\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}D\u{0301}e\u{0301}f\u{0301}"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(10)); // Uppercase followed by lowercase
            let res = Standard::rfind_boundary::<Grapheme>(&chunk[..10]);
            assert_eq!(res, NonZero::new(6)); // Before a non-greek titlecase

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}D\u{0301}e\u{0301}f\u{0301}"; // Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(11)); // Uppercase followed by lowercase
            let res = Standard::rfind_boundary::<Grapheme>(&chunk[..11]);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 6));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 9));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 10));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 6));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 9));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 10));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 14));

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 6));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 9));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 10));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 11));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 14));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 15));
        }
    }

    // -------------------------------------------------------------------------
    mod lower_digits {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<LowerDigitOnly>;

        #[test]
        fn find_boundary() {
            let chunk = "1\u{0301}2\u{0301}3\u{0301}a\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "1\u{0301}2\u{0301}3\u{0301}A\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}c\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "a\u{0301}b\u{0301}C\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}1\u{0301}2\u{0301}3\u{0301}"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(10)); // non-greek titlecase to digit
            let res = Standard::find_boundary::<Grapheme>(&chunk[10..]);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}1\u{0301}2\u{0301}3\u{0301}"; // Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}ǅ\u{0301}b\u{0301}c\u{0301}"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}ᾈ\u{0301}b\u{0301}c\u{0301}"; // Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "1\u{0301}2\u{0301}3\u{0301}a\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "1\u{0301}2\u{0301}3\u{0301}A\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}c\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "a\u{0301}b\u{0301}C\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}1\u{0301}2\u{0301}3\u{0301}"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(10)); // non-greek titlecase to digit
            let res = Standard::rfind_boundary::<Grapheme>(&chunk[..10]);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}1\u{0301}2\u{0301}3\u{0301}"; // Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}ǅ\u{0301}b\u{0301}c\u{0301}"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}ᾈ\u{0301}b\u{0301}c\u{0301}"; // Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "1\u{0301}2\u{0301}3\u{0301}a\u{0301}b\u{0301}c\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 6));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 9));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 10));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));
        }
    }

    // -------------------------------------------------------------------------
    mod upper_digits {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<UpperDigitOnly>;

        #[test]
        fn find_boundary() {
            let chunk = "1\u{0301}2\u{0301}3\u{0301}a\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}A\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "a\u{0301}b\u{0301}c\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}C\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}1\u{0301}2\u{0301}3\u{0301}"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}1\u{0301}2\u{0301}3\u{0301}"; // Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(11)); // greek titlecase to digit
            let res = Standard::find_boundary::<Grapheme>(&chunk[11..]);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}ǅ\u{0301}b\u{0301}c\u{0301}"; // Non-Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "1\u{0301}2\u{0301}3\u{0301}ᾈ\u{0301}b\u{0301}c\u{0301}"; // Greek titlecase
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "1\u{0301}2\u{0301}3\u{0301}a\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}A\u{0301}B\u{0301}C\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "a\u{0301}b\u{0301}c\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "A\u{0301}B\u{0301}C\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "a\u{0301}b\u{0301}ǅ\u{0301}1\u{0301}2\u{0301}3\u{0301}"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}ᾈ\u{0301}1\u{0301}2\u{0301}3\u{0301}"; // Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(11)); // greek titlecase to digit
            let res = Standard::rfind_boundary::<Grapheme>(&chunk[..11]);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}ǅ\u{0301}b\u{0301}c\u{0301}"; // Non-Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "1\u{0301}2\u{0301}3\u{0301}ᾈ\u{0301}b\u{0301}c\u{0301}"; // Greek titlecase
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "1\u{0301}2\u{0301}3\u{0301}A\u{0301}b\u{0301}c\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 6));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 9));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 10));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));
        }
    }

    // -------------------------------------------------------------------------
    mod default {
        use super::*;

        // Options Under Test...
        type Standard = super::Standard<Default>;

        #[test]
        fn find_boundary() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "A\u{0301}B\u{0301}C\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "1\u{0301}2\u{0301}3\u{0301}a\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}A\u{0301}B\u{0301}C\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}c\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "A\u{0301}B\u{0301}C\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::find_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn rfind_boundary() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "A\u{0301}B\u{0301}C\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, NonZero::new(9));

            let chunk = "1\u{0301}2\u{0301}3\u{0301}a\u{0301}b\u{0301}c\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "1\u{0301}2\u{0301}3\u{0301}A\u{0301}B\u{0301}C\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "a\u{0301}b\u{0301}c\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);

            let chunk = "A\u{0301}B\u{0301}C\u{0301}1\u{0301}2\u{0301}3\u{0301}";
            let res = Standard::rfind_boundary::<Grapheme>(chunk);
            assert_eq!(res, None);
        }

        #[test]
        fn has_boundary_at() {
            let chunk = "a\u{0301}b\u{0301}c\u{0301}D\u{0301}e\u{0301}f\u{0301}";
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 0));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 1));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 2));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 3));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 4));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 5));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 6));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 7));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 8));
            assert!(Standard::has_boundary_at::<Grapheme>(chunk, 9));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 10));
            // assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 11));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 12));
            assert!(!Standard::has_boundary_at::<Grapheme>(chunk, 13));
        }
    }
}
