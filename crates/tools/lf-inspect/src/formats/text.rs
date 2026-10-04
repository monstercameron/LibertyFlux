//! `text`: the GXT text database and the font and front-end files
//! (`lf-text`).
//!
//! - `summarize <path>`: names, counts and sizes, chosen by file name
//!   (the former `lf_text_summary` example, same output). GXT string
//!   contents are never written out.
//! - `list <file.gxt>`: one table per line: name and entry count.

use std::path::Path;

use lf_text::{FontFile, FrontendLayout, GxtFile, HudColours, HudFile, MenuFile, RadioHudFile};

use crate::cli::{CliError, CliResult, EXIT_USAGE, Io, has_ext, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list"];

const USAGE: &str = "lf-inspect text summarize <path> | lf-inspect text list <file.gxt>";

/// Run one `text` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments or an unknown file kind and a
/// failure when the file cannot be read or parsed.
// One branch per file kind, as in the original summary tool; splitting it
// would scatter the file-name dispatch.
#[allow(clippy::too_many_lines)]
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if !COMMANDS.contains(&cmd) {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let path = one_path(args, USAGE)?;
    let data = read_file(path)?;
    let parse_error = |e: &dyn std::fmt::Display| CliError::failure(format!("parse error: {e}"));
    if cmd == "list" {
        let file = GxtFile::parse(&data).map_err(|e| parse_error(&e))?;
        for t in file.tables() {
            writeln!(io.out, "{} {}", t.name, t.entries.len())?;
        }
        return Ok(());
    }
    let o = &mut *io.out;
    writeln!(o, "file: {path} ({} bytes)", data.len())?;
    let name = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    if has_ext(name, "gxt") {
        let file = GxtFile::parse(&data).map_err(|e| parse_error(&e))?;
        writeln!(
            o,
            "kind: GXT version={} bits-per-char={}",
            file.version(),
            file.bits_per_char()
        )?;
        writeln!(
            o,
            "tables: {} entries: {}",
            file.tables().len(),
            file.entry_count()
        )?;
        let mut names: Vec<&str> = file.tables().iter().map(|t| t.name.as_str()).collect();
        names.sort_unstable();
        let head: Vec<&&str> = names.iter().take(8).collect();
        writeln!(o, "table names (first 8 of {}): {head:?}", names.len())?;
        for table in file.tables().iter().take(3) {
            writeln!(o, "  {:8} entries: {}", table.name, table.entries.len())?;
        }
    } else if name.starts_with("fonts") {
        let file = FontFile::parse(&data).map_err(|e| parse_error(&e))?;
        writeln!(
            o,
            "kind: fonts.dat resolution={}x{}",
            file.resolution.0, file.resolution.1
        )?;
        writeln!(
            o,
            "buttons: {} radar_blip: {} fonts: {}",
            file.buttons.len(),
            file.radar_blip,
            file.fonts.len()
        )?;
        for font in &file.fonts {
            writeln!(
                o,
                "  id={} slots={} main={:?} sub1={:?} sub2={:?} common={:?} unprop={} spacing={:?} whitespace={}",
                font.id,
                font.map.len(),
                font.main,
                font.sub1,
                font.sub2,
                font.common,
                font.unprop,
                font.spacing,
                font.whitespace
            )?;
        }
    } else if name == "hud.dat" {
        let file = HudFile::parse(&data).map_err(|e| parse_error(&e))?;
        writeln!(o, "kind: hud.dat")?;
        for section in &file.sections {
            writeln!(o, "  [{}] items: {}", section.name, section.items.len())?;
        }
    } else if name.eq_ignore_ascii_case("hudcolor.dat") {
        let file = HudColours::parse(&data).map_err(|e| parse_error(&e))?;
        writeln!(o, "kind: hudColor.dat")?;
        for section in &file.sections {
            writeln!(o, "  [{}] colours: {}", section.name, section.colours.len())?;
        }
    } else if name.starts_with("frontend") && has_ext(name, "dat") {
        let file = FrontendLayout::parse(&data).map_err(|e| parse_error(&e))?;
        writeln!(o, "kind: frontend layout")?;
        for section in &file.sections {
            writeln!(o, "  [{}] rows: {}", section.name, section.values.len())?;
        }
    } else if name == "radiohud.dat" {
        match RadioHudFile::parse(&data).map_err(|e| parse_error(&e))? {
            RadioHudFile::Full(file) => {
                writeln!(o, "kind: radiohud.dat (full)")?;
                writeln!(
                    o,
                    "containers: {} stations: {}",
                    file.containers.len(),
                    file.stations.len()
                )?;
            }
            RadioHudFile::Simple(rows) => {
                writeln!(o, "kind: radiohud.dat (simple)")?;
                writeln!(o, "stations: {}", rows.len())?;
            }
        }
    } else if name == "frontend_menus.xml" {
        let file = MenuFile::parse(&data).map_err(|e| parse_error(&e))?;
        writeln!(o, "kind: frontend_menus.xml version={}", file.version)?;
        for section in &file.sections {
            let options: usize = section.menus.iter().map(|m| m.options.len()).sum();
            writeln!(
                o,
                "  {} menus: {} options: {options}",
                section.name,
                section.menus.len()
            )?;
        }
        writeln!(o, "labels referenced: {}", file.iter_labels().count())?;
    } else {
        return Err(CliError {
            code: EXIT_USAGE,
            message: format!("unknown file kind for {name}"),
        });
    }
    Ok(())
}
