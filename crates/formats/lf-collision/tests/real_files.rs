//! Whole-corpus integration test: parse every collision file in the game.
//!
//! Runs only when `LIBERTYFLUX_GAME_DIR` points at a game install; otherwise it
//! reports a skip and passes. When enabled it lists every IMG archive,
//! extracts each `.wbn`/`.wbd` entry in memory, parses it with the crate, and
//! checks the invariants the format survey established:
//!
//! - every file parses without error;
//! - every vertex lies inside its bound's stated bounding box;
//! - every polygon index is in range;
//! - every mesh marker word is `0xFFFFFFFF`;
//! - capsule base radius equals cylinder radius + length / 2;
//! - every box is exactly 8 vertices and 6 polygons.
//!
//! Archive handling (key location, IMG table decryption) lives in this test
//! only: the key is located in the owner's executable at run time through the
//! published per-build offsets, validated by trial decryption, kept in memory
//! only, and never printed, logged or stored. If `LIBERTYFLUX_STATS_OUT` names a file,
//! a JSON summary is written there for the lane report.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use aes::Aes256;
use cipher::{BlockDecrypt, KeyInit};
use lf_collision::{Bound, CollisionFile, ValidationReport};

const IMG_MAGIC: u32 = 0xA94E_2A52;
const IMG_VERSION: u32 = 3;

/// Published per-build key offsets, newest build first. Offsets only.
const KEY_OFFSETS: &[(&str, usize)] = &[
    ("1.2.0.59", 0xC5B73C),
    ("1.2.0.32", 0xC5B33C),
    ("1.0.8", 0xC95FD8),
    ("1.0.7", 0xBE7540),
];

fn game_dir() -> Option<PathBuf> {
    let dir = std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)?;
    if dir.is_dir() { Some(dir) } else { None }
}

/// AES-256-ECB applied 16 times over the 16-byte-aligned prefix.
fn aes16(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256::new(key.into());
    let mut out = data.to_vec();
    let n = out.len() & !0xF;
    for chunk in out[..n].chunks_mut(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        for _ in 0..16 {
            cipher.decrypt_block((&mut block).into());
        }
        chunk.copy_from_slice(&block);
    }
    out
}

fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

/// Locate the archive key: trial-decrypt an encrypted archive header with the
/// bytes at each published offset until the known-plaintext magic appears.
fn locate_key(game: &Path, exe: &[u8]) -> Option<[u8; 32]> {
    let sample_path = game.join("GTAIV/common/data/cdimages/script.img");
    let sample = fs::read(sample_path).ok()?;
    if sample.len() < 16 {
        return None;
    }
    for (label, off) in KEY_OFFSETS {
        if off + 32 > exe.len() {
            println!("key trial {label}: offset out of range");
            continue;
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(&exe[*off..*off + 32]);
        let dec = aes16(&sample[..16], &key);
        let ok = u32le(&dec, 0) == IMG_MAGIC && u32le(&dec, 4) == IMG_VERSION;
        println!("key trial {label}: {}", if ok { "OK" } else { "no match" });
        if ok {
            return Some(key);
        }
    }
    None
}

struct ArchiveEntry {
    name: String,
    offset: usize,
    size: usize,
}

fn read_all(path: &Path) -> Vec<u8> {
    fs::read(path).unwrap_or_default()
}

fn parse_img(path: &Path, key: &[u8; 32]) -> Result<Vec<ArchiveEntry>, String> {
    use std::io::{Read as _, Seek as _, SeekFrom};
    let mut f = fs::File::open(path).map_err(|e| format!("open: {e}"))?;
    let mut head = [0u8; 20];
    f.read_exact(&mut head)
        .map_err(|e| format!("header: {e}"))?;
    let encrypted = u32le(&head, 0) != IMG_MAGIC;
    if encrypted {
        let dec = aes16(&head, key);
        head.copy_from_slice(&dec);
    }
    let (magic, ver, count, toc_size, item) = (
        u32le(&head, 0),
        u32le(&head, 4),
        u32le(&head, 8),
        u32le(&head, 12),
        u16::from_le_bytes([head[16], head[17]]),
    );
    if magic != IMG_MAGIC || ver != IMG_VERSION || item != 0x10 {
        return Err(format!("bad header magic={magic:#x} ver={ver} item={item}"));
    }
    f.seek(SeekFrom::Start(20))
        .map_err(|e| format!("seek: {e}"))?;
    let mut toc = vec![0u8; toc_size as usize];
    f.read_exact(&mut toc).map_err(|e| format!("toc: {e}"))?;
    if encrypted {
        toc = aes16(&toc, key);
    }
    let count = count as usize;
    let mut entries = Vec::with_capacity(count);
    for i in 0..count {
        let o = i * 16;
        let w0 = u32le(&toc, o);
        let blk_off = i32::from_le_bytes([toc[o + 8], toc[o + 9], toc[o + 10], toc[o + 11]]);
        let used = u16::from_le_bytes([toc[o + 12], toc[o + 13]]) as usize;
        let flags = u16::from_le_bytes([toc[o + 14], toc[o + 15]]) as usize;
        let is_rsc = w0 & 0xC000_0000 != 0;
        let size = if is_rsc {
            used * 0x800 - (flags & 0x7FF)
        } else {
            w0 as usize
        };
        entries.push((size, (blk_off as usize) * 0x800));
    }
    let names_blob = &toc[count * 16..];
    let names: Vec<&[u8]> = names_blob.split(|b| *b == 0).collect();
    Ok(entries
        .into_iter()
        .enumerate()
        .map(|(i, (size, offset))| ArchiveEntry {
            name: names
                .get(i)
                .map(|n| String::from_utf8_lossy(n).into_owned())
                .unwrap_or_else(|| format!("?{i}")),
            offset,
            size,
        })
        .collect())
}

fn list_img_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(read) = fs::read_dir(dir) else { return };
    for entry in read.flatten() {
        let path = entry.path();
        if path.is_dir() {
            list_img_files(&path, out);
        } else if path
            .extension()
            .map(|e| e.eq_ignore_ascii_case("img"))
            .unwrap_or(false)
        {
            out.push(path);
        }
    }
}

fn read_at(path: &Path, offset: usize, size: usize) -> Vec<u8> {
    use std::io::{Read as _, Seek as _, SeekFrom};
    let mut f = fs::File::open(path).expect("archive readable");
    f.seek(SeekFrom::Start(offset as u64)).expect("seek");
    let mut buf = vec![0u8; size];
    f.read_exact(&mut buf).expect("entry bytes");
    buf
}

#[test]
fn whole_corpus_parses_and_validates() {
    let Some(game) = game_dir() else {
        println!("SKIP: set LIBERTYFLUX_GAME_DIR to a game install to run the corpus test");
        return;
    };
    println!("game dir: {}", game.display());

    // Locate the owner's executable (standard install layout, generic paths).
    let exe_bytes = ["GTAIV/GTAIV.exe", "GTAIV.exe"]
        .iter()
        .map(|rel| game.join(rel))
        .map(|p| read_all(&p))
        .find(|b| !b.is_empty())
        .expect("owner's GTAIV.exe readable under LIBERTYFLUX_GAME_DIR");
    let key = locate_key(&game, &exe_bytes).expect("archive key located and validated");
    println!("key: located and validated (in memory only)");

    let mut img_files = Vec::new();
    list_img_files(&game, &mut img_files);
    img_files.sort();
    println!("img archives: {}", img_files.len());

    let mut total = 0usize;
    let mut wbn = 0usize;
    let mut wbd = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut distinct_errors: BTreeMap<String, usize> = BTreeMap::new();
    let mut aggregate = ValidationReport::default();
    let mut kinds: BTreeMap<u8, usize> = BTreeMap::new();
    let mut vtables: BTreeMap<(u8, u32), usize> = BTreeMap::new();
    let mut capsule_rule_bad = 0usize;
    let mut sphere_tail_bad = 0usize;
    let mut box_shape_bad = 0usize;
    let mut comp_count_mismatch = 0usize;
    let mut tree_flag_bad = 0usize;
    let mut shrunk_unexpected = 0usize;
    let mut aux_patterns: BTreeMap<String, usize> = BTreeMap::new();
    let mut names_len_total = 0usize;

    for img in &img_files {
        let entries = match parse_img(img, &key) {
            Ok(e) => e,
            Err(e) => {
                println!("SKIP archive {}: {e}", img.display());
                continue;
            }
        };
        for entry in entries {
            let lower = entry.name.to_lowercase();
            let is_wbn = lower.ends_with(".wbn");
            let is_wbd = lower.ends_with(".wbd");
            if !is_wbn && !is_wbd {
                continue;
            }
            total += 1;
            if is_wbn {
                wbn += 1;
            } else {
                wbd += 1;
            }
            names_len_total += entry.name.len();
            let raw = read_at(img, entry.offset, entry.size);
            let file = match lf_collision::parse(&raw) {
                Ok(f) => f,
                Err(e) => {
                    failures.push(format!(
                        "{} :: {} :: {e}",
                        img.file_name().unwrap().to_string_lossy(),
                        entry.name
                    ));
                    *distinct_errors.entry(format!("{e}")).or_insert(0) += 1;
                    continue;
                }
            };
            // File-kind cross-check against the entry extension.
            let kind_ok = (is_wbn && file.is_wbn()) || (is_wbd && !file.is_wbn());
            if !kind_ok {
                failures.push(format!(
                    "{} :: {} :: kind mismatch (extension says {}, parsed as {})",
                    img.file_name().unwrap().to_string_lossy(),
                    entry.name,
                    if is_wbn { "wbn" } else { "wbd" },
                    if file.is_wbn() { "wbn" } else { "wbd" },
                ));
            }
            if let CollisionFile::Wbn(f) = &file {
                let tag = f.aux >> 28;
                *aux_patterns.entry(format!("tag={tag}")).or_insert(0) += 1;
            }
            for bound in file.all_bounds() {
                let h = bound.header();
                *kinds.entry(h.bound_type.byte()).or_insert(0) += 1;
                *vtables.entry((h.bound_type.byte(), h.vtable)).or_insert(0) += 1;
                match bound {
                    Bound::Sphere(s) => {
                        if (s.radius_vec.x - h.radius).abs() > 1e-3
                            || (s.radius_vec.y - h.radius).abs() > 1e-3
                            || (s.radius_vec.z - h.radius).abs() > 1e-3
                        {
                            sphere_tail_bad += 1;
                        }
                    }
                    Bound::Capsule(c) => {
                        let expect = c.radius_vec.x + c.length_vec.x / 2.0;
                        if (expect - h.radius).abs() > 1e-3 {
                            capsule_rule_bad += 1;
                        }
                    }
                    Bound::Mesh(m) => {
                        if m.mesh_kind == lf_collision::MeshKind::Box
                            && (m.vertices.len() != 8 || m.polygons.len() != 6)
                        {
                            box_shape_bad += 1;
                        }
                        let want_flag = if m.mesh_kind == lf_collision::MeshKind::Bvh {
                            1
                        } else {
                            0
                        };
                        if m.tree_flag != want_flag {
                            tree_flag_bad += 1;
                        }
                        let want_shrunk = m.mesh_kind == lf_collision::MeshKind::Geometry;
                        if m.shrunk_vertices.is_some() != want_shrunk {
                            shrunk_unexpected += 1;
                        }
                    }
                    Bound::Composite(c) => {
                        if c.max_bounds != c.num_bounds {
                            comp_count_mismatch += 1;
                        }
                    }
                    Bound::Unparsed(_) => {}
                }
            }
            aggregate.merge(&file.validate());
        }
    }

    println!("collision files: {total} (wbn={wbn} wbd={wbd})");
    println!("bound kinds: {kinds:?}");
    println!("vtable words per kind: {vtables:?}");
    println!(
        "aggregate: meshes={} verts={} polys={} tris={} quads={} max_material={}",
        aggregate.meshes,
        aggregate.vertices,
        aggregate.polygons,
        aggregate.triangles,
        aggregate.quads,
        aggregate.max_material
    );
    println!(
        "validation: outside_bbox={} index_oob={} bad_normals={} neighbour_oob={} \
         bad_markers={} unparsed={}",
        aggregate.outside_bbox,
        aggregate.index_oob,
        aggregate.bad_normals,
        aggregate.neighbour_oob,
        aggregate.bad_markers,
        aggregate.unparsed
    );
    println!(
        "shape rules: capsule_rule_bad={capsule_rule_bad} sphere_tail_bad={sphere_tail_bad} \
         box_shape_bad={box_shape_bad} comp_count_mismatch={comp_count_mismatch} \
         tree_flag_bad={tree_flag_bad} shrunk_unexpected={shrunk_unexpected}"
    );
    println!("wbn aux pointer tags: {aux_patterns:?}");
    println!("failures: {}", failures.len());
    for f in failures.iter().take(20) {
        println!("FAIL {f}");
    }
    println!("distinct errors: {distinct_errors:?}");
    // Touch names_len_total so name handling stays in the measured path.
    println!("name bytes seen: {names_len_total}");

    if let Some(out) = std::env::var_os("LIBERTYFLUX_STATS_OUT") {
        let json = format!(
            "{{\n  \"files\": {total}, \"wbn\": {wbn}, \"wbd\": {wbd},\n  \
             \"failures\": {},\n  \
             \"kinds\": {{{}}},\n  \
             \"aggregate\": {{\"meshes\": {}, \"vertices\": {}, \"polygons\": {}, \
             \"triangles\": {}, \"quads\": {}, \"max_material\": {}, \
             \"outside_bbox\": {}, \"index_oob\": {}, \"bad_normals\": {}, \
             \"neighbour_oob\": {}, \"bad_markers\": {}, \"unparsed\": {}}},\n  \
             \"shape_rules\": {{\"capsule\": {capsule_rule_bad}, \
             \"sphere\": {sphere_tail_bad}, \"box\": {box_shape_bad}, \
             \"composite_counts\": {comp_count_mismatch}, \"tree_flag\": {tree_flag_bad}, \
             \"shrunk\": {shrunk_unexpected}}}\n}}\n",
            failures.len(),
            kinds
                .iter()
                .map(|(k, n)| format!("\"{k}\": {n}"))
                .collect::<Vec<_>>()
                .join(", "),
            aggregate.meshes,
            aggregate.vertices,
            aggregate.polygons,
            aggregate.triangles,
            aggregate.quads,
            aggregate.max_material,
            aggregate.outside_bbox,
            aggregate.index_oob,
            aggregate.bad_normals,
            aggregate.neighbour_oob,
            aggregate.bad_markers,
            aggregate.unparsed,
        );
        fs::write(&out, json).expect("stats output writable");
        println!("wrote stats to {}", Path::new(&out).display());
    }

    assert!(
        total > 1000,
        "expected the full corpus, found {total} files"
    );
    assert!(
        failures.is_empty(),
        "{} files failed to parse",
        failures.len()
    );
    assert_eq!(aggregate.outside_bbox, 0, "vertices outside bounding boxes");
    assert_eq!(aggregate.index_oob, 0, "polygon indices out of range");
    assert_eq!(aggregate.bad_markers, 0, "mesh markers not 0xFFFFFFFF");
    assert_eq!(aggregate.bad_normals, 0, "non-unit face normals");
    assert_eq!(aggregate.neighbour_oob, 0, "neighbour indices out of range");
    assert_eq!(capsule_rule_bad, 0, "capsule radius rule violations");
    assert_eq!(sphere_tail_bad, 0, "sphere tail mismatches");
    assert_eq!(box_shape_bad, 0, "boxes not 8 verts / 6 polys");
    assert_eq!(comp_count_mismatch, 0, "composite max != num");
    assert_eq!(tree_flag_bad, 0, "tree flag mismatches");
    assert_eq!(shrunk_unexpected, 0, "unexpected second vertex arrays");
}
