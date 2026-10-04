//! Print a summary of one save game file. Read-only: never writes.
//!
//! Usage: `lf_save_summarize <path-to-save-file>`

use std::process::ExitCode;

use lf_save::{BlockKind, SaveFile};

fn main() -> ExitCode {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: lf_save_summarize <path-to-save-file>");
            return ExitCode::from(2);
        }
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            return ExitCode::from(1);
        }
    };
    let save = match SaveFile::parse(&bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("parse error: {e}");
            return ExitCode::from(1);
        }
    };

    let header = save.header();
    println!("file bytes: {}", save.len());
    println!("version: {}", header.version);
    println!("stored size: {}", header.file_size);
    println!("globals size: {}", header.globals_size);
    println!("mission: {}", header.mission());
    println!("blocks: {}", save.blocks().len());
    if save.blocks().len() != BlockKind::COUNT {
        println!(
            "warning: expected {} blocks, found {}",
            BlockKind::COUNT,
            save.blocks().len()
        );
    }
    println!("index name              offset     size  payload  class");
    for block in save.blocks() {
        let name = block.kind.name().unwrap_or("<past documented range>");
        let class = match save.classify(block) {
            lf_save::Payload::Unknown => "raw".to_string(),
            lf_save::Payload::Resource { kind, codec } => {
                format!("rsc5 kind={kind:#x} codec={codec:#x}")
            }
            lf_save::Payload::Rpf => "rpf".to_string(),
            lf_save::Payload::Img => "img".to_string(),
        };
        println!(
            "{:>5} {:<17} {:>8} {:>8} {:>8}  {class}",
            block.index,
            name,
            block.offset,
            block.total_len,
            block.payload_len(),
        );
        if let Some(fixed) = block.kind.documented_len() {
            if fixed != block.total_len {
                println!(
                    "  warning: documented size is {fixed}, file holds {}",
                    block.total_len
                );
            }
        }
    }
    match save.checksum() {
        Some(c) => {
            let verdict = match save.verify_checksum() {
                Some(true) => "match",
                Some(false) => "MISMATCH",
                None => "unreachable",
            };
            println!(
                "checksum at {}: stored {:#010x} ({verdict})",
                c.offset, c.stored
            );
        }
        None => println!("checksum: absent"),
    }
    match save.end() {
        Some(e) => println!("end block at {}: word {:#x}", e.offset, e.word),
        None => println!("end block: absent"),
    }
    if !save.trailing_bytes().is_empty() {
        println!("trailing bytes: {}", save.trailing_bytes().len());
    }
    ExitCode::SUCCESS
}
