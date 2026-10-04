//! `resource`: the RSC5 container (`lf-resource`).
//!
//! - `summarize <file>`: header, segment sizes, block map and pointer scan
//!   (the former `lf_resource_summary` example, same output).
//! - `dump <file>`: header fields and segment sizes as JSON-ish text.

use lf_resource::{Resource, Segment};

use crate::cli::{CliError, CliResult, Io, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "dump"];

/// Run one `resource` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the file
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    let path = one_path(args, "lf-inspect resource summarize|dump <resource-file>")?;
    let bytes = read_file(path)?;
    let res = Resource::parse(&bytes)
        .map_err(|e| CliError::failure(format!("cannot parse {path}: {e}")))?;
    match cmd {
        "summarize" => summarize(path, &bytes, &res, io),
        "dump" => {
            let h = res.header();
            writeln!(
                io.out,
                "{{\"kind\": {}, \"flags\": {}, \"codec\": {}, \"system\": {}, \"graphics\": {}}}",
                h.kind.raw(),
                h.flags,
                h.codec.raw(),
                res.system().len(),
                res.graphics().len()
            )?;
            Ok(())
        }
        _ => Err(CliError::usage(
            "usage: lf-inspect resource summarize|dump <resource-file>",
        )),
    }
}

fn summarize(path: &str, bytes: &[u8], res: &Resource, io: &mut Io) -> CliResult {
    let o = &mut *io.out;
    let header = res.header();
    writeln!(o, "file:      {path}")?;
    writeln!(o, "disk size: {} bytes", bytes.len())?;
    writeln!(o, "kind:      {}", header.kind)?;
    writeln!(o, "flags:     {:#010x}", header.flags)?;
    writeln!(o, "codec:     {:?}", header.codec)?;
    writeln!(o, "system:    {} bytes", res.system().len())?;
    writeln!(o, "graphics:  {} bytes", res.graphics().len())?;
    match res.pg_base() {
        Ok(pg) => {
            writeln!(
                o,
                "pgBase:    vtable={:#010x} blockmap={}",
                pg.vtable, pg.block_map
            )?;
            match res.block_map() {
                Ok(bm) => writeln!(o, "blockmap:  {:?} target={}", bm.state, bm.target)?,
                Err(e) => writeln!(o, "blockmap:  error: {e}")?,
            }
        }
        Err(e) => writeln!(o, "pgBase:    unreadable: {e}")?,
    }
    for (seg, _) in res.segments() {
        let n = res.scan_pointers(seg).count();
        let name = match seg {
            Segment::System => "system",
            Segment::Graphics => "graphics",
        };
        writeln!(o, "plausible pointers in {name}: {n}")?;
    }
    Ok(())
}
