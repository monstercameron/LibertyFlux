//! Runs the Khronos `spirv-val` over a corpus of translated hand-built
//! programs. `spirv-val` is not installed in the environment this crate
//! was written in, so the test is ignored by default; run it with
//! `LF_SPIRV_VAL=/path/to/spirv-val cargo test -p lf-dxso-spirv -- --ignored`.
//! The `.spv` files go to Cargo's per-test temporary folder inside the
//! build directory. The corpus is hand-built; nothing comes from the game.

mod common;

use std::path::PathBuf;
use std::process::Command;

#[test]
#[ignore = "needs spirv-val: set LF_SPIRV_VAL to its path"]
fn spirv_val_accepts_the_corpus() {
    let Some(tool) = std::env::var_os("LF_SPIRV_VAL") else {
        eprintln!("skipping: LF_SPIRV_VAL is not set");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("lf-dxso-spirv-val");
    std::fs::create_dir_all(&dir).expect("create temp folder");
    let mut failures = Vec::new();
    for (name, words, opts) in common::corpus() {
        let (m, _) = common::check_with(&words, opts);
        let path = dir.join(format!("{name}.spv"));
        std::fs::write(&path, m.to_bytes()).expect("write .spv");
        let out = Command::new(&tool)
            .arg("--target-env")
            .arg("vulkan1.0")
            .arg(&path)
            .output()
            .expect("run spirv-val");
        if !out.status.success() {
            failures.push(format!(
                "{name}: {}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "spirv-val rejected:\n{}",
        failures.join("\n")
    );
}

#[test]
fn corpus_passes_the_internal_validator() {
    for (name, words, opts) in common::corpus() {
        let (_, rep) = common::check_with(&words, opts);
        assert!(rep.instructions > 0, "{name}");
    }
}
