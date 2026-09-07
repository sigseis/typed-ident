// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Cli;
use crate::common::{Format, Profile as CliProfile};
use std::process::ExitCode;
use typed_ident::Error;
use typed_ident::syntax::profile::{AppendClosed, Ascii, CharProfile, Profile, Unicode};
use typed_ident::syntax::segmentation;

// =============================================================================
// TYPES: ASCII Open
// =============================================================================

// -----------------------------------------------------------------------------
enum AsciiOpen {}

// -----------------------------------------------------------------------------
impl Profile for AsciiOpen {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Empty;
    type BaseProfile = Self;
    type Segmentation = segmentation::Char;

    #[inline(always)]
    fn is_ident_start(c: char) -> bool {
        Ascii::is_ident_start(c)
    }
    #[inline(always)]
    fn is_chunk_start(c: char) -> bool {
        Ascii::in_profile(c)
    }
    #[inline(always)]
    fn in_profile(c: char) -> bool {
        Ascii::in_profile(c)
    }
    #[inline(always)]
    fn is_chunk_continue(c: char) -> bool {
        Ascii::in_profile(c)
    }
}

// -----------------------------------------------------------------------------
impl CharProfile for AsciiOpen {}

// =============================================================================
// TYPES: Unicode Open
// =============================================================================

// -----------------------------------------------------------------------------
enum UnicodeOpen {}

// -----------------------------------------------------------------------------
impl Profile for UnicodeOpen {
    const APPEND_CLOSED: AppendClosed = AppendClosed::Empty;
    type BaseProfile = Self;
    type Segmentation = segmentation::Grapheme;

    #[inline(always)]
    fn is_ident_start(c: char) -> bool {
        Unicode::is_ident_start(c)
    }
    #[inline(always)]
    fn is_chunk_start(c: char) -> bool {
        Unicode::in_profile(c)
    }
    #[inline(always)]
    fn in_profile(c: char) -> bool {
        Unicode::in_profile(c)
    }
    #[inline(always)]
    fn is_chunk_continue(c: char) -> bool {
        Unicode::in_profile(c)
    }
}

// -----------------------------------------------------------------------------
impl CharProfile for UnicodeOpen {}

// =============================================================================
// COMMAND: Profile Configurations
// =============================================================================

// -----------------------------------------------------------------------------
fn validate_ascii(cli: &Cli, ident: &str) -> Option<Error> {
    use typed_ident::presets::generic::*;
    match cli.format {
        Format::Camel => CamelIdent::<AsciiOpen>::new(ident).err(),
        Format::UpperCamel => UpperCamelIdent::<AsciiOpen>::new(ident).err(),
        Format::LowerCamel => LowerCamelIdent::<AsciiOpen>::new(ident).err(),
        Format::Snake => SnakeIdent::<AsciiOpen>::new(ident).err(),
        Format::UpperSnake => UpperSnakeIdent::<AsciiOpen>::new(ident).err(),
        Format::LowerSnake => LowerSnakeIdent::<AsciiOpen>::new(ident).err(),
        Format::Kebab => KebabIdent::<AsciiOpen>::new(ident).err(),
        Format::UpperKebab => UpperKebabIdent::<AsciiOpen>::new(ident).err(),
        Format::LowerKebab => LowerKebabIdent::<AsciiOpen>::new(ident).err(),
        Format::Hybrid => HybridIdent::<AsciiOpen>::new(ident).err(),
        Format::UpperHybrid => UpperHybridIdent::<AsciiOpen>::new(ident).err(),
        Format::LowerHybrid => LowerHybridIdent::<AsciiOpen>::new(ident).err(),
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
    use typed_ident::presets::generic::*;
    match cli.format {
        Format::Camel => CamelIdent::<UnicodeOpen>::new(ident).err(),
        Format::UpperCamel => UpperCamelIdent::<UnicodeOpen>::new(ident).err(),
        Format::LowerCamel => LowerCamelIdent::<UnicodeOpen>::new(ident).err(),
        Format::Snake => SnakeIdent::<UnicodeOpen>::new(ident).err(),
        Format::UpperSnake => UpperSnakeIdent::<UnicodeOpen>::new(ident).err(),
        Format::LowerSnake => LowerSnakeIdent::<UnicodeOpen>::new(ident).err(),
        Format::Kebab => KebabIdent::<UnicodeOpen>::new(ident).err(),
        Format::UpperKebab => UpperKebabIdent::<UnicodeOpen>::new(ident).err(),
        Format::LowerKebab => LowerKebabIdent::<UnicodeOpen>::new(ident).err(),
        Format::Hybrid => HybridIdent::<UnicodeOpen>::new(ident).err(),
        Format::UpperHybrid => UpperHybridIdent::<UnicodeOpen>::new(ident).err(),
        Format::LowerHybrid => LowerHybridIdent::<UnicodeOpen>::new(ident).err(),
    }
}

// =============================================================================
// COMMAND
// =============================================================================

// -----------------------------------------------------------------------------
pub fn validate_open(cli: &Cli) -> ExitCode {
    let mut failed = false;
    for ident in cli.input.iter() {
        let error = match cli.profile {
            CliProfile::Ascii => validate_ascii(cli, ident),
            CliProfile::Strict => validate_strict(cli, ident),
            CliProfile::Unicode => validate_unicode(cli, ident),
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
