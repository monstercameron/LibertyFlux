//! Integration test: parse every `.cut` file in the game folder.
//!
//! Reads real game files when `LIBERTYFLUX_GAME_DIR` is set, and skips otherwise:
//!
//! ```text
//! LIBERTYFLUX_GAME_DIR="C:\path\to\Grand Theft Auto IV" cargo test -- --nocapture
//! ```
//!
//! The test never writes to the game folder. The archive key is located in
//! the owner's installed executable at run time and kept in memory only.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use lf_archive::crypto;
use lf_cutscene::catalog;

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

fn load_key(game_dir: &Path) -> lf_archive::crypto::Key {
    let exe = game_dir.join("GTAIV").join("GTAIV.exe");
    crypto::load_key_from_exe(&exe)
        .unwrap_or_else(|e| panic!("cannot locate archive key in {}: {e}", exe.display()))
}

#[test]
// One linear census pass; splitting it would scatter the shared counters.
#[allow(clippy::too_many_lines)]
fn all_cut_files_parse() {
    let Some(dir) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR not set; skipping");
        return;
    };
    let key = load_key(&dir);

    let cuts = catalog::find_archives(&dir, "cuts.img");
    let props = catalog::find_archives(&dir, "cutsprops.img");
    println!("cuts.img: {}", cuts.len());
    println!("cutsprops.img: {}", props.len());
    assert_eq!(cuts.len(), 3, "expected base + 2 episode cuts.img");
    assert_eq!(props.len(), 3, "expected base + 2 episode cutsprops.img");

    // Parse every .cut entry.
    let mut parsed = Vec::new();
    for path in &cuts {
        let mut cuts_in = catalog::parse_cuts_in_archive(path, &key)
            .unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()));
        println!("{}: {} .cut files", path.display(), cuts_in.len());
        parsed.append(&mut cuts_in);
    }
    println!("total .cut files: {}", parsed.len());
    assert_eq!(parsed.len(), 344, "expected 344 .cut files");

    // Totals across all files.
    let mut groups = 0usize;
    let mut sections = 0usize;
    let mut texts = 0usize;
    let mut models = 0usize;
    let mut durations = 0usize;
    let mut duration_sum = 0.0f64;
    let mut warn_files: Vec<String> = Vec::new();
    let mut all_warnings = 0usize;
    for cut in &parsed {
        if !cut.file.warnings.is_empty() {
            warn_files.push(format!("{}/{}", cut.episode, cut.name));
            for w in &cut.file.warnings {
                println!(
                    "  warn {}/{} line {} {:?}: {}",
                    cut.episode, cut.name, w.line, w.kind, w.message
                );
                all_warnings += 1;
            }
        }
        if !cut.file.orphans.is_empty() {
            println!(
                "  orphans in {}/{}: {:?}",
                cut.episode, cut.name, cut.file.orphans
            );
        }
        // The only mid-file strays in shipped data are exporter debug
        // lines ("VARIATION FOUND!") baked into four files.
        let key = format!("{}/{}", cut.episode, cut.name);
        let expected_orphans: &[(usize, &str)] = match key.as_str() {
            "base/fau3_a.cut" => &[(53, "VARIATION FOUND!"), (420, "VARIATION FOUND!")],
            "base/imgm4b.cut" => &[(32, "VARIATION FOUND!")],
            "base/show_4.cut" => &[(128, "VARIATION FOUND!")],
            "base/vla3_a.cut" => &[
                (85, "VARIATION FOUND!"),
                (202, "VARIATION FOUND!"),
                (334, "VARIATION FOUND!"),
            ],
            _ => &[],
        };
        assert_eq!(
            cut.file
                .orphans
                .iter()
                .map(|o| (o.line, o.text.as_str()))
                .collect::<Vec<_>>(),
            expected_orphans,
            "orphan set changed in {key}"
        );
        for g in &cut.file.groups {
            groups += 1;
            texts += g.texts.len();
            sections += g.sections.len();
            for s in &g.sections {
                models += s.models.len();
                durations += s.durations_ms.len();
                duration_sum += s.durations_ms.iter().map(|d| f64::from(*d)).sum::<f64>();
            }
        }
    }
    println!("groups: {groups}, sections: {sections}, texts: {texts}, models: {models}");
    println!("durations: {durations} values, sum {duration_sum:.0} ms");
    println!(
        "files with warnings: {} ({all_warnings} warnings)",
        warn_files.len()
    );
    assert_eq!(groups, 419);
    assert_eq!(sections, 810);
    assert_eq!(texts, 8767);
    assert_eq!(models, 7242);
    assert_eq!(durations, 810);

    // Warnings happen only in the known-quirky files (tag typos, repeated
    // opens, one 5-field vehicle row). Anything else is a regression.
    warn_files.sort();
    let expected = [
        "base/fau3_a.cut",
        "base/imfau4.cut",
        "base/imgm4b.cut",
        "base/mich_a.cut",
        "base/rpmobd1.cut",
        "base/show_4.cut",
        "base/vla3_a.cut",
        "tbogt/e2imgm5.cut",
        "tbogt/gt07_aa.cut",
        "tbogt/gt7aap2.cut",
        "tbogt/y1_aza.cut",
        "tlad/ab02_ba.cut",
        "tlad/ab03_za.cut",
        "tlad/bg02_za.cut",
    ];
    assert_eq!(warn_files, expected, "warning file set changed");

    // Header arity is sections + 1, except the one subtitles-only group.
    let mut arity_ok = 0usize;
    let mut arity_off: Vec<String> = Vec::new();
    for cut in &parsed {
        for (gi, g) in cut.file.groups.iter().enumerate() {
            if g.header_frames.len() == g.sections.len() + 1 {
                arity_ok += 1;
            } else {
                arity_off.push(format!(
                    "{}/{} group {gi}: {} frames, {} sections",
                    cut.episode,
                    cut.name,
                    g.header_frames.len(),
                    g.sections.len()
                ));
            }
        }
    }
    println!("header arity ok: {arity_ok}, off: {arity_off:?}");
    assert_eq!(arity_ok, 418);
    assert_eq!(
        arity_off,
        ["tbogt/gt07_aa.cut group 1: 2 frames, 0 sections"]
    );

    // ANIM references resolve to .wad entries (case-insensitive).
    let mut wads: HashMap<String, HashSet<String>> = HashMap::new();
    for path in &cuts {
        let names = catalog::entry_names_lower(path, &key).unwrap();
        println!("{}: {} entries", path.display(), names.len());
        wads.insert(catalog::episode_of(path), names);
    }
    let (anim_ok, anim_missing) = catalog::check_anims(&parsed, &wads);
    println!("ANIM resolved: {anim_ok}, missing: {anim_missing:?}");
    assert_eq!(anim_ok, 809);
    assert_eq!(
        anim_missing,
        [("tbogt/gt11_xa.cut".to_string(), "GT11_XA_0".to_string())]
    );

    // MODELS references resolve against same-episode or base cutsprops.
    let mut stems: HashMap<String, HashSet<String>> = HashMap::new();
    for path in &props {
        let names = catalog::entry_stems_lower(path, &key).unwrap();
        println!("{}: {} model stems", path.display(), names.len());
        stems.insert(catalog::episode_of(path), names);
    }
    let base_stems = stems.get("base").cloned().unwrap_or_default();
    let mut model_ok = 0usize;
    let mut model_missing: HashMap<String, usize> = HashMap::new();
    for cut in &parsed {
        let ep_stems = stems.get(&cut.episode);
        for g in &cut.file.groups {
            for s in &g.sections {
                for m in &s.models {
                    let stem = m.model.to_ascii_lowercase();
                    let found =
                        ep_stems.is_some_and(|e| e.contains(&stem)) || base_stems.contains(&stem);
                    if found {
                        model_ok += 1;
                    } else {
                        *model_missing.entry(stem).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    println!(
        "MODELS resolved in cutsprops: {model_ok}, unresolved stems: {}",
        model_missing.len()
    );
    let mut missing: Vec<(&String, &usize)> = model_missing.iter().collect();
    missing.sort_by(|a, b| b.1.cmp(a.1));
    for (stem, count) in missing.iter().take(15) {
        println!("  {count:>5}  {stem}");
    }
    // The player model ships in playerped.rpf, not in cutsprops.
    assert!(
        model_missing.contains_key("player"),
        "player should be unresolved in cutsprops"
    );

    // .wad payloads are RSC generic resources (header check only).
    let mut wad_count = 0usize;
    for path in &cuts {
        let headers = catalog::wad_headers(path, &key).unwrap();
        for (name, type_id, codec) in &headers {
            assert_eq!(*type_id, 0x01, "unexpected .wad type in {name}");
            assert_eq!(codec, "Deflate", "unexpected .wad codec in {name}");
        }
        wad_count += headers.len();
    }
    println!(".wad headers checked: {wad_count}");
    assert_eq!(wad_count, 667);

    // AUDIO names vs cutscenes.rpf hashes (resolution is best-effort).
    let audio_archives = catalog::find_archives(&dir, "cutscenes.rpf");
    println!("cutscenes.rpf: {}", audio_archives.len());
    if let Some(rpf) = audio_archives.first() {
        let hashes = catalog::rpf_file_hashes(rpf, &key).unwrap();
        println!("cutscenes.rpf file entries: {}", hashes.len());
        let mut audio_names: HashSet<String> = HashSet::new();
        for cut in &parsed {
            for g in &cut.file.groups {
                for s in &g.sections {
                    audio_names.extend(s.audios.iter().cloned());
                }
            }
        }
        let mut hits = 0usize;
        for name in &audio_names {
            if catalog::audio_hash_candidates(name)
                .iter()
                .any(|(_, h)| hashes.contains(h))
            {
                hits += 1;
            }
        }
        println!("AUDIO names: {}, hash hits: {hits}", audio_names.len());
        // Bare AUDIO names hash-match entries in cutscenes.rpf; episode
        // banks (EP1_SFX/EP2_SFX) hold the episode names, and 55 mostly
        // TBoGT names match no shipped RPF entry (see the lane report).
        assert_eq!(audio_names.len(), 315);
        assert_eq!(hits, 152);
    }
}
