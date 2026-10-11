//! Integration test over real game files.
//!
//! Reads the loose drawable dictionaries from the installed game when
//! `LIBERTYFLUX_GAME_DIR` is set, plus every sample in `LIBERTYFLUX_MODEL_SAMPLES` when
//! that is set too. Skipped (passing) when `LIBERTYFLUX_GAME_DIR` is unset.

use lf_model::{Drawable, DrawableDictionary, Fragment, Resource};
use std::collections::BTreeMap;

fn game_dir() -> Option<String> {
    std::env::var("LIBERTYFLUX_GAME_DIR").ok()
}

fn check_drawable(res: &Resource, d: &Drawable, tag: &str) {
    for g in d.geometries() {
        let idx = g.indices(res).expect("indices readable");
        assert_eq!(idx.len() as u32, g.index_count, "{tag}: index count");
        for &i in &idx {
            assert!(
                u32::from(i) < u32::from(g.vertex_count),
                "{tag}: index {i} out of range for {} verts",
                g.vertex_count
            );
        }
        let verts = g.vertices(res).expect("vertices decode");
        assert_eq!(
            verts.len(),
            usize::from(g.vertex_count),
            "{tag}: vert count"
        );
        let mn = d.bounds_min;
        let mx = d.bounds_max;
        for v in &verts {
            for (a, lo, hi) in [
                (v.pos[0], mn[0], mx[0]),
                (v.pos[1], mn[1], mx[1]),
                (v.pos[2], mn[2], mx[2]),
            ] {
                assert!(
                    a.is_finite() && a >= lo - 1.0 && a <= hi + 1.0,
                    "{tag}: position {a} outside [{lo}, {hi}]"
                );
            }
        }
    }
}

#[test]
fn real_files_parse_and_validate() {
    let Some(game) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset; skipping real-file test");
        return;
    };
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for name in ["plantsmgr.wdd", "radar.wdd"] {
        let p = std::path::Path::new(&game)
            .join("GTAIV/pc/models")
            .join(name);
        if p.exists() {
            files.push(p);
        }
    }
    assert!(
        !files.is_empty(),
        "no loose dictionaries found under {game}"
    );
    if let Ok(samples) = std::env::var("LIBERTYFLUX_MODEL_SAMPLES") {
        let mut extra: Vec<_> = std::fs::read_dir(&samples)
            .expect("samples dir readable")
            .filter_map(|e| e.ok().map(|x| x.path()))
            .filter(|p| {
                p.extension().is_some_and(|x| {
                    x.eq_ignore_ascii_case("wdr")
                        || x.eq_ignore_ascii_case("wdd")
                        || x.eq_ignore_ascii_case("wft")
                })
            })
            .collect();
        extra.sort();
        // Samples live one level down per archive; also scan subdirs.
        let mut nested = Vec::new();
        for scope in std::iter::once(std::path::PathBuf::from(&samples)) {
            for e in std::fs::read_dir(&scope).unwrap() {
                let p = e.unwrap().path();
                if p.is_dir() {
                    for f in std::fs::read_dir(&p).unwrap() {
                        nested.push(f.unwrap().path());
                    }
                }
            }
        }
        nested.sort();
        extra.extend(nested.into_iter().filter(|p| {
            p.extension().is_some_and(|x| {
                x.eq_ignore_ascii_case("wdr")
                    || x.eq_ignore_ascii_case("wdd")
                    || x.eq_ignore_ascii_case("wft")
            })
        }));
        files.extend(extra);
    }
    let mut ok = 0u32;
    let mut kinds: BTreeMap<u32, u32> = BTreeMap::new();
    let mut failures: Vec<String> = Vec::new();
    let mut verts = 0u64;
    let mut tris = 0u64;
    let mut geoms = 0u64;
    for f in &files {
        let tag = f.file_name().unwrap().to_string_lossy().into_owned();
        let bytes = std::fs::read(f).expect("sample readable");
        let res = match Resource::open(&bytes) {
            Ok(r) => r,
            Err(e) => {
                failures.push(format!("{tag}: resource: {e}"));
                continue;
            }
        };
        *kinds.entry(res.kind).or_insert(0) += 1;
        let parsed = (|| -> Result<(), lf_model::Error> {
            if tag.ends_with(".wft") {
                let frag = Fragment::parse(&res)?;
                check_drawable(&res, &frag.drawable, &tag);
                for ch in &frag.children {
                    assert!(ch.node.is_some(), "{tag}: child without node");
                }
            } else if tag.ends_with(".wdd") {
                let dict = DrawableDictionary::parse(&res)?;
                for d in &dict.entries {
                    check_drawable(&res, d, &tag);
                }
            } else {
                let d = Drawable::parse(&res)?;
                check_drawable(&res, &d, &tag);
            }
            Ok(())
        })();
        match parsed {
            Ok(()) => {
                ok += 1;
                // Count geometry stats on success.
                let mut count = |d: &Drawable| {
                    for g in d.geometries() {
                        geoms += 1;
                        verts += u64::from(g.vertex_count);
                        tris += u64::from(g.triangle_count().unwrap_or(0));
                    }
                };
                if tag.ends_with(".wft") {
                    count(&Fragment::parse(&res).unwrap().drawable);
                } else if tag.ends_with(".wdd") {
                    for d in &DrawableDictionary::parse(&res).unwrap().entries {
                        count(d);
                    }
                } else {
                    count(&Drawable::parse(&res).unwrap());
                }
            }
            Err(e) => failures.push(format!("{tag}: parse: {e}")),
        }
    }
    eprintln!(
        "files: {}, parsed: {}, kinds: {kinds:?}, geoms: {geoms}, verts: {verts}, tris: {tris}",
        files.len(),
        ok
    );
    for f in &failures {
        eprintln!("FAIL {f}");
    }
    assert!(failures.is_empty(), "{} failures", failures.len());
}
