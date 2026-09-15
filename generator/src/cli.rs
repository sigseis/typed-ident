// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::{Command, Srcdirs};
use anyhow::{Result, anyhow};
use clap::Parser;
use std::path::PathBuf;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Parser, Debug)]
pub struct Cli {
    /// Whether or not to skip the cleanup step after downloading UCD data.
    #[arg(long)]
    pub skip_cleanup: bool,

    /// Whether or not to skip the download step when updating generator tables.
    #[arg(long)]
    pub skip_download: bool,

    /// The path to the root of the `typed-ident` repo.
    ///
    /// This will be used for both `update-tables` and `generate`, and must
    /// point to the root of the source directory tree.
    #[arg(long, value_enum)]
    srcdir: Option<PathBuf>,

    /// The version of the Unicode standard to use for generation.
    ///
    /// If no version is provided, we will attempt to find the latest.
    ///
    /// This flag only has meaning when provided to the `update-tables` command.
    #[arg(long)]
    pub unicode_version: Option<String>,

    /// The command to run for the generator.
    pub command: Command,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Cli {
    pub fn srcdirs(&self) -> Result<Srcdirs> {
        let srcdir = self.srcdir.clone().unwrap_or_else(|| PathBuf::from("."));
        if !srcdir.join("Cargo.toml").exists() {
            return Err(anyhow!(
                "no `Cargo.toml` file found in --srcdir={srcdir:?} - are you pointing at the incorrect directory?"
            ));
        }
        let generator_tables = srcdir.join("generator/src/generated");
        if !generator_tables.exists() {
            return Err(anyhow!(
                "no `generator/src/generated` file found in --srcdir={srcdir:?} - are you pointing at the incorrect directory?"
            ));
        }
        let syntax = srcdir.join("src/syntax");
        if !syntax.exists() {
            return Err(anyhow!(
                "no `src/syntax` file found in --srcdir={srcdir:?} - are you pointing at the incorrect directory?"
            ));
        }
        Ok(Srcdirs {
            generator_tables,
            syntax,
        })
    }
}
