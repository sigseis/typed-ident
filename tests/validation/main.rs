#![cfg(feature = "alloc")]
#![cfg(feature = "presets")]
#![cfg(feature = "unicode")]
// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! log {
    ($($arg:tt)+) => {
        if crate::DEBUG_FILTER.is_some() {
            eprintln!($($arg)*)
        }
    };
}

// -----------------------------------------------------------------------------
macro_rules! log_for {
    ($test:ident, $($arg:tt)+) => {
        if crate::DEBUG_FILTER.as_deref().is_some_and(|f| $test.name.contains(f)) {
            eprintln!($($arg)*)
        }
    };
}

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
pub mod helpers;
pub mod runner;
pub mod test_data;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use helpers::TestResult;
use std::process::ExitCode;
use std::sync::LazyLock;

// =============================================================================
// CONSTANTS
// =============================================================================

// -----------------------------------------------------------------------------
pub static DEBUG_FILTER: LazyLock<Option<String>> =
    LazyLock::new(|| std::env::var("DEBUG_TYPED_IDENT").ok());

// =============================================================================
// MAIN
// =============================================================================

// -----------------------------------------------------------------------------
#[test]
fn main() -> ExitCode {
    let mut failures = 0;
    log!("===================================================================");
    log!("starting stress tests with logging enabled");
    log!("if this is not intended, unset DEBUG_TYPED_IDENT");
    for test in test_data::VALID {
        let checked_name = test.build_string();
        if checked_name != test.name {
            eprintln!("{}: name check from segments mismatched", test.name);
            eprintln!("  expected: {}", test.name);
            eprintln!("    actual: {checked_name}");
            failures += 1;
            continue;
        }
        failures += match runner::test_valid(test) {
            TestResult::Pass => 0,
            TestResult::Fail => 1,
        };
    }
    for test in test_data::INVALID {
        failures += match runner::test_invalid(test) {
            TestResult::Pass => 0,
            TestResult::Fail => 1,
        };
    }
    log!("===================================================================");
    match failures {
        0 => {
            if DEBUG_FILTER.is_some() {
                eprintln!();
                eprintln!(
                    "no tests failed, but we are forcing a failure because of DEBUG_TYPED_IDENT"
                );
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        n => {
            eprintln!();
            eprintln!("aborting stress test due to prior {n} failures");
            ExitCode::FAILURE
        }
    }
}
