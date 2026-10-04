//! Print a summary of one map file (names, counts, sizes).
//!
//! Usage: `lf_mapdata_summarize <file.ide|file.ipl|file.wpl|gta.dat>`.
//! Reads the file and writes the summary to stdout; never writes anywhere.

use lf_mapdata::{ide::IdeFile, ipl::IplFile, loadlist::LoadList, wpl::WplFile};
use std::collections::BTreeMap;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_mapdata_summarize <file>");
        std::process::exit(2);
    });
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    println!("file: {path}");
    println!("size: {} bytes", bytes.len());
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".ide") {
        summarize_ide(&bytes);
    } else if lower.ends_with(".ipl") {
        summarize_ipl(&bytes);
    } else if lower.ends_with(".wpl") {
        summarize_wpl(&bytes);
    } else if lower.ends_with(".dat") || lower.ends_with(".txt") {
        summarize_loadlist(&bytes);
    } else {
        eprintln!("unknown extension; treating as load list");
        summarize_loadlist(&bytes);
    }
}

fn summarize_ide(bytes: &[u8]) {
    match IdeFile::parse(bytes) {
        Ok(f) => {
            println!("format: IDE");
            println!("records: {}", f.len());
            let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
            kinds.insert("objs", f.objs.len());
            kinds.insert("tobj", f.tobj.len());
            kinds.insert("anim", f.anim.len());
            kinds.insert("tanm", f.tanm.len());
            kinds.insert("cars", f.cars.len());
            kinds.insert("peds", f.peds.len());
            kinds.insert("weap", f.weap.len());
            kinds.insert("txdp", f.txdp.len());
            kinds.insert("amat", f.amat.len());
            kinds.insert("agrps", f.agrps.len());
            kinds.insert("hier", f.hier.len());
            kinds.insert("mlo", f.mlo.len());
            kinds.insert("2dfx", f.fx.len());
            for (k, v) in &kinds {
                if *v > 0 {
                    println!("  {k}: {v}");
                }
            }
            let mut fxkinds: BTreeMap<u32, usize> = BTreeMap::new();
            for e in &f.fx {
                *fxkinds.entry(e.kind).or_default() += 1;
            }
            if !fxkinds.is_empty() {
                println!("  2dfx kinds: {fxkinds:?}");
            }
            println!("errors: {}", f.errors.len());
            for e in f.errors.iter().take(10) {
                println!("  line {} [{}]: {}", e.line, e.section, e.error);
            }
            for u in &f.unknown_sections {
                println!("  unknown section '{}': {} records", u.name, u.records);
            }
        }
        Err(e) => println!("parse failed: {e}"),
    }
}

fn summarize_ipl(bytes: &[u8]) {
    match IplFile::parse(bytes) {
        Ok(f) => {
            println!("format: IPL (text)");
            println!("records: {}", f.len());
            println!("  blok: {}", f.blok.len());
            println!("  cull: {}", f.cull.len());
            println!("  occl: {}", f.occl.len());
            println!("  vnod: {}", f.vnod.len());
            println!("  link: {}", f.link.len());
            println!("  2dfx: {}", f.fx.len());
            println!("declared sections: {}", f.declared.join(","));
            println!("errors: {}", f.errors.len());
            for e in f.errors.iter().take(10) {
                println!("  line {} [{}]: {}", e.line, e.section, e.error);
            }
        }
        Err(e) => println!("parse failed: {e}"),
    }
}

fn summarize_wpl(bytes: &[u8]) {
    match WplFile::parse(bytes) {
        Ok(f) => {
            println!("format: WPL (binary)");
            println!("records: {}", f.len());
            println!("  inst: {}", f.inst.len());
            println!("  grge: {}", f.grge.len());
            println!("  cars: {}", f.cars.len());
            println!("  tcyc: {}", f.tcyc.len());
            println!("  mlop: {}", f.mlop.len());
            println!("  blok: {}", f.blok.len());
            println!("  lodm: {}", f.lodm.len());
            println!("  slow: {}", f.slow.len());
            println!(
                "trailing bytes: {} (all zero: {})",
                f.trailing_bytes.len(),
                f.trailing_is_zero_padding()
            );
        }
        Err(e) => println!("parse failed: {e}"),
    }
}

fn summarize_loadlist(bytes: &[u8]) {
    match LoadList::parse(bytes) {
        Ok(l) => {
            println!("format: load list");
            println!("directives: {}", l.directives.len());
            let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
            for d in &l.directives {
                *kinds.entry(d.keyword.clone()).or_default() += 1;
            }
            for (k, v) in &kinds {
                println!("  {k}: {v}");
            }
        }
        Err(e) => println!("parse failed: {e}"),
    }
}
