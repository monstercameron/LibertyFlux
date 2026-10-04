//! Integration test against the real game data.
//!
//! Reads archives and loose files from the folder in `LIBERTYFLUX_GAME_DIR`, extracts
//! members to memory only (nothing is written anywhere), and parses every
//! `.nod`, `.wnv` and `paths*.ipl` file. Skipped when the variable is unset.
//!
//! Archive keys: IMG tables in this build are AES-encrypted. The 32-byte key
//! is located at run time in the owner's executable at publicly documented
//! per-build offsets, validated by trial decryption of an archive header
//! (known-plaintext magic), kept in memory only, and never printed, logged,
//! stored or reported. This test prints only OK/no-match per trial offset.

use aes::Aes256;
use aes::cipher::{BlockDecrypt, KeyInit, generic_array::GenericArray};
use lf_nav::{ipl::IplPaths, nod::Nod, rsc::Resource, wnv::Tile};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

const IMG_MAGIC: u32 = 0xA94E_2A52;
const IMG_VERSION: u32 = 3;

/// Publicly documented per-build key offsets (offsets only, no key material).
const KEY_OFFSETS: &[(&str, usize)] = &[
    ("1.2.0.59", 0xC5B73C),
    ("1.2.0.32", 0xC5B33C),
    ("1.0.8", 0xC95FD8),
    ("1.0.7", 0xBE7540),
];

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

fn aes16(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new_from_slice(key).expect("32-byte key");
    let mut buf = data.to_vec();
    let n = buf.len() & !0x0F;
    for _ in 0..16 {
        for chunk in buf[..n].chunks_mut(16) {
            cipher.decrypt_block(GenericArray::from_mut_slice(chunk));
        }
    }
    buf
}

fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// Locate a working key using only trial-decrypt validation. Returns the key
/// bytes (memory only) or `None`.
fn load_key(game: &std::path::Path, trial_img: &[u8]) -> Option<Vec<u8>> {
    let exe = std::fs::read(game.join("GTAIV").join("GTAIV.exe")).ok()?;
    for (label, off) in KEY_OFFSETS {
        let cand = exe.get(*off..*off + 32)?;
        let dec = aes16(&trial_img[..16.min(trial_img.len())], cand);
        let ok = dec.len() >= 8 && u32le(&dec, 0) == IMG_MAGIC && u32le(&dec, 4) == IMG_VERSION;
        eprintln!(
            "  key trial {label}: {}",
            if ok { "OK" } else { "no match" }
        );
        if ok {
            return Some(cand.to_vec());
        }
    }
    None
}

struct Member {
    name: String,
    bytes: Vec<u8>,
}

/// List and extract the members of one IMG archive with `key` (`None` reads
/// open archives only).
fn read_img(path: &std::path::Path, key: Option<&[u8]>) -> Result<(bool, Vec<Member>), String> {
    let file = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if file.len() < 20 {
        return Err("short IMG header".to_string());
    }
    let encrypted = u32le(&file, 0) != IMG_MAGIC;
    let head = if encrypted {
        let key = key.ok_or_else(|| "encrypted IMG but no key".to_string())?;
        aes16(&file[..20], key)
    } else {
        file[..20].to_vec()
    };
    if u32le(&head, 0) != IMG_MAGIC || u32le(&head, 4) != IMG_VERSION {
        return Err("bad IMG magic/version".to_string());
    }
    let count = u32le(&head, 8) as usize;
    let toc_size = u32le(&head, 12) as usize;
    let toc = file
        .get(20..20 + toc_size)
        .ok_or_else(|| "short IMG toc".to_string())?;
    let toc = if encrypted {
        aes16(toc, key.unwrap())
    } else {
        toc.to_vec()
    };
    let mut entries = Vec::with_capacity(count);
    for i in 0..count {
        let e = &toc[i * 16..(i + 1) * 16];
        let w0 = u32le(e, 0);
        let blk_off = u32le(e, 8) as usize;
        let used = u16::from_le_bytes(e[12..14].try_into().unwrap()) as usize;
        let flags = u16::from_le_bytes(e[14..16].try_into().unwrap());
        let is_rsc = w0 & 0xC000_0000 != 0;
        let size = if is_rsc {
            used * 0x800 - (flags & 0x7FF) as usize
        } else {
            w0 as usize
        };
        entries.push((blk_off * 0x800, size));
    }
    let names: Vec<&[u8]> = toc[count * 16..].split(|b| *b == 0).collect();
    let mut out = Vec::with_capacity(count);
    for (i, (off, size)) in entries.iter().enumerate() {
        let name = names
            .get(i)
            .map(|n| String::from_utf8_lossy(n).into_owned())
            .unwrap_or_else(|| format!("?{i}"));
        let bytes = file
            .get(*off..off + size)
            .ok_or_else(|| format!("short member {name}"))?
            .to_vec();
        out.push(Member { name, bytes });
    }
    Ok((encrypted, out))
}

#[test]
fn nod_archives() {
    let Some(game) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset: skipping");
        return;
    };
    let mut files = 0;
    let mut nodes = 0u64;
    let mut links = 0u64;
    let mut failures: Vec<String> = Vec::new();
    for sub in [
        "pc/data/cdimages/paths.img",
        "TLAD/pc/data/cdimages/paths.img",
        "TBoGT/pc/data/cdimages/paths.img",
    ] {
        let rel: PathBuf = sub.split('/').collect();
        let (_enc, members) =
            read_img(&game.join("GTAIV").join(rel), None).expect("open paths.img");
        assert_eq!(members.len(), 64);
        // Per-file parse + partition check; collect ids for target validation.
        let mut ids: HashSet<(u16, u16)> = HashSet::new();
        let mut parsed: Vec<(String, Vec<u8>)> = Vec::new();
        for m in &members {
            files += 1;
            match Nod::parse(&m.bytes) {
                Ok(n) => {
                    nodes += u64::from(n.node_count());
                    links += u64::from(n.link_count());
                    if !n.validate_partition().is_clean() {
                        failures.push(format!("{}: partition", m.name));
                    }
                    for node in n.nodes() {
                        ids.insert((node.area(), node.id()));
                    }
                    parsed.push((m.name.clone(), m.bytes.clone()));
                }
                Err(e) => failures.push(format!("{}: {e}", m.name)),
            }
        }
        // Target existence + symmetry over the whole set.
        let mut adj: HashMap<(u16, u16), HashSet<(u16, u16)>> = HashMap::new();
        for (name, bytes) in &parsed {
            let n = Nod::parse(bytes).expect("parsed above");
            for i in 0..n.node_count() {
                let node = n.node(i).unwrap();
                let from = (node.area(), node.id());
                for l in n.links_of(i).unwrap() {
                    let to = (l.area(), l.node());
                    if !ids.contains(&to) {
                        failures.push(format!("{name}: missing target {to:?}"));
                    }
                    if to != from {
                        adj.entry(from).or_default().insert(to);
                    }
                }
            }
        }
        let mut asym = 0u64;
        for (a, set) in &adj {
            for b in set {
                if !adj.get(b).is_some_and(|s| s.contains(a)) {
                    asym += 1;
                }
            }
        }
        if asym > 0 {
            failures.push(format!("{sub}: {asym} asymmetric edges"));
        }
    }
    eprintln!(
        "nod: {files} files, {nodes} nodes, {links} links, failures: {}",
        failures.len()
    );
    for f in failures.iter().take(20) {
        eprintln!("  failure: {f}");
    }
    assert_eq!(files, 192);
    assert!(failures.is_empty(), "{} distinct failures", failures.len());
}

#[test]
fn wnv_archives() {
    let Some(game) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset: skipping");
        return;
    };
    let nav_rel: PathBuf = ["GTAIV", "pc", "data", "cdimages", "navmeshes.img"]
        .iter()
        .collect();
    let trial = std::fs::read(game.join(&nav_rel)).expect("read navmeshes.img");
    let key = load_key(&game, &trial).expect("working key");
    let mut files = 0;
    let mut verts = 0u64;
    let mut idx = 0u64;
    let mut polys = 0u64;
    let mut kinds: HashMap<String, u64> = HashMap::new();
    for sub in [
        "pc/data/cdimages/navmeshes.img",
        "TLAD/pc/data/cdimages/navmeshes.img",
        "TBoGT/pc/data/cdimages/navmeshes.img",
        "pc/data/cdimages/navmeshes_animviewer.img",
    ] {
        let rel: PathBuf = sub.split('/').collect();
        let (_enc, members) =
            read_img(&game.join("GTAIV").join(rel), Some(&key)).expect("open nav img");
        for m in members {
            files += 1;
            let fail = |kinds: &mut HashMap<String, u64>, what: &str| {
                *kinds.entry(what.to_string()).or_default() += 1;
            };
            let r = Resource::parse(&m.bytes);
            let r = match r {
                Ok(r) => r,
                Err(e) => {
                    fail(&mut kinds, &format!("rsc: {e}"));
                    continue;
                }
            };
            let t = match Tile::parse(r.payload()) {
                Ok(t) => t,
                Err(e) => {
                    fail(&mut kinds, &format!("tile: {e}"));
                    continue;
                }
            };
            verts += u64::from(t.vert_count());
            idx += u64::from(t.index_count());
            polys += u64::from(t.poly_count());
            let ir = t.validate_indices();
            if ir.out_of_range > 0 {
                fail(&mut kinds, "index out of range");
            }
            if t.validate_polygons().non_monotonic > 0 {
                fail(&mut kinds, "polygon non-monotonic");
            }
            if t.edge_count() != t.index_count() {
                eprintln!(
                    "  note: {} edge records {} vs {} indices",
                    m.name,
                    t.edge_count(),
                    t.index_count()
                );
            }
        }
    }
    eprintln!("wnv: {files} files, {verts} verts, {idx} indices, {polys} polys");
    eprintln!("wnv distinct failures: {kinds:?}");
    assert_eq!(files, 3608 + 3608 + 3611 + 12);
    assert!(kinds.is_empty(), "failures: {kinds:?}");
}

#[test]
fn paths_ipl() {
    let Some(game) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset: skipping");
        return;
    };
    let mut tn = 0;
    let mut tl = 0;
    let mut tv = 0;
    for f in ["paths.ipl", "paths2.ipl", "paths3.ipl", "paths4.ipl"] {
        let text = std::fs::read_to_string(
            game.join("GTAIV")
                .join("common")
                .join("data")
                .join("maps")
                .join(f),
        )
        .expect("read ipl");
        let p = IplPaths::parse(&text);
        eprintln!(
            "{f}: {} nodes, {} links, {} valid, {} skipped",
            p.nodes.len(),
            p.links.len(),
            p.valid_links().count(),
            p.skipped
        );
        tn += p.nodes.len();
        tl += p.links.len();
        tv += p.valid_links().count();
    }
    eprintln!("ipl total: {tn} nodes, {tl} links, {tv} valid");
    assert!(tn > 30000 && tv > 25000);
}
