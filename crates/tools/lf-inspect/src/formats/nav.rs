//! `nav`: navigation meshes and path graphs (`lf-nav`).
//!
//! - `summarize <file.wnv|file.nod|paths.ipl>`: tile size and validation,
//!   node and link counts, or text path counts (the former
//!   `lf_nav_summarize` example, same output). The kind follows the file
//!   extension; anything else is read as path text.

use std::collections::HashSet;

use lf_nav::{ipl::IplPaths, nod::Nod, rsc::Resource, wnv::Tile};

use crate::cli::{CliError, CliResult, Io, has_ext, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize"];

const USAGE: &str = "lf-inspect nav summarize <file.wnv|file.nod|paths.ipl>";

/// Run one `nav` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the file
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if cmd != "summarize" {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let path = one_path(args, USAGE)?;
    let bytes = read_file(path)?;
    let o = &mut *io.out;
    writeln!(o, "file: {path} ({} bytes)", bytes.len())?;
    let fail = |what: &str, e: lf_nav::Error| CliError::failure(format!("{what}: {e}"));
    if has_ext(path, "wnv") {
        let r = Resource::parse(&bytes).map_err(|e| fail("rsc parse", e))?;
        writeln!(
            o,
            "container: RSC5 type {} flags {:#x}, payload {} bytes",
            r.resource_type(),
            r.flags(),
            r.payload().len()
        )?;
        let t = Tile::parse(r.payload()).map_err(|e| fail("tile parse", e))?;
        writeln!(
            o,
            "tile: {:.1} x {:.1} m, z extent {:.1} m, {} verts, {} indices, {} polys",
            t.size_x(),
            t.size_y(),
            t.z_extent(),
            t.vert_count(),
            t.index_count(),
            t.poly_count()
        )?;
        let ir = t.validate_indices();
        let pr = t.validate_polygons();
        writeln!(
            o,
            "validation: {} indices out of range, {}/{} verts referenced, {} non-monotonic polys, verts/poly {}..{}",
            ir.out_of_range,
            ir.distinct,
            t.vert_count(),
            pr.non_monotonic,
            pr.min_verts,
            pr.max_verts
        )?;
    } else if has_ext(path, "nod") {
        let n = Nod::parse(&bytes).map_err(|e| fail("nod parse", e))?;
        writeln!(
            o,
            "nodes: {} (car {}, isec {}), links: {}",
            n.node_count(),
            n.car_count(),
            n.isec_count(),
            n.link_count()
        )?;
        writeln!(o, "partition clean: {}", n.validate_partition().is_clean())?;
        let mut targets = HashSet::new();
        let mut owned = 0u64;
        for i in 0..n.node_count() {
            let links = n.links_of(i).map_err(|e| fail("links_of", e))?;
            owned += links.len() as u64;
            for l in &links {
                targets.insert((l.area(), l.node()));
            }
        }
        writeln!(
            o,
            "owned links: {owned}, distinct targets: {}",
            targets.len()
        )?;
        if n.node_count() > 0 {
            let n0 = n.node(0).map_err(|e| fail("node 0", e))?;
            writeln!(
                o,
                "node 0: area {} id {} pos ({:.1}, {:.1}, {:.1})",
                n0.area(),
                n0.id(),
                n0.x(),
                n0.y(),
                n0.z()
            )?;
        }
    } else {
        let text = String::from_utf8_lossy(&bytes);
        let p = IplPaths::parse(&text);
        writeln!(
            o,
            "ipl: {} nodes, {} links ({} valid), {} skipped lines",
            p.nodes.len(),
            p.links.len(),
            p.valid_links().count(),
            p.skipped
        )?;
    }
    Ok(())
}
