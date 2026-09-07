// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use convert_case::ccase;
use std::process::ExitCode;

// =============================================================================
// MAIN
// =============================================================================

// -----------------------------------------------------------------------------
fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let format = args.next().expect("expected at least one format argument");
    let identifier = args.next().expect("expected at least one input argument");
    let result = match format.as_str() {
        "lower-camel" => ccase!(camel, identifier),
        "upper-camel" => ccase!(upper_camel, identifier),
        "lower-kebab" => ccase!(kebab, identifier),
        "upper-kebab" => ccase!(upper_kebab, identifier),
        "lower-snake" => ccase!(snake, identifier),
        "upper-snake" => ccase!(upper_snake, identifier),
        unknown => {
            eprintln!("unknown command: {unknown}");
            return ExitCode::FAILURE;
        }
    };
    println!("converted: {result}");
    ExitCode::SUCCESS
}
