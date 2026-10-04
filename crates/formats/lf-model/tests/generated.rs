//! Property and fuzz tests on generated model resources. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); no game files
//! are needed.
//!
//! - Property: random drawables (LOD slots, models, geometries with vertex
//!   declarations, vertex and index data, skinning palettes, shader groups
//!   with every parameter kind, skeletons), drawable dictionaries and
//!   fragments are laid out in system and graphics segments, wrapped in an
//!   RSC5 file, and parse back to the same counts, names, numbers and
//!   decoded vertices.
//! - Invariants: index and vertex reads stay inside the graphics segment,
//!   per-geometry tables have one entry per geometry.
//! - Fuzz: mutated system segments (fed straight to the parsers, and also
//!   re-wrapped in a container) never panic.

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

use lf_model::drawable::PrimitiveType;
use lf_model::rsc5::{TYPE_DRAWABLE, TYPE_FRAGMENT};
use lf_model::{Drawable, DrawableDictionary, Fragment, Resource, ShaderParam};
use support::{Buf, Rng, fuzz, rsc5};

const SYS: u32 = 0x5000_0000;
const GFX: u32 = 0x6000_0000;
/// Vertex declaration usage bits (slot order) and nibble types used here.
const USE_POSITION: u32 = 1 << 0;
const USE_NORMAL: u32 = 1 << 3;
const USE_COLOR0: u32 = 1 << 4;
const USE_TEXCOORD0: u32 = 1 << 6;
const NIBBLE_F32X2: u64 = 5;
const NIBBLE_F32X3: u64 = 6;
const NIBBLE_COLOR: u64 = 9;
/// Fragment header fields used by the reader.
const FRAG_MAIN_DRAWABLE: usize = 0xB4;
const FRAG_CHILD_LIST: usize = 0xD4;
const FRAG_CHILD_COUNT: usize = 0x1F3;
const FRAG_CHILD_LEN: usize = 0xA8;
const BONE_LEN: usize = 224;

/// System segment with a bump allocator.
struct Sys(Buf);

impl Sys {
    fn new(reserve: usize) -> Sys {
        Sys(Buf::zeroed(reserve))
    }
    fn alloc(&mut self, len: usize) -> usize {
        self.0.align(16);
        let at = self.0.len();
        self.0.put(at, &vec![0; len.max(1)]);
        at
    }
    fn ptr(at: usize) -> u32 {
        SYS | u32::try_from(at).unwrap()
    }
    fn cstr(&mut self, s: &str) -> usize {
        let at = self.alloc(s.len() + 1);
        self.0.put(at, s.as_bytes());
        at
    }
    /// A pointer collection (list pointer, count, size) written at `at`.
    fn ptr_list(&mut self, at: usize, items: &[usize]) {
        let n = u16::try_from(items.len()).unwrap();
        if items.is_empty() {
            self.0.put_u32(at, 0).put_u16(at + 4, 0).put_u16(at + 6, 0);
            return;
        }
        let list = self.alloc(items.len() * 4);
        for (i, &item) in items.iter().enumerate() {
            self.0.put_u32(list + 4 * i, Sys::ptr(item));
        }
        self.0
            .put_u32(at, Sys::ptr(list))
            .put_u16(at + 4, n)
            .put_u16(at + 6, n);
    }
}

#[derive(Clone, Debug)]
struct Vertex {
    pos: [f32; 3],
    normal: [f32; 3],
    color: u32,
    uv: [f32; 2],
}

#[derive(Clone, Debug)]
struct GeomSpec {
    mask: u32,
    verts: Vec<Vertex>,
    indices: Vec<u16>,
    primitive: u16,
    palette: Vec<u16>,
}

#[derive(Clone, Debug)]
struct ModelSpec {
    geoms: Vec<GeomSpec>,
    shader_map: Vec<u16>,
    bytes: [u8; 6],
}

#[derive(Clone, Debug)]
enum ParamSpec {
    Texture(String),
    Vector([f32; 4]),
    Matrix([f32; 16]),
    Float(f32),
}

#[derive(Clone, Debug)]
struct ShaderSpec {
    name: String,
    hash: u32,
    params: Vec<(u32, ParamSpec)>,
}

#[derive(Clone, Debug)]
struct BoneSpec {
    name: String,
    index: i16,
    id: i16,
    position: [f32; 4],
    parent: i32,
}

#[derive(Clone, Debug)]
struct DrawSpec {
    center: [f32; 4],
    lods: [Option<Vec<ModelSpec>>; 4],
    shaders: Option<(Vec<ShaderSpec>, [u32; 12])>,
    bones: Option<Vec<BoneSpec>>,
}

fn vec4(rng: &mut Rng) -> [f32; 4] {
    [
        rng.f32_in(-100.0, 100.0),
        rng.f32_in(-100.0, 100.0),
        rng.f32_in(-100.0, 100.0),
        rng.f32_in(0.0, 50.0),
    ]
}

fn random_geom(rng: &mut Rng) -> GeomSpec {
    let mut mask = USE_POSITION;
    for bit in [USE_NORMAL, USE_COLOR0, USE_TEXCOORD0] {
        if rng.chance(1, 2) {
            mask |= bit;
        }
    }
    let n = rng.range(1, 24);
    let verts = (0..n)
        .map(|_| Vertex {
            pos: [
                rng.f32_in(-9.0, 9.0),
                rng.f32_in(-9.0, 9.0),
                rng.f32_in(-9.0, 9.0),
            ],
            normal: [
                rng.f32_in(-1.0, 1.0),
                rng.f32_in(-1.0, 1.0),
                rng.f32_in(-1.0, 1.0),
            ],
            color: rng.next_u32(),
            uv: [rng.f32_in(0.0, 1.0), rng.f32_in(0.0, 1.0)],
        })
        .collect();
    let tris = rng.range(1, 10);
    let indices = (0..tris * 3).map(|_| rng.below(n) as u16).collect();
    let palette = if rng.chance(1, 3) {
        (0..rng.range(1, 8)).map(|_| rng.below(64) as u16).collect()
    } else {
        Vec::new()
    };
    GeomSpec {
        mask,
        verts,
        indices,
        primitive: *rng.pick(&[3, 3, 4, 5]),
        palette,
    }
}

fn random_models(rng: &mut Rng, shaders: usize) -> Vec<ModelSpec> {
    (0..rng.range(1, 3))
        .map(|_| {
            let geoms: Vec<GeomSpec> = (0..rng.range(1, 3)).map(|_| random_geom(rng)).collect();
            let shader_map = geoms
                .iter()
                .map(|_| rng.below(shaders.max(1)) as u16)
                .collect();
            let b = rng.bytes(6);
            ModelSpec {
                geoms,
                shader_map,
                bytes: [b[0], b[1], b[2], b[3], b[4], b[5]],
            }
        })
        .collect()
}

fn random_shaders(rng: &mut Rng) -> Vec<ShaderSpec> {
    (0..rng.range(1, 4))
        .map(|_| ShaderSpec {
            name: format!("gta_{}", rng.ident(2, 12)),
            hash: rng.next_u32(),
            params: (0..rng.below(5))
                .map(|_| {
                    let p = match rng.below(4) {
                        0 => ParamSpec::Texture(rng.ident(1, 20)),
                        1 => ParamSpec::Vector(vec4(rng)),
                        2 => {
                            let mut m = [0.0; 16];
                            for x in &mut m {
                                *x = rng.f32_in(-2.0, 2.0);
                            }
                            ParamSpec::Matrix(m)
                        }
                        _ => ParamSpec::Float(rng.f32_in(-5.0, 5.0)),
                    };
                    (rng.next_u32(), p)
                })
                .collect(),
        })
        .collect()
}

fn random_bones(rng: &mut Rng) -> Vec<BoneSpec> {
    (0..rng.range(1, 6))
        .map(|i| BoneSpec {
            name: rng.ident(1, 16),
            index: i as i16,
            id: rng.next_u32() as i16,
            position: vec4(rng),
            parent: i as i32 - 1,
        })
        .collect()
}

fn random_drawable(rng: &mut Rng) -> DrawSpec {
    let shaders = rng.chance(3, 4).then(|| {
        let s = random_shaders(rng);
        let mut passes = [0u32; 12];
        for p in &mut passes {
            *p = rng.next_u32() & 0xFF;
        }
        (s, passes)
    });
    let n_shaders = shaders.as_ref().map_or(0, |s| s.0.len());
    let mut lods: [Option<Vec<ModelSpec>>; 4] = [None, None, None, None];
    lods[0] = Some(random_models(rng, n_shaders));
    for lod in lods.iter_mut().skip(1) {
        if rng.chance(1, 3) {
            *lod = Some(random_models(rng, n_shaders));
        }
    }
    DrawSpec {
        center: vec4(rng),
        lods,
        shaders,
        bones: rng.chance(1, 2).then(|| random_bones(rng)),
    }
}

fn stride_of(mask: u32) -> usize {
    let mut s = 12;
    if mask & USE_NORMAL != 0 {
        s += 12;
    }
    if mask & USE_COLOR0 != 0 {
        s += 4;
    }
    if mask & USE_TEXCOORD0 != 0 {
        s += 8;
    }
    s
}

fn write_geom(sys: &mut Sys, gfx: &mut Buf, g: &GeomSpec) -> usize {
    let stride = stride_of(g.mask);
    // Declaration.
    let decl = sys.alloc(16);
    let packed = NIBBLE_F32X3 | (NIBBLE_F32X3 << 12) | (NIBBLE_COLOR << 16) | (NIBBLE_F32X2 << 24);
    sys.0
        .put_u32(decl, g.mask)
        .put_u16(decl + 4, stride as u16)
        .put(decl + 8, &packed.to_le_bytes());
    // Vertex bytes in slot order.
    gfx.align(16);
    let vdata = gfx.len();
    for v in &g.verts {
        for x in v.pos {
            gfx.f32(x);
        }
        if g.mask & USE_NORMAL != 0 {
            for x in v.normal {
                gfx.f32(x);
            }
        }
        if g.mask & USE_COLOR0 != 0 {
            gfx.u32(v.color);
        }
        if g.mask & USE_TEXCOORD0 != 0 {
            gfx.f32(v.uv[0]).f32(v.uv[1]);
        }
    }
    gfx.align(16);
    let idata = gfx.len();
    for &i in &g.indices {
        gfx.u16(i);
    }
    let vb = sys.alloc(20);
    sys.0
        .put_u32(vb, 0x0067_0000)
        .put_u16(vb + 4, g.verts.len() as u16)
        .put_u32(vb + 8, GFX | vdata as u32)
        .put_u32(vb + 12, stride as u32)
        .put_u32(vb + 16, Sys::ptr(decl));
    let ib = sys.alloc(12);
    sys.0
        .put_u32(ib, 0x0067_0004)
        .put_u32(ib + 4, g.indices.len() as u32)
        .put_u32(ib + 8, GFX | idata as u32);
    let palette = if g.palette.is_empty() {
        0
    } else {
        let at = sys.alloc(g.palette.len() * 2);
        for (i, &b) in g.palette.iter().enumerate() {
            sys.0.put_u16(at + 2 * i, b);
        }
        Sys::ptr(at)
    };
    let at = sys.alloc(64);
    sys.0
        .put_u32(at, 0x0067_0008)
        .put_u32(at + 12, Sys::ptr(vb))
        .put_u32(at + 28, Sys::ptr(ib))
        .put_u32(at + 44, g.indices.len() as u32)
        .put_u32(at + 48, (g.indices.len() / 3) as u32)
        .put_u16(at + 52, g.verts.len() as u16)
        .put_u16(at + 54, g.primitive)
        .put_u32(at + 56, palette)
        .put_u16(at + 60, stride as u16)
        .put_u16(at + 62, g.palette.len() as u16);
    at
}

fn write_model(sys: &mut Sys, gfx: &mut Buf, m: &ModelSpec) -> usize {
    let geoms: Vec<usize> = m.geoms.iter().map(|g| write_geom(sys, gfx, g)).collect();
    let bounds = sys.alloc((geoms.len() + 1) * 32);
    for i in 0..(geoms.len() + 1) * 8 {
        sys.0.put_f32(bounds + 4 * i, i as f32);
    }
    let map = sys.alloc(m.shader_map.len() * 2);
    for (i, &s) in m.shader_map.iter().enumerate() {
        sys.0.put_u16(map + 2 * i, s);
    }
    let at = sys.alloc(28);
    sys.ptr_list(at + 4, &geoms);
    sys.0
        .put_u32(at, 0x0067_000C)
        .put_u32(at + 12, Sys::ptr(bounds))
        .put_u32(at + 16, Sys::ptr(map))
        .put(at + 20, &m.bytes)
        .put_u16(at + 26, geoms.len() as u16);
    at
}

fn write_shader(sys: &mut Sys, s: &ShaderSpec) -> usize {
    let n = s.params.len();
    let offsets = sys.alloc(n * 4);
    let types = sys.alloc(n);
    let names = sys.alloc(n * 4);
    for (i, (hash, p)) in s.params.iter().enumerate() {
        let (kind, at) = match p {
            ParamSpec::Texture(name) => {
                let name_at = sys.cstr(name);
                let at = sys.alloc(32);
                sys.0.put_u32(at + 20, Sys::ptr(name_at));
                (0u8, at)
            }
            ParamSpec::Vector(v) => {
                let at = sys.alloc(16);
                for (k, x) in v.iter().enumerate() {
                    sys.0.put_f32(at + 4 * k, *x);
                }
                (1, at)
            }
            ParamSpec::Matrix(m) => {
                let at = sys.alloc(64);
                for (k, x) in m.iter().enumerate() {
                    sys.0.put_f32(at + 4 * k, *x);
                }
                (4, at)
            }
            ParamSpec::Float(f) => {
                let at = sys.alloc(16);
                sys.0.put_f32(at, *f);
                (16, at)
            }
        };
        sys.0
            .put_u32(offsets + 4 * i, Sys::ptr(at))
            .put(types + i, &[kind])
            .put_u32(names + 4 * i, *hash);
    }
    let name = sys.cstr(&s.name);
    let source = sys.cstr(&format!("{}.fx", s.name));
    let at = sys.alloc(76);
    sys.0
        .put_u32(at + 20, Sys::ptr(offsets))
        .put_u32(at + 28, n as u32)
        .put_u32(at + 36, Sys::ptr(types))
        .put_u32(at + 40, s.hash)
        .put_u32(at + 52, Sys::ptr(names))
        .put_u32(at + 68, Sys::ptr(name))
        .put_u32(at + 72, Sys::ptr(source));
    at
}

fn write_skeleton(sys: &mut Sys, bones: &[BoneSpec]) -> usize {
    let n = bones.len();
    let names: Vec<usize> = bones.iter().map(|b| sys.cstr(&b.name)).collect();
    let records = sys.alloc(n * BONE_LEN);
    for (i, b) in bones.iter().enumerate() {
        let r = records + i * BONE_LEN;
        sys.0
            .put_u32(r, Sys::ptr(names[i]))
            .put_u32(r + 20, 0)
            .put_u16(r + 20, b.index as u16)
            .put_u16(r + 22, b.id as u16);
        for (k, x) in b.position.iter().enumerate() {
            sys.0.put_f32(r + 32 + 4 * k, *x);
        }
        if i > 0 {
            sys.0
                .put_u32(r + 16, Sys::ptr(records + (i - 1) * BONE_LEN));
        }
    }
    let parents = sys.alloc(n * 4);
    for (i, b) in bones.iter().enumerate() {
        sys.0.put_u32(parents + 4 * i, b.parent as u32);
    }
    let mats: Vec<usize> = (0..3)
        .map(|k| {
            let at = sys.alloc(n * 64);
            for i in 0..n * 16 {
                sys.0.put_f32(at + 4 * i, (k * 1000 + i) as f32);
            }
            at
        })
        .collect();
    let ids = sys.alloc(n * 4);
    for (i, b) in bones.iter().enumerate() {
        sys.0
            .put_u16(ids + 4 * i, b.id as u16)
            .put_u16(ids + 4 * i + 2, i as u16);
    }
    let at = sys.alloc(64);
    sys.0
        .put_u32(at, Sys::ptr(records))
        .put_u32(at + 4, Sys::ptr(parents))
        .put_u32(at + 8, Sys::ptr(mats[0]))
        .put_u32(at + 12, Sys::ptr(mats[1]))
        .put_u32(at + 16, Sys::ptr(mats[2]))
        .put_u16(at + 20, n as u16)
        .put_u32(at + 32, Sys::ptr(ids))
        .put_u16(at + 36, n as u16)
        .put_u16(at + 38, n as u16);
    at
}

/// Write a drawable header at `at` (already allocated, 96 bytes).
fn write_drawable_at(sys: &mut Sys, gfx: &mut Buf, at: usize, d: &DrawSpec) {
    let sg = d.shaders.as_ref().map(|(shaders, passes)| {
        let offs: Vec<usize> = shaders.iter().map(|s| write_shader(sys, s)).collect();
        let usage = sys.alloc(8);
        sys.0.put_u32(usage, 0xAAAA).put_u32(usage + 4, 0xBBBB);
        let g = sys.alloc(80);
        sys.ptr_list(g + 8, &offs);
        for (i, p) in passes.iter().enumerate() {
            sys.0.put_u32(g + 16 + 4 * i, *p);
        }
        sys.0
            .put_u32(g + 64, Sys::ptr(usage))
            .put_u16(g + 68, 2)
            .put_u16(g + 70, 2);
        g
    });
    let sk = d.bones.as_ref().map(|b| write_skeleton(sys, b));
    let lods: Vec<Option<usize>> = d
        .lods
        .iter()
        .map(|lod| {
            lod.as_ref().map(|models| {
                let offs: Vec<usize> = models.iter().map(|m| write_model(sys, gfx, m)).collect();
                let coll = sys.alloc(8);
                sys.ptr_list(coll, &offs);
                coll
            })
        })
        .collect();
    sys.0
        .put_u32(at, 0x0069_5254)
        .put_u32(at + 8, sg.map_or(0, Sys::ptr))
        .put_u32(at + 12, sk.map_or(0, Sys::ptr));
    for (k, x) in d.center.iter().enumerate() {
        sys.0.put_f32(at + 16 + 4 * k, *x);
    }
    for (i, lod) in lods.iter().enumerate() {
        sys.0.put_u32(at + 64 + 4 * i, lod.map_or(0, Sys::ptr));
    }
    for k in 0..4 {
        sys.0.put_f32(at + 80 + 4 * k, 9999.0);
    }
}

fn check_drawable(res: &Resource, got: &Drawable, want: &DrawSpec) {
    assert_eq!(got.center, want.center);
    assert_eq!(got.abs_max, [9999.0; 4]);
    let want_lods: Vec<&Vec<ModelSpec>> = want.lods.iter().flatten().collect();
    assert_eq!(got.lods.len(), want_lods.len());
    for (lod, wl) in got.lods.iter().zip(want_lods) {
        assert_eq!(lod.models.len(), wl.len());
        for (m, wm) in lod.models.iter().zip(wl) {
            assert_eq!(m.geometries.len(), wm.geoms.len());
            assert_eq!(m.bounds.len(), (wm.geoms.len() + 1) * 2);
            assert_eq!(m.shader_map, wm.shader_map);
            let b = wm.bytes;
            assert_eq!(
                [
                    m.matrix_count,
                    m.flags,
                    m.kind,
                    m.matrix_index,
                    m.render_mask,
                    m.skin_flag
                ],
                b
            );
            for (g, wg) in m.geometries.iter().zip(&wm.geoms) {
                assert_eq!(usize::from(g.vertex_count), wg.verts.len());
                assert_eq!(g.index_count as usize, wg.indices.len());
                assert_eq!(g.primitive, PrimitiveType::from_file(wg.primitive).unwrap());
                assert_eq!(usize::from(g.stride), stride_of(wg.mask));
                assert_eq!(g.matrix_palette, wg.palette);
                assert_eq!(g.declaration.usage_mask, wg.mask);
                assert_eq!(g.indices(res).unwrap(), wg.indices);
                let verts = g.vertices(res).unwrap();
                for (v, wv) in verts.iter().zip(&wg.verts) {
                    assert_eq!(v.pos, wv.pos);
                    if wg.mask & USE_NORMAL != 0 {
                        assert_eq!(v.normal, wv.normal);
                    }
                    if wg.mask & USE_COLOR0 != 0 {
                        assert_eq!(v.diffuse, wv.color);
                    }
                    if wg.mask & USE_TEXCOORD0 != 0 {
                        assert_eq!(v.uv, wv.uv);
                    }
                }
            }
        }
    }
    match (&got.shaders, &want.shaders) {
        (None, None) => {}
        (Some(g), Some((ws, passes))) => {
            assert_eq!(&g.pass_indices, passes);
            assert_eq!(g.usage_flags, [0xAAAA, 0xBBBB]);
            assert_eq!(g.shaders.len(), ws.len());
            for (s, w) in g.shaders.iter().zip(ws) {
                assert_eq!(s.name, w.name);
                assert_eq!(s.source, format!("{}.fx", w.name));
                assert_eq!(s.hash, w.hash);
                assert_eq!(s.params.len(), w.params.len());
                for ((h, p), (wh, wp)) in s.params.iter().zip(&w.params) {
                    assert_eq!(h, wh);
                    match (p, wp) {
                        (ShaderParam::Texture { name }, ParamSpec::Texture(w)) => {
                            assert_eq!(name, w);
                        }
                        (ShaderParam::Vector4(v), ParamSpec::Vector(w)) => assert_eq!(v, w),
                        (ShaderParam::Matrix4x4(m), ParamSpec::Matrix(w)) => {
                            assert_eq!(m.concat(), w.to_vec());
                        }
                        (ShaderParam::Float(f), ParamSpec::Float(w)) => assert_eq!(f, w),
                        other => panic!("parameter kind mismatch: {other:?}"),
                    }
                }
            }
        }
        other => panic!("shader group presence mismatch: {other:?}"),
    }
    match (&got.skeleton, &want.bones) {
        (None, None) => {}
        (Some(sk), Some(bones)) => {
            assert_eq!(sk.bones.len(), bones.len());
            assert_eq!(
                sk.parents,
                bones.iter().map(|b| b.parent).collect::<Vec<_>>()
            );
            assert_eq!(sk.default_pose.len(), bones.len());
            assert_eq!(sk.global_pose.len(), bones.len());
            for (i, (b, w)) in sk.bones.iter().zip(bones).enumerate() {
                assert_eq!(b.name, w.name);
                assert_eq!((b.index, b.bone_id), (w.index, w.id));
                assert_eq!(b.position, w.position);
                assert_eq!(sk.id_mappings[i], (w.id as u16, i as u16));
            }
        }
        other => panic!("skeleton presence mismatch: {other:?}"),
    }
}

fn write_single(d: &DrawSpec) -> (Vec<u8>, Vec<u8>) {
    let mut sys = Sys::new(96);
    let mut gfx = Buf::new();
    write_drawable_at(&mut sys, &mut gfx, 0, d);
    (sys.0.0, gfx.0)
}

#[test]
fn drawable_round_trip() {
    let mut rng = Rng::for_test("model drawable");
    for _ in 0..60 {
        let spec = random_drawable(&mut rng);
        let (sys, gfx) = write_single(&spec);
        let res = Resource::open(&rsc5(TYPE_DRAWABLE, &sys, &gfx)).expect("container");
        assert_eq!(res.kind, TYPE_DRAWABLE);
        let draw = Drawable::parse(&res).expect("generated drawable parses");
        check_drawable(&res, &draw, &spec);
        let want_verts: usize = spec
            .lods
            .iter()
            .flatten()
            .flatten()
            .flat_map(|m| &m.geoms)
            .map(|g| g.verts.len())
            .sum();
        assert_eq!(draw.vertex_count() as usize, want_verts);
    }
}

#[test]
fn dictionary_round_trip() {
    let mut rng = Rng::for_test("model dictionary");
    for _ in 0..30 {
        let specs: Vec<DrawSpec> = (0..rng.range(1, 4))
            .map(|_| random_drawable(&mut rng))
            .collect();
        let hashes: Vec<u32> = specs.iter().map(|_| rng.next_u32()).collect();
        let mut sys = Sys::new(32);
        let mut gfx = Buf::new();
        let mut offs = Vec::new();
        for s in &specs {
            let at = sys.alloc(96);
            write_drawable_at(&mut sys, &mut gfx, at, s);
            offs.push(at);
        }
        let list = sys.alloc(hashes.len() * 4);
        for (i, h) in hashes.iter().enumerate() {
            sys.0.put_u32(list + 4 * i, *h);
        }
        let n = hashes.len() as u16;
        sys.0
            .put_u32(0, 0x0069_5244)
            .put_u32(12, 1)
            .put_u32(16, Sys::ptr(list))
            .put_u16(20, n)
            .put_u16(22, n);
        sys.ptr_list(24, &offs);
        let res = Resource::open(&rsc5(TYPE_DRAWABLE, &sys.0.0, &gfx.0)).unwrap();
        let dict = DrawableDictionary::parse(&res).expect("generated dictionary parses");
        assert_eq!(dict.hashes, hashes);
        assert_eq!(dict.usage_count, 1);
        assert_eq!(dict.len(), specs.len());
        for (d, s) in dict.entries.iter().zip(&specs) {
            check_drawable(&res, d, s);
        }
    }
}

#[test]
fn fragment_round_trip() {
    let mut rng = Rng::for_test("model fragment");
    for _ in 0..30 {
        let spec = random_drawable(&mut rng);
        let mut sys = Sys::new(0x200);
        let mut gfx = Buf::new();
        let main = sys.alloc(96);
        write_drawable_at(&mut sys, &mut gfx, main, &spec);
        let n = rng.below(6);
        let mut children = Vec::new();
        let mut want = Vec::new();
        for _ in 0..n {
            let at = sys.alloc(FRAG_CHILD_LEN);
            let flags = rng.next_u32() as u8;
            let bone = rng.next_u32() as i16;
            let node = sys.alloc(16);
            sys.0.put_u32(node, 0x00AB_CDEF);
            sys.0
                .put(at + 0x0C, &[flags])
                .put_u16(at + 0x0E, bone as u16)
                .put_u32(at + 0x90, Sys::ptr(node))
                .put_u32(at + 0x98, Sys::ptr(node));
            children.push(at);
            want.push((flags, bone));
        }
        let list = sys.alloc(n * 4);
        for (i, c) in children.iter().enumerate() {
            sys.0.put_u32(list + 4 * i, Sys::ptr(*c));
        }
        sys.0
            .put_u32(FRAG_MAIN_DRAWABLE, Sys::ptr(main))
            .put_u32(FRAG_CHILD_LIST, Sys::ptr(list))
            .put(FRAG_CHILD_COUNT, &[n as u8]);
        let res = Resource::open(&rsc5(TYPE_FRAGMENT, &sys.0.0, &gfx.0)).unwrap();
        let frag = Fragment::parse(&res).expect("generated fragment parses");
        check_drawable(&res, &frag.drawable, &spec);
        assert_eq!(frag.children.len(), n);
        for (c, (flags, bone)) in frag.children.iter().zip(want) {
            assert_eq!((c.flags, c.bone_index), (flags, bone));
            assert_eq!(c.node_vtable, Some(0x00AB_CDEF));
            assert_eq!(c.physics.len(), 1);
        }
    }
}

/// Every parser on one resource; reads must stay inside the segments.
fn exercise(res: &Resource) {
    let check = |d: &Drawable| {
        for g in d.geometries() {
            if let Ok(idx) = g.indices(res) {
                assert_eq!(idx.len(), g.index_count as usize);
            }
            if let Ok(v) = g.vertices(res) {
                assert_eq!(v.len(), usize::from(g.vertex_count));
            }
            let _ = g.triangle_count();
        }
        for m in d.models() {
            assert!(m.shader_map.is_empty() || m.shader_map.len() == m.geometries.len());
        }
        let _ = (d.vertex_count(), d.index_count());
    };
    if let Ok(d) = Drawable::parse(res) {
        check(&d);
    }
    if let Ok(d) = DrawableDictionary::parse(res) {
        d.entries.iter().for_each(check);
    }
    if let Ok(f) = Fragment::parse(res) {
        check(&f.drawable);
    }
}

fn small_drawable(rng: &mut Rng) -> DrawSpec {
    let mut d = random_drawable(rng);
    d.lods = [Some(random_models(rng, 1)), None, None, None];
    d
}

#[test]
fn fuzz_system_segment() {
    let mut rng = Rng::for_test("model fuzz seeds");
    let pairs: Vec<(Vec<u8>, Vec<u8>)> = (0..4)
        .map(|_| write_single(&small_drawable(&mut rng)))
        .collect();
    let seeds: Vec<Vec<u8>> = pairs.iter().map(|(s, _)| s.clone()).collect();
    fuzz("model system segment", &seeds, 3000, |s| {
        let gfx = &pairs[s.len() % pairs.len()].1;
        exercise(&Resource {
            kind: TYPE_DRAWABLE,
            flags: 0,
            sys: s.to_vec(),
            gfx: gfx.clone(),
        });
    });
}

#[test]
fn fuzz_wrapped_files() {
    let mut rng = Rng::for_test("model fuzz files");
    let (sys, gfx) = write_single(&small_drawable(&mut rng));
    fuzz("model wrapped system", &[sys], 1000, |s| {
        if let Ok(res) = Resource::open(&rsc5(TYPE_FRAGMENT, s, &gfx)) {
            exercise(&res);
        }
    });
    let (sys, gfx) = write_single(&small_drawable(&mut rng));
    fuzz(
        "model whole file",
        &[rsc5(TYPE_DRAWABLE, &sys, &gfx)],
        1000,
        |f| {
            if let Ok(res) = Resource::open(f) {
                exercise(&res);
            }
        },
    );
}

#[test]
fn regression_colour_slot_with_wrong_size_is_an_error() {
    // Colour slot declared as a three-float element: decoding used to
    // unwrap a 12-byte slice into a u32 and panic.
    let mut decl = Buf::new();
    decl.u32(USE_COLOR0).u16(12).u8(0).u8(0);
    decl.bytes(&(NIBBLE_F32X3 << 16).to_le_bytes());
    let decl = lf_model::VertexDecl::parse(&decl.0, 0).unwrap();
    assert!(lf_model::vertex::decode_vertex(&decl, &[0u8; 12]).is_err());
}
