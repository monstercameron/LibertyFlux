//! `model`: drawables, drawable dictionaries and fragments (`lf-model`).
//!
//! - `summarize <file.wdr|file.wdd|file.wft>`: LODs, models, geometry
//!   totals and shader names (the former `lf_model_summary` example, same
//!   output). The kind follows the file extension.
//! - `list <file>`: one geometry per line: LOD, model, vertices, indices.

use std::collections::BTreeMap;

use lf_model::{Drawable, DrawableDictionary, Fragment, Resource};

use crate::cli::{CliError, CliResult, Io, has_ext, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list"];

const USAGE: &str = "lf-inspect model summarize|list <file.wdr|file.wdd|file.wft>";

/// Run one `model` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the file
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if !COMMANDS.contains(&cmd) {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let path = one_path(args, USAGE)?;
    let bytes = read_file(path)?;
    let res =
        Resource::open(&bytes).map_err(|e| CliError::failure(format!("not a resource: {e}")))?;
    let summary = cmd == "summarize";
    let o = &mut *io.out;
    if summary {
        writeln!(o, "file: {path}")?;
        writeln!(o, "bytes: {}", bytes.len())?;
        writeln!(o, "kind: {:#x}", res.kind)?;
        writeln!(o, "sys: {}, gfx: {}", res.sys.len(), res.gfx.len())?;
    }
    let mut drawables: Vec<Drawable> = Vec::new();
    if has_ext(path, "wft") {
        let f = Fragment::parse(&res)
            .map_err(|e| CliError::failure(format!("fragment parse failed: {e}")))?;
        if summary {
            writeln!(o, "fragment children: {}", f.children.len())?;
        }
        drawables.push(f.drawable);
    } else if has_ext(path, "wdd") {
        let d = DrawableDictionary::parse(&res)
            .map_err(|e| CliError::failure(format!("dictionary parse failed: {e}")))?;
        if summary {
            writeln!(o, "dictionary entries: {}", d.entries.len())?;
        }
        drawables.extend(d.entries);
    } else {
        drawables.push(
            Drawable::parse(&res)
                .map_err(|e| CliError::failure(format!("drawable parse failed: {e}")))?,
        );
    }
    if !summary {
        for (di, d) in drawables.iter().enumerate() {
            for (li, lod) in d.lods.iter().enumerate() {
                for (mi, m) in lod.models.iter().enumerate() {
                    for g in &m.geometries {
                        writeln!(
                            o,
                            "drawable {di} lod {li} model {mi}: verts={} indices={}",
                            g.vertex_count, g.index_count
                        )?;
                    }
                }
            }
        }
        return Ok(());
    }
    let mut shaders: BTreeMap<String, u32> = BTreeMap::new();
    let mut geoms = 0u32;
    let mut verts = 0u32;
    let mut idx = 0u32;
    for d in &drawables {
        writeln!(
            o,
            "drawable: lods={} models={} skeleton={} shaders={}",
            d.lods.len(),
            d.models().count(),
            d.skeleton.as_ref().map_or(0, |s| s.bones.len()),
            d.shaders.as_ref().map_or(0, |s| s.shaders.len())
        )?;
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
    writeln!(o, "geometries: {geoms}, vertices: {verts}, indices: {idx}")?;
    writeln!(o, "shader names:")?;
    for (name, n) in &shaders {
        writeln!(o, "  {n}x {name}")?;
    }
    Ok(())
}
