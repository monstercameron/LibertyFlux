//! Translates every program in the game's shader containers when
//! `LIBERTYFLUX_GAME_DIR` points at the game folder (the install folder
//! holding `GTAIV/`), and skips otherwise. Read-only: the SPIR-V stays in
//! memory and nothing is written anywhere.
//!
//! Every program must either translate and pass the structural validator,
//! or be rejected with `Error::Unsupported`; any other error is a decoder
//! or translator bug. A histogram of unsupported reasons is printed.

use lf_dxso_spirv::validate::validate;
use lf_dxso_spirv::{Error, translate_bytes};
use lf_shaderpack::ShaderPack;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

const VARIANTS: [&str; 6] = [
    "win32_30",
    "win32_30_atidx10",
    "win32_30_low_ati",
    "win32_30_nv6",
    "win32_30_nv7",
    "win32_30_nv8",
];

#[test]
fn real_shader_programs_translate() {
    let Some(root) = std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from) else {
        eprintln!("skipping: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    let shaders = root.join("GTAIV").join("common").join("shaders");
    if !shaders.is_dir() {
        eprintln!("skipping: {} is not a directory", shaders.display());
        return;
    }
    let mut distinct: BTreeSet<Vec<u8>> = BTreeSet::new();
    let mut programs = 0usize;
    let mut translated = 0usize;
    let mut unsupported: BTreeMap<String, usize> = BTreeMap::new();
    let mut bugs: Vec<String> = Vec::new();
    for variant in VARIANTS {
        let dir = shaders.join(variant);
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        let mut names: Vec<_> = entries.map(|e| e.expect("dir entry").path()).collect();
        names.sort();
        for path in names {
            if path.extension().is_none_or(|e| e != "fxc") {
                continue;
            }
            let bytes = fs::read(&path).expect("readable shader file");
            let Ok(pack) = ShaderPack::parse(&bytes) else {
                bugs.push(format!("{}: container does not parse", path.display()));
                continue;
            };
            for (i, (_, program)) in pack.programs().enumerate() {
                programs += 1;
                if !distinct.insert(program.bytecode.to_vec()) {
                    continue;
                }
                let label = format!(
                    "{variant}/{}#{i}",
                    path.file_name().unwrap().to_string_lossy()
                );
                match translate_bytes(program.bytecode) {
                    Ok(m) => match validate(&m.words) {
                        Ok(_) => translated += 1,
                        Err(e) => bugs.push(format!("{label}: validator: {e}")),
                    },
                    Err(Error::Unsupported { what, .. }) => {
                        *unsupported.entry(what).or_default() += 1;
                    }
                    Err(e) => bugs.push(format!("{label}: {e}")),
                }
            }
        }
    }
    eprintln!(
        "programs: {programs}, distinct: {}, translated and validated: {translated}",
        distinct.len()
    );
    for (what, n) in &unsupported {
        eprintln!("unsupported x{n}: {what}");
    }
    assert!(
        bugs.is_empty(),
        "{} programs failed:\n{}",
        bugs.len(),
        bugs.join("\n")
    );
}
