// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Cli;
use crate::common::Format;
use std::process::ExitCode;
use typed_ident::presets::unicode::*;
use typed_ident::{Error, Identifier};

// =============================================================================
// COMMAND: Profile Configurations
// =============================================================================

// -----------------------------------------------------------------------------
fn ident_segments<I: Identifier + ?Sized>(ident: &str) -> Option<Error> {
    let ident = match I::new(ident) {
        Ok(ident) => ident,
        Err(error) => return Some(error),
    };
    println!("{ident} segments:");
    for segment in ident.segments().type_erased() {
        println!("* {segment:?}");
    }
    println!();
    None
}

// =============================================================================
// COMMAND
// =============================================================================

// -----------------------------------------------------------------------------
pub fn segment(cli: &Cli) -> ExitCode {
    let mut failed = false;
    for ident in cli.input.iter() {
        let error = match cli.format {
            Format::Camel => ident_segments::<CamelIdent>(ident),
            Format::UpperCamel => ident_segments::<UpperCamelIdent>(ident),
            Format::LowerCamel => ident_segments::<LowerCamelIdent>(ident),
            Format::Snake => ident_segments::<SnakeIdent>(ident),
            Format::UpperSnake => ident_segments::<UpperSnakeIdent>(ident),
            Format::LowerSnake => ident_segments::<LowerSnakeIdent>(ident),
            Format::Kebab => ident_segments::<KebabIdent>(ident),
            Format::UpperKebab => ident_segments::<UpperKebabIdent>(ident),
            Format::LowerKebab => ident_segments::<LowerKebabIdent>(ident),
            Format::Hybrid => ident_segments::<HybridIdent>(ident),
            Format::UpperHybrid => ident_segments::<UpperHybridIdent>(ident),
            Format::LowerHybrid => ident_segments::<LowerHybridIdent>(ident),
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
