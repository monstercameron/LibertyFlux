//! Print a human-readable summary of one `.fxc` file. Read-only: it never
//! writes game content anywhere.
//!
//! Usage: `lf_shaderpack_summarize <file.fxc>`

use lf_shaderpack::ShaderPack;
use std::fs;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_shaderpack_summarize <file.fxc>");
        std::process::exit(2);
    });
    let bytes = fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {e}", path);
        std::process::exit(1);
    });
    let pack = ShaderPack::parse(&bytes).unwrap_or_else(|e| {
        eprintln!("parse error: {e}");
        std::process::exit(1);
    });
    println!("file: {path} ({} bytes)", bytes.len());
    println!(
        "programs: {} vertex, {} pixel",
        pack.vs_programs.len(),
        pack.ps_programs.len()
    );
    for (stage, program) in pack.programs() {
        let stage_name = match stage {
            lf_shaderpack::Stage::Vertex => "vs",
            lf_shaderpack::Stage::Pixel => "ps",
        };
        match program.validate() {
            Ok(info) => println!(
                "  {stage_name}: {} bytes, version {}.{}, {} instructions, {} bound names{}",
                program.bytecode.len(),
                info.major,
                info.minor,
                info.instr_count,
                program.vars.len(),
                if info.has_ctab { ", has CTAB" } else { "" },
            ),
            Err(e) => println!("  {stage_name}: INVALID: {e}"),
        }
        for var in program.vars.iter().take(8) {
            println!(
                "    type {} reg c{} {}",
                var.type_code, var.register, var.name
            );
        }
        if program.vars.len() > 8 {
            println!("    ... ({} more)", program.vars.len() - 8);
        }
    }
    println!(
        "params: {} shared, {} material",
        pack.shared_params.len(),
        pack.material_params.len()
    );
    for param in pack.all_params() {
        println!(
            "  {:?} {} (semantic {:?}, {} annotations, {} default dwords)",
            param.kind(),
            param.name,
            param.semantic,
            param.annotations.len(),
            param.defaults.len()
        );
    }
    println!("techniques: {}", pack.techniques.len());
    for tech in &pack.techniques {
        for pass in &tech.passes {
            println!(
                "  {}: vs {} ps {:?} ({} states)",
                tech.name,
                pass.vs_index,
                pass.ps_index,
                pass.states.len()
            );
        }
    }
    if !pack.trailing_bytes.is_empty() {
        println!("trailing bytes: {}", pack.trailing_bytes.len());
    }
}
