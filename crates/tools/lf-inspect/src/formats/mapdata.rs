//! `mapdata`: map definition and placement files (`lf-mapdata`).
//!
//! - `summarize <file.ide|file.ipl|file.wpl|gta.dat>`: record counts per
//!   section, errors and declared sections (the former
//!   `lf_mapdata_summarize` example, same output, including its warning on
//!   standard error for an unknown extension).

use std::collections::BTreeMap;

use lf_mapdata::{ide::IdeFile, ipl::IplFile, loadlist::LoadList, wpl::WplFile};

use crate::cli::{CliError, CliResult, Io, has_ext, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize"];

const USAGE: &str = "lf-inspect mapdata summarize <file>";

/// Run one `mapdata` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the file
/// cannot be read.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if cmd != "summarize" {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let path = one_path(args, USAGE)?;
    let bytes = read_file(path)?;
    writeln!(io.out, "file: {path}")?;
    writeln!(io.out, "size: {} bytes", bytes.len())?;
    let lower = path.to_ascii_lowercase();
    let o = &mut *io.out;
    if has_ext(&lower, "ide") {
        summarize_ide(o, &bytes)?;
    } else if has_ext(&lower, "ipl") {
        summarize_ipl(o, &bytes)?;
    } else if has_ext(&lower, "wpl") {
        summarize_wpl(o, &bytes)?;
    } else if has_ext(&lower, "dat") || has_ext(&lower, "txt") {
        summarize_loadlist(o, &bytes)?;
    } else {
        writeln!(io.err, "unknown extension; treating as load list")?;
        summarize_loadlist(o, &bytes)?;
    }
    Ok(())
}

fn summarize_ide(o: &mut dyn std::io::Write, bytes: &[u8]) -> std::io::Result<()> {
    match IdeFile::parse(bytes) {
        Ok(f) => {
            writeln!(o, "format: IDE")?;
            writeln!(o, "records: {}", f.len())?;
            let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
            kinds.insert("objs", f.objs.len());
            kinds.insert("tobj", f.tobj.len());
            kinds.insert("anim", f.anim.len());
            kinds.insert("tanm", f.tanm.len());
            kinds.insert("cars", f.cars.len());
            kinds.insert("peds", f.peds.len());
            kinds.insert("weap", f.weap.len());
            kinds.insert("txdp", f.txdp.len());
            kinds.insert("amat", f.amat.len());
            kinds.insert("agrps", f.agrps.len());
            kinds.insert("hier", f.hier.len());
            kinds.insert("mlo", f.mlo.len());
            kinds.insert("2dfx", f.fx.len());
            for (k, v) in &kinds {
                if *v > 0 {
                    writeln!(o, "  {k}: {v}")?;
                }
            }
            let mut fxkinds: BTreeMap<u32, usize> = BTreeMap::new();
            for e in &f.fx {
                *fxkinds.entry(e.kind).or_default() += 1;
            }
            if !fxkinds.is_empty() {
                writeln!(o, "  2dfx kinds: {fxkinds:?}")?;
            }
            writeln!(o, "errors: {}", f.errors.len())?;
            for e in f.errors.iter().take(10) {
                writeln!(o, "  line {} [{}]: {}", e.line, e.section, e.error)?;
            }
            for u in &f.unknown_sections {
                writeln!(o, "  unknown section '{}': {} records", u.name, u.records)?;
            }
        }
        Err(e) => writeln!(o, "parse failed: {e}")?,
    }
    Ok(())
}

fn summarize_ipl(o: &mut dyn std::io::Write, bytes: &[u8]) -> std::io::Result<()> {
    match IplFile::parse(bytes) {
        Ok(f) => {
            writeln!(o, "format: IPL (text)")?;
            writeln!(o, "records: {}", f.len())?;
            writeln!(o, "  blok: {}", f.blok.len())?;
            writeln!(o, "  cull: {}", f.cull.len())?;
            writeln!(o, "  occl: {}", f.occl.len())?;
            writeln!(o, "  vnod: {}", f.vnod.len())?;
            writeln!(o, "  link: {}", f.link.len())?;
            writeln!(o, "  2dfx: {}", f.fx.len())?;
            writeln!(o, "declared sections: {}", f.declared.join(","))?;
            writeln!(o, "errors: {}", f.errors.len())?;
            for e in f.errors.iter().take(10) {
                writeln!(o, "  line {} [{}]: {}", e.line, e.section, e.error)?;
            }
        }
        Err(e) => writeln!(o, "parse failed: {e}")?,
    }
    Ok(())
}

fn summarize_wpl(o: &mut dyn std::io::Write, bytes: &[u8]) -> std::io::Result<()> {
    match WplFile::parse(bytes) {
        Ok(f) => {
            writeln!(o, "format: WPL (binary)")?;
            writeln!(o, "records: {}", f.len())?;
            writeln!(o, "  inst: {}", f.inst.len())?;
            writeln!(o, "  grge: {}", f.grge.len())?;
            writeln!(o, "  cars: {}", f.cars.len())?;
            writeln!(o, "  tcyc: {}", f.tcyc.len())?;
            writeln!(o, "  mlop: {}", f.mlop.len())?;
            writeln!(o, "  blok: {}", f.blok.len())?;
            writeln!(o, "  lodm: {}", f.lodm.len())?;
            writeln!(o, "  slow: {}", f.slow.len())?;
            writeln!(
                o,
                "trailing bytes: {} (all zero: {})",
                f.trailing_bytes.len(),
                f.trailing_is_zero_padding()
            )?;
        }
        Err(e) => writeln!(o, "parse failed: {e}")?,
    }
    Ok(())
}

fn summarize_loadlist(o: &mut dyn std::io::Write, bytes: &[u8]) -> std::io::Result<()> {
    match LoadList::parse(bytes) {
        Ok(l) => {
            writeln!(o, "format: load list")?;
            writeln!(o, "directives: {}", l.directives.len())?;
            let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
            for d in &l.directives {
                *kinds.entry(d.keyword.clone()).or_default() += 1;
            }
            for (k, v) in &kinds {
                writeln!(o, "  {k}: {v}")?;
            }
        }
        Err(e) => writeln!(o, "parse failed: {e}")?,
    }
    Ok(())
}
