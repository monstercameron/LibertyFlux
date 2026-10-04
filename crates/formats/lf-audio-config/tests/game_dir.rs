//! Real-data test: parse and fully decode every audio metadata file.
//!
//! Runs only when `LIBERTYFLUX_GAME_DIR` points at the game installation (the folder
//! holding `GTAIV/`); otherwise every test passes trivially. Read-only: no
//! game content is written anywhere.

use lf_audio_config::{
    container::MetaFile, decode::decode_object, hash::name_hash, schema::Schema, speech::SpeechFile,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

const VERSIONED: &[&str] = &[
    "GTAIV/pc/audio/config/categories.dat15",
    "GTAIV/pc/audio/config/curves.dat12",
    "GTAIV/pc/audio/config/effects.dat11",
    "GTAIV/pc/audio/config/game.dat16",
    "GTAIV/pc/audio/config/sounds.dat15",
    "GTAIV/TLAD/pc/audio/config/EP1_CURVES.DAT12",
    "GTAIV/TLAD/pc/audio/config/EP1_GAME.DAT16",
    "GTAIV/TLAD/pc/audio/config/EP1_RADIO_GAME.DAT16",
    "GTAIV/TLAD/pc/audio/config/EP1_RADIO_SOUNDS.DAT15",
    "GTAIV/TLAD/pc/audio/config/EP1_SOUNDS.DAT15",
    "GTAIV/TBoGT/pc/audio/config/CATEGORIES.DAT15",
    "GTAIV/TBoGT/pc/audio/config/EP2_CURVES.DAT12",
    "GTAIV/TBoGT/pc/audio/config/EP2_GAME.DAT16",
    "GTAIV/TBoGT/pc/audio/config/EP2_RADIO_GAME.DAT16",
    "GTAIV/TBoGT/pc/audio/config/EP2_RADIO_SOUNDS.DAT15",
    "GTAIV/TBoGT/pc/audio/config/EP2_SOUNDS.DAT15",
];

const SPEECH: &[&str] = &[
    "GTAIV/pc/audio/config/speech.dat",
    "GTAIV/TLAD/pc/audio/config/EP1_SPEECH.DAT",
    "GTAIV/TLAD/pc/audio/config/EP1_RADIO_SPEECH.DAT",
    "GTAIV/TBoGT/pc/audio/config/EP2_SPEECH.DAT",
    "GTAIV/TBoGT/pc/audio/config/EP2_RADIO_SPEECH.DAT",
];

/// Objects whose schema legitimately leaves trailing bytes:
/// (file stem, object name, type id, trailing length).
const KNOWN_TRAILING: &[(&str, &str, u8, usize)] =
    &[("curves.dat12", "DEBUG_VISUALISATION", 14, 12)];

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

#[test]
fn all_versioned_files_parse_and_decode() {
    let Some(root) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset; skipping");
        return;
    };
    let mut total_objects = 0usize;
    let mut total_archives = 0usize;
    let mut failures: Vec<String> = Vec::new();
    for rel in VERSIONED {
        let path = root.join(rel);
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {rel}: {e}"));
        let file = MetaFile::parse(&bytes).unwrap_or_else(|e| panic!("parse {rel}: {e}"));
        let stem = rel.rsplit('/').next().unwrap();
        let schema = Schema::detect(stem).unwrap_or_else(|| panic!("no schema detected for {rel}"));
        assert_eq!(schema.suffix(), file.suffix(), "suffix mismatch for {rel}");
        total_objects += file.objects().len();
        total_archives += file.archives().len();
        // No duplicate object names.
        let mut seen = BTreeSet::new();
        for entry in file.objects() {
            assert!(seen.insert(entry.name()), "duplicate name in {rel}");
        }
        for entry in file.objects() {
            let obj = match decode_object(&schema, entry) {
                Ok(o) => o,
                Err(e) => {
                    failures.push(format!("{rel}::{}: {e}", entry.name()));
                    continue;
                }
            };
            if obj.type_name.is_none() {
                failures.push(format!(
                    "{}::{}: unknown type id {}",
                    rel,
                    entry.name(),
                    obj.type_id
                ));
            }
            if !obj.trailing.is_empty() {
                let known = KNOWN_TRAILING.iter().any(|(f, n, t, l)| {
                    stem.eq_ignore_ascii_case(f)
                        && *n == entry.name()
                        && *t == obj.type_id
                        && *l == obj.trailing.len()
                });
                if !known {
                    failures.push(format!(
                        "{}::{}: {} unexpected trailing bytes",
                        rel,
                        entry.name(),
                        obj.trailing.len()
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} decode failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
    // Census locked in: 16 files, these object/archive totals.
    assert_eq!(VERSIONED.len(), 16);
    assert_eq!(total_objects, 31832);
    assert_eq!(total_archives, 1787);
}

#[test]
fn all_speech_files_parse() {
    let Some(root) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset; skipping");
        return;
    };
    let mut voices = 0usize;
    let mut contexts = 0usize;
    let mut banks = 0usize;
    for rel in SPEECH {
        let path = root.join(rel);
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {rel}: {e}"));
        let file = SpeechFile::parse(&bytes).unwrap_or_else(|e| panic!("parse {rel}: {e}"));
        voices += file.voices().len();
        contexts += file.contexts().len();
        banks += file.banks().len();
        // Every voice run and bank link resolves (checked again here).
        for v in file.voices() {
            assert!(!file.contexts_of(v).is_empty() || v.context_count() == 0);
        }
    }
    assert_eq!(SPEECH.len(), 5);
    assert_eq!(voices, 1554);
    assert_eq!(contexts, 56052);
    assert_eq!(banks, 1563);
}

#[test]
fn category_child_hashes_resolve() {
    let Some(root) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset; skipping");
        return;
    };
    // Every child-category hash must equal the hash of a category name in
    // the same file: this pins the hash function against real data.
    let bytes = std::fs::read(root.join(VERSIONED[0])).unwrap();
    let file = MetaFile::parse(&bytes).unwrap();
    let schema = Schema::Categories;
    let names: BTreeMap<u32, &str> = file
        .objects()
        .iter()
        .map(|o| (name_hash(o.name()), o.name()))
        .collect();
    let mut checked = 0usize;
    for entry in file.objects() {
        let obj = decode_object(&schema, entry).unwrap();
        for field in &obj.body {
            if field.name == "child_categories" {
                let lf_audio_config::decode::Value::Array(items) = &field.value else {
                    panic!("child_categories not an array")
                };
                for item in items {
                    let lf_audio_config::decode::Value::Hash(h) = item else {
                        panic!("child category not a hash")
                    };
                    assert!(names.contains_key(h), "unresolved child hash {h:#x}");
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 219);
}
