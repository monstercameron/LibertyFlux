//! `archive`: RPF and IMG containers (`lf-archive`).
//!
//! - `summarize <archive> [exe-with-key]`: header, counts and entries
//!   (the former `lf_archive_summarize` example, same output).
//! - `walk <game-dir> <exe-with-key>`: one JSON census line per archive
//!   (the former `lf_archive_walk` example, same output).
//! - `list <archive> [exe-with-key]`: one line per entry: kind, size, path.
//! - `dump <archive> [exe-with-key]`: the parsed entries as JSON-ish text.
//!
//! The optional executable supplies the table key; it is located at run
//! time and kept in memory only. Open archives need no key.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use lf_archive::{AnyArchive, Archive, EntryKind, Key, open};

use crate::cli::{CliError, CliResult, Io, json_str, load_key};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "walk", "list", "dump"];

const USAGE: &str = "lf-inspect archive summarize|list|dump <archive> [exe-with-key]";

/// Run one `archive` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the archive
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    match cmd {
        "walk" => walk(args, io),
        "summarize" | "list" | "dump" => {
            let (archive, _) = open_args(args)?;
            match cmd {
                "summarize" => summarize(&archive, io),
                "list" => list(&archive, io),
                _ => dump(&archive, io),
            }
        }
        _ => Err(CliError::usage(format!("usage: {USAGE}"))),
    }
}

/// Open `<archive> [exe]` from the arguments.
fn open_args(args: &[String]) -> Result<(AnyArchive, Option<Key>), CliError> {
    if args.is_empty() || args.len() > 2 {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let key = args.get(1).map(|exe| load_key(exe)).transpose()?;
    let file = File::open(&args[0])
        .map_err(|e| CliError::failure(format!("cannot open {}: {e}", args[0])))?;
    let mut reader = BufReader::new(file);
    let archive = open(&mut reader, key.as_ref())
        .map_err(|e| CliError::failure(format!("cannot parse {}: {e}", args[0])))?;
    Ok((archive, key))
}

fn summarize(archive: &AnyArchive, io: &mut Io) -> CliResult {
    let o = &mut *io.out;
    match archive {
        AnyArchive::Rpf(a) => {
            let h = a.header();
            writeln!(o, "format: RPF{}", h.version)?;
            writeln!(o, "table encrypted: {}", h.toc_encrypted)?;
            writeln!(o, "content encrypted: {}", h.content_encrypted)?;
            writeln!(o, "records: {}", h.entry_count)?;
        }
        AnyArchive::Img(a) => {
            let h = a.header();
            writeln!(o, "format: IMG v3")?;
            writeln!(o, "table encrypted: {}", h.encrypted)?;
            writeln!(o, "entries: {}", h.entry_count)?;
        }
    }
    let entries = archive.entries();
    let files = entries.iter().filter(|e| e.kind == EntryKind::File).count();
    let dirs = entries.len() - files;
    let bytes: u64 = entries.iter().map(|e| e.size).sum();
    let stored: u64 = entries.iter().map(|e| e.stored_size).sum();
    let resource = entries.iter().filter(|e| e.resource.is_some()).count();
    let compressed = entries.iter().filter(|e| e.compressed).count();
    writeln!(
        o,
        "files: {files}  dirs: {dirs}  resource: {resource}  compressed: {compressed}"
    )?;
    writeln!(o, "logical bytes: {bytes}  stored bytes: {stored}")?;
    writeln!(o, "--- entries ---")?;
    for entry in entries {
        let what = match entry.kind {
            EntryKind::File => {
                let mut tags = String::new();
                if entry.resource.is_some() {
                    tags.push('R');
                }
                if entry.compressed {
                    tags.push('C');
                }
                format!("file {tags:2} {:>10} @{}", entry.size, entry.offset)
            }
            EntryKind::Directory => "dir".to_string(),
        };
        writeln!(o, "{what}  {}", entry.path)?;
    }
    Ok(())
}

fn list(archive: &AnyArchive, io: &mut Io) -> CliResult {
    for e in archive.entries() {
        let kind = if e.is_dir() { "dir " } else { "file" };
        writeln!(io.out, "{kind} {:>10} {}", e.size, e.path)?;
    }
    Ok(())
}

fn dump(archive: &AnyArchive, io: &mut Io) -> CliResult {
    let o = &mut *io.out;
    let kind = match archive {
        AnyArchive::Rpf(a) => format!("RPF{}", a.header().version),
        AnyArchive::Img(_) => "IMG3".to_string(),
    };
    writeln!(o, "{{\"kind\": {}, \"entries\": [", json_str(&kind))?;
    let entries = archive.entries();
    for (i, e) in entries.iter().enumerate() {
        let resource = e.resource.map_or("null".to_string(), |r| {
            format!("{{\"type\": {}, \"flags\": {}}}", r.type_id, r.flags)
        });
        let sep = if i + 1 < entries.len() { "," } else { "" };
        writeln!(
            o,
            "  {{\"path\": {}, \"dir\": {}, \"size\": {}, \"stored\": {}, \"offset\": {}, \"compressed\": {}, \"resource\": {resource}}}{sep}",
            json_str(&e.path),
            e.is_dir(),
            e.size,
            e.stored_size,
            e.offset,
            e.compressed,
        )?;
    }
    writeln!(o, "]}}")?;
    Ok(())
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn walk(args: &[String], io: &mut Io) -> CliResult {
    let [dir, exe] = args else {
        return Err(CliError::usage(
            "usage: lf-inspect archive walk <game-dir> <exe-with-key>",
        ));
    };
    let dir = Path::new(dir);
    let key = load_key(exe)?;
    let mut paths = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(top) = stack.pop() {
        let entries = std::fs::read_dir(&top)
            .map_err(|e| CliError::failure(format!("cannot list {}: {e}", top.display())))?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str())
                && (ext.eq_ignore_ascii_case("rpf") || ext.eq_ignore_ascii_case("img"))
            {
                paths.push(path);
            }
        }
    }
    paths.sort();
    for path in &paths {
        let rel = path.strip_prefix(dir).unwrap_or(path);
        let rel = rel.display().to_string().replace('/', "\\");
        let file = File::open(path)
            .map_err(|e| CliError::failure(format!("cannot open {}: {e}", path.display())))?;
        let mut reader = BufReader::new(file);
        match open(&mut reader, Some(&key)) {
            Ok(archive) => {
                let kind = match &archive {
                    AnyArchive::Rpf(a) => format!("RPF{}", a.header().version),
                    AnyArchive::Img(_) => "IMG3".to_string(),
                };
                let entries = archive.entries();
                let files = entries.iter().filter(|e| e.kind == EntryKind::File).count();
                let resource = entries.iter().filter(|e| e.resource.is_some()).count();
                let compressed = entries.iter().filter(|e| e.compressed).count();
                let bytes: u64 = entries.iter().map(|e| e.size).sum();
                writeln!(
                    io.out,
                    "{{\"file\":\"{}\",\"kind\":\"{kind}\",\"entries\":{},\"files\":{files},\"resource\":{resource},\"compressed\":{compressed},\"bytes\":{bytes},\"error\":null}}",
                    escape(&rel),
                    entries.len(),
                )?;
            }
            Err(e) => {
                writeln!(
                    io.out,
                    "{{\"file\":\"{}\",\"kind\":null,\"entries\":0,\"files\":0,\"resource\":0,\"compressed\":0,\"bytes\":0,\"error\":\"{}\"}}",
                    escape(&rel),
                    escape(&e.to_string()),
                )?;
            }
        }
    }
    Ok(())
}
