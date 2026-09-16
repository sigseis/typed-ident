// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Cli;
use crate::common::Format;
use std::process::ExitCode;
use typed_ident::presets::unicode::*;
use typed_ident::syntax::delimiter::*;
use typed_ident::{Error, Identifier};

// =============================================================================
// COMMAND: Profile Configurations
// =============================================================================

// -----------------------------------------------------------------------------
fn join_fragments<I: Identifier + ?Sized>(cli: &Cli, delim: I::Delimiter) -> Option<Error> {
    let mut ident = I::new_fragment_buffer();
    println!("starting join operations...");
    for fragment in cli.input.iter() {
        println!("  joining '{ident}' + '{fragment}' ->");
        if let Err(error) = ident.push_bounded_with(fragment, delim) {
            return Some(error);
        }
        println!("    success: {ident}");
    }

    // Interpret the results as an identifier, this is the final check...
    let ident = match ident.into_boxed_ident() {
        Ok(ident) => ident,
        Err(error) => return Some(error),
    };
    println!("final identifier: {ident}");
    println!();
    None
}

// =============================================================================
// COMMAND
// =============================================================================

// -----------------------------------------------------------------------------
pub fn join(cli: &Cli) -> ExitCode {
    let error = match cli.format {
        Format::Camel => join_fragments::<CamelIdent>(cli, LowLine),
        Format::CasedCamel => join_fragments::<CasedCamelIdent>(cli, LowLine),
        Format::UpperCamel => join_fragments::<UpperCamelIdent>(cli, LowLine),
        Format::LowerCamel => join_fragments::<LowerCamelIdent>(cli, LowLine),
        Format::Snake => join_fragments::<SnakeIdent>(cli, LowLine),
        Format::CasedSnake => join_fragments::<CasedSnakeIdent>(cli, LowLine),
        Format::UpperSnake => join_fragments::<UpperSnakeIdent>(cli, LowLine),
        Format::LowerSnake => join_fragments::<LowerSnakeIdent>(cli, LowLine),
        Format::Kebab => join_fragments::<KebabIdent>(cli, HyphenMinus),
        Format::CasedKebab => join_fragments::<CasedKebabIdent>(cli, HyphenMinus),
        Format::UpperKebab => join_fragments::<UpperKebabIdent>(cli, HyphenMinus),
        Format::LowerKebab => join_fragments::<LowerKebabIdent>(cli, HyphenMinus),
        Format::Hybrid => join_fragments::<HybridIdent>(cli, AsciiFlatLine::LowLine),
        Format::CasedHybrid => join_fragments::<CasedHybridIdent>(cli, AsciiFlatLine::LowLine),
        Format::UpperHybrid => join_fragments::<UpperHybridIdent>(cli, AsciiFlatLine::LowLine),
        Format::LowerHybrid => join_fragments::<LowerHybridIdent>(cli, AsciiFlatLine::LowLine),
    };
    match error {
        None => ExitCode::SUCCESS,
        Some(error) => {
            eprintln!("failed join: {error}");
            ExitCode::FAILURE
        }
    }
}
