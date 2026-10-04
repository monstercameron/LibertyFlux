//! `shaderpack`: compiled shader containers, `.fxc` (`lf-shaderpack`).
//!
//! - `summarize <file.fxc>`: programs, bound names, parameters and
//!   techniques (the former `lf_shaderpack_summarize` example, same output).
//! - `list <file.fxc>`: one parameter name per line, then one technique
//!   name per line.

use lf_shaderpack::ShaderPack;

use crate::cli::{CliError, CliResult, Io, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list"];

const USAGE: &str = "lf-inspect shaderpack summarize|list <file.fxc>";

/// Run one `shaderpack` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the file
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if !COMMANDS.contains(&cmd) {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let path = one_path(args, USAGE)?;
    let bytes = read_file(path)?;
    let pack =
        ShaderPack::parse(&bytes).map_err(|e| CliError::failure(format!("parse error: {e}")))?;
    let o = &mut *io.out;
    if cmd == "list" {
        for p in pack.all_params() {
            writeln!(o, "param {}", p.name)?;
        }
        for t in &pack.techniques {
            writeln!(o, "technique {}", t.name)?;
        }
        return Ok(());
    }
    writeln!(o, "file: {path} ({} bytes)", bytes.len())?;
    writeln!(
        o,
        "programs: {} vertex, {} pixel",
        pack.vs_programs.len(),
        pack.ps_programs.len()
    )?;
    for (stage, program) in pack.programs() {
        let stage_name = match stage {
            lf_shaderpack::Stage::Vertex => "vs",
            lf_shaderpack::Stage::Pixel => "ps",
        };
        match program.validate() {
            Ok(info) => writeln!(
                o,
                "  {stage_name}: {} bytes, version {}.{}, {} instructions, {} bound names{}",
                program.bytecode.len(),
                info.major,
                info.minor,
                info.instr_count,
                program.vars.len(),
                if info.has_ctab { ", has CTAB" } else { "" },
            )?,
            Err(e) => writeln!(o, "  {stage_name}: INVALID: {e}")?,
        }
        for var in program.vars.iter().take(8) {
            writeln!(
                o,
                "    type {} reg c{} {}",
                var.type_code, var.register, var.name
            )?;
        }
        if program.vars.len() > 8 {
            writeln!(o, "    ... ({} more)", program.vars.len() - 8)?;
        }
    }
    writeln!(
        o,
        "params: {} shared, {} material",
        pack.shared_params.len(),
        pack.material_params.len()
    )?;
    for param in pack.all_params() {
        writeln!(
            o,
            "  {:?} {} (semantic {:?}, {} annotations, {} default dwords)",
            param.kind(),
            param.name,
            param.semantic,
            param.annotations.len(),
            param.defaults.len()
        )?;
    }
    writeln!(o, "techniques: {}", pack.techniques.len())?;
    for tech in &pack.techniques {
        for pass in &tech.passes {
            writeln!(
                o,
                "  {}: vs {} ps {:?} ({} states)",
                tech.name,
                pass.vs_index,
                pass.ps_index,
                pass.states.len()
            )?;
        }
    }
    if !pack.trailing_bytes.is_empty() {
        writeln!(o, "trailing bytes: {}", pack.trailing_bytes.len())?;
    }
    Ok(())
}
