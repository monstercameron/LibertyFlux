//! `audio-bank`: audio archives, banks, streamed files and configuration
//! headers (`lf-audio-bank`).
//!
//! - `summarize [--exe GTAIV.exe] [--names hashes.txt] FILE`: the former
//!   `lf_audio_bank_summarize` example, same output. `--exe` supplies the
//!   owner's executable so encrypted archive tables can be read (the key is
//!   located at run time and never printed); `--names` loads an external
//!   `hash=name` map purely for display.

use std::collections::HashMap;

use lf_archive::Archive;
use lf_archive::rpf::RpfArchive;
use lf_audio_bank::{bank, dat, streamed};

use crate::cli::{CliError, CliResult, Io, load_key, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize"];

const USAGE: &str = "lf-inspect audio-bank summarize [--exe GTAIV.exe] [--names hashes.txt] FILE";

/// Run one `audio-bank` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the input
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if cmd != "summarize" {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let usage = || CliError::usage(format!("usage: {USAGE}"));
    let mut exe_path: Option<&str> = None;
    let mut names_path: Option<&str> = None;
    let mut file_path: Option<&str> = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--exe" => exe_path = Some(it.next().ok_or_else(usage)?),
            "--names" => names_path = Some(it.next().ok_or_else(usage)?),
            _ if file_path.is_none() => file_path = Some(arg),
            _ => return Err(usage()),
        }
    }
    let file_path = file_path.ok_or_else(usage)?;
    let names = names_path.map(load_names).transpose()?.unwrap_or_default();
    let buf = read_file(file_path)?;
    let o = &mut *io.out;
    if buf.len() >= 4 && &buf[0..3] == b"RPF" {
        summarize_rpf(o, &buf, exe_path, &names)
    } else if let Ok(container) = lf_audio_bank::detect(&buf) {
        match container {
            lf_audio_bank::Container::Bank(b) => summarize_bank(o, &b),
            lf_audio_bank::Container::Streamed(s) => summarize_streamed(o, &s),
        }
    } else if let Ok(d) = dat::DatConfig::parse(&buf) {
        writeln!(
            o,
            "config: version={} names_off={} y={} z={} size={}",
            d.version(),
            d.names_off(),
            d.field_y(),
            d.field_z(),
            buf.len()
        )?;
        writeln!(
            o,
            "objects={} bytes names={} bytes runs={}",
            d.objects().len(),
            d.names_region().len(),
            d.name_runs(4).len()
        )?;
        Ok(())
    } else {
        Err(CliError::failure(
            "unknown file kind (not RPF, bank, streamed or config)",
        ))
    }
}

fn load_names(path: &str) -> Result<HashMap<u32, String>, CliError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| CliError::failure(format!("cannot read names file: {e}")))?;
    let mut map = HashMap::new();
    for line in text.lines() {
        let Some((hash, name)) = line.trim().split_once('=') else {
            continue;
        };
        if let Ok(hash) = hash.trim().parse::<u32>() {
            map.insert(hash, name.trim().to_string());
        }
    }
    Ok(map)
}

fn show(hash: u32, names: &HashMap<u32, String>) -> String {
    match names.get(&hash) {
        Some(n) => format!("{n} ({hash:#x})"),
        None => format!("{hash:#x}"),
    }
}

fn summarize_rpf(
    o: &mut dyn std::io::Write,
    buf: &[u8],
    exe_path: Option<&str>,
    names: &HashMap<u32, String>,
) -> CliResult {
    // Peek at the encrypted flag to decide whether a key is needed.
    let encrypted = buf.len() >= 20 && buf[16..20] != [0, 0, 0, 0];
    let key = if encrypted {
        let exe = exe_path
            .ok_or_else(|| CliError::failure("archive is encrypted; pass --exe GTAIV.exe"))?;
        Some(load_key(exe)?)
    } else {
        None
    };
    let mut reader = std::io::Cursor::new(buf);
    let archive = RpfArchive::open(&mut reader, key.as_ref())
        .map_err(|e| CliError::failure(format!("cannot parse archive: {e}")))?;
    let header = archive.header();
    writeln!(
        o,
        "archive: version={} toc={} entries={} encrypted={}",
        header.version, header.toc_size, header.entry_count, header.toc_encrypted
    )?;
    for (index, entry) in archive.entries().iter().enumerate() {
        if !entry.is_file() {
            writeln!(o, "dir  {}", entry.path)?;
            continue;
        }
        let hash = entry.hash.unwrap_or(0);
        let data = archive
            .read_file(&mut reader, index, key.as_ref())
            .unwrap_or_default();
        let kind = match lf_audio_bank::detect(&data) {
            Ok(lf_audio_bank::Container::Bank(b)) => format!("bank streams={}", b.stream_count()),
            Ok(lf_audio_bank::Container::Streamed(s)) => {
                format!(
                    "streamed blocks={} ch={}",
                    s.block_count(),
                    s.channel_count()
                )
            }
            Err(e) => format!("unparsed ({e})"),
        };
        writeln!(
            o,
            "file {} size={} offset={} {kind}",
            show(hash, names),
            entry.size,
            entry.offset
        )?;
    }
    Ok(())
}

fn summarize_bank(o: &mut dyn std::io::Write, b: &bank::Bank<'_>) -> CliResult {
    writeln!(
        o,
        "bank: streams={} base={} records_end={} field_14={}",
        b.stream_count(),
        b.base(),
        b.records_end(),
        b.field_14()
    )?;
    let entries = b
        .entries()
        .map_err(|e| CliError::failure(format!("cannot read table: {e}")))?;
    for entry in &entries {
        match b.stream(entry) {
            Ok(s) => {
                let rate = s.sample_rate;
                let secs = f64::from(s.sample_count) / f64::from(rate);
                writeln!(
                    o,
                    "stream {:4} hash={:#x} samples={} rate={} secs={:.2} bytes={} codec={:#x} trailer={}",
                    entry.index,
                    entry.name_hash,
                    s.sample_count,
                    rate,
                    secs,
                    s.size,
                    s.codec,
                    s.trailer.len()
                )?;
            }
            Err(e) => writeln!(o, "stream {:4} ERROR {e}", entry.index)?,
        }
    }
    Ok(())
}

fn summarize_streamed(o: &mut dyn std::io::Write, s: &streamed::Streamed<'_>) -> CliResult {
    writeln!(
        o,
        "streamed: blocks={} chunk={} channels={} flags={:#x} data_off={}",
        s.block_count(),
        s.block_chunk(),
        s.channel_count(),
        s.flags_28(),
        s.data_off()
    )?;
    match s.channel_waves() {
        Ok(waves) => {
            for w in &waves {
                let secs = f64::from(w.sample_count) / f64::from(w.sample_rate);
                writeln!(
                    o,
                    "channel {} samples={} rate={} secs={:.2} codec={:#x} trailer={}",
                    w.index,
                    w.sample_count,
                    w.sample_rate,
                    secs,
                    w.codec,
                    w.trailer.len()
                )?;
            }
        }
        Err(e) => writeln!(o, "channel waves ERROR {e}")?,
    }
    writeln!(
        o,
        "aux_region={} bytes",
        s.aux_region().map_or(0, <[u8]>::len)
    )?;
    // The block loop runs only once the file table has bounded the block
    // count by the file size, so a hostile count cannot print forever.
    let table_ok = s.file_table().is_ok();
    match s.file_table() {
        Ok(table) => {
            writeln!(
                o,
                "file_table={} entries first_sample_0={} last_first={}",
                table.len(),
                table.first().map_or(0, |e| e.first_sample),
                table.last().map_or(0, |e| e.first_sample)
            )?;
        }
        Err(e) => writeln!(o, "file table ERROR {e}")?,
    }
    for b in 0..if table_ok { s.block_count() } else { 0 } {
        match s.block(b) {
            Ok(block) => {
                let ch: Vec<String> = block
                    .channels
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        format!(
                            "ch{i}:start={} n={} samples={}",
                            c.start_entry, c.entries, c.sample_count
                        )
                    })
                    .collect();
                writeln!(
                    o,
                    "block {b} data_start={} {}",
                    block.data_start,
                    ch.join(" ")
                )?;
            }
            Err(e) => writeln!(o, "block {b} ERROR {e}")?,
        }
    }
    Ok(())
}
