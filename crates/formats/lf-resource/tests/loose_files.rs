//! Census over the game's real loose resources.
//!
//! Reads every file under `LIBERTYFLUX_GAME_DIR`, parses each one whose first four
//! bytes are the RSC5 magic, and reports how many parsed and every distinct
//! failure. Skipped (passing, with a notice) when `LIBERTYFLUX_GAME_DIR` is unset so
//! `cargo test` stays green without a game install. Read-only: never writes.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use lf_resource::{Codec, Resource};

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
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

fn is_rsc(path: &Path) -> bool {
    use std::io::Read;
    let mut head = [0u8; 4];
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    file.read_exact(&mut head).is_ok() && head == [b'R', b'S', b'C', 5]
}

#[test]
fn loose_resources_all_parse() {
    let game_dir = match std::env::var("LIBERTYFLUX_GAME_DIR") {
        Ok(d) => d,
        Err(_) => {
            println!("LIBERTYFLUX_GAME_DIR unset: skipping real-file census");
            return;
        }
    };
    let mut files = Vec::new();
    collect(Path::new(&game_dir), &mut files);
    let rsc: Vec<&PathBuf> = files.iter().filter(|p| is_rsc(p)).collect();
    println!("files scanned: {}", files.len());
    println!("loose RSC files: {}", rsc.len());
    assert!(
        !rsc.is_empty(),
        "no RSC files found under LIBERTYFLUX_GAME_DIR"
    );

    let mut kinds: BTreeMap<u32, usize> = BTreeMap::new();
    let mut codecs: BTreeMap<String, usize> = BTreeMap::new();
    let mut failures: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut sys_total: u64 = 0;
    let mut gfx_total: u64 = 0;

    for path in &rsc {
        let bytes = std::fs::read(path).unwrap();
        match Resource::parse(&bytes) {
            Ok(res) => {
                *kinds.entry(res.header().kind.raw()).or_insert(0) += 1;
                let codec = match res.header().codec {
                    Codec::Deflate => "deflate".to_string(),
                    Codec::Lzx => "lzx".to_string(),
                    Codec::Unknown(id) => format!("unknown({id:#x})"),
                };
                *codecs.entry(codec).or_insert(0) += 1;
                sys_total += res.system().len() as u64;
                gfx_total += res.graphics().len() as u64;
            }
            Err(e) => {
                failures
                    .entry(e.to_string())
                    .or_default()
                    .push(path.display().to_string());
            }
        }
    }

    println!("type ids: {kinds:?}");
    println!("codecs: {codecs:?}");
    println!("inflated bytes: sys={sys_total} gfx={gfx_total}");
    let failed: usize = failures.values().map(Vec::len).sum();
    println!("parsed ok: {}, failed: {}", rsc.len() - failed, failed);
    for (err, paths) in &failures {
        println!("FAIL [{err}] ({} files):", paths.len());
        for p in paths.iter().take(10) {
            println!("  {p}");
        }
    }
    assert!(failures.is_empty(), "some real resources failed to parse");
}
