//! Integration test over the real game data. Reads the shader folders when
//! `LIBERTYFLUX_GAME_DIR` points at the game folder (the install folder holding `GTAIV/`),
//! and skips otherwise. Read-only: it never writes anywhere.

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

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

#[test]
fn real_shader_files_parse_and_validate() {
    let Some(root) = game_dir() else {
        eprintln!("skipping: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    let shaders = root.join("GTAIV").join("common").join("shaders");
    if !shaders.is_dir() {
        eprintln!("skipping: {} is not a directory", shaders.display());
        return;
    }

    let mut files = 0;
    let mut failures: Vec<String> = Vec::new();
    let mut n_vs = 0;
    let mut n_ps = 0;
    let mut versions: BTreeMap<u32, usize> = BTreeMap::new();
    let mut creators: BTreeMap<String, usize> = BTreeMap::new();
    let mut shared_names: BTreeSet<String> = BTreeSet::new();
    let mut material_names: BTreeSet<String> = BTreeSet::new();
    let mut techniques: BTreeSet<String> = BTreeSet::new();
    let mut max_vs = 0;
    let mut max_ps = 0;

    for variant in VARIANTS {
        let dir = shaders.join(variant);
        let mut names: Vec<_> = fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("cannot list {}: {e}", dir.display()))
            .map(|e| e.expect("dir entry").file_name())
            .collect();
        names.sort();
        for name in names {
            let name = name.to_string_lossy().into_owned();
            if !name.ends_with(".fxc") {
                continue;
            }
            files += 1;
            let path = dir.join(&name);
            let bytes = fs::read(&path).expect("readable shader file");
            let label = format!("{variant}/{name}");
            let pack = match ShaderPack::parse(&bytes) {
                Ok(p) => p,
                Err(e) => {
                    failures.push(format!("{label}: parse error {e}"));
                    continue;
                }
            };
            if !pack.trailing_bytes.is_empty() {
                failures.push(format!(
                    "{label}: {} trailing bytes",
                    pack.trailing_bytes.len()
                ));
            }
            n_vs += pack.vs_programs.len();
            n_ps += pack.ps_programs.len();
            max_vs = max_vs.max(pack.vs_programs.len());
            max_ps = max_ps.max(pack.ps_programs.len());
            for (stage, program) in pack.programs() {
                match program.validate() {
                    Ok(info) => {
                        if info.stage != stage {
                            failures.push(format!("{label}: stage mismatch in blob"));
                        }
                        if info.end_offset != program.bytecode.len() {
                            failures.push(format!("{label}: end offset mismatch"));
                        }
                        if let Some(v) = program.version_token() {
                            *versions.entry(v).or_default() += 1;
                        }
                        if let Some(c) = creator_of(program.bytecode) {
                            *creators.entry(c).or_default() += 1;
                        }
                    }
                    Err(e) => failures.push(format!("{label}: blob error {e}")),
                }
                if program.declared_size != program.declared_size_copy {
                    failures.push(format!("{label}: size fields disagree"));
                }
            }
            for p in &pack.shared_params {
                shared_names.insert(p.name.clone());
            }
            for p in &pack.material_params {
                material_names.insert(p.name.clone());
            }
            for t in &pack.techniques {
                techniques.insert(t.name.clone());
                for pass in &t.passes {
                    if pass.vs_index >= pack.vs_programs.len() {
                        failures.push(format!("{label}: vs index out of range"));
                    }
                    if let Some(ps) = pass.ps_index {
                        if ps >= pack.ps_programs.len() {
                            failures.push(format!("{label}: ps index out of range"));
                        }
                    }
                }
            }
        }
    }

    println!("files parsed: {files}");
    println!("vertex programs: {n_vs}, pixel programs: {n_ps}");
    println!("max programs in one file: vs {max_vs}, ps {max_ps}");
    println!("bytecode versions: {versions:?}");
    println!("compilers: {creators:?}");
    println!(
        "unique names: shared {}, material {}, techniques {}",
        shared_names.len(),
        material_names.len(),
        techniques.len()
    );
    for f in failures.iter().take(20) {
        println!("FAILURE: {f}");
    }
    assert!(
        failures.is_empty(),
        "{} failures (showing 20 above)",
        failures.len()
    );
    // Pinned counts for this game version; a data change should fail loudly.
    assert_eq!(files, 612, "shader file count");
    assert_eq!(n_vs, 4998, "vertex program count");
    assert_eq!(n_ps, 5136, "pixel program count");
    assert_eq!(versions.len(), 2, "only vs_3_0 and ps_3_0 expected");
}

/// Pull the `Microsoft (R) HLSL Shader Compiler ...` tag out of a blob.
fn creator_of(blob: &[u8]) -> Option<String> {
    const TAG: &[u8] = b"Microsoft (R) HLSL Shader Compiler";
    let at = blob.windows(TAG.len()).position(|w| w == TAG)?;
    let rest = &blob[at + TAG.len()..];
    let end = rest
        .iter()
        .position(|&b| !(b.is_ascii_digit() || b == b'.'))?;
    Some(format!(
        "{} {}",
        String::from_utf8_lossy(TAG),
        String::from_utf8_lossy(&rest[..end])
    ))
}

#[test]
fn real_sidecars_parse() {
    let Some(root) = game_dir() else {
        eprintln!("skipping: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    let shaders = root.join("GTAIV").join("common").join("shaders");
    if !shaders.is_dir() {
        eprintln!("skipping: {} is not a directory", shaders.display());
        return;
    }
    // Vertex declarations: one per shader, stems match the containers.
    let mut dcl_ok = 0;
    let mut dcl_fail: Vec<String> = Vec::new();
    let mut dcl_stems = BTreeSet::new();
    for entry in fs::read_dir(shaders.join("dcl")).expect("list dcl") {
        let path = entry.expect("entry").path();
        if path.extension().map(|e| e == "dcl").unwrap_or(false) {
            let text = fs::read_to_string(&path).expect("read dcl");
            match lf_shaderpack::dcl::parse_dcl(&text) {
                Ok(_) => dcl_ok += 1,
                Err(e) => dcl_fail.push(format!("{}: {e}", path.display())),
            }
            dcl_stems.insert(path.file_stem().unwrap().to_string_lossy().into_owned());
        }
    }
    let mut fxc_stems = BTreeSet::new();
    for entry in fs::read_dir(shaders.join("win32_30")).expect("list fxc") {
        let path = entry.expect("entry").path();
        fxc_stems.insert(path.file_stem().unwrap().to_string_lossy().into_owned());
    }
    println!("dcl parsed: {dcl_ok}, failures: {}", dcl_fail.len());
    for f in dcl_fail.iter().take(10) {
        println!("DCL FAILURE: {f}");
    }
    assert!(dcl_fail.is_empty());
    assert_eq!(dcl_stems, fxc_stems, "dcl stems match fxc stems");

    // Material presets: parsed best-effort, every failure reported.
    let mut sps_ok = 0;
    let mut sps_fail: Vec<String> = Vec::new();
    for entry in fs::read_dir(shaders.join("db")).expect("list db") {
        let path = entry.expect("entry").path();
        let text = fs::read_to_string(&path).expect("read sps");
        match lf_shaderpack::sps::parse_preset(&text) {
            Ok(_) => sps_ok += 1,
            Err(e) => sps_fail.push(format!("{}: {e}", path.display())),
        }
    }
    println!("sps parsed: {sps_ok}, failures: {}", sps_fail.len());
    for f in sps_fail.iter().take(10) {
        println!("SPS FAILURE: {f}");
    }
    assert!(sps_fail.is_empty());
}
