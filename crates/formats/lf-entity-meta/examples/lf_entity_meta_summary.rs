//! Print a summary of one entity file: names, counts, sizes.
//!
//! Usage: `summary <file.wft|file.wdr|file.wdd>` or
//! `summary <archive.img:entry.wft>`. Reads only; never writes game content.

use lf_archive::Archive;
use std::io::BufReader;

// The label is lowercased before the extension checks, so the
// comparisons are already case-insensitive.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_entity_meta_summary <file> | <archive:entry>");
        std::process::exit(2);
    });
    let (bytes, label) = load(&arg).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });
    println!("file: {label} ({} bytes)", bytes.len());
    let low = label.to_lowercase();
    if low.ends_with(".wft") {
        summarize_fragment(&bytes);
    } else if low.ends_with(".wdr") {
        summarize_drawable(&bytes);
    } else if low.ends_with(".wdd") {
        summarize_dict(&bytes);
    } else {
        eprintln!("unknown extension: expected .wft, .wdr or .wdd");
        std::process::exit(1);
    }
}

fn load(arg: &str) -> Result<(Vec<u8>, String), String> {
    if let Some((arch, entry)) = arg.split_once(':')
        && std::path::Path::new(arch).exists()
    {
        // Archive member: the key comes from the owner's executable at
        // run time (LIBERTYFLUX_GAME_DIR/GTAIV/GTAIV.exe) and stays in memory.
        let game = std::env::var_os("LIBERTYFLUX_GAME_DIR")
            .ok_or("LIBERTYFLUX_GAME_DIR needed for archive:key form")?;
        let exe = std::path::PathBuf::from(game).join("GTAIV/GTAIV.exe");
        let key = lf_archive::crypto::load_key_from_exe(&exe).map_err(|e| e.to_string())?;
        let mut r = BufReader::new(std::fs::File::open(arch).map_err(|e| e.to_string())?);
        let a = lf_archive::open(&mut r, Some(&key)).map_err(|e| e.to_string())?;
        let idx = a
            .entries()
            .iter()
            .position(|e| e.path.to_lowercase().ends_with(&entry.to_lowercase()))
            .ok_or_else(|| format!("entry {entry} not found in {arch}"))?;
        let mut r = BufReader::new(std::fs::File::open(arch).map_err(|e| e.to_string())?);
        let bytes = a
            .read_file(&mut r, idx, Some(&key))
            .map_err(|e| e.to_string())?;
        return Ok((bytes, format!("{arch}:{entry}")));
    }
    let bytes = std::fs::read(arg).map_err(|e| e.to_string())?;
    Ok((bytes, arg.to_string()))
}

fn summarize_fragment(bytes: &[u8]) {
    let res = lf_model::Resource::open(bytes).expect("container");
    println!(
        "resource kind: {:#x}, sys: {} gfx: {} bytes",
        res.kind,
        res.sys.len(),
        res.gfx.len()
    );
    let frag = lf_model::Fragment::parse(&res).expect("fragment");
    let lay = lf_entity_meta::VehicleLayout::from_fragment(&frag);
    let ped = lf_entity_meta::PedModel::from_parts(&frag, None);
    println!("fragment children: {}", frag.children.len());
    println!(
        "bones: {} (vehicle kind guess: {:?}, ped rig: standard={} toes={:?})",
        lay.bone_count,
        lay.kind(),
        ped.rig.is_standard,
        ped.rig.toes
    );
    println!(
        "seats: {} doors: {} wheels: {} lights: {} extras: {:?} sirens: {}",
        lay.seat_count(),
        lay.door_count(),
        lay.wheel_count(),
        lay.lights.len(),
        lay.extra_numbers(),
        lay.sirens.len()
    );
    for m in lay.mounts() {
        println!("  {:16} local={:?} world={:?}", m.name, m.local, m.world);
    }
}

fn summarize_drawable(bytes: &[u8]) {
    let res = lf_model::Resource::open(bytes).expect("container");
    println!(
        "resource kind: {:#x}, sys: {} gfx: {} bytes",
        res.kind,
        res.sys.len(),
        res.gfx.len()
    );
    let draw = lf_model::Drawable::parse(&res).expect("drawable");
    let w = lf_entity_meta::WeaponMounts::from_drawable(&draw);
    println!("bones: {} firearm: {}", w.bone_count, w.is_firearm());
    if let Some(r) = &w.root {
        println!(
            "  root {:16} local={:?} world={:?}",
            r.name, r.local, r.world
        );
    }
    for m in w.mounts() {
        println!("  {:16} local={:?} world={:?}", m.name, m.local, m.world);
    }
    println!(
        "lods: {} models: {} verts: {}",
        draw.lods.len(),
        draw.models().count(),
        draw.vertex_count()
    );
}

fn summarize_dict(bytes: &[u8]) {
    let res = lf_model::Resource::open(bytes).expect("container");
    println!(
        "resource kind: {:#x}, sys: {} gfx: {} bytes",
        res.kind,
        res.sys.len(),
        res.gfx.len()
    );
    let dict = lf_model::DrawableDictionary::parse(&res).expect("dictionary");
    println!("entries: {}", dict.len());
    for c in lf_entity_meta::ped::components_of(&dict) {
        println!(
            "  hash={:08x} lods={} models={} verts={} indices={}",
            c.hash, c.lods, c.models, c.vertices, c.indices
        );
    }
}
