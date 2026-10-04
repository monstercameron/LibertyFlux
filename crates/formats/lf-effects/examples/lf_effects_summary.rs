//! Print a summary of one effect file. Read-only: never writes.
//!
//! Usage: `summary <path-to-wpfl|dat|xml>`

use std::collections::BTreeMap;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_effects_summary <effect-file>");
        std::process::exit(2);
    });
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    if bytes.starts_with(b"RSC\x05") {
        summarize_wpfl(&bytes);
    } else if let Ok(text) = std::str::from_utf8(&bytes) {
        let trimmed = text.trim_start();
        if trimmed.starts_with("<?xml") {
            summarize_xml(text);
        } else {
            summarize_dat(text);
        }
    } else {
        eprintln!("unknown file kind (not RSC, xml or text)");
        std::process::exit(1);
    }
}

fn summarize_wpfl(bytes: &[u8]) {
    let pkg = lf_effects::WpflFile::parse(bytes).unwrap_or_else(|e| {
        eprintln!("parse error: {e}");
        std::process::exit(1);
    });
    println!("kind: {:#x}", pkg.kind());
    println!("banks: {}", pkg.banks().len());
    println!("entries: {}", pkg.entry_count());
    println!("resolved names: {}", pkg.resolved_count());
    for bank in pkg.banks() {
        let mut classes: BTreeMap<u32, usize> = BTreeMap::new();
        for entry in bank.entries() {
            *classes.entry(entry.vtable).or_insert(0) += 1;
        }
        let classes: Vec<String> = classes
            .iter()
            .map(|(vtable, n)| format!("{vtable:#010x}x{n}"))
            .collect();
        println!(
            "bank slot {}: vtable={:#010x} entries={} classes={}",
            bank.slot,
            bank.vtable,
            bank.len(),
            classes.join(", ")
        );
    }
    // Show a few resolved names as a sanity check (at most five).
    let mut shown = 0;
    for (slot, entry) in pkg.entries() {
        if shown >= 5 {
            break;
        }
        if let Some(name) = pkg.name_of(entry.hash) {
            println!("slot {slot} entry {:#010x}: {name}", entry.hash);
            shown += 1;
        }
    }
}

fn summarize_dat(text: &str) {
    let file = lf_effects::FxFile::parse(text).unwrap_or_else(|e| {
        eprintln!("parse error: {e}");
        std::process::exit(1);
    });
    println!("version: {}", file.version);
    println!("tables: {}", file.tables.len());
    for table in &file.tables {
        println!(
            "table {}: rows={} width={:?}",
            table.name,
            table.len(),
            table.width()
        );
    }
}

fn summarize_xml(text: &str) {
    let file = lf_effects::emitter::parse(text).unwrap_or_else(|e| {
        eprintln!("parse error: {e}");
        std::process::exit(1);
    });
    println!("root: {}", file.root);
    println!("properties: {}", file.props.len());
    for prop in &file.props {
        let attrs: Vec<String> = prop.attrs.iter().map(|(k, v)| format!("{k}={v}")).collect();
        println!("{}: {}", prop.name, attrs.join(" "));
    }
}
