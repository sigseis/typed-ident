// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use std::process::ExitCode;
use typed_ident::convert::*;
use typed_ident::presets::ascii::*;

// =============================================================================
// MAIN
// =============================================================================

// -----------------------------------------------------------------------------
fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let format = args.next().expect("expected at least one format argument");
    let identifier = args.next().expect("expected at least one input argument");
    let identifier = HybridIdent::new(&identifier).expect("expected a hybrid-ident identifier");
    let result: String = match format.as_str() {
        "lower-camel" => identifier.to_lower_camel(),
        "upper-camel" => identifier.to_upper_camel(),
        "lower-kebab" => identifier.to_lower_kebab(),
        "upper-kebab" => identifier.to_upper_kebab(),
        "lower-snake" => identifier.to_lower_snake(),
        "upper-snake" => identifier.to_upper_snake(),
        unknown => {
            eprintln!("unknown command: {unknown}");
            return ExitCode::FAILURE;
        }
    }
    .expect("failed to convert to the target format");
    println!("converted: {result}");
    ExitCode::SUCCESS
}
