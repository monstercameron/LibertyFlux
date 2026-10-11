//! Census over the game's real effect files.
//!
//! Parses every particle/effect file under `LIBERTYFLUX_GAME_DIR`: the five `*.wpfl`
//! packages, the ten `*Fx.dat` rule tables and the eight rain/storm XML
//! files. Skipped (passing, with a notice) when `LIBERTYFLUX_GAME_DIR` is unset so
//! `cargo test` stays green without a game install. Read-only: never writes.
//!
//! A second test scans every archive for particle resources; the archive key
//! comes from the owner's executable under `LIBERTYFLUX_GAME_DIR`. It is
//! skipped unless the variable is set.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use lf_archive::Archive;
use lf_effects::{FxFile, WpflFile, emitter};

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.is_file() {
            out.push(path);
        }
    }
}

fn game_files() -> Option<Vec<PathBuf>> {
    let Ok(game_dir) = std::env::var("LIBERTYFLUX_GAME_DIR") else {
        println!("LIBERTYFLUX_GAME_DIR unset: skipping real-file census");
        return None;
    };
    let mut files = Vec::new();
    collect(Path::new(&game_dir), &mut files);
    Some(files)
}

#[test]
fn wpfl_packages_all_parse() {
    let Some(files) = game_files() else { return };
    let mut wpfl: Vec<&PathBuf> = files
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "wpfl"))
        .collect();
    wpfl.sort();
    println!("files scanned: {}", files.len());
    println!("wpfl packages: {}", wpfl.len());
    assert_eq!(
        wpfl.len(),
        5,
        "expected 5 wpfl packages, found {}",
        wpfl.len()
    );

    let mut failures: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut total_entries = 0;
    let mut total_resolved = 0;
    for path in &wpfl {
        let bytes = std::fs::read(path).unwrap();
        match WpflFile::parse(&bytes) {
            Ok(pkg) => {
                println!(
                    "{}: kind={:#x} banks={} entries={} resolved={}",
                    path.file_name().unwrap().to_string_lossy(),
                    pkg.kind(),
                    pkg.banks().len(),
                    pkg.entry_count(),
                    pkg.resolved_count()
                );
                assert_eq!(pkg.banks().len(), 5, "expected 5 banks in {path:?}");
                assert!(pkg.entry_count() > 0);
                // Every record's head word must be readable.
                for (_, entry) in pkg.entries() {
                    pkg.record_bytes(entry, 4).unwrap();
                }
                total_entries += pkg.entry_count();
                total_resolved += pkg.resolved_count();
            }
            Err(e) => {
                failures
                    .entry(e.to_string())
                    .or_default()
                    .push(path.display().to_string());
            }
        }
    }
    println!("total bank entries: {total_entries}");
    println!("entries resolved to names: {total_resolved}");
    assert!(failures.is_empty(), "wpfl failures: {failures:?}");
}

#[test]
fn fxdat_tables_all_parse() {
    let Some(files) = game_files() else { return };
    let mut dats: Vec<&PathBuf> = files
        .iter()
        .filter(|p| {
            p.extension().is_some_and(|e| e == "dat")
                && p.file_stem()
                    .is_some_and(|s| s.to_string_lossy().ends_with("Fx"))
        })
        .collect();
    dats.sort();
    println!("effect dat files: {}", dats.len());
    assert_eq!(
        dats.len(),
        10,
        "expected 10 *Fx.dat files, found {}",
        dats.len()
    );

    let mut failures: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for path in &dats {
        let text = std::fs::read_to_string(path).unwrap();
        match FxFile::parse(&text) {
            Ok(file) => {
                println!(
                    "{}: version={} tables={} rows={}",
                    path.file_name().unwrap().to_string_lossy(),
                    file.version,
                    file.tables.len(),
                    file.row_count()
                );
                assert!(!file.tables.is_empty());
            }
            Err(e) => {
                failures
                    .entry(e.to_string())
                    .or_default()
                    .push(path.display().to_string());
            }
        }
    }
    assert!(failures.is_empty(), "dat failures: {failures:?}");
}

#[test]
fn emitter_xml_all_parse() {
    let Some(files) = game_files() else { return };
    let mut xmls: Vec<&PathBuf> = files
        .iter()
        .filter(|p| {
            p.extension().is_some_and(|e| e == "xml")
                && p.file_stem().is_some_and(|s| {
                    let n = s.to_string_lossy();
                    n == "gtaRainEmitter"
                        || n == "gtaRainRender"
                        || n == "gtaStormEmitter"
                        || n == "gtaStormRender"
                })
        })
        .collect();
    xmls.sort();
    println!("emitter xml files: {}", xmls.len());
    assert_eq!(
        xmls.len(),
        8,
        "expected 8 emitter xml files, found {}",
        xmls.len()
    );

    let mut failures: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for path in &xmls {
        let text = std::fs::read_to_string(path).unwrap();
        match emitter::parse(&text) {
            Ok(file) => {
                println!(
                    "{}: root={} props={}",
                    path.display(),
                    file.root,
                    file.props.len()
                );
                assert!(
                    file.root == "rage__ptxSimpleEmitter"
                        || file.root == "rage__ptxgpuRenderSettings"
                );
            }
            Err(e) => {
                failures
                    .entry(e.to_string())
                    .or_default()
                    .push(path.display().to_string());
            }
        }
    }
    assert!(failures.is_empty(), "xml failures: {failures:?}");
}

#[test]
// Entry names are lowercased before the extension check below, so the
// comparison is already case-insensitive.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn no_particle_resources_in_archives() {
    let Ok(game_dir) = std::env::var("LIBERTYFLUX_GAME_DIR") else {
        println!("LIBERTYFLUX_GAME_DIR unset: skipping archive scan");
        return;
    };
    let exe = Path::new(&game_dir).join("GTAIV").join("GTAIV.exe");
    let key = lf_archive::crypto::load_key_from_exe(&exe).expect("key must load from exe");
    let mut files = Vec::new();
    collect(Path::new(&game_dir), &mut files);
    let mut archives: Vec<&PathBuf> = files
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "img" || e == "rpf"))
        .collect();
    archives.sort();
    println!("archives: {}", archives.len());

    let mut hits: Vec<String> = Vec::new();
    for path in &archives {
        let mut reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
        let archive = match lf_archive::open(&mut reader, Some(&key)) {
            Ok(a) => a,
            Err(e) => panic!("cannot open {}: {e}", path.display()),
        };
        for entry in archive.entries() {
            let name_hit = entry.name.as_deref().is_some_and(|n| {
                let low = n.to_lowercase();
                low.ends_with(".wpfl") || low.ends_with(".xpfl") || low.ends_with(".pfl")
            });
            let type_hit = entry
                .resource
                .is_some_and(|r| r.type_id == 0x1B || r.type_id == 0x24);
            if name_hit || type_hit {
                hits.push(format!("{} :: {}", path.display(), entry.path));
            }
        }
    }
    println!("particle entries in archives: {}", hits.len());
    assert!(hits.is_empty(), "unexpected archived particles: {hits:?}");
}
