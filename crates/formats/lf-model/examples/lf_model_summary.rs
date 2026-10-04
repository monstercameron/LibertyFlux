//! Print a one-file summary: names, counts, sizes.
//!
//! Usage: `lf_model_summary <file.wdr|file.wdd|file.wft>`. Reads only; writes
//! nothing. Game content is never written anywhere by this tool.

use lf_model::{Drawable, DrawableDictionary, Fragment, Resource};
use std::collections::BTreeMap;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_model_summary <file>");
        std::process::exit(2);
    });
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    let res = Resource::open(&bytes).unwrap_or_else(|e| {
        eprintln!("not a resource: {e}");
        std::process::exit(1);
    });
    println!("file: {path}");
    println!("bytes: {}", bytes.len());
    println!("kind: {:#x}", res.kind);
    println!("sys: {}, gfx: {}", res.sys.len(), res.gfx.len());
    let mut drawables: Vec<Drawable> = Vec::new();
    if path.ends_with(".wft") {
        match Fragment::parse(&res) {
            Ok(f) => {
                println!("fragment children: {}", f.children.len());
                drawables.push(f.drawable);
            }
            Err(e) => {
                eprintln!("fragment parse failed: {e}");
                std::process::exit(1);
            }
        }
    } else if path.ends_with(".wdd") {
        match DrawableDictionary::parse(&res) {
            Ok(d) => {
                println!("dictionary entries: {}", d.entries.len());
                drawables.extend(d.entries);
            }
            Err(e) => {
                eprintln!("dictionary parse failed: {e}");
                std::process::exit(1);
            }
        }
    } else {
        match Drawable::parse(&res) {
            Ok(d) => drawables.push(d),
            Err(e) => {
                eprintln!("drawable parse failed: {e}");
                std::process::exit(1);
            }
        }
    }
    let mut shaders: BTreeMap<String, u32> = BTreeMap::new();
    let mut geoms = 0u32;
    let mut verts = 0u32;
    let mut idx = 0u32;
    for d in &drawables {
        println!(
            "drawable: lods={} models={} skeleton={} shaders={}",
            d.lods.len(),
            d.models().count(),
            d.skeleton.as_ref().map(|s| s.bones.len()).unwrap_or(0),
            d.shaders.as_ref().map(|s| s.shaders.len()).unwrap_or(0)
        );
        if let Some(sg) = &d.shaders {
            for s in &sg.shaders {
                *shaders.entry(s.name.clone()).or_insert(0) += 1;
            }
        }
        for g in d.geometries() {
            geoms += 1;
            verts += u32::from(g.vertex_count);
            idx += g.index_count;
        }
    }
    println!("geometries: {geoms}, vertices: {verts}, indices: {idx}");
    println!("shader names:");
    for (name, n) in &shaders {
        println!("  {n}x {name}");
    }
}
