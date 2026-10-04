//! Integration test over the real game data.
//!
//! Walks every loose `.ide`, `.ipl` and `.wpl` file under `LIBERTYFLUX_GAME_DIR`, plus
//! the load lists, parses each, and reports counts and every distinct failure.
//! Skipped unless `LIBERTYFLUX_GAME_DIR` is set. Read-only: nothing is written anywhere.

use lf_mapdata::{ide::IdeFile, ipl::IplFile, loadlist::LoadList, wpl::WplFile};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

fn collect(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    let Ok(read) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read.flatten() {
        let p = entry.path();
        if p.is_dir() {
            collect(&p, ext, out);
        } else if p.extension().is_some_and(|e| e.eq_ignore_ascii_case(ext)) {
            out.push(p);
        }
    }
}

#[test]
fn parse_all_map_files() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR not set");
        return;
    };
    let mut ide = Vec::new();
    let mut ipl = Vec::new();
    let mut wpl = Vec::new();
    collect(&root, "ide", &mut ide);
    collect(&root, "ipl", &mut ipl);
    collect(&root, "wpl", &mut wpl);
    ide.sort();
    ipl.sort();
    wpl.sort();
    println!(
        "files: {} ide, {} ipl, {} wpl",
        ide.len(),
        ipl.len(),
        wpl.len()
    );

    let mut failures: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut ide_sections: BTreeSet<String> = BTreeSet::new();
    let mut ipl_sections: BTreeSet<String> = BTreeSet::new();
    let mut ide_fx_kinds: BTreeMap<u32, usize> = BTreeMap::new();
    let mut ipl_fx_kinds: BTreeMap<u32, usize> = BTreeMap::new();
    let mut ide_unknown: BTreeMap<String, usize> = BTreeMap::new();
    let mut ipl_empty: BTreeMap<String, usize> = BTreeMap::new();
    let mut wpl_counts = [0u64; 16];
    let mut wpl_trailing = 0usize;
    let (mut ide_ok, mut ipl_ok, mut wpl_ok) = (0, 0, 0);
    let (mut ide_recs, mut ipl_recs, mut wpl_recs) = (0usize, 0usize, 0usize);

    for p in &ide {
        let bytes = std::fs::read(p).unwrap();
        match IdeFile::parse(&bytes) {
            Ok(f) => {
                ide_ok += 1;
                ide_recs += f.len();
                for k in [
                    ("objs", f.objs.len()),
                    ("tobj", f.tobj.len()),
                    ("anim", f.anim.len()),
                    ("tanm", f.tanm.len()),
                    ("cars", f.cars.len()),
                    ("peds", f.peds.len()),
                    ("weap", f.weap.len()),
                    ("txdp", f.txdp.len()),
                    ("amat", f.amat.len()),
                    ("agrps", f.agrps.len()),
                    ("hier", f.hier.len()),
                    ("mlo", f.mlo.len()),
                    ("2dfx", f.fx.len()),
                ] {
                    if k.1 > 0 {
                        ide_sections.insert(k.0.to_string());
                    }
                }
                if f.has_tree {
                    ide_sections.insert("tree".to_string());
                }
                if f.has_path {
                    ide_sections.insert("path".to_string());
                }
                for e in &f.fx {
                    *ide_fx_kinds.entry(e.kind).or_default() += 1;
                }
                for u in &f.unknown_sections {
                    *ide_unknown.entry(u.name.clone()).or_default() += u.records;
                }
                for e in &f.errors {
                    failures
                        .entry(format!("ide record: {}", e.error))
                        .or_default()
                        .push(format!("{}:{}", p.display(), e.line));
                }
            }
            Err(e) => {
                failures
                    .entry(format!("ide file: {e}"))
                    .or_default()
                    .push(p.display().to_string());
            }
        }
    }

    for p in &ipl {
        let bytes = std::fs::read(p).unwrap();
        match IplFile::parse(&bytes) {
            Ok(f) => {
                ipl_ok += 1;
                ipl_recs += f.len();
                for d in &f.declared {
                    ipl_sections.insert(d.clone());
                    let nonempty = match d.as_str() {
                        "blok" => f.blok.len(),
                        "cull" => f.cull.len(),
                        "occl" => f.occl.len(),
                        "vnod" => f.vnod.len(),
                        "link" => f.link.len(),
                        "2dfx" => f.fx.len(),
                        _ => 0,
                    };
                    if nonempty == 0 {
                        *ipl_empty.entry(d.clone()).or_default() += 1;
                    }
                }
                for e in &f.fx {
                    *ipl_fx_kinds.entry(e.kind).or_default() += 1;
                }
                for e in &f.errors {
                    failures
                        .entry(format!("ipl record: {}", e.error))
                        .or_default()
                        .push(format!("{}:{}", p.display(), e.line));
                }
            }
            Err(e) => {
                failures
                    .entry(format!("ipl file: {e}"))
                    .or_default()
                    .push(p.display().to_string());
            }
        }
    }

    for p in &wpl {
        let bytes = std::fs::read(p).unwrap();
        match WplFile::parse(&bytes) {
            Ok(f) => {
                wpl_ok += 1;
                wpl_recs += f.len();
                for (i, c) in f.counts.iter().enumerate() {
                    wpl_counts[i] += u64::from(*c);
                }
                if !f.trailing_bytes.is_empty() {
                    wpl_trailing += 1;
                    if !f.trailing_is_zero_padding() {
                        failures
                            .entry("wpl: non-zero trailing bytes".to_string())
                            .or_default()
                            .push(p.display().to_string());
                    }
                }
            }
            Err(e) => {
                failures
                    .entry(format!("wpl file: {e}"))
                    .or_default()
                    .push(p.display().to_string());
            }
        }
    }

    // Load lists.
    for name in [
        "common/data/gta.dat",
        "common/data/cj_gta.dat",
        "common/data/default.dat",
    ] {
        for prefix in ["GTAIV", "GTAIV/TBoGT", "GTAIV/TLAD"] {
            let p = root.join(prefix).join(name);
            if p.is_file() {
                let bytes = std::fs::read(&p).unwrap();
                match LoadList::parse(&bytes) {
                    Ok(l) => println!(
                        "{}: {} directives ({} IDE, {} IPL)",
                        p.display(),
                        l.directives.len(),
                        l.ide_files().len(),
                        l.ipl_files().len()
                    ),
                    Err(e) => {
                        failures
                            .entry(format!("loadlist: {e}"))
                            .or_default()
                            .push(p.display().to_string());
                    }
                }
            }
        }
    }

    println!(
        "parsed ok: {ide_ok}/{} ide, {ipl_ok}/{} ipl, {wpl_ok}/{} wpl",
        ide.len(),
        ipl.len(),
        wpl.len()
    );
    println!("records: {ide_recs} ide, {ipl_recs} ipl, {wpl_recs} wpl");
    println!("ide sections seen: {ide_sections:?}");
    println!("ide 2dfx kinds: {ide_fx_kinds:?}");
    println!("ide unknown sections: {ide_unknown:?}");
    println!("ipl sections declared: {ipl_sections:?}");
    println!("ipl always-empty counts: {ipl_empty:?}");
    println!("ipl 2dfx kinds: {ipl_fx_kinds:?}");
    println!("wpl total counts by section: {wpl_counts:?}");
    println!("wpl files with trailing bytes: {wpl_trailing}");
    println!("distinct failures: {}", failures.len());
    for (kind, files) in &failures {
        println!("FAIL {kind} ({}x) e.g. {}", files.len(), files[0]);
    }
    assert!(failures.is_empty(), "parse failures: {failures:?}");
}
