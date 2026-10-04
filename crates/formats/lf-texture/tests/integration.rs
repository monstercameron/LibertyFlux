//! Real-file integration test: parse every reachable texture dictionary.
//!
//! Reads real game files only when `LIBERTYFLUX_GAME_DIR` points at the installed
//! game folder; otherwise the test passes trivially (skipped). For every
//! `*.wtd` found it parses the dictionary, checks each texture's stride and
//! hash rules, decodes mip level 0 to RGBA8, and builds an in-memory DDS,
//! then reports exact counts. Any parse or decode failure fails the test.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use lf_texture::{D3DFormat, Dictionary, hash_title, level_byte_size, title_of, to_dds};

fn collect_wtd(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_wtd(&path, out);
        } else if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("wtd"))
        {
            out.push(path);
        }
    }
}

#[test]
fn real_dictionaries() {
    let Some(game_dir) = std::env::var_os("LIBERTYFLUX_GAME_DIR") else {
        eprintln!("LIBERTYFLUX_GAME_DIR not set; skipping real-file test");
        return;
    };
    let mut files = Vec::new();
    collect_wtd(Path::new(&game_dir), &mut files);
    files.sort();
    println!("WTD files found: {}", files.len());
    assert!(
        !files.is_empty(),
        "no .wtd files under LIBERTYFLUX_GAME_DIR"
    );

    let mut parsed = 0usize;
    let mut textures = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut formats: BTreeMap<String, usize> = BTreeMap::new();
    let mut decoded_px: u64 = 0;

    for path in &files {
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                failures.push(format!("{}: read error {e}", path.display()));
                continue;
            }
        };
        let dict = match Dictionary::parse(&bytes) {
            Ok(d) => d,
            Err(e) => {
                failures.push(format!("{}: parse error {e}", path.display()));
                continue;
            }
        };
        parsed += 1;
        for entry in dict.entries() {
            textures += 1;
            *formats.entry(entry.record.format.name()).or_insert(0) += 1;
            let r = &entry.record;
            // Stride rule: stride * height == level-0 byte size.
            match level_byte_size(r.format, r.width, r.height, 0) {
                Ok(top) => {
                    if u64::from(r.stride) * u64::from(r.height) != top {
                        failures.push(format!(
                            "{}: {} stride {} x height {} != top size {top}",
                            path.display(),
                            entry.name,
                            r.stride,
                            r.height
                        ));
                    }
                }
                Err(lf_texture::Error::UnknownFormat(_)) => {}
                Err(e) => failures.push(format!(
                    "{}: {} sizing error {e}",
                    path.display(),
                    entry.name
                )),
            }
            // Hash rule: stored hash == hash of title.
            if entry.hash != hash_title(title_of(&entry.name)) {
                failures.push(format!(
                    "{}: {} hash {:#010x} != computed {:#010x}",
                    path.display(),
                    entry.name,
                    entry.hash,
                    hash_title(title_of(&entry.name))
                ));
            }
            // Decode level 0 and build a DDS; both must succeed.
            match dict.decode_rgba8(entry, 0) {
                Ok(img) => {
                    decoded_px += u64::from(img.width) * u64::from(img.height);
                    assert_eq!(img.pixels.len(), (img.width * img.height * 4) as usize);
                }
                Err(e) => failures.push(format!(
                    "{}: {} decode error {e}",
                    path.display(),
                    entry.name
                )),
            }
            match to_dds(&dict, entry) {
                Ok(dds) => {
                    assert_eq!(&dds[0..4], b"DDS ");
                    let w = u32::from_le_bytes(dds[16..20].try_into().unwrap());
                    let h = u32::from_le_bytes(dds[12..16].try_into().unwrap());
                    assert_eq!((w, h), (u32::from(r.width), u32::from(r.height)));
                }
                Err(e) => {
                    failures.push(format!("{}: {} DDS error {e}", path.display(), entry.name))
                }
            }
            // Every mip level must slice inside the graphics segment.
            for level in 0..r.levels {
                if let Err(e) = dict.level_data(entry, level) {
                    failures.push(format!(
                        "{}: {} level {level} error {e}",
                        path.display(),
                        entry.name
                    ));
                }
            }
        }
    }

    println!("parsed: {parsed}/{}", files.len());
    println!("textures: {textures}");
    println!("formats: {formats:?}");
    println!("decoded level-0 texels: {decoded_px}");
    for f in failures.iter().take(20) {
        println!("FAILURE: {f}");
    }
    if failures.len() > 20 {
        println!("... and {} more", failures.len() - 20);
    }
    assert!(
        failures.is_empty(),
        "{} failures ({} files, {} textures)",
        failures.len(),
        files.len(),
        textures
    );
    // Cross-check the dictionary lookup path on real data.
    if parsed > 0 {
        let bytes = std::fs::read(&files[0]).unwrap();
        let dict = Dictionary::parse(&bytes).unwrap();
        if let Some(first) = dict.entries().first() {
            let by_hash = dict.find(&first.name);
            assert_eq!(by_hash.map(|e| &e.name), Some(&first.name));
            assert!(dict.find("no-such-texture-xyz").is_none());
        }
        let _ = D3DFormat::Dxt1;
    }
}
