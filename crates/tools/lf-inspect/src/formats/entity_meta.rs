//! `entity-meta`: vehicle, weapon and ped data inside model resources
//! (`lf-entity-meta`).
//!
//! - `summarize <file.wft|file.wdr|file.wdd>` or
//!   `summarize <archive.img:entry.wft>`: mounts, counts and sizes (the
//!   former `lf_entity_meta_summary` example, same output). The archive form
//!   needs `LIBERTYFLUX_GAME_DIR` so the key can be located in the owner's
//!   executable at run time (memory only).

use std::io::BufReader;

use lf_archive::Archive;

use crate::cli::{CliError, CliResult, Io, has_ext, load_key, one_path};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize"];

const USAGE: &str = "lf-inspect entity-meta summarize <file> | <archive:entry>";

/// Run one `entity-meta` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the input
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if cmd != "summarize" {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let arg = one_path(args, USAGE)?;
    let (bytes, label) = load(arg).map_err(|e| CliError::failure(format!("error: {e}")))?;
    writeln!(io.out, "file: {label} ({} bytes)", bytes.len())?;
    let low = label.to_lowercase();
    let o = &mut *io.out;
    if has_ext(&low, "wft") {
        summarize_fragment(o, &bytes)
    } else if has_ext(&low, "wdr") {
        summarize_drawable(o, &bytes)
    } else if has_ext(&low, "wdd") {
        summarize_dict(o, &bytes)
    } else {
        Err(CliError::failure(
            "unknown extension: expected .wft, .wdr or .wdd",
        ))
    }
}

fn load(arg: &str) -> Result<(Vec<u8>, String), String> {
    if let Some((arch, entry)) = arg.split_once(':')
        && std::path::Path::new(arch).exists()
    {
        let game = std::env::var_os("LIBERTYFLUX_GAME_DIR")
            .ok_or("LIBERTYFLUX_GAME_DIR needed for archive:key form")?;
        let exe = std::path::PathBuf::from(game).join("GTAIV/GTAIV.exe");
        let key = load_key(&exe.to_string_lossy()).map_err(|e| e.message)?;
        let mut r = BufReader::new(std::fs::File::open(arch).map_err(|e| e.to_string())?);
        let a = lf_archive::open(&mut r, Some(&key)).map_err(|e| e.to_string())?;
        let idx = a
            .entries()
            .iter()
            .position(|e| e.path.to_lowercase().ends_with(&entry.to_lowercase()))
            .ok_or_else(|| format!("entry {entry} not found in {arch}"))?;
        let bytes = a
            .read_file(&mut r, idx, Some(&key))
            .map_err(|e| e.to_string())?;
        return Ok((bytes, format!("{arch}:{entry}")));
    }
    let bytes = std::fs::read(arg).map_err(|e| e.to_string())?;
    Ok((bytes, arg.to_string()))
}

fn open(bytes: &[u8]) -> Result<lf_model::Resource, CliError> {
    lf_model::Resource::open(bytes).map_err(|e| CliError::failure(format!("container: {e}")))
}

fn header(o: &mut dyn std::io::Write, res: &lf_model::Resource) -> std::io::Result<()> {
    writeln!(
        o,
        "resource kind: {:#x}, sys: {} gfx: {} bytes",
        res.kind,
        res.sys.len(),
        res.gfx.len()
    )
}

fn summarize_fragment(o: &mut dyn std::io::Write, bytes: &[u8]) -> CliResult {
    let res = open(bytes)?;
    header(o, &res)?;
    let frag =
        lf_model::Fragment::parse(&res).map_err(|e| CliError::failure(format!("fragment: {e}")))?;
    let lay = lf_entity_meta::VehicleLayout::from_fragment(&frag);
    let ped = lf_entity_meta::PedModel::from_parts(&frag, None);
    writeln!(o, "fragment children: {}", frag.children.len())?;
    writeln!(
        o,
        "bones: {} (vehicle kind guess: {:?}, ped rig: standard={} toes={:?})",
        lay.bone_count,
        lay.kind(),
        ped.rig.is_standard,
        ped.rig.toes
    )?;
    writeln!(
        o,
        "seats: {} doors: {} wheels: {} lights: {} extras: {:?} sirens: {}",
        lay.seat_count(),
        lay.door_count(),
        lay.wheel_count(),
        lay.lights.len(),
        lay.extra_numbers(),
        lay.sirens.len()
    )?;
    for m in lay.mounts() {
        writeln!(o, "  {:16} local={:?} world={:?}", m.name, m.local, m.world)?;
    }
    Ok(())
}

fn summarize_drawable(o: &mut dyn std::io::Write, bytes: &[u8]) -> CliResult {
    let res = open(bytes)?;
    header(o, &res)?;
    let draw =
        lf_model::Drawable::parse(&res).map_err(|e| CliError::failure(format!("drawable: {e}")))?;
    let w = lf_entity_meta::WeaponMounts::from_drawable(&draw);
    writeln!(o, "bones: {} firearm: {}", w.bone_count, w.is_firearm())?;
    if let Some(r) = &w.root {
        writeln!(
            o,
            "  root {:16} local={:?} world={:?}",
            r.name, r.local, r.world
        )?;
    }
    for m in w.mounts() {
        writeln!(o, "  {:16} local={:?} world={:?}", m.name, m.local, m.world)?;
    }
    writeln!(
        o,
        "lods: {} models: {} verts: {}",
        draw.lods.len(),
        draw.models().count(),
        draw.vertex_count()
    )?;
    Ok(())
}

fn summarize_dict(o: &mut dyn std::io::Write, bytes: &[u8]) -> CliResult {
    let res = open(bytes)?;
    header(o, &res)?;
    let dict = lf_model::DrawableDictionary::parse(&res)
        .map_err(|e| CliError::failure(format!("dictionary: {e}")))?;
    writeln!(o, "entries: {}", dict.len())?;
    for c in lf_entity_meta::ped::components_of(&dict) {
        writeln!(
            o,
            "  hash={:08x} lods={} models={} verts={} indices={}",
            c.hash, c.lods, c.models, c.vertices, c.indices
        )?;
    }
    Ok(())
}
