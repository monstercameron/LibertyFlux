//! `audio-config`: versioned audio metadata and speech files
//! (`lf-audio-config`).
//!
//! - `summarize <file>`: names, counts, sizes and decoded object types (the
//!   former `lf_audio_config_summarize` example, same output). Speech files
//!   are detected by the `speech` stem; anything else is parsed as a
//!   versioned container with the schema matching its name.
//! - `list <file>`: one object per line: name, type id, size.

use std::collections::BTreeMap;

use lf_audio_config::{
    container::MetaFile, decode::decode_object, schema::Schema, speech::SpeechFile,
};

use crate::cli::{CliError, CliResult, Io, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list"];

const USAGE: &str = "lf-inspect audio-config summarize|list <audio-metadata-file>";

/// Run one `audio-config` subcommand.
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
    let stem = path
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(path)
        .to_ascii_lowercase();
    if cmd == "list" {
        let file =
            MetaFile::parse(&bytes).map_err(|e| CliError::failure(format!("parse failed: {e}")))?;
        for o in file.objects() {
            writeln!(
                io.out,
                "{} type={} size={}",
                o.name(),
                o.type_id(),
                o.size()
            )?;
        }
        return Ok(());
    }
    if stem.contains("speech") {
        summarize_speech(path, &bytes, io)
    } else {
        summarize_versioned(path, &stem, &bytes, io)
    }
}

fn summarize_versioned(path: &str, stem: &str, bytes: &[u8], io: &mut Io) -> CliResult {
    let file =
        MetaFile::parse(bytes).map_err(|e| CliError::failure(format!("parse failed: {e}")))?;
    let o = &mut *io.out;
    writeln!(o, "file: {path}")?;
    writeln!(o, "size: {} bytes", bytes.len())?;
    writeln!(o, "suffix: {}", file.suffix())?;
    writeln!(o, "blob: {} bytes", file.blob().len())?;
    writeln!(o, "archives: {}", file.archives().len())?;
    for a in file.archives().iter().take(8) {
        writeln!(o, "  archive: {}", a.name())?;
    }
    if file.archives().len() > 8 {
        writeln!(o, "  ... and {} more", file.archives().len() - 8)?;
    }
    writeln!(o, "objects: {}", file.objects().len())?;
    writeln!(o, "hash relocations: {}", file.hash_offsets().len())?;
    writeln!(o, "archive relocations: {}", file.archive_offsets().len())?;
    let Some(schema) = Schema::detect(stem) else {
        writeln!(o, "no schema for this file name; directory only")?;
        return Ok(());
    };
    if schema.suffix() != file.suffix() {
        writeln!(
            o,
            "warning: schema expects suffix {}, file has {}",
            schema.suffix(),
            file.suffix()
        )?;
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
                    writeln!(o, "decode error for {}: {e}", entry.name())?;
                }
            }
        }
    }
    writeln!(o, "object types:")?;
    for ((id, name), count) in &hist {
        writeln!(o, "  id {id:3} {}: {count}", name.unwrap_or("<unknown>"))?;
    }
    writeln!(o, "decode errors: {decode_errors}")?;
    writeln!(o, "objects with trailing bytes: {}", trailing.len())?;
    for (name, id, len) in trailing.iter().take(10) {
        writeln!(o, "  {name} (type {id}): {len} trailing bytes")?;
    }
    Ok(())
}

fn summarize_speech(path: &str, bytes: &[u8], io: &mut Io) -> CliResult {
    let file =
        SpeechFile::parse(bytes).map_err(|e| CliError::failure(format!("parse failed: {e}")))?;
    let o = &mut *io.out;
    writeln!(o, "file: {path}")?;
    writeln!(o, "size: {} bytes", bytes.len())?;
    writeln!(o, "variation blob: {} bytes", file.variation_data().len())?;
    writeln!(o, "contexts: {}", file.contexts().len())?;
    writeln!(o, "voices: {}", file.voices().len())?;
    writeln!(o, "banks: {}", file.banks().len())?;
    for b in file.banks().iter().take(8) {
        writeln!(o, "  bank: {b}")?;
    }
    if file.banks().len() > 8 {
        writeln!(o, "  ... and {} more", file.banks().len() - 8)?;
    }
    Ok(())
}
