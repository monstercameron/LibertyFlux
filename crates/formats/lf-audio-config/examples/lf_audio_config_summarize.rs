//! Print a summary of one audio metadata file: names, counts, sizes.
//!
//! Usage: `lf_audio_config_summarize <path-to-dat-file>`. Reads the file, never writes.
//! Speech files are detected by the `speech` stem; anything else is parsed
//! as a versioned container with the schema matching its name.

use lf_audio_config::{
    container::MetaFile, decode::decode_object, schema::Schema, speech::SpeechFile,
};
use std::collections::BTreeMap;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_audio_config_summarize <audio-metadata-file>");
        std::process::exit(2);
    });
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    let stem = path
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(&path)
        .to_ascii_lowercase();
    if stem.contains("speech") {
        summarize_speech(&path, &bytes);
    } else {
        summarize_versioned(&path, &stem, &bytes);
    }
}

fn summarize_versioned(path: &str, stem: &str, bytes: &[u8]) {
    let file = MetaFile::parse(bytes).unwrap_or_else(|e| {
        eprintln!("parse failed: {e}");
        std::process::exit(1);
    });
    println!("file: {path}");
    println!("size: {} bytes", bytes.len());
    println!("suffix: {}", file.suffix());
    println!("blob: {} bytes", file.blob().len());
    println!("archives: {}", file.archives().len());
    for a in file.archives().iter().take(8) {
        println!("  archive: {}", a.name());
    }
    if file.archives().len() > 8 {
        println!("  ... and {} more", file.archives().len() - 8);
    }
    println!("objects: {}", file.objects().len());
    println!("hash relocations: {}", file.hash_offsets().len());
    println!("archive relocations: {}", file.archive_offsets().len());

    let Some(schema) = Schema::detect(stem) else {
        println!("no schema for this file name; directory only");
        return;
    };
    if schema.suffix() != file.suffix() {
        println!(
            "warning: schema expects suffix {}, file has {}",
            schema.suffix(),
            file.suffix()
        );
    }
    let mut hist: BTreeMap<(u8, Option<&str>), usize> = BTreeMap::new();
    let mut decode_errors = 0usize;
    let mut trailing: Vec<(&str, u8, usize)> = Vec::new();
    for entry in file.objects() {
        match decode_object(&schema, entry) {
            Ok(obj) => {
                *hist.entry((obj.type_id, obj.type_name)).or_insert(0) += 1;
                if !obj.trailing.is_empty() {
                    trailing.push((entry.name(), obj.type_id, obj.trailing.len()));
                }
            }
            Err(e) => {
                decode_errors += 1;
                if decode_errors <= 5 {
                    println!("decode error for {}: {e}", entry.name());
                }
            }
        }
    }
    println!("object types:");
    for ((id, name), count) in &hist {
        println!("  id {id:3} {}: {count}", name.unwrap_or("<unknown>"));
    }
    println!("decode errors: {decode_errors}");
    println!("objects with trailing bytes: {}", trailing.len());
    for (name, id, len) in trailing.iter().take(10) {
        println!("  {name} (type {id}): {len} trailing bytes");
    }
}

fn summarize_speech(path: &str, bytes: &[u8]) {
    let file = SpeechFile::parse(bytes).unwrap_or_else(|e| {
        eprintln!("parse failed: {e}");
        std::process::exit(1);
    });
    println!("file: {path}");
    println!("size: {} bytes", bytes.len());
    println!("variation blob: {} bytes", file.variation_data().len());
    println!("contexts: {}", file.contexts().len());
    println!("voices: {}", file.voices().len());
    println!("banks: {}", file.banks().len());
    for b in file.banks().iter().take(8) {
        println!("  bank: {b}");
    }
    if file.banks().len() > 8 {
        println!("  ... and {} more", file.banks().len() - 8);
    }
}
