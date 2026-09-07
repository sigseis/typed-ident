// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::{CasedFormat, Command, Format, Profile};
use clap::Parser;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The identifier format to use when parsing input.
    #[arg(short, long, value_enum, default_value_t)]
    pub format: Format,

    /// The profile to use when parsing input (only supported in `validate` command).
    #[arg(short, long, value_enum, default_value_t)]
    pub profile: Profile,

    /// The identifier format to convert to during a conversion operations (only supported in `convert` command).
    #[arg(short, long, value_enum)]
    pub to: Option<CasedFormat>,

    /// Enables processing commands using other crates for comparison (only supported in `convert` command).
    #[arg(short, long)]
    pub compare: bool,

    /// Runs the hidden version of certain commands (this is usually used for binary analysis).
    #[arg(long, hide = true)]
    pub hidden: bool,

    /// The command to run on the provided input strings.
    pub command: Command,

    /// The input that the selected command operates over.
    pub input: Vec<String>,
}
