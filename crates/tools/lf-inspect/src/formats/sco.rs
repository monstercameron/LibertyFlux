//! `sco`: compiled scripts (`lf-sco`).
//!
//! - `summarize <file.sco> [--exe <GTAIV.exe>] [--natives <natives.json>]
//!   [--disasm [N]]`: header, native call census and optionally the first N
//!   disassembled instructions (the former `sco_summary` example, same
//!   output). Encrypted files need `--exe` so the key can be located in
//!   memory at run time.
//! - `disasm` (same arguments): every instruction, one per line.

use std::collections::HashMap;

use lf_sco::{container, disasm, isa, key, natives};

use crate::cli::{CliError, CliResult, Io, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "disasm"];

const USAGE: &str = "lf-inspect sco summarize|disasm <file.sco> [--exe <GTAIV.exe>] [--natives <natives.json>] [--disasm [N]]";

/// Parsed command-line options.
struct Opts<'a> {
    file: &'a str,
    exe: Option<&'a str>,
    natives: Option<&'a str>,
    disasm: Option<usize>,
}

fn parse_opts(args: &[String]) -> Result<Opts<'_>, CliError> {
    let usage = || CliError::usage(format!("usage: {USAGE}"));
    let mut file = None;
    let mut exe = None;
    let mut natives_path = None;
    let mut disasm_count = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--exe" if i + 1 < args.len() => {
                exe = Some(args[i + 1].as_str());
                i += 2;
            }
            "--natives" if i + 1 < args.len() => {
                natives_path = Some(args[i + 1].as_str());
                i += 2;
            }
            "--disasm" => {
                let n = args.get(i + 1).and_then(|s| s.parse::<usize>().ok());
                if n.is_some() {
                    i += 1;
                }
                disasm_count = Some(n.unwrap_or(25));
                i += 1;
            }
            s if s.starts_with("--") || file.is_some() => return Err(usage()),
            s => {
                file = Some(s);
                i += 1;
            }
        }
    }
    Ok(Opts {
        file: file.ok_or_else(usage)?,
        exe,
        natives: natives_path,
        disasm: disasm_count,
    })
}

/// Run one `sco` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the script
/// cannot be read, decrypted or decoded.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if !COMMANDS.contains(&cmd) {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let opts = parse_opts(args)?;
    let file = opts.file;
    let bytes = read_file(file)?;
    let header =
        container::parse_header(&bytes).map_err(|e| CliError::failure(format!("{file}: {e}")))?;
    let summary = cmd == "summarize";
    let o = &mut *io.out;
    if summary {
        writeln!(o, "file bytes: {}", bytes.len())?;
        writeln!(o, "magic: 0x{:08X} ({:?})", header.magic, header.kind)?;
        writeln!(o, "code bytes: {}", header.code_len)?;
        writeln!(o, "statics slots: {}", header.statics_count)?;
        writeln!(o, "globals slots: {}", header.globals_count)?;
        writeln!(o, "args slots: {}", header.args_count)?;
        writeln!(o, "globals signature: 0x{:08X}", header.globals_signature)?;
    }
    // Locate the key in memory only when the file needs it.
    let needs_key = matches!(
        header.kind,
        container::PayloadKind::Encrypted | container::PayloadKind::EncryptedZlib
    );
    let key_buf = if needs_key {
        let exe_path = opts.exe.ok_or_else(|| {
            CliError::failure("file is encrypted; pass --exe <GTAIV.exe> to decrypt")
        })?;
        let exe_bytes = read_file(exe_path)?;
        Some(
            key::find_key(&exe_bytes)
                .ok_or_else(|| CliError::failure(format!("key not found in {exe_path}")))?,
        )
    } else {
        None
    };
    let script = container::load(&bytes, key_buf.as_ref())
        .map_err(|e| CliError::failure(format!("{file}: {e}")))?;
    if summary {
        writeln!(o, "decoded bytes: {}", script.code.len())?;
    }
    let db = match opts.natives {
        Some(path) => {
            let text = std::fs::read_to_string(path)
                .map_err(|e| CliError::failure(format!("cannot read {path}: {e}")))?;
            let db = natives::NativeDb::from_p0_natives_json(&text)
                .map_err(|e| CliError::failure(format!("cannot parse {path}: {e}")))?;
            if summary {
                writeln!(o, "native names loaded: {}", db.len())?;
            }
            Some(db)
        }
        None => None,
    };
    let insts = isa::decode_all(&script.code)
        .map_err(|e| CliError::failure(format!("disassembly stops: {e}")))?;
    if !summary {
        for inst in &insts {
            writeln!(o, "{}", disasm::format_instruction(inst, db.as_ref()))?;
        }
        return Ok(());
    }
    writeln!(o, "instructions: {}", insts.len())?;
    let mut native_calls: u64 = 0;
    let mut native_names: HashMap<String, u64> = HashMap::new();
    for inst in &insts {
        if let isa::Operand::Native { hash, .. } = inst.operand {
            native_calls += 1;
            let label = db
                .as_ref()
                .and_then(|d| d.lookup(hash))
                .map_or_else(|| format!("0x{hash:08X}"), str::to_string);
            *native_names.entry(label).or_default() += 1;
        }
    }
    writeln!(
        o,
        "native calls: {native_calls} in {} distinct targets",
        native_names.len()
    )?;
    let mut top: Vec<(&String, &u64)> = native_names.iter().collect();
    // Most-called first; ties by name so the listing is stable.
    top.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    for (name, count) in top.iter().take(10) {
        writeln!(o, "  native {name}: {count}")?;
    }
    if let Some(n) = opts.disasm {
        for inst in insts.iter().take(n) {
            writeln!(o, "{}", disasm::format_instruction(inst, db.as_ref()))?;
        }
        if insts.len() > n {
            writeln!(o, "... ({} more)", insts.len() - n)?;
        }
    }
    Ok(())
}
