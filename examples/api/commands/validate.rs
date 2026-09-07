// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Cli;
use crate::common::{Format, Profile};
use std::process::ExitCode;
use typed_ident::Error;

// =============================================================================
// COMMAND: Profile Configurations
// =============================================================================

// -----------------------------------------------------------------------------
fn validate_ascii(cli: &Cli, ident: &str) -> Option<Error> {
    use typed_ident::presets::ascii::*;
    match cli.format {
        Format::Camel => CamelIdent::new(ident).err(),
        Format::UpperCamel => UpperCamelIdent::new(ident).err(),
        Format::LowerCamel => LowerCamelIdent::new(ident).err(),
        Format::Snake => SnakeIdent::new(ident).err(),
        Format::UpperSnake => UpperSnakeIdent::new(ident).err(),
        Format::LowerSnake => LowerSnakeIdent::new(ident).err(),
        Format::Kebab => KebabIdent::new(ident).err(),
        Format::UpperKebab => UpperKebabIdent::new(ident).err(),
        Format::LowerKebab => LowerKebabIdent::new(ident).err(),
        Format::Hybrid => HybridIdent::new(ident).err(),
        Format::UpperHybrid => UpperHybridIdent::new(ident).err(),
        Format::LowerHybrid => LowerHybridIdent::new(ident).err(),
    }
}

// -----------------------------------------------------------------------------
fn validate_strict(cli: &Cli, ident: &str) -> Option<Error> {
    use typed_ident::presets::strict::*;
    match cli.format {
        Format::Camel => CamelIdent::new(ident).err(),
        Format::UpperCamel => UpperCamelIdent::new(ident).err(),
        Format::LowerCamel => LowerCamelIdent::new(ident).err(),
        Format::Snake => SnakeIdent::new(ident).err(),
        Format::UpperSnake => UpperSnakeIdent::new(ident).err(),
        Format::LowerSnake => LowerSnakeIdent::new(ident).err(),
        Format::Kebab => KebabIdent::new(ident).err(),
        Format::UpperKebab => UpperKebabIdent::new(ident).err(),
        Format::LowerKebab => LowerKebabIdent::new(ident).err(),
        Format::Hybrid => HybridIdent::new(ident).err(),
        Format::UpperHybrid => UpperHybridIdent::new(ident).err(),
        Format::LowerHybrid => LowerHybridIdent::new(ident).err(),
    }
}

// -----------------------------------------------------------------------------
fn validate_unicode(cli: &Cli, ident: &str) -> Option<Error> {
    use typed_ident::presets::unicode::*;
    match cli.format {
        Format::Camel => CamelIdent::new(ident).err(),
        Format::UpperCamel => UpperCamelIdent::new(ident).err(),
        Format::LowerCamel => LowerCamelIdent::new(ident).err(),
        Format::Snake => SnakeIdent::new(ident).err(),
        Format::UpperSnake => UpperSnakeIdent::new(ident).err(),
        Format::LowerSnake => LowerSnakeIdent::new(ident).err(),
        Format::Kebab => KebabIdent::new(ident).err(),
        Format::UpperKebab => UpperKebabIdent::new(ident).err(),
        Format::LowerKebab => LowerKebabIdent::new(ident).err(),
        Format::Hybrid => HybridIdent::new(ident).err(),
        Format::UpperHybrid => UpperHybridIdent::new(ident).err(),
        Format::LowerHybrid => LowerHybridIdent::new(ident).err(),
    }
}

// =============================================================================
// COMMAND
// =============================================================================

// -----------------------------------------------------------------------------
pub fn validate(cli: &Cli) -> ExitCode {
    let mut failed = false;
    for ident in cli.input.iter() {
        let error = match cli.profile {
            Profile::Ascii => validate_ascii(cli, ident),
            Profile::Strict => validate_strict(cli, ident),
            Profile::Unicode => validate_unicode(cli, ident),
        };
        match error {
            None => println!("{ident}: valid ident"),
            Some(error) => {
                failed = true;
                eprintln!("{ident}: invalid ident: {error}")
            }
        }
    }
    match failed {
        true => ExitCode::FAILURE,
        false => ExitCode::SUCCESS,
    }
}
