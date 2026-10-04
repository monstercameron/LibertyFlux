//! Corpus test over the real script archives.
//!
//! Reads the script IMG archives from the game folder named by `LIBERTYFLUX_GAME_DIR`,
//! carves compiled scripts by magic header (archive data blocks are stored
//! raw, so no archive key is needed), locates the script AES key inside the
//! owner's executable at run time, then parses and fully disassembles every
//! script. Skipped silently when `LIBERTYFLUX_GAME_DIR` is not set.
//!
//! When `LIBERTYFLUX_STATS_OUT` names a file, per-script rows plus the opcode
//! histogram and top called natives are written there as JSON for reporting.
//! Native names come from `LIBERTYFLUX_NATIVES_JSON`, defaulting to the p0-natives
//! lane data next to this crate.

use lf_sco::{container, disasm, isa, key, natives};
use std::collections::HashMap;

const ARCHIVES: &[&str] = &[
    "GTAIV/common/data/cdimages/script.img",
    "GTAIV/common/data/cdimages/script_network.img",
    "GTAIV/common/data/cdimages/navgen_script.img",
    "GTAIV/TLAD/common/data/cdimages/script.img",
    "GTAIV/TLAD/common/data/cdimages/script_network.img",
    "GTAIV/TBoGT/common/data/cdimages/script.img",
    "GTAIV/TBoGT/common/data/cdimages/script_network.img",
];

fn read_u32_le(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

struct Carved {
    archive: &'static str,
    offset: usize,
    bytes: Vec<u8>,
}

/// Carve script files by magic. Data blocks in these archives sit at 0x800
/// boundaries, so only aligned hits with fitting length fields are kept.
fn carve(archive: &'static str, data: &[u8]) -> Vec<Carved> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while at + 24 <= data.len() {
        let magic = &data[at..at + 4];
        let is_sco =
            magic == b"SCR\x0e" || magic == b"scr\x0e" || magic == b"Scr\x0e" || magic == b"SCR\r";
        if !is_sco || !at.is_multiple_of(0x800) {
            at += 1;
            continue;
        }
        let code_len = read_u32_le(data, at + 4);
        let statics = read_u32_le(data, at + 8);
        let globals = read_u32_le(data, at + 12);
        let sane = code_len < 10_000_000 && statics < 2_000_000 && globals < 2_000_000;
        let total = if magic == b"Scr\x0e" {
            if at + 28 > data.len() {
                at += 1;
                continue;
            }
            28 + read_u32_le(data, at + 24) as usize
        } else {
            24 + code_len as usize + statics as usize * 4 + globals as usize * 4
        };
        if !sane || at + total > data.len() {
            at += 1;
            continue;
        }
        out.push(Carved {
            archive,
            offset: at,
            bytes: data[at..at + total].to_vec(),
        });
        at += total;
    }
    out
}

#[test]
fn real_scripts_parse_and_disassemble() {
    let game_dir = match std::env::var("LIBERTYFLUX_GAME_DIR") {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("LIBERTYFLUX_GAME_DIR not set; skipping real-file test");
            return;
        }
    };

    let exe_path = format!("{game_dir}/GTAIV/GTAIV.exe");
    let exe_bytes = std::fs::read(&exe_path)
        .unwrap_or_else(|e| panic!("cannot read owner's executable at {exe_path}: {e}"));
    let aes_key = key::find_key(&exe_bytes).expect("key must be found in the owner's executable");

    let natives_default = format!(
        "{}/../../p0-natives/natives.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let natives_path = std::env::var("LIBERTYFLUX_NATIVES_JSON").unwrap_or(natives_default);
    let db = match std::fs::read_to_string(&natives_path) {
        Ok(text) => {
            natives::NativeDb::from_p0_natives_json(&text).expect("native database must parse")
        }
        Err(e) => {
            eprintln!("warning: no native names ({natives_path}: {e})");
            natives::NativeDb::empty()
        }
    };
    eprintln!("native names loaded: {}", db.len());

    let mut failures: Vec<String> = Vec::new();
    let mut rows: Vec<serde_json::Value> = Vec::new();
    let mut histogram: HashMap<String, u64> = HashMap::new();
    let mut native_counts: HashMap<u32, u64> = HashMap::new();
    let mut native_users: HashMap<u32, Vec<String>> = HashMap::new();
    let mut total_instructions: u64 = 0;
    let mut total_archives = 0usize;

    for archive in ARCHIVES {
        let path = format!("{game_dir}/{archive}");
        let data = match std::fs::read(&path) {
            Ok(d) => d,
            Err(e) => {
                failures.push(format!("{archive}: cannot read archive: {e}"));
                continue;
            }
        };
        total_archives += 1;
        for carved in carve(archive, &data) {
            let label = format!("{}@{:#x}", carved.archive, carved.offset);
            let script = match container::load(&carved.bytes, Some(&aes_key)) {
                Ok(s) => s,
                Err(e) => {
                    failures.push(format!("{label}: load failed: {e}"));
                    continue;
                }
            };
            let insts = match disasm::disassemble(&script.code) {
                Ok(i) => i,
                Err(e) => {
                    failures.push(format!("{label}: disassembly failed: {e}"));
                    continue;
                }
            };
            let mut calls = 0u64;
            for inst in &insts {
                *histogram.entry(inst.opcode.mnemonic()).or_default() += 1;
                if let isa::Operand::Native { hash, .. } = inst.operand {
                    *native_counts.entry(hash).or_default() += 1;
                    native_users.entry(hash).or_default().push(label.clone());
                    calls += 1;
                }
            }
            total_instructions += insts.len() as u64;
            rows.push(serde_json::json!({
                "archive": carved.archive,
                "offset": carved.offset,
                "magic": format!("0x{:08X}", script.header.magic),
                "code_len": script.header.code_len,
                "statics": script.header.statics_count,
                "globals": script.header.globals_count,
                "args": script.header.args_count,
                "globals_signature": format!("0x{:08X}", script.header.globals_signature),
                "instructions": insts.len(),
                "native_calls": calls,
            }));
        }
    }

    eprintln!("archives read: {total_archives}");
    eprintln!("scripts carved: {}", rows.len());
    eprintln!("total instructions: {total_instructions}");
    eprintln!("distinct opcodes seen: {}", histogram.len());
    eprintln!("distinct native hashes called: {}", native_counts.len());
    let resolved = native_counts
        .keys()
        .filter(|h| db.lookup(**h).is_some())
        .count();
    eprintln!(
        "called hashes resolved to names: {resolved} of {}",
        native_counts.len()
    );
    let mut distinct_failures: HashMap<String, u64> = HashMap::new();
    for failure in &failures {
        // Group by the message with the file label stripped.
        let message = failure.split_once(": ").map(|(_, m)| m).unwrap_or(failure);
        *distinct_failures.entry(message.to_string()).or_default() += 1;
        eprintln!("FAILURE: {failure}");
    }
    eprintln!("failures: {}", failures.len());

    if let Ok(out_path) = std::env::var("LIBERTYFLUX_STATS_OUT") {
        let mut top_natives: Vec<(&u32, &u64)> = native_counts.iter().collect();
        top_natives.sort_by(|a, b| b.1.cmp(a.1));
        let top: Vec<serde_json::Value> = top_natives
            .iter()
            .take(30)
            .map(|(hash, count)| {
                serde_json::json!({
                    "hash": format!("0x{hash:08X}"),
                    "name": db.lookup(**hash),
                    "calls": *count,
                })
            })
            .collect();
        let mut all_natives: Vec<(&u32, &u64)> = native_counts.iter().collect();
        all_natives.sort_by(|a, b| b.1.cmp(a.1));
        let native_calls: Vec<serde_json::Value> = all_natives
            .iter()
            .map(|(hash, count)| {
                let mut users = native_users.get(*hash).cloned().unwrap_or_default();
                users.sort();
                users.dedup();
                let script_count = users.len();
                users.truncate(5);
                serde_json::json!({
                    "hash": format!("0x{hash:08X}"),
                    "name": db.lookup(**hash),
                    "calls": *count,
                    "scripts": script_count,
                    "sample_users": users,
                })
            })
            .collect();
        let stats = serde_json::json!({
            "archives": ARCHIVES,
            "scripts": rows.len(),
            "total_instructions": total_instructions,
            "distinct_opcodes": histogram.len(),
            "distinct_natives_called": native_counts.len(),
            "natives_resolved": resolved,
            "failures": failures.len(),
            "distinct_failures": distinct_failures,
            "opcode_histogram": histogram,
            "top_natives": top,
            "native_calls": native_calls,
            "rows": rows,
        });
        std::fs::write(&out_path, serde_json::to_string_pretty(&stats).unwrap())
            .unwrap_or_else(|e| panic!("cannot write {out_path}: {e}"));
        eprintln!("stats written to {out_path}");
    }

    assert!(
        failures.is_empty(),
        "{} of {} scripts failed; see FAILURE lines above",
        failures.len(),
        rows.len() + failures.len()
    );
}
