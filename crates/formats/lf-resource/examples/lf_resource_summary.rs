//! Print a summary of one RSC5 resource file. Read-only: never writes.
//!
//! Usage: `cargo run --example summary -- <path-to-resource>`

use std::path::PathBuf;

use lf_resource::{Resource, Segment};

fn main() {
    let path: PathBuf = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            eprintln!("usage: lf_resource_summary <resource-file>");
            std::process::exit(2);
        });
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {e}", path.display());
        std::process::exit(1);
    });
    let res = Resource::parse(&bytes).unwrap_or_else(|e| {
        eprintln!("cannot parse {}: {e}", path.display());
        std::process::exit(1);
    });
    let header = res.header();
    println!("file:      {}", path.display());
    println!("disk size: {} bytes", bytes.len());
    println!("kind:      {}", header.kind);
    println!("flags:     {:#010x}", header.flags);
    println!("codec:     {:?}", header.codec);
    println!("system:    {} bytes", res.system().len());
    println!("graphics:  {} bytes", res.graphics().len());
    match res.pg_base() {
        Ok(pg) => {
            println!(
                "pgBase:    vtable={:#010x} blockmap={}",
                pg.vtable, pg.block_map
            );
            match res.block_map() {
                Ok(bm) => println!("blockmap:  {:?} target={}", bm.state, bm.target),
                Err(e) => println!("blockmap:  error: {e}"),
            }
        }
        Err(e) => println!("pgBase:    unreadable: {e}"),
    }
    for (seg, _) in res.segments() {
        let n = res.scan_pointers(seg).count();
        let name = match seg {
            Segment::System => "system",
            Segment::Graphics => "graphics",
        };
        println!("plausible pointers in {name}: {n}");
    }
}
