//! Print a summary of one texture dictionary: names, counts, sizes.
//!
//! Usage: `wtd_summary <file.wtd>`
//!
//! Reads the file, parses it, and prints to stdout only. It never writes
//! game content anywhere.

use std::path::PathBuf;

use lf_texture::{Dictionary, level_byte_size, level_dims};

fn main() {
    if let Err(e) = run() {
        eprintln!("wtd_summary: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), lf_texture::Error> {
    let Some(arg) = std::env::args_os().nth(1) else {
        eprintln!("usage: wtd_summary <file.wtd>");
        std::process::exit(2);
    };
    let path = PathBuf::from(arg);
    let bytes = std::fs::read(&path).map_err(lf_texture::Error::from)?;
    let dict = Dictionary::parse(&bytes)?;
    let res = dict.resource();
    println!("file: {}", path.display());
    println!("file bytes: {}", bytes.len());
    println!("resource type: {}", res.header.resource_type);
    println!(
        "segments: system {} bytes, graphics {} bytes",
        res.system.len(),
        res.graphics.len()
    );
    println!("textures: {}", dict.len());
    for (i, entry) in dict.entries().iter().enumerate() {
        let r = &entry.record;
        let (w0, h0) = level_dims(r.width, r.height, 0);
        let top = match level_byte_size(r.format, r.width, r.height, 0) {
            Ok(n) => n.to_string(),
            Err(_) => "unknown-format".to_string(),
        };
        println!(
            "[{i}] hash {:#010x} {} ({}x{}, {}, {} mips, stride {}, top-level {} bytes)",
            entry.hash, entry.name, w0, h0, r.format, r.levels, r.stride, top
        );
    }
    Ok(())
}
