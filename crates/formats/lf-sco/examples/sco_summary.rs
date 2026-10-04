//! Print a summary of one compiled script file.
//!
//! Usage: `sco_summary <file.sco> [--exe <GTAIV.exe>] [--natives <natives.json>] [--disasm [N]]`
//!
//! Encrypted files need `--exe` pointing at the owner's executable so the key
//! can be located in memory at run time. Native hashes resolve to names when
//! `--natives` points at the JSON name table. `--disasm N` also prints the
//! first N disassembled instructions (default 25). This tool only reads; it
//! never writes game content anywhere.

use lf_sco::{container, disasm, isa, key, natives};
use std::collections::HashMap;
use std::process::ExitCode;

fn usage() -> String {
    "usage: sco_summary <file.sco> [--exe <GTAIV.exe>] [--natives <natives.json>] [--disasm [N]]"
        .to_string()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut file: Option<&str> = None;
    let mut exe: Option<&str> = None;
    let mut natives_path: Option<&str> = None;
    let mut disasm_count: Option<usize> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--exe" if i + 1 < args.len() => {
                exe = Some(&args[i + 1]);
                i += 2;
            }
            "--natives" if i + 1 < args.len() => {
                natives_path = Some(&args[i + 1]);
                i += 2;
            }
            "--disasm" => {
                let n = args
                    .get(i + 1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(25);
                if args.get(i + 1).is_some_and(|s| s.parse::<usize>().is_ok()) {
                    i += 1;
                }
                disasm_count = Some(n);
                i += 1;
            }
            s if s.starts_with("--") || file.is_some() => {
                eprintln!("{usage}", usage = usage());
                return ExitCode::from(2);
            }
            s => {
                file = Some(s);
                i += 1;
            }
        }
    }
    let Some(file) = file else {
        eprintln!("{usage}", usage = usage());
        return ExitCode::from(2);
    };

    let bytes = match std::fs::read(file) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cannot read {file}: {e}");
            return ExitCode::from(1);
        }
    };
    let header = match container::parse_header(&bytes) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{file}: {e}");
            return ExitCode::from(1);
        }
    };
    println!("file bytes: {}", bytes.len());
    println!("magic: 0x{:08X} ({:?})", header.magic, header.kind);
    println!("code bytes: {}", header.code_len);
    println!("statics slots: {}", header.statics_count);
    println!("globals slots: {}", header.globals_count);
    println!("args slots: {}", header.args_count);
    println!("globals signature: 0x{:08X}", header.globals_signature);

    // Locate the key in memory only when the file needs it.
    let mut key_buf = None;
    let needs_key = matches!(
        header.kind,
        container::PayloadKind::Encrypted | container::PayloadKind::EncryptedZlib
    );
    if needs_key {
        let Some(exe_path) = exe else {
            eprintln!("file is encrypted; pass --exe <GTAIV.exe> to decrypt");
            return ExitCode::from(1);
        };
        let exe_bytes = match std::fs::read(exe_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("cannot read {exe_path}: {e}");
                return ExitCode::from(1);
            }
        };
        match key::find_key(&exe_bytes) {
            Some(k) => key_buf = Some(k),
            None => {
                eprintln!("key not found in {exe_path}");
                return ExitCode::from(1);
            }
        }
    }
    let script = match container::load(&bytes, key_buf.as_ref()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{file}: {e}");
            return ExitCode::from(1);
        }
    };
    println!("decoded bytes: {}", script.code.len());

    let db = match natives_path {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => match natives::NativeDb::from_p0_natives_json(&text) {
                Ok(db) => {
                    println!("native names loaded: {}", db.len());
                    Some(db)
                }
                Err(e) => {
                    eprintln!("cannot parse {path}: {e}");
                    return ExitCode::from(1);
                }
            },
            Err(e) => {
                eprintln!("cannot read {path}: {e}");
                return ExitCode::from(1);
            }
        },
        None => None,
    };

    match isa::decode_all(&script.code) {
        Ok(insts) => {
            println!("instructions: {}", insts.len());
            let mut histogram: HashMap<String, u64> = HashMap::new();
            let mut native_calls: u64 = 0;
            let mut native_names: HashMap<String, u64> = HashMap::new();
            for inst in &insts {
                *histogram.entry(inst.opcode.mnemonic()).or_default() += 1;
                if let isa::Operand::Native { hash, .. } = inst.operand {
                    native_calls += 1;
                    let label = db
                        .as_ref()
                        .and_then(|d| d.lookup(hash))
                        .map(str::to_string)
                        .unwrap_or_else(|| format!("0x{hash:08X}"));
                    *native_names.entry(label).or_default() += 1;
                }
            }
            println!(
                "native calls: {native_calls} in {} distinct targets",
                native_names.len()
            );
            let mut top: Vec<(&String, &u64)> = native_names.iter().collect();
            top.sort_by(|a, b| b.1.cmp(a.1));
            for (name, count) in top.iter().take(10) {
                println!("  native {name}: {count}");
            }
            if let Some(n) = disasm_count {
                for inst in insts.iter().take(n) {
                    println!("{}", disasm::format_instruction(inst, db.as_ref()));
                }
                if insts.len() > n {
                    println!("... ({} more)", insts.len() - n);
                }
            }
        }
        Err(e) => {
            eprintln!("disassembly stops: {e}");
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
