// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use std::process::ExitCode;
use typed_ident::alloc::convert::*;
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
    let result = match format.as_str() {
        "lower-camel" => identifier.to_lower_camel_canonical(),
        "upper-camel" => identifier.to_upper_camel_canonical(),
        "lower-kebab" => identifier.to_lower_kebab_canonical(),
        "upper-kebab" => identifier.to_upper_kebab_canonical(),
        "lower-snake" => identifier.to_lower_snake_canonical(),
        "upper-snake" => identifier.to_upper_snake_canonical(),
        unknown => {
            eprintln!("unknown command: {unknown}");
            return ExitCode::FAILURE;
        }
    };
    println!("converted: {result}");
    ExitCode::SUCCESS
}
