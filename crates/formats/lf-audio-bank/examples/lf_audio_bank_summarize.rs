//! Print a summary of one audio file: archive, bank, streamed or config.
//!
//! Usage: `summarize [--exe GTAIV.exe] [--names hashes.txt] FILE`
//!
//! `--exe` supplies the owner's executable so encrypted archive tables can be
//! read (the key is located at run time and never printed). `--names` loads
//! an external `hash=name` map purely for display. This tool only reads and
//! prints; it never writes game content anywhere.

use lf_archive::Archive;
use lf_archive::rpf::RpfArchive;
use lf_audio_bank::{bank, dat, streamed};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;

fn usage() -> ! {
    eprintln!("usage: lf_audio_bank_summarize [--exe GTAIV.exe] [--names hashes.txt] FILE");
    exit(2);
}

fn load_names(path: &str) -> HashMap<u32, String> {
    let mut map = HashMap::new();
    let text = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("cannot read names file: {e}");
        exit(1);
    });
    for line in text.lines() {
        let line = line.trim();
        let Some((hash, name)) = line.split_once('=') else {
            continue;
        };
        if let Ok(hash) = hash.trim().parse::<u32>() {
            map.insert(hash, name.trim().to_string());
        }
    }
    map
}

fn show(hash: u32, names: &HashMap<u32, String>) -> String {
    match names.get(&hash) {
        Some(n) => format!("{n} ({hash:#x})"),
        None => format!("{hash:#x}"),
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let mut exe_path: Option<String> = None;
    let mut names_path: Option<String> = None;
    let mut file_path: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--exe" => exe_path = Some(args.next().unwrap_or_else(|| usage())),
            "--names" => names_path = Some(args.next().unwrap_or_else(|| usage())),
            _ if file_path.is_none() => file_path = Some(arg),
            _ => usage(),
        }
    }
    let file_path = file_path.unwrap_or_else(|| usage());
    let names = names_path.map(|p| load_names(&p)).unwrap_or_default();
    let buf = fs::read(&file_path).unwrap_or_else(|e| {
        eprintln!("cannot read {file_path}: {e}");
        exit(1);
    });

    if buf.len() >= 4 && &buf[0..3] == b"RPF" {
        summarize_rpf(&buf, exe_path.as_deref(), &names);
    } else if let Ok(container) = lf_audio_bank::detect(&buf) {
        match container {
            lf_audio_bank::Container::Bank(b) => summarize_bank(&b),
            lf_audio_bank::Container::Streamed(s) => summarize_streamed(&s),
        }
    } else if let Ok(d) = dat::DatConfig::parse(&buf) {
        println!(
            "config: version={} names_off={} y={} z={} size={}",
            d.version(),
            d.names_off(),
            d.field_y(),
            d.field_z(),
            buf.len()
        );
        println!(
            "objects={} bytes names={} bytes runs={}",
            d.objects().len(),
            d.names_region().len(),
            d.name_runs(4).len()
        );
    } else {
        eprintln!("unknown file kind (not RPF, bank, streamed or config)");
        exit(1);
    }
}

fn summarize_rpf(buf: &[u8], exe_path: Option<&str>, names: &HashMap<u32, String>) {
    // Peek at the encrypted flag to decide whether a key is needed.
    let encrypted = buf.len() >= 20 && u32::from_le_bytes(buf[16..20].try_into().unwrap()) != 0;
    let key;
    let key_ref;
    if encrypted {
        let exe_path = exe_path.unwrap_or_else(|| {
            eprintln!("archive is encrypted; pass --exe GTAIV.exe");
            exit(1);
        });
        key = lf_archive::crypto::load_key_from_exe(exe_path).unwrap_or_else(|e| {
            eprintln!("key not found in executable: {e}");
            exit(1);
        });
        key_ref = Some(&key);
    } else {
        key_ref = None;
    }
    let mut reader = std::io::Cursor::new(buf);
    let archive = RpfArchive::open(&mut reader, key_ref).unwrap_or_else(|e| {
        eprintln!("cannot parse archive: {e}");
        exit(1);
    });
    let header = archive.header();
    println!(
        "archive: version={} toc={} entries={} encrypted={}",
        header.version, header.toc_size, header.entry_count, header.toc_encrypted
    );
    for (index, entry) in archive.entries().iter().enumerate() {
        if !entry.is_file() {
            println!("dir  {}", entry.path);
            continue;
        }
        let hash = entry.hash.unwrap_or(0);
        let data = archive
            .read_file(&mut reader, index, key_ref)
            .unwrap_or_default();
        let kind = match lf_audio_bank::detect(&data) {
            Ok(lf_audio_bank::Container::Bank(b)) => {
                format!("bank streams={}", b.stream_count())
            }
            Ok(lf_audio_bank::Container::Streamed(s)) => {
                format!(
                    "streamed blocks={} ch={}",
                    s.block_count(),
                    s.channel_count()
                )
            }
            Err(e) => format!("unparsed ({e})"),
        };
        println!(
            "file {} size={} offset={} {kind}",
            show(hash, names),
            entry.size,
            entry.offset
        );
    }
}

fn summarize_bank(b: &bank::Bank<'_>) {
    println!(
        "bank: streams={} base={} records_end={} field_14={}",
        b.stream_count(),
        b.base(),
        b.records_end(),
        b.field_14()
    );
    let entries = b.entries().unwrap_or_else(|e| {
        eprintln!("cannot read table: {e}");
        exit(1);
    });
    for entry in &entries {
        match b.stream(entry) {
            Ok(s) => {
                let rate = s.sample_rate;
                let secs = f64::from(s.sample_count) / f64::from(rate);
                println!(
                    "stream {:4} hash={:#x} samples={} rate={} secs={:.2} bytes={} codec={:#x} trailer={}",
                    entry.index,
                    entry.name_hash,
                    s.sample_count,
                    rate,
                    secs,
                    s.size,
                    s.codec,
                    s.trailer.len()
                );
            }
            Err(e) => println!("stream {:4} ERROR {e}", entry.index),
        }
    }
}

fn summarize_streamed(s: &streamed::Streamed<'_>) {
    println!(
        "streamed: blocks={} chunk={} channels={} flags={:#x} data_off={}",
        s.block_count(),
        s.block_chunk(),
        s.channel_count(),
        s.flags_28(),
        s.data_off()
    );
    match s.channel_waves() {
        Ok(waves) => {
            for w in &waves {
                let secs = f64::from(w.sample_count) / f64::from(w.sample_rate);
                println!(
                    "channel {} samples={} rate={} secs={:.2} codec={:#x} trailer={}",
                    w.index,
                    w.sample_count,
                    w.sample_rate,
                    secs,
                    w.codec,
                    w.trailer.len()
                );
            }
        }
        Err(e) => println!("channel waves ERROR {e}"),
    }
    println!("aux_region={} bytes", s.aux_region().map_or(0, <[u8]>::len));
    match s.file_table() {
        Ok(table) => {
            println!(
                "file_table={} entries first_sample_0={} last_first={}",
                table.len(),
                table.first().map_or(0, |e| e.first_sample),
                table.last().map_or(0, |e| e.first_sample)
            );
        }
        Err(e) => println!("file table ERROR {e}"),
    }
    for b in 0..s.block_count() {
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
                println!("block {b} data_start={} {}", block.data_start, ch.join(" "));
            }
            Err(e) => println!("block {b} ERROR {e}"),
        }
    }
}
