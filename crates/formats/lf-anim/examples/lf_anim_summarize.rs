//! Print a summary of one animation dictionary. Read-only: never writes.
//!
//! Usage:
//!
//! ```text
//! lf_anim_summarize <anim.img> <entry.wad> <exe-with-key>
//! lf_anim_summarize <file.wad>
//! ```
//!
//! The first form extracts the entry from the archive (locating the archive
//! key in the executable at run time, memory only). The second form reads a
//! raw `.wad` file. Only counts, names, and validation results print.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;

use lf_anim::{AnimDictionary, ChannelData, Codec};

// One linear printout; splitting it would scatter the shared counters.
#[allow(clippy::too_many_lines)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bytes: Vec<u8>;
    let label: String;
    if args.len() == 4 {
        let key = lf_archive::crypto::load_key_from_exe(std::path::Path::new(&args[3]))
            .unwrap_or_else(|e| {
                eprintln!("cannot locate archive key in {}: {e}", args[3]);
                std::process::exit(1);
            });
        let file = File::open(&args[1]).unwrap_or_else(|e| {
            eprintln!("cannot open {}: {e}", args[1]);
            std::process::exit(1);
        });
        let mut reader = BufReader::new(file);
        let archive =
            lf_archive::img::ImgArchive::open(&mut reader, Some(&key)).unwrap_or_else(|e| {
                eprintln!("cannot parse {}: {e}", args[1]);
                std::process::exit(1);
            });
        let index = lf_archive::Archive::entries(&archive)
            .iter()
            .position(|e| {
                e.name.as_deref() == Some(args[2].as_str())
                    || e.path.trim_start_matches('/') == args[2]
            })
            .unwrap_or_else(|| {
                eprintln!("no entry {} in {}", args[2], args[1]);
                std::process::exit(1);
            });
        bytes = lf_archive::Archive::read_file(&archive, &mut reader, index, Some(&key))
            .unwrap_or_else(|e| {
                eprintln!("cannot read entry: {e}");
                std::process::exit(1);
            });
        label = format!("{} : {}", args[1], args[2]);
    } else if args.len() == 2 {
        bytes = std::fs::read(&args[1]).unwrap_or_else(|e| {
            eprintln!("cannot read {}: {e}", args[1]);
            std::process::exit(1);
        });
        label = args[1].clone();
    } else {
        eprintln!(
            "usage: lf_anim_summarize <anim.img> <entry.wad> <exe> | lf_anim_summarize <file.wad>"
        );
        std::process::exit(2);
    }

    let dict = AnimDictionary::parse_bytes(&bytes).unwrap_or_else(|e| {
        eprintln!("cannot parse {label}: {e}");
        std::process::exit(1);
    });
    println!("file:       {label}");
    println!("disk size:  {} bytes", bytes.len());
    println!("build tag:  {:#x}", dict.build_tag());
    println!("clips:      {}", dict.len());
    match dict.pool() {
        Some(pool) => println!("pool:       {} entries", pool.len()),
        None => println!("pool:       absent"),
    }
    println!(
        "root u28:   {}  root u2c: {}",
        dict.root_unknown_28(),
        dict.root_unknown_2c()
    );

    let mut codecs: BTreeMap<String, u64> = BTreeMap::new();
    let mut frame_bad = 0u32;
    let mut hash_bad = 0u32;
    let mut quat_bad = 0u32;
    println!("--- clips ---");
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
        println!(
            "{} frames={} dur={:.3}s tracks={} {:?} hash-ok={} frame-ok={}",
            clip.short_name(),
            clip.frames(),
            clip.duration(),
            clip.tracks().len(),
            counts,
            clip.validate_hash(),
            clip.validate_frame_duration(),
        );
    }
    println!("--- codec tags ---");
    for (tag, n) in &codecs {
        println!("  {tag} x{n}");
    }
    println!("validation: frame-bad={frame_bad} hash-bad={hash_bad} quat-bad={quat_bad}");
}
