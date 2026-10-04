//! `save`: save game containers (`lf-save`).
//!
//! - `summarize <save-file>`: header, block table, checksum and end block
//!   (the former `lf_save_summarize` example, same output).
//! - `list <save-file>`: one block per line: index, name, offset, size.

use lf_save::{BlockKind, SaveFile};

use crate::cli::{CliError, CliResult, Io, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list"];

const USAGE: &str = "lf-inspect save summarize|list <path-to-save-file>";

/// Run one `save` subcommand.
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
    let save =
        SaveFile::parse(&bytes).map_err(|e| CliError::failure(format!("parse error: {e}")))?;
    let o = &mut *io.out;
    if cmd == "list" {
        for b in save.blocks() {
            writeln!(
                o,
                "{} {} {} {}",
                b.index,
                b.kind.name().unwrap_or("?"),
                b.offset,
                b.total_len
            )?;
        }
        return Ok(());
    }
    let header = save.header();
    writeln!(o, "file bytes: {}", save.len())?;
    writeln!(o, "version: {}", header.version)?;
    writeln!(o, "stored size: {}", header.file_size)?;
    writeln!(o, "globals size: {}", header.globals_size)?;
    writeln!(o, "mission: {}", header.mission())?;
    writeln!(o, "blocks: {}", save.blocks().len())?;
    if save.blocks().len() != BlockKind::COUNT {
        writeln!(
            o,
            "warning: expected {} blocks, found {}",
            BlockKind::COUNT,
            save.blocks().len()
        )?;
    }
    writeln!(o, "index name              offset     size  payload  class")?;
    for block in save.blocks() {
        let name = block.kind.name().unwrap_or("<past documented range>");
        let class = match save.classify(block) {
            lf_save::Payload::Unknown => "raw".to_string(),
            lf_save::Payload::Resource { kind, codec } => {
                format!("rsc5 kind={kind:#x} codec={codec:#x}")
            }
            lf_save::Payload::Rpf => "rpf".to_string(),
            lf_save::Payload::Img => "img".to_string(),
        };
        writeln!(
            o,
            "{:>5} {:<17} {:>8} {:>8} {:>8}  {class}",
            block.index,
            name,
            block.offset,
            block.total_len,
            block.payload_len(),
        )?;
        if let Some(fixed) = block.kind.documented_len()
            && fixed != block.total_len
        {
            writeln!(
                o,
                "  warning: documented size is {fixed}, file holds {}",
                block.total_len
            )?;
        }
    }
    match save.checksum() {
        Some(c) => {
            let verdict = match save.verify_checksum() {
                Some(true) => "match",
                Some(false) => "MISMATCH",
                None => "unreachable",
            };
            writeln!(
                o,
                "checksum at {}: stored {:#010x} ({verdict})",
                c.offset, c.stored
            )?;
        }
        None => writeln!(o, "checksum: absent")?,
    }
    match save.end() {
        Some(e) => writeln!(o, "end block at {}: word {:#x}", e.offset, e.word)?,
        None => writeln!(o, "end block: absent")?,
    }
    if !save.trailing_bytes().is_empty() {
        writeln!(o, "trailing bytes: {}", save.trailing_bytes().len())?;
    }
    Ok(())
}
