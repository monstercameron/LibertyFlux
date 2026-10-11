//! Integration test against the real game data.
//!
//! Reads every `.wad` entry from the six animation archives, parses each
//! through `lf-resource` and `lf-anim`, and checks the verified rules:
//! frame/duration agreement, name-hash agreement, and unit static quats.
//!
//! Run with `LIBERTYFLUX_GAME_DIR` set to the game install folder (the folder
//! that holds the `GTAIV` directory). Without the variable the test passes
//! trivially. The archive key is located in the owner's executable at run
//! time and kept in memory only.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use lf_anim::{AnimDictionary, ChannelData};
use lf_archive::img::ImgArchive;
use lf_archive::{Archive, crypto};

fn game_root() -> Option<PathBuf> {
    let dir = std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)?;
    let root = dir.join("GTAIV");
    if root.join("GTAIV.exe").is_file() {
        Some(root)
    } else {
        panic!("LIBERTYFLUX_GAME_DIR is set but no GTAIV executable found under it");
    }
}

fn archives(root: &Path) -> Vec<PathBuf> {
    [
        "pc/anim/anim.img",
        "pc/anim/cuts.img",
        "TLAD/pc/anim/anim.img",
        "TLAD/pc/anim/cuts.img",
        "TBoGT/pc/anim/anim.img",
        "TBoGT/pc/anim/cuts.img",
    ]
    .iter()
    .map(|rel| root.join(rel))
    .collect()
}

#[test]
fn all_game_wads_parse() {
    let Some(root) = game_root() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset: skipping game-data test");
        return;
    };
    let key = crypto::load_key_from_exe(root.join("GTAIV.exe")).expect("archive key");

    let mut files = 0u32;
    let mut parsed = 0u32;
    let mut failures: BTreeMap<String, u32> = BTreeMap::new();
    let mut clips = 0u32;
    let mut frame_bad = 0u32;
    let mut hash_bad = 0u32;
    let mut quats = 0u32;
    let mut quat_bad = 0u32;
    let mut worst_quat = 0.0f32;
    let mut codecs: BTreeMap<String, u64> = BTreeMap::new();
    let mut params: BTreeMap<String, u64> = BTreeMap::new();
    let mut track_counts: BTreeMap<u16, u64> = BTreeMap::new();

    for archive_path in archives(&root) {
        let file = File::open(&archive_path).expect("open archive");
        let mut reader = BufReader::new(file);
        let archive = ImgArchive::open(&mut reader, Some(&key)).expect("open img");
        for (index, entry) in archive.entries().iter().enumerate() {
            if !entry.is_file() || !entry.path.to_lowercase().ends_with(".wad") {
                continue;
            }
            files += 1;
            let bytes = archive
                .read_file(&mut reader, index, Some(&key))
                .expect("read entry");
            let dict = match AnimDictionary::parse_bytes(&bytes) {
                Ok(d) => {
                    parsed += 1;
                    d
                }
                Err(e) => {
                    *failures.entry(format!("{e}")).or_insert(0) += 1;
                    continue;
                }
            };
            for clip in dict.clips() {
                clips += 1;
                if !clip.validate_frame_duration() {
                    frame_bad += 1;
                }
                if !clip.validate_hash() {
                    hash_bad += 1;
                }
                let track_len = u16::try_from(clip.tracks().len()).unwrap_or(u16::MAX);
                *track_counts.entry(track_len).or_insert(0) += 1;
                for track in clip.tracks() {
                    let ch = track.channel();
                    *codecs.entry(format!("{:#x}", ch.tag())).or_insert(0) += 1;
                    *params
                        .entry(format!("{:#x}/{:#x}", ch.tag(), ch.param()))
                        .or_insert(0) += 1;
                    if let ChannelData::StaticQuat(q) = ch.data() {
                        quats += 1;
                        let d = (q.norm() - 1.0).abs();
                        worst_quat = worst_quat.max(d);
                        if !q.is_unit(1e-2) {
                            quat_bad += 1;
                        }
                    }
                }
            }
            if let Some(pool) = dict.pool() {
                for track in pool {
                    for ch in track.channels() {
                        *codecs.entry(format!("{:#x}", ch.tag())).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    println!("wad files: {files}, parsed: {parsed}");
    println!("clips: {clips}, frame/duration bad: {frame_bad}, hash bad: {hash_bad}");
    println!("static quats: {quats}, non-unit: {quat_bad}, worst |n-1|: {worst_quat:.6}");
    println!("distinct failures: {}", failures.len());
    for (e, n) in failures.iter().take(10) {
        println!("  {n}x {e}");
    }
    println!("codec tags (top 20):");
    let mut tags: Vec<_> = codecs.iter().collect();
    tags.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
    for (tag, n) in tags.iter().take(20) {
        println!("  {tag} x{n}");
    }
    println!("tag/param pairs: {}", params.len());
    println!("track-count values: {}", track_counts.len());

    assert_eq!(parsed, files, "every WAD must parse");
    assert!(clips > 20_000, "expected tens of thousands of clips");
    assert_eq!(frame_bad, 0, "frame/duration rule must hold everywhere");
    assert_eq!(hash_bad, 0, "name-hash rule must hold everywhere");
    assert_eq!(quat_bad, 0, "static quats must be unit length");
}
