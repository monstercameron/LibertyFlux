//! Translate every program in one `.fxc` container to SPIR-V and write
//! one `.spv` file per program into a folder the user names. Run it on
//! your own copy of the game; the output is derived from game files and
//! must not be committed or shared.
//!
//! Usage: `lf_dxso_convert <file.fxc> <output-folder>`
//!
//! Prints one line per program with its stage, size and interface. The
//! output folder must already exist; nothing is written anywhere else.

use lf_dxso_spirv::validate::validate;
use lf_dxso_spirv::{Module, translate_bytes};
use lf_shaderpack::{ShaderPack, Stage};
use std::fs;
use std::path::{Path, PathBuf};

fn summary(m: &Module) -> String {
    let ins: Vec<String> = m
        .inputs
        .iter()
        .map(|v| match (v.location, v.builtin) {
            (Some(l), _) => format!("loc{l}"),
            (None, Some(b)) => format!("{b:?}"),
            _ => "?".into(),
        })
        .collect();
    let outs: Vec<String> = m
        .outputs
        .iter()
        .map(|v| match (v.location, v.builtin) {
            (Some(l), _) => format!("loc{l}"),
            (None, Some(b)) => format!("{b:?}"),
            _ => "?".into(),
        })
        .collect();
    let samplers: Vec<String> = m
        .samplers
        .iter()
        .map(|s| format!("s{}:{:?}@{}.{}", s.register, s.dim, s.set, s.binding))
        .collect();
    format!(
        "in [{}] out [{}] samplers [{}] c-read {} rel {}",
        ins.join(" "),
        outs.join(" "),
        samplers.join(" "),
        m.constants.float_read.len(),
        m.constants.float_relative
    )
}

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(input), Some(out_dir)) = (args.next(), args.next()) else {
        eprintln!("usage: lf_dxso_convert <file.fxc> <output-folder>");
        std::process::exit(2);
    };
    let out_dir = PathBuf::from(out_dir);
    if !out_dir.is_dir() {
        eprintln!("output folder {} does not exist", out_dir.display());
        std::process::exit(2);
    }
    let bytes = fs::read(&input).unwrap_or_else(|e| {
        eprintln!("cannot read {input}: {e}");
        std::process::exit(1);
    });
    let pack = ShaderPack::parse(&bytes).unwrap_or_else(|e| {
        eprintln!("container parse error: {e}");
        std::process::exit(1);
    });
    let stem = Path::new(&input)
        .file_stem()
        .map_or_else(|| "shader".into(), |s| s.to_string_lossy().into_owned());
    let (mut ok, mut failed) = (0, 0);
    let mut counters = [0usize; 2];
    for (stage, program) in pack.programs() {
        let (tag, n) = match stage {
            Stage::Vertex => ("vs", &mut counters[0]),
            Stage::Pixel => ("ps", &mut counters[1]),
        };
        let name = format!("{stem}.{tag}{n}.spv");
        *n += 1;
        match translate_bytes(program.bytecode) {
            Ok(m) => {
                if let Err(e) = validate(&m.words) {
                    eprintln!("{name}: internal validator rejected the output: {e}");
                    failed += 1;
                    continue;
                }
                let path = out_dir.join(&name);
                fs::write(&path, m.to_bytes()).unwrap_or_else(|e| {
                    eprintln!("cannot write {}: {e}", path.display());
                    std::process::exit(1);
                });
                println!("{name}: {} words, {}", m.words.len(), summary(&m));
                ok += 1;
            }
            Err(e) => {
                eprintln!("{name}: {e}");
                failed += 1;
            }
        }
    }
    println!("{ok} translated, {failed} failed");
    if failed > 0 {
        std::process::exit(1);
    }
}
