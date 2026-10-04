//! Print a summary of one navigation or path file. Reads only; writes nothing.
//!
//! Usage: `lf_nav_summarize <file.wnv | file.nod | paths.ipl>`. For archived members,
//! pass an extracted copy; this tool never writes game content anywhere.

use lf_nav::{ipl::IplPaths, nod::Nod, rsc::Resource, wnv::Tile};
use std::collections::HashSet;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_nav_summarize <file.wnv|file.nod|paths.ipl>");
        std::process::exit(2);
    });
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    println!("file: {path} ({} bytes)", bytes.len());
    if path.ends_with(".wnv") {
        let r = Resource::parse(&bytes).expect("rsc parse");
        println!(
            "container: RSC5 type {} flags {:#x}, payload {} bytes",
            r.resource_type(),
            r.flags(),
            r.payload().len()
        );
        let t = Tile::parse(r.payload()).expect("tile parse");
        println!(
            "tile: {:.1} x {:.1} m, z extent {:.1} m, {} verts, {} indices, {} polys",
            t.size_x(),
            t.size_y(),
            t.z_extent(),
            t.vert_count(),
            t.index_count(),
            t.poly_count()
        );
        let ir = t.validate_indices();
        let pr = t.validate_polygons();
        println!(
            "validation: {} indices out of range, {}/{} verts referenced, {} non-monotonic polys, verts/poly {}..{}",
            ir.out_of_range,
            ir.distinct,
            t.vert_count(),
            pr.non_monotonic,
            pr.min_verts,
            pr.max_verts
        );
    } else if path.ends_with(".nod") {
        let n = Nod::parse(&bytes).expect("nod parse");
        println!(
            "nodes: {} (car {}, isec {}), links: {}",
            n.node_count(),
            n.car_count(),
            n.isec_count(),
            n.link_count()
        );
        println!("partition clean: {}", n.validate_partition().is_clean());
        let mut targets = HashSet::new();
        let mut owned = 0u32;
        for i in 0..n.node_count() {
            let links = n.links_of(i).expect("links_of");
            owned += links.len() as u32;
            for l in &links {
                targets.insert((l.area(), l.node()));
            }
        }
        println!("owned links: {owned}, distinct targets: {}", targets.len());
        if n.node_count() > 0 {
            let n0 = n.node(0).unwrap();
            println!(
                "node 0: area {} id {} pos ({:.1}, {:.1}, {:.1})",
                n0.area(),
                n0.id(),
                n0.x(),
                n0.y(),
                n0.z()
            );
        }
    } else {
        let text = String::from_utf8_lossy(&bytes);
        let p = IplPaths::parse(&text);
        println!(
            "ipl: {} nodes, {} links ({} valid), {} skipped lines",
            p.nodes.len(),
            p.links.len(),
            p.valid_links().count(),
            p.skipped
        );
    }
}
