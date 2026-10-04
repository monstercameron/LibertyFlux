//! Print a short summary of one collision file.
//!
//! Usage: `lf_collision_summary <path-to-.wbn-or-.wbd>` (a raw extracted resource file).
//! Reads the file, parses it, and prints names, counts and sizes to stdout.
//! It never writes anything anywhere.

use std::env;
use std::fs;
use std::process::ExitCode;

use lf_collision::{Bound, CollisionFile};

fn describe_bound(bound: &Bound, indent: &str) {
    let h = bound.header();
    print!(
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
    );
    match bound {
        Bound::Sphere(s) => println!(" sphere_r={:.4}", s.radius_vec.x),
        Bound::Capsule(c) => println!(
            " capsule_r={:.4} len={:.4} (r+h/2={:.4})",
            c.radius_vec.x,
            c.length_vec.x,
            c.radius_vec.x + c.length_vec.x / 2.0
        ),
        Bound::Mesh(m) => println!(
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
            println!(
                " children={} (max={} num={})",
                c.children.len(),
                c.max_bounds,
                c.num_bounds
            );
            for (i, child) in c.children.iter().enumerate() {
                let t = c.current_matrices.get(i).map(|m| m.translation());
                if let Some(t) = t {
                    println!(
                        "{indent}  child[{i}] translate=[{:.3},{:.3},{:.3}]",
                        t.x, t.y, t.z
                    );
                }
                describe_bound(child, &format!("{indent}    "));
            }
        }
        Bound::Unparsed(u) => println!(" kind_byte={} (tail unknown)", u.kind_byte),
    }
}

fn main() -> ExitCode {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_collision_summary <collision-file>");
        std::process::exit(2);
    });
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            return ExitCode::from(1);
        }
    };
    println!("file: {path} ({} bytes on disk)", bytes.len());
    let file = match lf_collision::parse(&bytes) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("parse error: {e}");
            return ExitCode::from(1);
        }
    };
    match &file {
        CollisionFile::Wbn(f) => {
            println!(
                "kind: single-bound (root vtable {:#x}, aux {:#x})",
                f.root_vtable, f.aux
            );
            describe_bound(&f.root, "");
        }
        CollisionFile::Wbd(f) => {
            println!(
                "kind: dictionary (root vtable {:#x}, {} entries)",
                f.root_vtable,
                f.entries.len()
            );
            for entry in &f.entries {
                println!("entry hash={:#010x}", entry.hash);
                describe_bound(&entry.bound, "  ");
            }
        }
    }
    let counts = file.bound_type_counts();
    let summary: Vec<String> = counts
        .iter()
        .map(|(k, n)| format!("{}={n}", k.name()))
        .collect();
    println!("bound types: {}", summary.join(" "));
    let report = file.validate();
    println!(
        "meshes={} verts={} polys={} tris={} quads={} max_material={} clean={}",
        report.meshes,
        report.vertices,
        report.polygons,
        report.triangles,
        report.quads,
        report.max_material,
        report.is_clean()
    );
    if !report.is_clean() {
        println!(
            "issues: outside_bbox={} index_oob={} bad_normals={} neighbour_oob={} bad_markers={}",
            report.outside_bbox,
            report.index_oob,
            report.bad_normals,
            report.neighbour_oob,
            report.bad_markers
        );
    }
    ExitCode::SUCCESS
}
