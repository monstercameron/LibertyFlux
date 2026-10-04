//! Property and fuzz tests on generated navigation files. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); no game files
//! are needed.
//!
//! - Property: random `.nod` graphs (nodes, links partitioned by
//!   `link_id`), `.wnv` tiles (vertices, indices, polygons, edge records)
//!   and `paths.ipl` texts parse back to the same fields; per-node link
//!   slices partition the link table; polygon vertex lists match.
//! - Fuzz: mutated `.nod` files, tile payloads (raw and wrapped in an RSC5
//!   container) and path texts never panic in parsing, iteration or
//!   validation.

// Fixture builders narrow random words and lengths into smaller fields on
// purpose, and the property checks compare floats that were written
// bit-exactly, so these pedantic lints do not apply here.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::format_push_string,
    clippy::format_collect,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::many_single_char_names,
    clippy::similar_names
)]

#[path = "../../tests/support.rs"]
mod support;

use lf_nav::ipl::IplPaths;
use lf_nav::nod::{self, Nod};
use lf_nav::rsc::Resource;
use lf_nav::wnv::{self, Tile};
use support::{Buf, Rng, fuzz, rsc5};

#[derive(Clone, Debug)]
struct NodeSpec {
    area: u16,
    id: u16,
    street: u32,
    pos: [i16; 3],
    width: u8,
    flags: u32,
    links: Vec<(u16, u16, u8)>,
}

fn random_nod(rng: &mut Rng) -> Vec<NodeSpec> {
    (0..rng.below(20))
        .map(|i| NodeSpec {
            area: rng.below(64) as u16,
            id: i as u16,
            street: rng.next_u32(),
            pos: [
                rng.next_u32() as i16,
                rng.next_u32() as i16,
                rng.next_u32() as i16,
            ],
            width: rng.next_u32() as u8,
            flags: rng.next_u32(),
            links: (0..rng.below(5))
                .map(|_| {
                    (
                        rng.below(64) as u16,
                        rng.below(500) as u16,
                        rng.next_u32() as u8,
                    )
                })
                .collect(),
        })
        .collect()
}

fn write_nod(nodes: &[NodeSpec]) -> Vec<u8> {
    let links: usize = nodes.iter().map(|n| n.links.len()).sum();
    let mut b = Buf::new();
    b.u32(nodes.len() as u32)
        .u32(nodes.len() as u32 / 2)
        .u32(0)
        .u32(links as u32);
    let mut link_id = 0u16;
    for n in nodes {
        b.u32(0)
            .u32(0)
            .u16(n.area)
            .u16(n.id)
            .u32(n.street)
            .u16(0)
            .u16(link_id)
            .u16(n.pos[0] as u16)
            .u16(n.pos[1] as u16)
            .u16(n.pos[2] as u16)
            .u8(n.width)
            .u8(0)
            .u32(n.flags);
        link_id += n.links.len() as u16;
    }
    for n in nodes {
        for &(area, node, len) in &n.links {
            b.u16(area).u16(node).u8(len).u8(0).u16(0);
        }
    }
    b.0
}

#[test]
fn nod_round_trip() {
    let mut rng = Rng::for_test("nav nod");
    for _ in 0..100 {
        let spec = random_nod(&mut rng);
        let bytes = write_nod(&spec);
        let n = Nod::parse(&bytes).expect("generated nod parses");
        assert_eq!(n.node_count() as usize, spec.len());
        assert!(n.validate_partition().is_clean());
        let mut owned = 0;
        for (i, s) in spec.iter().enumerate() {
            let node = n.node(i as u32).unwrap();
            assert_eq!(
                (node.area(), node.id(), node.street()),
                (s.area, s.id, s.street)
            );
            assert_eq!(node.x(), f64::from(s.pos[0]) / 8.0);
            assert_eq!(node.z(), f64::from(s.pos[2]) / 128.0);
            assert_eq!(node.flags(), s.flags);
            let links = n.links_of(i as u32).unwrap();
            assert_eq!(links.len(), s.links.len());
            for (l, &(area, target, len)) in links.iter().zip(&s.links) {
                assert_eq!((l.area(), l.node(), l.length_m()), (area, target, len));
            }
            owned += links.len();
        }
        assert_eq!(owned, n.link_count() as usize);
        assert_eq!(n.nodes().count(), spec.len());
        assert_eq!(n.links().count(), owned);
        assert!(n.node(spec.len() as u32).is_err());
    }
}

#[derive(Clone, Debug)]
struct TileSpec {
    size: [f32; 3],
    verts: Vec<[u16; 3]>,
    /// Polygons as index lists.
    polys: Vec<Vec<u16>>,
}

fn random_tile(rng: &mut Rng) -> TileSpec {
    let nv = rng.range(1, 40);
    TileSpec {
        size: [
            rng.f32_in(1.0, 100.0),
            rng.f32_in(1.0, 100.0),
            rng.f32_in(0.0, 50.0),
        ],
        verts: (0..nv)
            .map(|_| {
                [
                    rng.next_u32() as u16,
                    rng.next_u32() as u16,
                    rng.next_u32() as u16,
                ]
            })
            .collect(),
        polys: (0..rng.below(12))
            .map(|_| (0..rng.range(3, 6)).map(|_| rng.below(nv) as u16).collect())
            .collect(),
    }
}

fn write_tile(t: &TileSpec) -> Vec<u8> {
    let indices: Vec<u16> = t.polys.iter().flatten().copied().collect();
    let mut b = Buf::zeroed(wnv::HEADER_LEN);
    let vert_base = b.len();
    for v in &t.verts {
        b.u16(v[0]).u16(v[1]).u16(v[2]);
    }
    b.align(16);
    let index_base = b.len();
    for &i in &indices {
        b.u16(i);
    }
    b.align(16);
    let poly_base = b.len();
    let mut first = 0u16;
    for p in &t.polys {
        let mut rec = [0u8; wnv::POLYGON_LEN];
        rec[4..6].copy_from_slice(&first.to_le_bytes());
        b.bytes(&rec);
        first += p.len() as u16;
    }
    let edge_base = b.len();
    for k in 0..indices.len() {
        b.u16(k as u16).u16(0xFFFF).u16(0).u16(0);
    }
    let tail_base = b.len();
    b.u32(0);
    let ptr = |off: usize| 0x5000_0000 | off as u32;
    b.put_f32(0x40, t.size[0])
        .put_f32(0x44, t.size[1])
        .put_f32(0x48, t.size[2])
        .put_u32(0x58, ptr(vert_base))
        .put_u32(0x60, ptr(index_base))
        .put_u32(0x64, ptr(edge_base))
        .put_u32(0x68, indices.len() as u32)
        .put_u32(0x6c, ptr(poly_base))
        .put_u32(0x70, ptr(tail_base))
        .put_u32(0x78, t.verts.len() as u32)
        .put_u32(0x7c, t.polys.len() as u32);
    b.0
}

#[test]
fn tile_round_trip() {
    let mut rng = Rng::for_test("nav wnv");
    for _ in 0..100 {
        let spec = random_tile(&mut rng);
        let payload = write_tile(&spec);
        let file = rsc5(1, &payload, &[]);
        let res = Resource::parse(&file).expect("container");
        assert_eq!(res.resource_type(), 1);
        let t = Tile::parse(res.payload()).expect("generated tile parses");
        assert_eq!([t.size_x(), t.size_y(), t.z_extent()], spec.size);
        assert_eq!(t.vert_count() as usize, spec.verts.len());
        for (i, v) in spec.verts.iter().enumerate() {
            let raw = t.vertex(i as u32).unwrap();
            assert_eq!([raw.qx, raw.qy, raw.qz], *v);
        }
        assert_eq!(t.poly_count() as usize, spec.polys.len());
        for (i, p) in spec.polys.iter().enumerate() {
            assert_eq!(&t.polygon_vertices(i as u32).unwrap(), p);
            assert_eq!(t.polygon_vertex_count(i as u32).unwrap() as usize, p.len());
        }
        let ir = t.validate_indices();
        assert_eq!(ir.out_of_range, 0);
        let pr = t.validate_polygons();
        assert_eq!(pr.non_monotonic, 0);
        assert_eq!(t.edge_count(), t.index_count());
        for (k, e) in t.edge_records().enumerate() {
            assert_eq!(
                e,
                lf_nav::wnv::EdgeRecord {
                    a: k as u16,
                    b: 0xFFFF,
                    c: 0,
                    d: 0
                }
            );
        }
    }
}

fn random_ipl(rng: &mut Rng) -> (String, Vec<[f32; 3]>, Vec<(u32, u32, u32)>) {
    let mut text = String::from("# generated\ninst\nend\nvnod\n");
    let nodes: Vec<[f32; 3]> = (0..rng.below(10))
        .map(|_| {
            [
                f32::from(rng.below(20000) as u16) / 4.0 - 2000.0,
                f32::from(rng.below(20000) as u16) / 4.0 - 2000.0,
                f32::from(rng.below(800) as u16) / 8.0,
            ]
        })
        .collect();
    for n in &nodes {
        text.push_str(&format!(
            "{}, {}, {}, 0, 0, 0, 0, 0, 0, 7, 0\n",
            n[0], n[1], n[2]
        ));
    }
    text.push_str("end\nlink\n");
    let links: Vec<(u32, u32, u32)> = (0..rng.below(10))
        .map(|_| {
            (
                rng.below(12) as u32,
                rng.below(12) as u32,
                rng.range(1, 4) as u32,
            )
        })
        .collect();
    for l in &links {
        text.push_str(&format!("{}, {}, 0, {}, 0, 0\n", l.0, l.1, l.2));
    }
    text.push_str("end\n");
    (text, nodes, links)
}

#[test]
fn ipl_round_trip() {
    let mut rng = Rng::for_test("nav ipl");
    for _ in 0..100 {
        let (text, nodes, links) = random_ipl(&mut rng);
        let p = IplPaths::parse(&text);
        assert_eq!(p.skipped, 0);
        assert_eq!(p.nodes.len(), nodes.len());
        for (n, w) in p.nodes.iter().zip(&nodes) {
            assert_eq!([n.x, n.y, n.z], *w);
            assert_eq!(n.street, 7);
        }
        assert_eq!(p.links.len(), links.len());
        for (l, w) in p.links.iter().zip(&links) {
            assert_eq!((l.a, l.b, l.lanes), *w);
        }
        let valid = links
            .iter()
            .filter(|l| l.0 != l.1 && (l.0 as usize) < nodes.len() && (l.1 as usize) < nodes.len())
            .count();
        assert_eq!(p.valid_links().count(), valid);
    }
}

fn exercise_nod(bytes: &[u8]) {
    let Ok(n) = Nod::parse(bytes) else { return };
    assert_eq!(
        bytes.len(),
        nod::HEADER_LEN
            + n.node_count() as usize * nod::NODE_LEN
            + n.link_count() as usize * nod::LINK_LEN
    );
    let _ = n.validate_partition();
    for i in 0..n.node_count() {
        if let Ok(links) = n.links_of(i) {
            assert!(links.len() <= n.link_count() as usize);
        }
    }
    let _ = n.nodes().count() + n.links().count();
}

fn exercise_tile(payload: &[u8]) {
    let Ok(t) = Tile::parse(payload) else { return };
    let _ = t.vertices().count() + t.indices().count() + t.polygons().count();
    for i in 0..t.poly_count().min(256) {
        let _ = t.polygon_vertices(i);
        let _ = t.polygon_vertex_count(i);
    }
    let _ = (t.validate_indices(), t.validate_polygons());
    assert!(t.edge_region().len() <= payload.len());
    let _ = t.edge_records().count();
    for v in t.vertices().take(16) {
        let _ = (t.local_xy(v), t.local_z_rel(v));
    }
}

#[test]
fn fuzz_nod() {
    let mut rng = Rng::for_test("nav nod fuzz");
    let seeds: Vec<Vec<u8>> = (0..4)
        .map(|_| {
            loop {
                let n = random_nod(&mut rng);
                if n.len() > 2 {
                    return write_nod(&n);
                }
            }
        })
        .collect();
    fuzz("nod", &seeds, 3000, exercise_nod);
}

#[test]
fn fuzz_tiles() {
    let mut rng = Rng::for_test("nav wnv fuzz");
    let seeds: Vec<Vec<u8>> = (0..4).map(|_| write_tile(&random_tile(&mut rng))).collect();
    fuzz("wnv payload", &seeds, 3000, exercise_tile);
    fuzz("wnv file", &seeds[..1], 1000, |p| {
        if let Ok(r) = Resource::parse(&rsc5(1, p, &[])) {
            exercise_tile(r.payload());
        }
    });
}

#[test]
fn fuzz_ipl_text() {
    let mut rng = Rng::for_test("nav ipl fuzz");
    let seeds: Vec<Vec<u8>> = (0..3)
        .map(|_| random_ipl(&mut rng).0.into_bytes())
        .collect();
    fuzz("paths ipl", &seeds, 3000, |b| {
        let p = IplPaths::parse(&String::from_utf8_lossy(b));
        assert!(p.valid_links().count() <= p.links.len());
    });
}
