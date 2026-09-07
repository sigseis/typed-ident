// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use heck::*;
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
        "lower-camel" => identifier.to_lower_camel_case(),
        "upper-camel" => identifier.to_upper_camel_case(),
        "lower-kebab" => identifier.to_kebab_case(),
        "upper-kebab" => identifier.to_shouty_kebab_case(),
        "lower-snake" => identifier.to_snake_case(),
        "upper-snake" => identifier.to_shouty_snake_case(),
        unknown => {
            eprintln!("unknown command: {unknown}");
            return ExitCode::FAILURE;
        }
    };
    println!("converted: {result}");
    ExitCode::SUCCESS
}
