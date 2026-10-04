//! Real-file test: parse every save game file on the machine.
//!
//! Reads real files only when `LIBERTYFLUX_GAME_DIR` is set (to the game folder);
//! otherwise the test passes immediately with a skip notice. Save files do
//! not live inside the game folder, so the test also scans the standard
//! user save locations (see [`lf_save::save_dirs`]). It reports counts and
//! every distinct failure, and fails only when a file of this format does
//! not parse.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Recursively collect files whose name starts with `SGTA4`.
fn collect_slots(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Skip reparse points to avoid cycles; only descend into real dirs.
            if entry.file_type().is_ok_and(|t| t.is_dir()) {
                collect_slots(&path, out);
            }
        } else if entry
            .file_name()
            .to_str()
            .is_some_and(|n| n.starts_with("SGTA4"))
        {
            out.push(path);
        }
    }
}

#[test]
fn real_save_files() {
    let game_dir = if let Ok(d) = std::env::var("LIBERTYFLUX_GAME_DIR") {
        d
    } else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };

    let mut files = Vec::new();
    collect_slots(Path::new(&game_dir), &mut files);
    for dir in lf_save::save_dirs() {
        // Save folders hold profiles beneath them; scan recursively.
        collect_slots(&dir, &mut files);
    }
    files.sort();
    files.dedup();

    if files.is_empty() {
        eprintln!("no save files found (game folder plus user save locations)");
        return;
    }

    let mut parsed = 0u32;
    let mut failures: BTreeMap<String, u32> = BTreeMap::new();
    let mut block_counts: BTreeMap<usize, u32> = BTreeMap::new();
    let mut checksum_ok = 0u32;
    let mut checksum_bad = 0u32;
    let mut checksum_absent = 0u32;

    for path in &files {
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                *failures.entry(format!("unreadable: {e}")).or_insert(0) += 1;
                continue;
            }
        };
        match lf_save::SaveFile::parse(&bytes) {
            Ok(save) => {
                parsed += 1;
                *block_counts.entry(save.blocks().len()).or_insert(0) += 1;
                match save.verify_checksum() {
                    Some(true) => checksum_ok += 1,
                    Some(false) => checksum_bad += 1,
                    None => checksum_absent += 1,
                }
            }
            Err(e) => {
                *failures.entry(e.to_string()).or_insert(0) += 1;
            }
        }
    }

    eprintln!("save files found: {}", files.len());
    eprintln!("parsed without error: {parsed}");
    eprintln!("block-count histogram: {block_counts:?}");
    eprintln!("checksum: {checksum_ok} match, {checksum_bad} mismatch, {checksum_absent} absent");
    for (failure, count) in &failures {
        eprintln!("failure ({count}x): {failure}");
    }

    assert!(
        failures.is_empty(),
        "{} of {} save files failed to parse",
        failures.values().sum::<u32>(),
        files.len()
    );
}
