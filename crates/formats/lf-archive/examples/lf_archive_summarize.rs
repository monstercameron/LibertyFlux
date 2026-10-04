//! Print a summary of one archive file: header, counts and entries.
//!
//! Usage:
//!
//! ```text
//! summarize <archive> [exe-with-key]
//! ```
//!
//! The optional second argument points at the game executable the archive
//! key is located in (run time only, memory only). Open archives need no
//! key. This tool only reads; it never writes game content anywhere.

use std::fs::File;
use std::io::BufReader;

use lf_archive::{AnyArchive, Archive, EntryKind, Key, crypto, open};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("usage: lf_archive_summarize <archive> [exe-with-key]");
        std::process::exit(2);
    }
    let key: Option<Key> = args.get(2).map(|exe| {
        crypto::load_key_from_exe(std::path::Path::new(exe)).unwrap_or_else(|e| {
            eprintln!("cannot locate archive key in {exe}: {e}");
            std::process::exit(1);
        })
    });

    let file = File::open(&args[1]).unwrap_or_else(|e| {
        eprintln!("cannot open {}: {e}", args[1]);
        std::process::exit(1);
    });
    let mut reader = BufReader::new(file);
    let archive = open(&mut reader, key.as_ref()).unwrap_or_else(|e| {
        eprintln!("cannot parse {}: {e}", args[1]);
        std::process::exit(1);
    });

    match &archive {
        AnyArchive::Rpf(a) => {
            let h = a.header();
            println!("format: RPF{}", h.version);
            println!("table encrypted: {}", h.toc_encrypted);
            println!("content encrypted: {}", h.content_encrypted);
            println!("records: {}", h.entry_count);
        }
        AnyArchive::Img(a) => {
            let h = a.header();
            println!("format: IMG v3");
            println!("table encrypted: {}", h.encrypted);
            println!("entries: {}", h.entry_count);
        }
    }

    let entries = archive.entries();
    let files = entries.iter().filter(|e| e.kind == EntryKind::File).count();
    let dirs = entries.len() - files;
    let bytes: u64 = entries.iter().map(|e| e.size).sum();
    let stored: u64 = entries.iter().map(|e| e.stored_size).sum();
    let resource = entries.iter().filter(|e| e.resource.is_some()).count();
    let compressed = entries.iter().filter(|e| e.compressed).count();
    println!("files: {files}  dirs: {dirs}  resource: {resource}  compressed: {compressed}");
    println!("logical bytes: {bytes}  stored bytes: {stored}");
    println!("--- entries ---");
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
        println!("{what}  {}", entry.path);
    }
}
