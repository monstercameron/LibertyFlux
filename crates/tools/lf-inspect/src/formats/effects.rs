//! `effects`: effect packages (`.wpfl`), `*Fx.dat` tables and emitter XML
//! (`lf-effects`).
//!
//! - `summarize <file>`: banks, classes and names, or tables, or properties
//!   (the former `lf_effects_summary` example, same output).
//! - `list <file.wpfl>`: one line per bank entry: slot, hash, resolved name.
//! - `census <game-dir>`: JSON census of every effect file under a folder
//!   (the former `lf_effects_census` example, same output).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::cli::{CliError, CliResult, Io, json_str, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list", "census"];

const USAGE: &str =
    "lf-inspect effects summarize|list <effect-file> | lf-inspect effects census <game-dir>";

/// Run one `effects` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when a file
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    match cmd {
        "census" => census(one_path(args, USAGE)?, io),
        "list" => {
            let bytes = read_file(one_path(args, USAGE)?)?;
            let pkg = lf_effects::WpflFile::parse(&bytes)
                .map_err(|e| CliError::failure(format!("parse error: {e}")))?;
            for (slot, entry) in pkg.entries() {
                writeln!(
                    io.out,
                    "{slot} {:#010x} {}",
                    entry.hash,
                    pkg.name_of(entry.hash).unwrap_or("?")
                )?;
            }
            Ok(())
        }
        "summarize" => {
            let bytes = read_file(one_path(args, USAGE)?)?;
            if bytes.starts_with(b"RSC\x05") {
                summarize_wpfl(&bytes, io)
            } else if let Ok(text) = std::str::from_utf8(&bytes) {
                if text.trim_start().starts_with("<?xml") {
                    summarize_xml(text, io)
                } else {
                    summarize_dat(text, io)
                }
            } else {
                Err(CliError::failure(
                    "unknown file kind (not RSC, xml or text)",
                ))
            }
        }
        _ => Err(CliError::usage(format!("usage: {USAGE}"))),
    }
}

fn summarize_wpfl(bytes: &[u8], io: &mut Io) -> CliResult {
    let pkg = lf_effects::WpflFile::parse(bytes)
        .map_err(|e| CliError::failure(format!("parse error: {e}")))?;
    let o = &mut *io.out;
    writeln!(o, "kind: {:#x}", pkg.kind())?;
    writeln!(o, "banks: {}", pkg.banks().len())?;
    writeln!(o, "entries: {}", pkg.entry_count())?;
    writeln!(o, "resolved names: {}", pkg.resolved_count())?;
    for bank in pkg.banks() {
        let mut classes: BTreeMap<u32, usize> = BTreeMap::new();
        for entry in bank.entries() {
            *classes.entry(entry.vtable).or_insert(0) += 1;
        }
        let classes: Vec<String> = classes
            .iter()
            .map(|(vtable, n)| format!("{vtable:#010x}x{n}"))
            .collect();
        writeln!(
            o,
            "bank slot {}: vtable={:#010x} entries={} classes={}",
            bank.slot,
            bank.vtable,
            bank.len(),
            classes.join(", ")
        )?;
    }
    // Show a few resolved names as a sanity check (at most five).
    let mut shown = 0;
    for (slot, entry) in pkg.entries() {
        if shown >= 5 {
            break;
        }
        if let Some(name) = pkg.name_of(entry.hash) {
            writeln!(o, "slot {slot} entry {:#010x}: {name}", entry.hash)?;
            shown += 1;
        }
    }
    Ok(())
}

fn summarize_dat(text: &str, io: &mut Io) -> CliResult {
    let file = lf_effects::FxFile::parse(text)
        .map_err(|e| CliError::failure(format!("parse error: {e}")))?;
    writeln!(io.out, "version: {}", file.version)?;
    writeln!(io.out, "tables: {}", file.tables.len())?;
    for table in &file.tables {
        writeln!(
            io.out,
            "table {}: rows={} width={:?}",
            table.name,
            table.len(),
            table.width()
        )?;
    }
    Ok(())
}

fn summarize_xml(text: &str, io: &mut Io) -> CliResult {
    let file = lf_effects::emitter::parse(text)
        .map_err(|e| CliError::failure(format!("parse error: {e}")))?;
    writeln!(io.out, "root: {}", file.root)?;
    writeln!(io.out, "properties: {}", file.props.len())?;
    for prop in &file.props {
        let attrs: Vec<String> = prop.attrs.iter().map(|(k, v)| format!("{k}={v}")).collect();
        writeln!(io.out, "{}: {}", prop.name, attrs.join(" "))?;
    }
    Ok(())
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.is_file() {
            out.push(path);
        }
    }
}

fn read_text(path: &Path) -> Result<String, CliError> {
    std::fs::read_to_string(path)
        .map_err(|e| CliError::failure(format!("cannot read {}: {e}", path.display())))
}

fn census(dir: &str, io: &mut Io) -> CliResult {
    let root = Path::new(dir);
    let mut files = Vec::new();
    collect(root, &mut files);
    files.sort();
    let rel = |path: &Path| {
        path.strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    };
    let bad = |path: &Path, e: &dyn std::fmt::Display| {
        CliError::failure(format!("{}: {e}", path.display()))
    };
    let mut wpfl_json = Vec::new();
    let mut dat_json = Vec::new();
    let mut xml_json = Vec::new();
    for path in &files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if ext == "wpfl" {
            let bytes = std::fs::read(path).map_err(|e| bad(path, &e))?;
            let pkg = lf_effects::WpflFile::parse(&bytes).map_err(|e| bad(path, &e))?;
            let banks: Vec<String> = pkg
                .banks()
                .iter()
                .map(|b| {
                    let resolved = b
                        .entries()
                        .iter()
                        .filter(|e| pkg.name_of(e.hash).is_some())
                        .count();
                    format!(
                        "{{\"slot\":{},\"vtable\":\"{:#010x}\",\"entries\":{},\"resolved\":{}}}",
                        b.slot,
                        b.vtable,
                        b.len(),
                        resolved
                    )
                })
                .collect();
            wpfl_json.push(format!(
                "{{\"file\":{},\"kind\":\"{:#04x}\",\"banks\":[{}]}}",
                json_str(name),
                pkg.kind(),
                banks.join(",")
            ));
        } else if ext == "dat" && name.ends_with("Fx.dat") {
            let text = read_text(path)?;
            let file = lf_effects::FxFile::parse(&text).map_err(|e| bad(path, &e))?;
            let tables: Vec<String> = file
                .tables
                .iter()
                .map(|t| {
                    format!(
                        "{{\"name\":{},\"rows\":{},\"width\":{}}}",
                        json_str(&t.name),
                        t.len(),
                        t.width().unwrap_or(0)
                    )
                })
                .collect();
            dat_json.push(format!(
                "{{\"file\":{},\"version\":{},\"tables\":[{}]}}",
                json_str(&rel(path)),
                json_str(&file.version),
                tables.join(",")
            ));
        } else if ext == "xml"
            && matches!(
                name,
                "gtaRainEmitter.xml"
                    | "gtaRainRender.xml"
                    | "gtaStormEmitter.xml"
                    | "gtaStormRender.xml"
            )
        {
            let text = read_text(path)?;
            let file = lf_effects::emitter::parse(&text).map_err(|e| bad(path, &e))?;
            xml_json.push(format!(
                "{{\"file\":{},\"root\":{},\"props\":{}}}",
                json_str(&rel(path)),
                json_str(&file.root),
                file.props.len()
            ));
        }
    }
    writeln!(
        io.out,
        "{{\"wpfl\":[{}],\"dat\":[{}],\"xml\":[{}]}}",
        wpfl_json.join(","),
        dat_json.join(","),
        xml_json.join(",")
    )?;
    Ok(())
}
