//! `anim`: animation dictionaries, `.wad` (`lf-anim`).
//!
//! - `summarize <file.wad>` or `summarize <anim.img> <entry.wad> <exe>`:
//!   clips, codecs and validation (the former `lf_anim_summarize` example,
//!   same output). The archive form locates the key in the executable at
//!   run time (memory only).
//! - `list` (same arguments): one clip per line: name, frames, tracks.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;

use lf_anim::{AnimDictionary, ChannelData, Codec};

use crate::cli::{CliError, CliResult, Io, load_key, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list"];

const USAGE: &str = "lf-inspect anim summarize|list <anim.img> <entry.wad> <exe> | <file.wad>";

/// Run one `anim` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the input
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if !COMMANDS.contains(&cmd) {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let (bytes, label) = load(args)?;
    let dict = AnimDictionary::parse_bytes(&bytes)
        .map_err(|e| CliError::failure(format!("cannot parse {label}: {e}")))?;
    if cmd == "list" {
        for clip in dict.clips() {
            writeln!(
                io.out,
                "{} frames={} tracks={}",
                clip.short_name(),
                clip.frames(),
                clip.tracks().len()
            )?;
        }
        return Ok(());
    }
    summarize(&bytes, &label, &dict, io)
}

/// Read the dictionary bytes from a loose file or an archive member.
fn load(args: &[String]) -> Result<(Vec<u8>, String), CliError> {
    match args {
        [img, entry, exe] => {
            let key = load_key(exe)?;
            let file = File::open(img)
                .map_err(|e| CliError::failure(format!("cannot open {img}: {e}")))?;
            let mut reader = BufReader::new(file);
            let archive = lf_archive::img::ImgArchive::open(&mut reader, Some(&key))
                .map_err(|e| CliError::failure(format!("cannot parse {img}: {e}")))?;
            let index = lf_archive::Archive::entries(&archive)
                .iter()
                .position(|e| {
                    e.name.as_deref() == Some(entry.as_str())
                        || e.path.trim_start_matches('/') == entry
                })
                .ok_or_else(|| CliError::failure(format!("no entry {entry} in {img}")))?;
            let bytes = lf_archive::Archive::read_file(&archive, &mut reader, index, Some(&key))
                .map_err(|e| CliError::failure(format!("cannot read entry: {e}")))?;
            Ok((bytes, format!("{img} : {entry}")))
        }
        [path] => Ok((read_file(path)?, path.clone())),
        _ => Err(CliError::usage(format!("usage: {USAGE}"))),
    }
}

fn summarize(bytes: &[u8], label: &str, dict: &AnimDictionary, io: &mut Io) -> CliResult {
    let o = &mut *io.out;
    writeln!(o, "file:       {label}")?;
    writeln!(o, "disk size:  {} bytes", bytes.len())?;
    writeln!(o, "build tag:  {:#x}", dict.build_tag())?;
    writeln!(o, "clips:      {}", dict.len())?;
    match dict.pool() {
        Some(pool) => writeln!(o, "pool:       {} entries", pool.len())?,
        None => writeln!(o, "pool:       absent")?,
    }
    writeln!(
        o,
        "root u28:   {}  root u2c: {}",
        dict.root_unknown_28(),
        dict.root_unknown_2c()
    )?;
    let mut codecs: BTreeMap<String, u64> = BTreeMap::new();
    let mut frame_bad = 0u32;
    let mut hash_bad = 0u32;
    let mut quat_bad = 0u32;
    writeln!(o, "--- clips ---")?;
    for clip in dict.clips() {
        if !clip.validate_frame_duration() {
            frame_bad += 1;
        }
        if !clip.validate_hash() {
            hash_bad += 1;
        }
        let mut counts = BTreeMap::new();
        for track in clip.tracks() {
            let ch = track.channel();
            let key = match ch.codec() {
                Codec::StaticQuaternion => "static-quat",
                Codec::AnimatedPacked => "animated",
                Codec::Unknown => "opaque",
            };
            *counts.entry(key).or_insert(0u32) += 1;
            *codecs.entry(format!("{:#x}", ch.tag())).or_insert(0) += 1;
            if let ChannelData::StaticQuat(q) = ch.data()
                && !q.is_unit(1e-2)
            {
                quat_bad += 1;
            }
        }
        writeln!(
            o,
            "{} frames={} dur={:.3}s tracks={} {:?} hash-ok={} frame-ok={}",
            clip.short_name(),
            clip.frames(),
            clip.duration(),
            clip.tracks().len(),
            counts,
            clip.validate_hash(),
            clip.validate_frame_duration(),
        )?;
    }
    writeln!(o, "--- codec tags ---")?;
    for (tag, n) in &codecs {
        writeln!(o, "  {tag} x{n}")?;
    }
    writeln!(
        o,
        "validation: frame-bad={frame_bad} hash-bad={hash_bad} quat-bad={quat_bad}"
    )?;
    Ok(())
}
