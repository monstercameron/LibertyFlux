//! Walk a game folder and print one JSON census line per archive.
//!
//! Usage:
//!
//! ```text
//! walk <game-dir> <exe-with-key>
//! ```
//!
//! Each line is a JSON object with the archive path (relative),
//! container kind, entry counts and any parse error. Read-only.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use lf_archive::{AnyArchive, Archive, EntryKind, open};

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: lf_archive_walk <game-dir> <exe-with-key>");
        std::process::exit(2);
    }
    let dir = Path::new(&args[1]);
    let key = lf_archive::crypto::load_key_from_exe(Path::new(&args[2])).unwrap_or_else(|e| {
        eprintln!("cannot locate archive key in {}: {e}", args[2]);
        std::process::exit(1);
    });

    let mut paths = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(top) = stack.pop() {
        for entry in std::fs::read_dir(&top).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("rpf") || ext.eq_ignore_ascii_case("img") {
                    paths.push(path);
                }
            }
        }
    }
    paths.sort();

    for path in &paths {
        let rel = path.strip_prefix(dir).unwrap_or(path);
        let rel = rel.display().to_string().replace('/', "\\");
        let file = File::open(path).unwrap();
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
                println!(
                    "{{\"file\":\"{}\",\"kind\":\"{kind}\",\"entries\":{},\"files\":{files},\"resource\":{resource},\"compressed\":{compressed},\"bytes\":{bytes},\"error\":null}}",
                    escape(&rel),
                    entries.len(),
                );
            }
            Err(e) => {
                println!(
                    "{{\"file\":\"{}\",\"kind\":null,\"entries\":0,\"files\":0,\"resource\":0,\"compressed\":0,\"bytes\":0,\"error\":\"{}\"}}",
                    escape(&rel),
                    escape(&e.to_string()),
                );
            }
        }
    }
}
