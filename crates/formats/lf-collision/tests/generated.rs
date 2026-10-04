//! Property and fuzz tests on generated collision files. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); no game files
//! are needed.
//!
//! - Property: random bound trees (spheres, capsules, boxes, geometry and
//!   BVH meshes with quantised vertices and polygons, composites with child
//!   transforms) written as single-bound and dictionary files parse back to
//!   the same kinds, headers, vertices, polygons and matrices, and a clean
//!   generated mesh validates clean.
//! - Invariants: the validation report's counts agree with the parsed tree.
//! - Fuzz: mutated system segments, re-wrapped in a valid container, never
//!   panic in parsing or validation.

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

use lf_collision::{Bound, CollisionFile, MeshKind};
use support::{Buf, Rng, fuzz, rsc5};

/// Resource type id of collision files.
const KIND_BOUNDS: u32 = 32;
const SYS: u32 = 0x5000_0000;
/// The NaN pad word after every three-float vector.
const NAN_PAD: u32 = 0x7F80_0001;
const BASE_SIZE: usize = 128;
const MESH_TAIL: usize = 80;
const COMPOSITE_TAIL: usize = 20;

/// A generated bound.
#[derive(Clone, Debug)]
enum Spec {
    Sphere(f32),
    Capsule(f32, f32),
    /// Mesh kind byte (3, 4 or 10), quantised vertices, polygons
    /// (vertex indices, neighbours, material), shrunk array present.
    Mesh(u8, Vec<[i16; 3]>, Vec<([u16; 4], [u16; 4], u8)>, bool),
    /// Children with their translations.
    Composite(Vec<(Spec, [f32; 3])>),
}

/// Quantisation used by every generated mesh.
const UNQUANT: [f32; 3] = [0.25, 0.5, 0.125];
const CENTRE: [f32; 3] = [10.0, -20.0, 5.0];

fn random_leaf(rng: &mut Rng) -> Spec {
    match rng.below(4) {
        0 => Spec::Sphere(rng.f32_in(0.1, 10.0)),
        1 => Spec::Capsule(rng.f32_in(0.1, 3.0), rng.f32_in(0.1, 5.0)),
        _ => {
            let kind = *rng.pick(&[3u8, 4, 10]);
            let nv = if kind == 3 { 8 } else { rng.range(3, 30) };
            let verts: Vec<[i16; 3]> = (0..nv)
                .map(|_| {
                    [
                        rng.range(0, 2000) as i16 - 1000,
                        rng.range(0, 2000) as i16 - 1000,
                        rng.range(0, 2000) as i16 - 1000,
                    ]
                })
                .collect();
            let np = if kind == 3 { 6 } else { rng.range(1, 20) };
            let polys = (0..np)
                .map(|_| {
                    let quad = rng.chance(1, 2);
                    let v = [
                        rng.below(nv) as u16,
                        rng.below(nv) as u16,
                        rng.below(nv) as u16,
                        if quad {
                            1 + rng.below(nv - 1) as u16
                        } else {
                            0
                        },
                    ];
                    let nb = [0u16; 4].map(|_| {
                        if rng.chance(1, 3) {
                            0xFFFF
                        } else {
                            rng.below(np) as u16
                        }
                    });
                    (v, nb, rng.below(64) as u8)
                })
                .collect();
            Spec::Mesh(kind, verts, polys, kind == 4)
        }
    }
}

fn random_bound(rng: &mut Rng) -> Spec {
    if rng.chance(1, 3) {
        let children = (0..rng.range(1, 5))
            .map(|_| {
                (
                    random_leaf(rng),
                    [
                        rng.f32_in(-5.0, 5.0),
                        rng.f32_in(-5.0, 5.0),
                        rng.f32_in(-5.0, 5.0),
                    ],
                )
            })
            .collect();
        Spec::Composite(children)
    } else {
        random_leaf(rng)
    }
}

fn kind_byte(s: &Spec) -> u8 {
    match s {
        Spec::Sphere(_) => 0,
        Spec::Capsule(..) => 1,
        Spec::Mesh(k, ..) => *k,
        Spec::Composite(_) => 12,
    }
}

/// Bounding box of a mesh's world-space vertices (min, max).
fn world(q: [i16; 3]) -> [f32; 3] {
    [
        CENTRE[0] + f32::from(q[0]) * UNQUANT[0],
        CENTRE[1] + f32::from(q[1]) * UNQUANT[1],
        CENTRE[2] + f32::from(q[2]) * UNQUANT[2],
    ]
}

fn vec3(b: &mut Buf, v: [f32; 3]) {
    b.f32(v[0]).f32(v[1]).f32(v[2]).u32(NAN_PAD);
}

struct Writer(Buf);

impl Writer {
    fn alloc(&mut self, len: usize) -> usize {
        self.0.align(16);
        let at = self.0.len();
        self.0.put(at, &vec![0; len.max(1)]);
        at
    }

    /// Write one bound; returns its offset.
    fn bound(&mut self, s: &Spec) -> usize {
        // Arrays first so the bound record can point at them.
        let (bbox_min, bbox_max) = match s {
            Spec::Mesh(_, verts, ..) => {
                let mut lo = [f32::MAX; 3];
                let mut hi = [f32::MIN; 3];
                for v in verts {
                    let w = world(*v);
                    for k in 0..3 {
                        lo[k] = lo[k].min(w[k]);
                        hi[k] = hi[k].max(w[k]);
                    }
                }
                (lo, hi)
            }
            _ => ([-1.0; 3], [1.0; 3]),
        };
        let mut tail = Buf::new();
        match s {
            Spec::Sphere(r) => vec3(&mut tail, [*r; 3]),
            Spec::Capsule(r, l) => {
                vec3(&mut tail, [*r; 3]);
                vec3(&mut tail, [*l; 3]);
                vec3(&mut tail, [0.0; 3]);
                vec3(&mut tail, [0.0; 3]);
            }
            Spec::Mesh(kind, verts, polys, shrunk) => {
                let write_verts = |w: &mut Writer| {
                    let at = w.alloc(verts.len() * 6);
                    for (i, v) in verts.iter().enumerate() {
                        for (k, q) in v.iter().enumerate() {
                            w.0.put_u16(at + i * 6 + k * 2, *q as u16);
                        }
                    }
                    at
                };
                let v_at = write_verts(self);
                let s_at = shrunk.then(|| write_verts(self));
                let p_at = self.alloc(polys.len() * 32);
                for (i, (vi, nb, mat)) in polys.iter().enumerate() {
                    let r = p_at + i * 32;
                    // Unit normal along +z; area float carries the material.
                    self.0
                        .put_f32(r, 0.0)
                        .put_f32(r + 4, 0.0)
                        .put_f32(r + 8, 1.0);
                    self.0
                        .put_u32(r + 12, (2.0f32.to_bits() & !0xFF) | u32::from(*mat));
                    for k in 0..4 {
                        self.0
                            .put_u16(r + 16 + 2 * k, vi[k])
                            .put_u16(r + 24 + 2 * k, nb[k]);
                    }
                }
                tail.u32(0)
                    .u32(s_at.map_or(0, |a| SYS | a as u32))
                    .u32(0)
                    .u32(SYS | p_at as u32);
                vec3(&mut tail, UNQUANT);
                vec3(&mut tail, CENTRE);
                tail.u32(SYS | v_at as u32)
                    .u32(0)
                    .u8(u8::from(*kind == 10))
                    .bytes(&[0; 3])
                    .u32(0xFFFF_FFFF)
                    .u32(0)
                    .u32(0)
                    .u32(verts.len() as u32)
                    .u32(polys.len() as u32);
                assert_eq!(tail.len(), MESH_TAIL);
            }
            Spec::Composite(children) => {
                let offs: Vec<usize> = children.iter().map(|(c, _)| self.bound(c)).collect();
                let n = children.len();
                let arr = self.alloc(n * 4);
                let cur = self.alloc(n * 64);
                let last = self.alloc(n * 64);
                let boxes = self.alloc(n * 32);
                for (i, (_, t)) in children.iter().enumerate() {
                    self.0.put_u32(arr + 4 * i, SYS | offs[i] as u32);
                    for m in [cur, last] {
                        let r = m + i * 64;
                        self.0
                            .put_f32(r, 1.0)
                            .put_f32(r + 20, 1.0)
                            .put_f32(r + 40, 1.0);
                        self.0
                            .put_f32(r + 48, t[0])
                            .put_f32(r + 52, t[1])
                            .put_f32(r + 56, t[2]);
                    }
                    let mut b = Buf::new();
                    vec3(&mut b, [-1.0; 3]);
                    vec3(&mut b, [1.0; 3]);
                    self.0.put(boxes + i * 32, &b.0);
                }
                tail.u32(SYS | arr as u32)
                    .u32(SYS | cur as u32)
                    .u32(SYS | last as u32)
                    .u32(SYS | boxes as u32)
                    .u16(n as u16)
                    .u16(n as u16);
                assert_eq!(tail.len(), COMPOSITE_TAIL);
            }
        }
        let radius = match s {
            Spec::Sphere(r) => *r,
            Spec::Capsule(r, l) => r + l / 2.0,
            _ => 1.0,
        };
        let mut rec = Buf::new();
        rec.u32(0x77)
            .u8(kind_byte(s))
            .u8(1)
            .u16(1)
            .f32(radius)
            .f32(0.0);
        vec3(&mut rec, bbox_max);
        vec3(&mut rec, bbox_min);
        for _ in 0..3 {
            vec3(&mut rec, [0.0; 3]);
        }
        rec.bytes(&[0; 32]);
        assert_eq!(rec.len(), BASE_SIZE);
        rec.bytes(&tail.0);
        let at = self.alloc(rec.len());
        self.0.put(at, &rec.0);
        at
    }
}

fn write_wbn(s: &Spec) -> Vec<u8> {
    let mut w = Writer(Buf::zeroed(16));
    let root = w.bound(s);
    w.0.put_u32(0, 0x0069_1111).put_u32(8, SYS | root as u32);
    w.0.0
}

fn write_wbd(entries: &[(u32, Spec)]) -> Vec<u8> {
    let mut w = Writer(Buf::zeroed(32));
    let offs: Vec<usize> = entries.iter().map(|(_, s)| w.bound(s)).collect();
    let n = entries.len();
    let hashes = w.alloc(n * 4);
    let bounds = w.alloc(n * 4);
    for (i, (h, _)) in entries.iter().enumerate() {
        w.0.put_u32(hashes + 4 * i, *h)
            .put_u32(bounds + 4 * i, SYS | offs[i] as u32);
    }
    w.0.put_u32(0, 0x0069_2222)
        .put_u32(12, 1)
        .put_u32(0x10, SYS | hashes as u32)
        .put_u16(0x14, n as u16)
        .put_u16(0x16, n as u16)
        .put_u32(0x18, SYS | bounds as u32)
        .put_u16(0x1C, n as u16)
        .put_u16(0x1E, n as u16);
    w.0.0
}

fn check(b: &Bound, s: &Spec) {
    assert_eq!(b.bound_type().byte(), kind_byte(s));
    match (b, s) {
        (Bound::Sphere(g), Spec::Sphere(r)) => {
            assert_eq!(g.radius_vec.x, *r);
            assert_eq!(g.header.radius, *r);
        }
        (Bound::Capsule(g), Spec::Capsule(r, l)) => {
            assert_eq!((g.radius_vec.x, g.length_vec.x), (*r, *l));
        }
        (Bound::Mesh(m), Spec::Mesh(kind, verts, polys, shrunk)) => {
            let want_kind = match kind {
                3 => MeshKind::Box,
                4 => MeshKind::Geometry,
                _ => MeshKind::Bvh,
            };
            assert_eq!(m.mesh_kind, want_kind);
            assert_eq!(m.tree_flag, u8::from(*kind == 10));
            assert_eq!(m.marker, 0xFFFF_FFFF);
            assert_eq!(m.vertices.len(), verts.len());
            for (v, q) in m.vertices.iter().zip(verts) {
                let w = world(*q);
                assert_eq!([v.x, v.y, v.z], w);
            }
            assert_eq!(m.shrunk_vertices.is_some(), *shrunk);
            assert_eq!(m.polygons.len(), polys.len());
            for (p, (vi, nb, mat)) in m.polygons.iter().zip(polys) {
                assert_eq!(&p.vertices, vi);
                assert_eq!(&p.neighbours, nb);
                assert_eq!(p.material_index(), *mat);
            }
            // Generated meshes are internally consistent.
            assert_eq!(m.outside_bbox_count(), 0);
            assert_eq!(m.index_oob_count(), 0);
            assert_eq!(m.neighbour_oob_count(), 0);
            assert_eq!(m.non_unit_normal_count(1e-4), 0);
        }
        (Bound::Composite(c), Spec::Composite(children)) => {
            assert_eq!(c.children.len(), children.len());
            assert_eq!(usize::from(c.num_bounds), children.len());
            for (i, (child, (cs, t))) in c.children.iter().zip(children).enumerate() {
                check(child, cs);
                let tr = c.current_matrices[i].translation();
                assert_eq!([tr.x, tr.y, tr.z], *t);
                assert_eq!(c.last_matrices[i], c.current_matrices[i]);
            }
            assert_eq!(c.local_boxes.len(), children.len());
        }
        other => panic!("bound kind mismatch: {other:?}"),
    }
}

#[test]
fn single_bound_round_trip() {
    let mut rng = Rng::for_test("collision wbn");
    for _ in 0..120 {
        let spec = random_bound(&mut rng);
        let file = rsc5(KIND_BOUNDS, &write_wbn(&spec), &[]);
        let parsed = lf_collision::parse(&file).expect("generated single bound parses");
        let CollisionFile::Wbn(f) = &parsed else {
            panic!("detected as a dictionary");
        };
        assert_eq!(f.root_vtable, 0x0069_1111);
        check(&f.root, &spec);
        assert!(parsed.validate().is_clean());
    }
}

#[test]
fn dictionary_round_trip() {
    let mut rng = Rng::for_test("collision wbd");
    for _ in 0..60 {
        let entries: Vec<(u32, Spec)> = (0..rng.range(1, 6))
            .map(|_| (rng.next_u32(), random_bound(&mut rng)))
            .collect();
        let file = rsc5(KIND_BOUNDS, &write_wbd(&entries), &[]);
        let parsed = lf_collision::parse(&file).expect("generated dictionary parses");
        let CollisionFile::Wbd(f) = &parsed else {
            panic!("detected as a single bound");
        };
        assert_eq!(f.entries.len(), entries.len());
        for (e, (h, s)) in f.entries.iter().zip(&entries) {
            assert_eq!(e.hash, *h);
            check(&e.bound, s);
        }
        // The report's totals agree with the tree.
        let report = parsed.validate();
        let meshes: Vec<&lf_collision::MeshBound> = parsed
            .all_bounds()
            .into_iter()
            .filter_map(|b| match b {
                Bound::Mesh(m) => Some(m),
                _ => None,
            })
            .collect();
        assert_eq!(report.meshes, meshes.len() as u64);
        let verts: usize = meshes.iter().map(|m| m.vertices.len()).sum();
        assert_eq!(report.vertices, verts as u64);
        let counted: usize = parsed.bound_type_counts().iter().map(|(_, n)| n).sum();
        assert_eq!(counted, parsed.all_bounds().len());
        assert!(report.is_clean());
    }
}

/// Parse and walk everything; nothing may panic.
fn exercise(sys: &[u8]) {
    let file = rsc5(KIND_BOUNDS, sys, &[]);
    if let Ok(parsed) = lf_collision::parse(&file) {
        let _ = parsed.validate();
        let _ = parsed.bound_type_counts();
        for b in parsed.all_bounds() {
            if let Bound::Mesh(m) = b {
                let _ = (
                    m.outside_bbox_count(),
                    m.index_oob_count(),
                    m.triangle_count(),
                );
            }
        }
    }
}

#[test]
fn fuzz_single_bounds() {
    let mut rng = Rng::for_test("collision fuzz wbn");
    let seeds: Vec<Vec<u8>> = (0..5).map(|_| write_wbn(&random_bound(&mut rng))).collect();
    fuzz("collision single bound", &seeds, 3000, exercise);
}

#[test]
fn fuzz_dictionaries() {
    let mut rng = Rng::for_test("collision fuzz wbd");
    let seeds: Vec<Vec<u8>> = (0..3)
        .map(|_| {
            let entries: Vec<(u32, Spec)> = (0..3)
                .map(|_| (rng.next_u32(), random_bound(&mut rng)))
                .collect();
            write_wbd(&entries)
        })
        .collect();
    fuzz("collision dictionary", &seeds, 2000, exercise);
}

#[test]
fn regression_layered_composites_finish_quickly() {
    // Seven layers of composites, each holding twelve pointers to the one
    // composite of the next layer, over a single sphere: a few kilobytes
    // that the depth cap alone let expand into 12^7 bound parses.
    let n = 12usize;
    let mut w = Writer(Buf::zeroed(16));
    let mut next = w.bound(&Spec::Sphere(1.0));
    for _ in 0..7 {
        let at = w.alloc(BASE_SIZE + COMPOSITE_TAIL);
        let arr = w.alloc(n * 4);
        let mats = w.alloc(n * 64);
        let boxes = w.alloc(n * 32);
        for i in 0..n {
            w.0.put_u32(arr + 4 * i, SYS | next as u32);
        }
        w.0.put_u32(at, 0x77).put(at + 4, &[12, 1]);
        w.0.put_u32(at + BASE_SIZE, SYS | arr as u32)
            .put_u32(at + BASE_SIZE + 4, SYS | mats as u32)
            .put_u32(at + BASE_SIZE + 8, SYS | mats as u32)
            .put_u32(at + BASE_SIZE + 12, SYS | boxes as u32)
            .put_u16(at + BASE_SIZE + 16, n as u16)
            .put_u16(at + BASE_SIZE + 18, n as u16);
        next = at;
    }
    w.0.put_u32(8, SYS | next as u32);
    let start = std::time::Instant::now();
    let result = lf_collision::parse(&rsc5(KIND_BOUNDS, &w.0.0, &[]));
    assert!(result.is_err(), "an expanding composite graph is rejected");
    assert!(start.elapsed().as_secs() < 5, "took {:?}", start.elapsed());
}
