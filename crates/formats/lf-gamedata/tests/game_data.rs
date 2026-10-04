//! Game-folder integration test: parse every gameplay data file.
//!
//! Reads real files only when `LIBERTYFLUX_GAME_DIR` points at the game install
//! (e.g. the folder holding `GTAIV/`); otherwise the test passes silently.
//! When `LIBERTYFLUX_REPORT_OUT` is also set, a JSON summary is written there
//! (counts and errors only, never game content).
//!
//! Scope: `GTAIV/common/data`, the two episode `common/data` folders, and
//! `pc/data/timecyc*.dat`. Skipped subtrees belong to other lanes:
//! `decision/`, `fragments/`, `paths/`, `maps/`, `streaming/`, `Controls/`,
//! `cdimages/`. Skipped extensions: `.xls` (dev spreadsheets), `.png`,
//! `.sc` (script source), `.cfg` (binary input maps).

use lf_gamedata::route::{Parsed, parse_file};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const SKIP_DIRS: &[&str] = &[
    "decision",
    "fragments",
    "paths",
    "maps",
    "streaming",
    "controls",
    "cdimages",
];
const SKIP_EXT: &[&str] = &["xls", "png", "sc", "cfg", "art"];

/// Priority files: the test fails if any of these do not parse cleanly.
const PRIORITY: &[&str] = &[
    "handling.dat",
    "vehicles.ide",
    "peds.ide",
    "default.ide",
    "carcols.dat",
    "cargrp.dat",
    "pedgrp.dat",
    "pedpersonality.dat",
    "pedvariations.dat",
    "pedprops.dat",
    "weaponinfo.xml",
    "thrownweaponinfo.xml",
    "timecyc.dat",
    "timecyclemodifiers.dat",
    "timecyclemodifiers2.dat",
    "timecyclemodifiers3.dat",
    "timecyclemodifiers4.dat",
    "water.dat",
    "shorelines.dat",
    "gta.dat",
    "default.dat",
    "images.txt",
    "popcycle.dat",
    "object.dat",
    "materials.dat",
    "procedural.dat",
    "mtl_convert.txt",
    "hud.dat",
    "hudcolor.dat",
    "visualsettings.dat",
    "ped.dat",
    "relationships.dat",
    "vehoff.csv",
    "songlist.csv",
    "numplate.dat",
    "version.txt",
];

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let skip = path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| SKIP_DIRS.contains(&n.to_lowercase().as_str()));
            if !skip {
                walk(&path, out);
            }
        } else if path.is_file() {
            let skip = path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| SKIP_EXT.contains(&e.to_lowercase().as_str()));
            if !skip {
                out.push(path);
            }
        }
    }
}

fn json_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o
}

#[test]
fn parse_game_data_folder() {
    let game_dir = match std::env::var("LIBERTYFLUX_GAME_DIR") {
        Ok(d) => PathBuf::from(d),
        Err(_) => {
            println!("LIBERTYFLUX_GAME_DIR not set; skipping game-data integration test");
            return;
        }
    };
    let roots = [
        game_dir.join("GTAIV").join("common").join("data"),
        game_dir
            .join("GTAIV")
            .join("TLAD")
            .join("common")
            .join("data"),
        game_dir
            .join("GTAIV")
            .join("TBoGT")
            .join("common")
            .join("data"),
        game_dir.join("GTAIV").join("pc").join("data"),
        game_dir.join("GTAIV").join("TLAD").join("pc").join("data"),
        game_dir.join("GTAIV").join("TBoGT").join("pc").join("data"),
    ];
    // From pc/data only the timecyc files are in scope.
    let mut files: Vec<PathBuf> = Vec::new();
    for (i, root) in roots.iter().enumerate() {
        let mut found = Vec::new();
        walk(root, &mut found);
        if i >= 3 {
            found.retain(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.to_lowercase().starts_with("timecyc"))
            });
        }
        files.extend(found);
    }
    files.sort();
    assert!(!files.is_empty(), "no data files found under {game_dir:?}");

    let mut ok = 0usize;
    let mut failed: Vec<(String, String)> = Vec::new();
    let mut binary = 0usize;
    let mut scanned = 0usize;
    let mut report = String::from("{\"files\":[");
    let mut first = true;
    // Distinct failure signatures for the summary.
    let mut failure_kinds: BTreeMap<String, usize> = BTreeMap::new();

    for path in &files {
        let rel = path
            .strip_prefix(&game_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = std::fs::read(path).expect("read data file");
        let size = bytes.len();
        let outcome: Result<Parsed, String> = parse_file(&rel, &bytes).map_err(|e| e.to_string());
        // Row-level errors count as failures worth reporting.
        let first_row_error: Option<(String, String)> = match &outcome {
            Ok(Parsed::Ide(o)) => o
                .row_errors
                .first()
                .map(|(sec, line, e)| (format!("ide:{sec}"), format!("ide row {sec}:{line}: {e}"))),
            Ok(Parsed::Object(o)) => o
                .row_errors
                .first()
                .map(|(line, e)| (String::from("object"), format!("object row {line}: {e}"))),
            _ => None,
        };
        let (parser, counts, error) = match &outcome {
            Ok(p) => {
                if matches!(p, Parsed::Binary) {
                    binary += 1;
                } else if matches!(p, Parsed::Scanned(_)) {
                    scanned += 1;
                } else {
                    ok += 1;
                }
                let counts = p
                    .counts()
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", json_escape(k), v))
                    .collect::<Vec<_>>()
                    .join(",");
                let err = match &first_row_error {
                    Some((kind, msg)) => {
                        failed.push((rel.clone(), msg.clone()));
                        *failure_kinds.entry(kind.clone()).or_insert(0) += 1;
                        format!("\"{}\"", json_escape(msg))
                    }
                    None => String::from("null"),
                };
                (p.parser().to_string(), counts, err)
            }
            Err(e) => {
                failed.push((rel.clone(), e.clone()));
                *failure_kinds.entry(short_kind(e)).or_insert(0) += 1;
                (
                    "error".to_string(),
                    String::new(),
                    format!("\"{}\"", json_escape(e)),
                )
            }
        };
        if !first {
            report.push(',');
        }
        first = false;
        report.push_str(&format!(
            "{{\"path\":\"{}\",\"bytes\":{},\"parser\":\"{}\",\"counts\":{{{}}},\"error\":{}}}",
            json_escape(&rel),
            size,
            parser,
            counts,
            error
        ));
        println!(
            "{rel} [{size} bytes] parser={parser} counts={{{counts}}}{}",
            if error == "null" {
                String::new()
            } else {
                format!(" ERROR {error}")
            }
        );
    }
    report.push_str("]}");

    println!("----");
    println!(
        "files={} typed_ok={} scanned_fallback={} binary={} failed={}",
        files.len(),
        ok,
        scanned,
        binary,
        failed.len()
    );
    println!("distinct failures: {failure_kinds:?}");

    if let Ok(out) = std::env::var("LIBERTYFLUX_REPORT_OUT") {
        std::fs::write(&out, &report).expect("write JSON report");
        println!("report written to {out}");
    }

    // Priority files must parse with no file-level error.
    let mut priority_failures = Vec::new();
    for (rel, err) in &failed {
        let base = rel.rsplit('/').next().unwrap_or_default().to_lowercase();
        let row_level = err.starts_with("ide row") || err.starts_with("object row");
        if PRIORITY.contains(&base.as_str()) && !row_level {
            priority_failures.push((rel.clone(), err.clone()));
        }
    }
    assert!(
        priority_failures.is_empty(),
        "priority files failed: {priority_failures:?}"
    );
}

/// Short signature of an error string for grouping.
fn short_kind(e: &str) -> String {
    let s = e.split(':').next_back().unwrap_or(e).trim();
    s.chars().take(48).collect()
}
