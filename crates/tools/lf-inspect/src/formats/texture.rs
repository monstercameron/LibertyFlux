//! `texture`: texture dictionaries, `.wtd` (`lf-texture`).
//!
//! - `summarize <file.wtd>`: names, formats, sizes (the former
//!   `wtd_summary` example, same output).
//! - `list <file.wtd>`: one texture name per line.
//! - `dump <file.wtd>`: every texture record as JSON-ish text.

use lf_texture::{Dictionary, level_byte_size, level_dims};

use crate::cli::{CliError, CliResult, Io, json_str, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list", "dump"];

const USAGE: &str = "lf-inspect texture summarize|list|dump <file.wtd>";

/// Run one `texture` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the file
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if !COMMANDS.contains(&cmd) {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let path = one_path(args, USAGE)?;
    let bytes = read_file(path)?;
    let dict =
        Dictionary::parse(&bytes).map_err(|e| CliError::failure(format!("wtd_summary: {e}")))?;
    let o = &mut *io.out;
    match cmd {
        "list" => {
            for e in dict.entries() {
                writeln!(o, "{}", e.name)?;
            }
        }
        "dump" => {
            writeln!(o, "[")?;
            for (i, e) in dict.entries().iter().enumerate() {
                let r = &e.record;
                let sep = if i + 1 < dict.len() { "," } else { "" };
                writeln!(
                    o,
                    "  {{\"name\": {}, \"hash\": {}, \"width\": {}, \"height\": {}, \"format\": {}, \"levels\": {}, \"stride\": {}, \"data_offset\": {}}}{sep}",
                    json_str(&e.name),
                    e.hash,
                    r.width,
                    r.height,
                    json_str(&r.format.name()),
                    r.levels,
                    r.stride,
                    r.data_offset
                )?;
            }
            writeln!(o, "]")?;
        }
        _ => {
            let res = dict.resource();
            writeln!(o, "file: {path}")?;
            writeln!(o, "file bytes: {}", bytes.len())?;
            writeln!(o, "resource type: {}", res.header.resource_type)?;
            writeln!(
                o,
                "segments: system {} bytes, graphics {} bytes",
                res.system.len(),
                res.graphics.len()
            )?;
            writeln!(o, "textures: {}", dict.len())?;
            for (i, entry) in dict.entries().iter().enumerate() {
                let r = &entry.record;
                let (w0, h0) = level_dims(r.width, r.height, 0);
                let top = match level_byte_size(r.format, r.width, r.height, 0) {
                    Ok(n) => n.to_string(),
                    Err(_) => "unknown-format".to_string(),
                };
                writeln!(
                    o,
                    "[{i}] hash {:#010x} {} ({}x{}, {}, {} mips, stride {}, top-level {} bytes)",
                    entry.hash, entry.name, w0, h0, r.format, r.levels, r.stride, top
                )?;
            }
        }
    }
    Ok(())
}
