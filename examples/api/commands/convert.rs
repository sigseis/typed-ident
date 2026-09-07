// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Cli;
use crate::common::{CasedFormat, Format};
use convert_case::ccase;
use heck::*;
use std::process::ExitCode;
use typed_ident::alloc::convert::*;
use typed_ident::presets::unicode::*;
use typed_ident::syntax::delimiter::*;
use typed_ident::{ConvertibleIdentifier, Error};

// =============================================================================
// COMMAND: Profile Configurations
// =============================================================================

// -----------------------------------------------------------------------------
fn reformat_from<I: ConvertibleIdentifier + ?Sized>(cli: &Cli, ident: &str) -> Option<Error> {
    let ident = match I::new(ident) {
        Ok(ident) => ident,
        Err(error) => return Some(error),
    };
    let Some(to) = cli.to else {
        panic!("you need to provide a --to conversion option to say what to convert to");
    };
    let (canonical, decorated, delimited) = match to {
        CasedFormat::UpperCamel => (
            ident.to_upper_camel_canonical(),
            ident.to_upper_camel_decorated(),
            Some(ident.to_upper_camel_delimited()),
        ),
        CasedFormat::LowerCamel => (
            ident.to_lower_camel_canonical(),
            ident.to_lower_camel_decorated(),
            Some(ident.to_lower_camel_delimited()),
        ),
        CasedFormat::UpperSnake => (
            ident.to_upper_snake_canonical(),
            ident.to_upper_snake_decorated(),
            None,
        ),
        CasedFormat::LowerSnake => (
            ident.to_lower_snake_canonical(),
            ident.to_lower_snake_decorated(),
            None,
        ),
        CasedFormat::UpperKebab => (
            ident.to_upper_kebab_canonical(),
            ident.to_upper_kebab_decorated(),
            None,
        ),
        CasedFormat::LowerKebab => (
            ident.to_lower_kebab_canonical(),
            ident.to_lower_kebab_decorated(),
            None,
        ),
        CasedFormat::UpperHybrid => (
            ident.to_upper_hybrid_canonical(AsciiFlatLine::LowLine),
            ident.to_upper_hybrid_decorated(AsciiFlatLine::LowLine),
            Some(ident.to_upper_hybrid_delimited(AsciiFlatLine::LowLine)),
        ),
        CasedFormat::LowerHybrid => (
            ident.to_lower_hybrid_canonical(AsciiFlatLine::LowLine),
            ident.to_lower_hybrid_decorated(AsciiFlatLine::LowLine),
            Some(ident.to_lower_hybrid_delimited(AsciiFlatLine::LowLine)),
        ),
    };
    let compare = match cli.compare {
        false => None,
        true => Some(match to {
            CasedFormat::UpperCamel => (
                ccase!(upper_camel, ident.as_str()),
                ident.as_str().to_upper_camel_case(),
            ),
            CasedFormat::LowerCamel => (
                ccase!(camel, ident.as_str()),
                ident.as_str().to_lower_camel_case(),
            ),
            CasedFormat::UpperSnake => (
                ccase!(upper_snake, ident.as_str()),
                ident.as_str().to_shouty_snake_case(),
            ),
            CasedFormat::LowerSnake => (
                ccase!(snake, ident.as_str()),
                ident.as_str().to_snake_case(),
            ),
            CasedFormat::UpperKebab => (
                ccase!(upper_kebab, ident.as_str()),
                ident.as_str().to_shouty_kebab_case(),
            ),
            CasedFormat::LowerKebab => (
                ccase!(kebab, ident.as_str()),
                ident.as_str().to_kebab_case(),
            ),
            CasedFormat::UpperHybrid => (String::from("N/A"), String::from("N/A")),
            CasedFormat::LowerHybrid => (String::from("N/A"), String::from("N/A")),
        }),
    };
    println!("{ident}:");
    match compare {
        None => println!("  canonical: {canonical}"),
        Some((ccase, heck)) => println!("  canonical: {canonical} (ccase: {ccase}, heck: {heck})"),
    }
    println!("  decorated: {decorated}");
    if let Some(delimited) = delimited {
        println!("  delimited: {delimited}");
    }
    println!();
    None
}

// =============================================================================
// COMMAND
// =============================================================================

// -----------------------------------------------------------------------------
pub fn convert(cli: &Cli) -> ExitCode {
    let mut failed = false;
    for ident in cli.input.iter() {
        let error = match cli.format {
            Format::Camel => reformat_from::<CamelIdent>(cli, ident),
            Format::UpperCamel => reformat_from::<UpperCamelIdent>(cli, ident),
            Format::LowerCamel => reformat_from::<LowerCamelIdent>(cli, ident),
            Format::Snake => reformat_from::<SnakeIdent>(cli, ident),
            Format::UpperSnake => reformat_from::<UpperSnakeIdent>(cli, ident),
            Format::LowerSnake => reformat_from::<LowerSnakeIdent>(cli, ident),
            Format::Kebab => reformat_from::<KebabIdent>(cli, ident),
            Format::UpperKebab => reformat_from::<UpperKebabIdent>(cli, ident),
            Format::LowerKebab => reformat_from::<LowerKebabIdent>(cli, ident),
            Format::Hybrid => reformat_from::<HybridIdent>(cli, ident),
            Format::UpperHybrid => reformat_from::<UpperHybridIdent>(cli, ident),
            Format::LowerHybrid => reformat_from::<LowerHybridIdent>(cli, ident),
        };
        if let Some(error) = error {
            failed = true;
            eprintln!("{ident}: failed conversion: {error}");
        }
    }
    match failed {
        true => ExitCode::FAILURE,
        false => ExitCode::SUCCESS,
    }
}
