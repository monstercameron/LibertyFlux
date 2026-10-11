//! Integration test: open every archive in the game folder and list entries.
//!
//! Reads real game files when `LIBERTYFLUX_GAME_DIR` is set, and skips otherwise:
//!
//! ```text
//! LIBERTYFLUX_GAME_DIR="C:\path\to\Grand Theft Auto IV" cargo test -- --nocapture
//! ```
//!
//! The test never writes to the game folder. The archive key is located in
//! the owner's installed executable at run time and kept in memory only.

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use lf_archive::{Archive, EntryKind, Key, crypto, img, rpf};

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

fn collect_archives(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(top) = stack.pop() {
        let entries = match std::fs::read_dir(&top) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case(ext))
            {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn load_key(game_dir: &Path) -> Key {
    let exe = game_dir.join("GTAIV").join("GTAIV.exe");
    crypto::load_key_from_exe(&exe)
        .unwrap_or_else(|e| panic!("cannot locate archive key in {}: {e}", exe.display()))
}

#[test]
fn all_rpf_archives_list() {
    let Some(dir) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR not set; skipping");
        return;
    };
    let key = load_key(&dir);
    let files = collect_archives(&dir, "rpf");
    println!("RPF files: {}", files.len());
    assert!(!files.is_empty(), "no .rpf files under {}", dir.display());

    let mut total_files = 0u64;
    let mut total_dirs = 0u64;
    let mut total_resource = 0u64;
    let mut total_compressed = 0u64;
    let mut content_encrypted = 0u32;
    let mut failures: Vec<String> = Vec::new();
    let mut rpf3_hashes: HashSet<u32> = HashSet::new();

    for path in &files {
        let rel = path
            .strip_prefix(&dir)
            .unwrap_or(path)
            .display()
            .to_string();
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                failures.push(format!("{rel}: open failed: {e}"));
                continue;
            }
        };
        let file_len = file.metadata().map_or(0, |m| m.len());
        let mut reader = BufReader::new(file);
        let archive = match rpf::RpfArchive::open(&mut reader, Some(&key)) {
            Ok(a) => a,
            Err(e) => {
                failures.push(format!("{rel}: parse failed: {e}"));
                continue;
            }
        };
        // Structural checks on every entry.
        let mut n_files = 0u64;
        let mut n_dirs = 0u64;
        for (i, entry) in archive.entries().iter().enumerate() {
            match entry.kind {
                EntryKind::Directory => {
                    n_dirs += 1;
                    assert_eq!(entry.size, 0, "{rel} entry {i}: dir with size");
                }
                EntryKind::File => {
                    n_files += 1;
                    assert!(
                        entry.offset.saturating_add(entry.stored_size) <= file_len,
                        "{rel} entry {i} ({}): range past end",
                        entry.path
                    );
                    if entry.resource.is_some() {
                        total_resource += 1;
                    }
                    if entry.compressed {
                        total_compressed += 1;
                    }
                    if let Some(h) = entry.hash {
                        rpf3_hashes.insert(h);
                    }
                    // Spot-read the first bytes of every file entry.
                    let mut probe = [0u8; 16];
                    let want = entry.stored_size.min(16) as usize;
                    if want > 0 {
                        reader.seek(SeekFrom::Start(entry.offset)).unwrap();
                        reader.read_exact(&mut probe[..want]).unwrap();
                    }
                    // Fully read small entries (exercises decompression and
                    // the content cipher where present).
                    if entry.stored_size <= 65536
                        && let Err(e) = archive.read_file(&mut reader, i, Some(&key))
                    {
                        failures.push(format!("{rel} {}: read failed: {e}", entry.path));
                    }
                }
            }
        }
        assert_eq!(
            n_files + n_dirs,
            u64::from(archive.header().entry_count),
            "{rel}: entry count mismatch"
        );
        total_files += n_files;
        total_dirs += n_dirs;
        if archive.header().content_encrypted {
            content_encrypted += 1;
        }
        println!(
            "OK RPF{} enc={} content_enc={} files={} dirs={} {}",
            archive.header().version,
            archive.header().toc_encrypted,
            archive.header().content_encrypted,
            n_files,
            n_dirs,
            rel
        );
    }

    println!(
        "RPF totals: files={total_files} dirs={total_dirs} resource={total_resource} compressed={total_compressed}"
    );
    println!("RPF content-encrypted archives: {content_encrypted}");
    println!("RPF3 distinct hashes: {}", rpf3_hashes.len());
    // Cross-check the hash function against real table data: the low
    // filler value sometimes seen on RPF3 roots is not a real hash.
    assert!(!rpf3_hashes.contains(&0));
    assert!(!rpf3_hashes.contains(&1));
    for f in &failures {
        println!("FAIL {f}");
    }
    assert!(failures.is_empty(), "{} RPF failures", failures.len());
}

#[test]
fn all_img_archives_list() {
    let Some(dir) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR not set; skipping");
        return;
    };
    let key = load_key(&dir);
    let files = collect_archives(&dir, "img");
    println!("IMG files: {}", files.len());
    assert!(!files.is_empty(), "no .img files under {}", dir.display());

    let mut total_entries = 0u64;
    let mut total_resource = 0u64;
    let mut encrypted = 0u32;
    let mut open = 0u32;
    let mut failures: Vec<String> = Vec::new();

    for path in &files {
        let rel = path
            .strip_prefix(&dir)
            .unwrap_or(path)
            .display()
            .to_string();
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                failures.push(format!("{rel}: open failed: {e}"));
                continue;
            }
        };
        let file_len = file.metadata().map_or(0, |m| m.len());
        let mut reader = BufReader::new(file);
        let archive = match img::ImgArchive::open(&mut reader, Some(&key)) {
            Ok(a) => a,
            Err(e) => {
                failures.push(format!("{rel}: parse failed: {e}"));
                continue;
            }
        };
        assert_eq!(
            archive.len(),
            archive.header().entry_count as usize,
            "{rel}: entry count mismatch"
        );
        if archive.header().encrypted {
            encrypted += 1;
        } else {
            open += 1;
        }
        for (i, entry) in archive.entries().iter().enumerate() {
            assert_eq!(
                entry.kind,
                EntryKind::File,
                "{rel} entry {i}: IMG has no dirs"
            );
            assert!(
                !entry.name.as_deref().unwrap_or("").is_empty(),
                "{rel} entry {i}: empty name"
            );
            assert!(
                entry.offset.saturating_add(entry.stored_size) <= file_len,
                "{rel} entry {i} ({}): range past end",
                entry.path
            );
            if entry.resource.is_some() {
                total_resource += 1;
            }
            let mut probe = [0u8; 16];
            let want = entry.stored_size.min(16) as usize;
            if want > 0 {
                reader.seek(SeekFrom::Start(entry.offset)).unwrap();
                reader.read_exact(&mut probe[..want]).unwrap();
            }
            if entry.stored_size <= 65536
                && let Err(e) = archive.read_file(&mut reader, i, Some(&key))
            {
                failures.push(format!("{rel} {}: read failed: {e}", entry.path));
            }
        }
        total_entries += archive.len() as u64;
    }

    println!(
        "IMG totals: files={} entries={total_entries} resource={total_resource} encrypted={encrypted} open={open}",
        files.len()
    );
    for f in &failures {
        println!("FAIL {f}");
    }
    assert!(failures.is_empty(), "{} IMG failures", failures.len());
}

#[test]
fn any_archive_opens_both_kinds() {
    let Some(dir) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR not set; skipping");
        return;
    };
    let key = load_key(&dir);
    let rpf = collect_archives(&dir, "rpf");
    let img = collect_archives(&dir, "img");
    for path in rpf.iter().take(1).chain(img.iter().take(1)) {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let archive = lf_archive::open(&mut reader, Some(&key)).unwrap();
        assert!(!archive.is_empty());
    }
}
