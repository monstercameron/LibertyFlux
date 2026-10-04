//! `collision`: collision bounds, `.wbn` and `.wbd` (`lf-collision`).
//!
//! - `summarize <file>`: bound tree, counts and validation report (the
//!   former `lf_collision_summary` example, same output).
//! - `dump <file>`: the validation report and bound-type counts as
//!   JSON-ish text.

use std::io::Write;

use lf_collision::{Bound, CollisionFile};

use crate::cli::{CliError, CliResult, Io, json_str, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "dump"];

const USAGE: &str = "lf-inspect collision summarize|dump <collision-file>";

/// Run one `collision` subcommand.
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
    if cmd == "dump" {
        let file = lf_collision::parse(&bytes)
            .map_err(|e| CliError::failure(format!("parse error: {e}")))?;
        let r = file.validate();
        let counts: Vec<String> = file
            .bound_type_counts()
            .iter()
            .map(|(k, n)| format!("{}: {n}", json_str(k.name())))
            .collect();
        writeln!(
            io.out,
            "{{\"kind\": {}, \"bound_types\": {{{}}}, \"meshes\": {}, \"vertices\": {}, \"polygons\": {}, \"triangles\": {}, \"quads\": {}, \"clean\": {}}}",
            json_str(if file.is_wbn() { "wbn" } else { "wbd" }),
            counts.join(", "),
            r.meshes,
            r.vertices,
            r.polygons,
            r.triangles,
            r.quads,
            r.is_clean()
        )?;
        return Ok(());
    }
    let o = &mut *io.out;
    writeln!(o, "file: {path} ({} bytes on disk)", bytes.len())?;
    let file =
        lf_collision::parse(&bytes).map_err(|e| CliError::failure(format!("parse error: {e}")))?;
    match &file {
        CollisionFile::Wbn(f) => {
            writeln!(
                o,
                "kind: single-bound (root vtable {:#x}, aux {:#x})",
                f.root_vtable, f.aux
            )?;
            describe_bound(o, &f.root, "")?;
        }
        CollisionFile::Wbd(f) => {
            writeln!(
                o,
                "kind: dictionary (root vtable {:#x}, {} entries)",
                f.root_vtable,
                f.entries.len()
            )?;
            for entry in &f.entries {
                writeln!(o, "entry hash={:#010x}", entry.hash)?;
                describe_bound(o, &entry.bound, "  ")?;
            }
        }
    }
    let counts = file.bound_type_counts();
    let summary: Vec<String> = counts
        .iter()
        .map(|(k, n)| format!("{}={n}", k.name()))
        .collect();
    writeln!(o, "bound types: {}", summary.join(" "))?;
    let report = file.validate();
    writeln!(
        o,
        "meshes={} verts={} polys={} tris={} quads={} max_material={} clean={}",
        report.meshes,
        report.vertices,
        report.polygons,
        report.triangles,
        report.quads,
        report.max_material,
        report.is_clean()
    )?;
    if !report.is_clean() {
        writeln!(
            o,
            "issues: outside_bbox={} index_oob={} bad_normals={} neighbour_oob={} bad_markers={}",
            report.outside_bbox,
            report.index_oob,
            report.bad_normals,
            report.neighbour_oob,
            report.bad_markers
        )?;
    }
    Ok(())
}

fn describe_bound(o: &mut dyn Write, bound: &Bound, indent: &str) -> std::io::Result<()> {
    let h = bound.header();
    write!(
        o,
        "{indent}{} vtable={:#x} flags={} part={} radius={:.4} \
         bbox=[{:.2},{:.2},{:.2}]..[{:.2},{:.2},{:.2}]",
        h.bound_type.name(),
        h.vtable,
        h.flags,
        h.part_index,
        h.radius,
        h.bbox_min.x,
        h.bbox_min.y,
        h.bbox_min.z,
        h.bbox_max.x,
        h.bbox_max.y,
        h.bbox_max.z,
    )?;
    match bound {
        Bound::Sphere(s) => writeln!(o, " sphere_r={:.4}", s.radius_vec.x),
        Bound::Capsule(c) => writeln!(
            o,
            " capsule_r={:.4} len={:.4} (r+h/2={:.4})",
            c.radius_vec.x,
            c.length_vec.x,
            c.radius_vec.x + c.length_vec.x / 2.0
        ),
        Bound::Mesh(m) => writeln!(
            o,
            " {:?} verts={} polys={} tris={} quads={} shrunk={} tree_flag={} marker={:#x}",
            m.mesh_kind,
            m.vertices.len(),
            m.polygons.len(),
            m.triangle_count(),
            m.polygons.iter().filter(|p| p.is_quad()).count(),
            m.shrunk_vertices.is_some(),
            m.tree_flag,
            m.marker
        ),
        Bound::Composite(c) => {
            writeln!(
                o,
                " children={} (max={} num={})",
                c.children.len(),
                c.max_bounds,
                c.num_bounds
            )?;
            for (i, child) in c.children.iter().enumerate() {
                if let Some(t) = c
                    .current_matrices
                    .get(i)
                    .map(lf_collision::Matrix4::translation)
                {
                    writeln!(
                        o,
                        "{indent}  child[{i}] translate=[{:.3},{:.3},{:.3}]",
                        t.x, t.y, t.z
                    )?;
                }
                describe_bound(o, child, &format!("{indent}    "))?;
            }
            Ok(())
        }
        Bound::Unparsed(u) => writeln!(o, " kind_byte={} (tail unknown)", u.kind_byte),
    }
}
