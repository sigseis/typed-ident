// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Cli;
use crate::generated::version::UNICODE_VERSION;
use anyhow::{Result, anyhow};
use core::cmp::Ordering::*;
use std::fmt::Write;

// =============================================================================
// HELPERS
// =============================================================================

// -----------------------------------------------------------------------------
fn compute_overlaps(t1: &[(u32, u32)], t2: &[(u32, u32)]) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
    let mut overlap = Vec::new();
    let mut non_overlap = Vec::new();
    for &(low, high) in t1 {
        let mut range = (low, low);
        let mut in_t2 = None;
        for c in low..=high {
            let is_greek = t2
                .binary_search_by(|&(low, high)| {
                    if low > c {
                        Greater
                    } else if high < c {
                        Less
                    } else {
                        Equal
                    }
                })
                .is_ok();
            match in_t2 {
                None => in_t2 = Some(is_greek),
                Some(currently_greek) if currently_greek != is_greek => {
                    if currently_greek {
                        overlap.push(range);
                    } else {
                        non_overlap.push(range);
                    }
                    range.0 = c;
                    range.1 = c;
                    in_t2 = Some(is_greek);
                }
                _ => range.1 = c,
            }
        }
        match in_t2 {
            Some(true) => overlap.push(range),
            Some(false) => non_overlap.push(range),
            None => (),
        }
    }
    (overlap, non_overlap)
}

// -----------------------------------------------------------------------------
fn merge_tables(t1: &[(u32, u32)], t2: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut merged = Vec::new();
    let mut t1 = t1.iter().copied().peekable();
    let mut t2 = t2.iter().copied().peekable();

    loop {
        // Take from the next smallest range - then return the other iterator.
        let next_smallest = if t1
            .peek()
            .is_some_and(|t1| t2.peek().is_none_or(|t2| t1.0 < t2.0))
        {
            t1.next()
        } else {
            t2.next()
        };

        // If we took no data, then there's nothing left to process - break.
        let Some(mut range) = next_smallest else {
            break;
        };

        // Merge with contiguous data - only skip combining if there's a gap.
        loop {
            let next_smallest_iter = if t1
                .peek()
                .is_some_and(|t1| t2.peek().is_none_or(|t2| t1.0 < t2.0))
            {
                &mut t1
            } else {
                &mut t2
            };
            if next_smallest_iter
                .peek()
                .is_none_or(|next| next.0 > range.1 + 1)
            {
                break;
            }
            let next = next_smallest_iter.next().unwrap();
            if next.1 > range.1 {
                range.1 = next.1;
            }
        }

        // We're done adding the contiguous ranges together - push it to results.
        merged.push(range);
    }

    merged
}

// -----------------------------------------------------------------------------
fn generate_in_range() -> Result<String> {
    Ok(format!(
        r#"fn in_range(c: char, table: &[(char, char)]) -> bool {{
    use core::cmp::Ordering::*;
    table
        .binary_search_by(|&(low, high)| {{
            if low > c {{
                Greater
            }} else if high < c {{
                Less
            }} else {{
                Equal
            }}
        }})
        .is_ok()
}}"#
    ))
}

// -----------------------------------------------------------------------------
fn generate_function(name: &str, table: &[(u32, u32)]) -> Result<String> {
    // Ensure that all table values are in ascending order, and that there's
    // no data that could be joined. We want to catch any generation errors.
    let mut last_largest = None;
    for (low, high) in table {
        if low > high {
            return Err(anyhow!("the table data provided is invalid for: {name}"));
        }
        last_largest = match last_largest {
            None => Some(high),
            Some(last) if low > last => Some(high),
            Some(last) => {
                return Err(anyhow!(
                    "the table data contains an invalid or unmerged contiguous range for: {name}: last_largest={last}, next=({low}, {high})"
                ));
            }
        };
    }

    // Write the data out, it appears to be valid.
    let min = table.iter().next().unwrap().0;
    let mut f = String::new();
    writeln!(f, r#"pub fn {name}(c: char) -> bool {{"#)?;
    writeln!(f, r#"    const TABLE: &[(char, char)] = &["#)?;
    for (low, high) in table {
        writeln!(f, r#"        ('\u{{{low:X}}}', '\u{{{high:X}}}'),"#)?;
    }
    writeln!(f, r#"    ];"#)?;
    writeln!(f, r#"    if c < '\u{{{min:X}}}' {{"#)?;
    writeln!(f, r#"        return false;"#)?;
    writeln!(f, r#"    }}"#)?;
    writeln!(f, r#"    in_range(c, TABLE)"#)?;
    write!(f, r#"}}"#)?;
    Ok(f)
}

// -----------------------------------------------------------------------------
fn generate_is_titlecase() -> Result<String> {
    generate_function(
        "is_titlecase",
        crate::generated::general_category::TITLECASE_LETTER,
    )
}

// -----------------------------------------------------------------------------
fn generate_is_titlecase_greek_variant() -> Result<String> {
    let (greek_titlecase, non_greek_titlecase) = compute_overlaps(
        crate::generated::general_category::TITLECASE_LETTER,
        crate::generated::script::GREEK,
    );

    // Check if we can just do some simple range comparison.
    let max_non_greek = non_greek_titlecase.iter().last().unwrap().1;
    let min_greek = greek_titlecase.iter().next().unwrap().0;
    if max_non_greek > min_greek {
        return Err(anyhow!(
            "cannot create a simple range check for is_titlecase_greek_variant: new code needs to be written"
        ));
    }

    Ok(format!(
        r#"pub fn is_titlecase_greek_variant(tc: char) -> bool {{
    tc >= '\u{{{min_greek:X}}}'
}}"#
    ))
}

// -----------------------------------------------------------------------------
fn generate_is_combining_mark() -> Result<String> {
    let merged = merge_tables(
        crate::generated::general_category::ENCLOSING_MARK,
        crate::generated::general_category::NONSPACING_MARK,
    );
    let merged = merge_tables(
        merged.as_slice(),
        crate::generated::general_category::SPACING_MARK,
    );
    generate_function("is_combining_mark", &merged)
}

// =============================================================================
// COMMAND
// =============================================================================

// -----------------------------------------------------------------------------
pub fn generate(cli: &Cli) -> Result<()> {
    if cli.unicode_version.is_some() {
        return Err(anyhow!(
            "The --unicode-version flag only impacts the `update-tables` command, it has no meaning for source generation"
        ));
    }
    let srcdirs = cli.srcdirs()?;
    let syntax_src = srcdirs.syntax.join("generated.rs");

    // Generate the is_titlecase() function first
    let in_range = generate_in_range()?;
    let is_titlecase = generate_is_titlecase()?;
    let is_titlecase_greek_variant = generate_is_titlecase_greek_variant()?;
    let is_combining_mark = generate_is_combining_mark()?;

    // Generate functions to help test this logic.
    let is_enclosing_mark = generate_function(
        "is_enclosing_mark",
        crate::generated::general_category::ENCLOSING_MARK,
    )?;
    let is_nonspacing_mark = generate_function(
        "is_nonspacing_mark",
        crate::generated::general_category::NONSPACING_MARK,
    )?;
    let is_spacing_mark = generate_function(
        "is_spacing_mark",
        crate::generated::general_category::SPACING_MARK,
    )?;

    println!("Generating functions to {syntax_src:?}");
    std::fs::write(
        syntax_src,
        format!(
            r#"// =============================================================================
//                          DO NOT HAND EDIT THIS FILE!
// -----------------------------------------------------------------------------
// This file was generated by `typed-ident-generator generate`. Please use
// that utility to update this file (see generator/README.md for more info.)
// -----------------------------------------------------------------------------
//                               Unicode {major}.{minor}.{patch}
// =============================================================================

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[cfg(not(miri))]
#[path = "generated.tests.rs"]
mod tests;

// =============================================================================
// CONSTANTS
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "unicode")]
pub const UNICODE_VERSION: (u32, u32, u32) = ({major}, {minor}, {patch});

// =============================================================================
// FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
{in_range}

/// Returns `true` if the character provided is `Titlecase`.
///
/// Titlecase characters are digraph characters in which the same code point
/// starts uppercase, but ends lowercase semantically.
///
/// For instance: ǅ, ǈ, ǋ, ǲ
{is_titlecase}

/// Returns `true` if the titlecase character provided is `Greek`.
///
/// This only has meaning if you already know that the character provided is
/// titlecase, it has no meaning if that is not the case.
///
/// Greek titlecase characters are important exceptions of the set of titlecase
/// characters, whereby the code point still semantically starts uppercase and
/// ends lowercase, but the code point visually appears entirely uppercase.
///
/// For instance: ᾈ, ᾨ, ῌ, ᾚ
{is_titlecase_greek_variant}

/// Returns true if the character is any one of the following:
///
/// * EnclosingMark
/// * SpacingMark
/// * NonspacingMark
#[cfg(feature = "unicode")]
{is_combining_mark}

// =============================================================================
// TEST FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
// A raw implementation of `EnclosingMark` for testing purposes.
// -----------------------------------------------------------------------------
#[cfg(test)]
#[cfg(not(miri))]
#[cfg(feature = "unicode")]
{is_enclosing_mark}

// -----------------------------------------------------------------------------
// A raw implementation of `NonspacingMark` for testing purposes.
// -----------------------------------------------------------------------------
#[cfg(test)]
#[cfg(not(miri))]
#[cfg(feature = "unicode")]
{is_nonspacing_mark}

// -----------------------------------------------------------------------------
// A raw implementation of `SpacingMark` for testing purposes.
// -----------------------------------------------------------------------------
#[cfg(test)]
#[cfg(not(miri))]
#[cfg(feature = "unicode")]
{is_spacing_mark}
"#,
            major = UNICODE_VERSION.0,
            minor = UNICODE_VERSION.1,
            patch = UNICODE_VERSION.2,
        ),
    )?;

    Ok(())
}
