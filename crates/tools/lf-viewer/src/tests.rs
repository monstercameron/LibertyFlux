//! End-to-end tests on hand-built model and texture files.
//!
//! Every fixture is invented: a single triangle, one shader with one texture
//! parameter, and 4x4 DXT1 textures, laid out by the rules the format
//! readers document. Nothing here comes from the game.

use std::ffi::OsString;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::Value;

use crate::cli::{self, CliError, Command};
use crate::convert::{self, LodChoice, Options};
use crate::gltf::Mode;
use crate::png;

/// RSC5 magic, little-endian `RSC\x05`.
const RSC_MAGIC: u32 = 0x0543_5352;
/// Model resource type (drawables).
const TYPE_DRAWABLE: u32 = 0x6e;
/// Texture dictionary resource type.
const TYPE_TEXTURE: u32 = 8;
/// Model system segment size: mantissa 8, shift 0, so 8 << 8.
const MODEL_SYS_LEN: usize = 0x800;
/// Graphics segment size in both fixtures: mantissa 1, so 1 << 8.
const GFX_LEN: usize = 0x100;
/// Texture system segment size: mantissa 1, so 1 << 8.
const TEX_SYS_LEN: usize = 0x100;
/// Flags word for the model fixture: system mantissa 8, graphics mantissa 1.
const MODEL_FLAGS: u32 = 8 | (1 << 15);
/// Flags word for the texture fixture: both mantissas 1.
const TEX_FLAGS: u32 = 1 | (1 << 15);
/// DXT1 format code.
const DXT1: u32 = 0x3154_5844;
/// The texture name both fixtures use.
const TEX_NAME: &str = "pack:/fixture_tex.dds";
/// What the shader parameter calls it.
const PARAM_TEX_NAME: &str = "fixture_tex";
/// Vertex stride: position (12) + normal (12) + colour (4) + uv (8).
const STRIDE: usize = 36;

fn put_u32(b: &mut [u8], at: usize, v: u32) {
    b[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

fn put_u16(b: &mut [u8], at: usize, v: u16) {
    b[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

fn put_f32s(b: &mut [u8], at: usize, vs: &[f32]) {
    for (i, v) in vs.iter().enumerate() {
        b[at + 4 * i..at + 4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
}

fn put_str(b: &mut [u8], at: usize, s: &str) {
    b[at..at + s.len()].copy_from_slice(s.as_bytes());
    b[at + s.len()] = 0;
}

/// A system-segment pointer.
fn sys_ptr(off: usize) -> u32 {
    0x5000_0000 | u32::try_from(off).unwrap()
}

/// A graphics-segment pointer.
fn gfx_ptr(off: usize) -> u32 {
    0x6000_0000 | u32::try_from(off).unwrap()
}

/// One DXT1 block with every texel at palette index `index`.
fn dxt1_block(c0: u16, c1: u16, index: u32) -> [u8; 8] {
    let mut b = [0u8; 8];
    b[0..2].copy_from_slice(&c0.to_le_bytes());
    b[2..4].copy_from_slice(&c1.to_le_bytes());
    let bits = (0..16).fold(0u32, |acc, i| acc | (index << (2 * i)));
    b[4..8].copy_from_slice(&bits.to_le_bytes());
    b
}

/// Writes a one-texture dictionary at `base` in `sys`, with records and the
/// name placed after it, and texel data at graphics offset `data`.
fn put_dictionary(sys: &mut [u8], base: usize, data: usize) {
    let hashes = base + 0x40;
    let list = base + 0x48;
    let record = base + 0x60;
    let name = base + 0xC0;
    put_u32(sys, base + 16, sys_ptr(hashes));
    put_u16(sys, base + 20, 1);
    put_u16(sys, base + 22, 1);
    put_u32(sys, base + 24, sys_ptr(list));
    put_u16(sys, base + 28, 1);
    put_u16(sys, base + 30, 1);
    put_u32(
        sys,
        hashes,
        lf_texture::hash_title(lf_texture::title_of(TEX_NAME)),
    );
    put_u32(sys, list, sys_ptr(record));
    put_u32(sys, record + 20, sys_ptr(name));
    put_u16(sys, record + 28, 4); // width
    put_u16(sys, record + 30, 4); // height
    put_u32(sys, record + 32, DXT1);
    put_u16(sys, record + 36, 2); // stride: 8 bytes over 4 rows
    sys[record + 38] = 0; // flat texture
    sys[record + 39] = 1; // one mip level
    put_u32(sys, record + 72, gfx_ptr(data));
    put_str(sys, name, TEX_NAME);
}

/// Fixture choices.
#[derive(Clone, Copy)]
struct ModelSpec {
    /// Give the shader group an embedded dictionary (red texture).
    embedded: bool,
    /// Primitive type code for the geometry.
    primitive: u16,
    /// Indices to store (all must be < 3 for a valid file).
    indices: [u16; 3],
}

impl Default for ModelSpec {
    fn default() -> Self {
        ModelSpec {
            embedded: false,
            primitive: 3, // triangle list
            indices: [0, 1, 2],
        }
    }
}

/// The model's two segments: one LOD, one model, one triangle, one shader.
fn model_segments(spec: ModelSpec) -> (Vec<u8>, Vec<u8>) {
    let mut sys = vec![0u8; MODEL_SYS_LEN];
    let mut gfx = vec![0u8; GFX_LEN];
    // Drawable header: shader group, no skeleton, LOD 0 only.
    put_u32(&mut sys, 8, sys_ptr(0x300));
    put_u32(&mut sys, 64, sys_ptr(0x100));
    // LOD collection -> one model.
    put_u32(&mut sys, 0x100, sys_ptr(0x110));
    put_u16(&mut sys, 0x104, 1);
    put_u16(&mut sys, 0x106, 1);
    put_u32(&mut sys, 0x110, sys_ptr(0x120));
    // Model: geometry collection -> one geometry, shader map -> shader 0.
    put_u32(&mut sys, 0x124, sys_ptr(0x140));
    put_u16(&mut sys, 0x128, 1);
    put_u16(&mut sys, 0x12A, 1);
    put_u32(&mut sys, 0x130, sys_ptr(0x148));
    put_u16(&mut sys, 0x13A, 1);
    put_u32(&mut sys, 0x140, sys_ptr(0x150));
    put_u16(&mut sys, 0x148, 0);
    // Geometry: buffers, counts, primitive, stride.
    put_u32(&mut sys, 0x150 + 12, sys_ptr(0x1A0));
    put_u32(&mut sys, 0x150 + 28, sys_ptr(0x1C0));
    put_u32(&mut sys, 0x150 + 44, 3);
    put_u32(&mut sys, 0x150 + 48, 1);
    put_u16(&mut sys, 0x150 + 52, 3);
    put_u16(&mut sys, 0x150 + 54, spec.primitive);
    put_u16(&mut sys, 0x150 + 60, u16::try_from(STRIDE).unwrap());
    // Vertex buffer -> graphics 0, declaration at 0x1E0.
    put_u16(&mut sys, 0x1A0 + 4, 3);
    put_u32(&mut sys, 0x1A0 + 8, gfx_ptr(0));
    put_u32(&mut sys, 0x1A0 + 12, u32::try_from(STRIDE).unwrap());
    put_u32(&mut sys, 0x1A0 + 16, sys_ptr(0x1E0));
    // Index buffer -> graphics 0x80.
    put_u32(&mut sys, 0x1C0 + 4, 3);
    put_u32(&mut sys, 0x1C0 + 8, gfx_ptr(0x80));
    // Declaration: position (slot 0, f32x3), normal (3, f32x3), colour 0
    // (4, packed colour), texcoord 0 (6, f32x2).
    put_u32(&mut sys, 0x1E0, 1 | (1 << 3) | (1 << 4) | (1 << 6));
    put_u16(&mut sys, 0x1E4, u16::try_from(STRIDE).unwrap());
    let packed: u64 = 6 | (6 << 12) | (9 << 16) | (5 << 24);
    sys[0x1E8..0x1F0].copy_from_slice(&packed.to_le_bytes());
    // Shader group: optional embedded dictionary, one shader.
    if spec.embedded {
        put_u32(&mut sys, 0x300 + 4, sys_ptr(0x500));
        put_dictionary(&mut sys, 0x500, 0xC0);
        gfx[0xC0..0xC8].copy_from_slice(&dxt1_block(0xF800, 0x001F, 0)); // opaque red
    }
    put_u32(&mut sys, 0x300 + 8, sys_ptr(0x380));
    put_u16(&mut sys, 0x300 + 12, 1);
    put_u16(&mut sys, 0x300 + 14, 1);
    put_u32(&mut sys, 0x380, sys_ptr(0x390));
    // Shader: one texture parameter, a name, no source.
    put_u32(&mut sys, 0x390 + 20, sys_ptr(0x400));
    put_u32(&mut sys, 0x390 + 28, 1);
    put_u32(&mut sys, 0x390 + 36, sys_ptr(0x410));
    put_u32(&mut sys, 0x390 + 52, sys_ptr(0x418));
    put_u32(&mut sys, 0x390 + 68, sys_ptr(0x420));
    put_u32(&mut sys, 0x400, sys_ptr(0x440));
    sys[0x410] = 0; // texture parameter
    put_u32(&mut sys, 0x418, 0x1234_5678);
    put_str(&mut sys, 0x420, "test_shader");
    put_u32(&mut sys, 0x440 + 20, sys_ptr(0x480));
    put_str(&mut sys, 0x480, PARAM_TEX_NAME);
    // Vertices: position, normal, colour (B, G, R, A bytes), uv.
    let verts: [([f32; 3], [f32; 3], [f32; 2]); 3] = [
        ([0.0, 0.0, 0.0], [0.0, 0.0, 2.0], [0.0, 0.0]),
        ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
        ([0.0, 1.0, 2.0], [0.0, 0.0, 0.0], [0.0, 1.0]),
    ];
    for (i, (p, n, uv)) in verts.iter().enumerate() {
        let at = i * STRIDE;
        put_f32s(&mut gfx, at, p);
        put_f32s(&mut gfx, at + 12, n);
        gfx[at + 24..at + 28].copy_from_slice(&[10, 20, 30, 255]);
        put_f32s(&mut gfx, at + 28, uv);
    }
    for (i, idx) in spec.indices.iter().enumerate() {
        put_u16(&mut gfx, 0x80 + 2 * i, *idx);
    }
    (sys, gfx)
}

/// The texture dictionary file's segments: one transparent 4x4 texture.
fn texture_segments() -> (Vec<u8>, Vec<u8>) {
    let mut sys = vec![0u8; TEX_SYS_LEN];
    let mut gfx = vec![0u8; GFX_LEN];
    put_dictionary(&mut sys, 0, 0);
    // c0 <= c1 and index 3: transparent black in DXT1.
    gfx[0..8].copy_from_slice(&dxt1_block(0x001F, 0xF800, 3));
    (sys, gfx)
}

/// A whole RSC5 file: header, then the zlib stream of both segments
/// (best compression, whose zlib header is the `78 DA` codec field).
fn rsc_file(kind: u32, flags: u32, sys: &[u8], gfx: &[u8]) -> Vec<u8> {
    let mut file = Vec::new();
    file.extend_from_slice(&RSC_MAGIC.to_le_bytes());
    file.extend_from_slice(&kind.to_le_bytes());
    file.extend_from_slice(&flags.to_le_bytes());
    let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
    z.write_all(sys).unwrap();
    z.write_all(gfx).unwrap();
    file.extend_from_slice(&z.finish().unwrap());
    file
}

fn model_resource(spec: ModelSpec) -> lf_model::Resource {
    let (sys, gfx) = model_segments(spec);
    lf_model::Resource::open(&rsc_file(TYPE_DRAWABLE, MODEL_FLAGS, &sys, &gfx)).unwrap()
}

fn texture_dictionary() -> lf_texture::Dictionary {
    let (sys, gfx) = texture_segments();
    lf_texture::Dictionary::parse(&rsc_file(TYPE_TEXTURE, TEX_FLAGS, &sys, &gfx)).unwrap()
}

/// Decodes the single IHDR of a PNG: (width, height, bit depth, colour type).
fn png_header(bytes: &[u8]) -> (u32, u32, u8, u8) {
    assert_eq!(&bytes[..8], &png::SIGNATURE);
    assert_eq!(&bytes[12..16], b"IHDR");
    let w = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let h = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    (w, h, bytes[24], bytes[25])
}

/// The RGBA bytes of the first pixel of one of our PNGs.
fn png_first_pixel(bytes: &[u8]) -> [u8; 4] {
    // IDAT data starts after signature (8) + IHDR chunk (25) + IDAT length
    // and type (8): zlib header (2), stored block header (5), filter byte.
    let at = 8 + 25 + 8 + 2 + 5 + 1;
    bytes[at..at + 4].try_into().unwrap()
}

#[test]
fn converts_a_drawable_with_an_external_dictionary() {
    let res = model_resource(ModelSpec::default());
    let drawable = lf_model::Drawable::parse(&res).unwrap();
    let dicts = vec![("ext.wtd".to_string(), texture_dictionary())];
    let out = convert::convert(
        "thing",
        &[("thing".into(), &drawable)],
        &res,
        &dicts,
        Options::default(),
    );
    assert_eq!(out.scene.meshes.len(), 1);
    assert_eq!(out.vertices, 3);
    assert_eq!(out.triangles, 1);
    let prim = &out.scene.meshes[0].primitives[0];
    assert_eq!(prim.positions[2], [0.0, 1.0, 2.0]);
    assert_eq!(prim.indices, vec![0, 1, 2]);
    assert_eq!(prim.mode, Some(Mode::Triangles));
    let normals = prim.normals.as_ref().unwrap();
    assert_eq!(normals[0], [0.0, 0.0, 1.0], "normalised");
    assert_eq!(normals[2], [0.0, 0.0, 1.0], "degenerate replaced");
    assert!(
        out.warnings
            .iter()
            .any(|w| w.contains("zero-length normals"))
    );
    assert_eq!(prim.uvs.as_ref().unwrap()[1], [1.0, 0.0]);
    assert!(prim.colors.is_none(), "vertex colours are opt-in");
    // The material takes the external texture, which is transparent.
    assert_eq!(prim.material, Some(0));
    let mat = &out.scene.materials[0];
    assert_eq!(mat.name, "test_shader");
    assert_eq!(mat.image, Some(0));
    assert!(mat.alpha_mask);
    assert_eq!(out.scene.images[0].uri, "textures/fixture_tex.png");
    assert_eq!(out.textures.len(), 1);
    assert_eq!(
        png_header(&out.textures[0].png),
        (4, 4, 8, png::COLOR_TYPE_RGBA)
    );
    assert_eq!(png_first_pixel(&out.textures[0].png), [0, 0, 0, 0]);
    // +Z up becomes +Y up: the root rotation maps Z onto Y.
    let q = out.scene.root_rotation.unwrap();
    let up = q * lf_math::Vec3::Z;
    assert!((up - lf_math::Vec3::Y).length() < 1e-6);
}

#[test]
fn embedded_dictionary_wins_over_external() {
    let res = model_resource(ModelSpec {
        embedded: true,
        ..ModelSpec::default()
    });
    let drawable = lf_model::Drawable::parse(&res).unwrap();
    assert_eq!(drawable.shaders.as_ref().unwrap().texture_dict, Some(0x500));
    let embedded = convert::embedded_dictionary(&res, 0x500).unwrap();
    assert_eq!(embedded.len(), 1);
    assert_eq!(embedded.entries()[0].name, TEX_NAME);
    let dicts = vec![("ext.wtd".to_string(), texture_dictionary())];
    let out = convert::convert(
        "thing",
        &[("thing".into(), &drawable)],
        &res,
        &dicts,
        Options::default(),
    );
    assert!(
        !out.scene.materials[0].alpha_mask,
        "the embedded texture is opaque"
    );
    assert_eq!(png_first_pixel(&out.textures[0].png), [255, 0, 0, 255]);
    assert!(convert::embedded_dictionary(&res, MODEL_SYS_LEN - 4).is_err());
}

#[test]
fn missing_textures_and_options() {
    let res = model_resource(ModelSpec::default());
    let drawable = lf_model::Drawable::parse(&res).unwrap();
    let opts = Options {
        vertex_colors: true,
        flip_winding: true,
        z_up_to_y_up: false,
        ..Options::default()
    };
    let out = convert::convert("t", &[("t".into(), &drawable)], &res, &[], opts);
    let prim = &out.scene.meshes[0].primitives[0];
    assert_eq!(
        prim.colors.as_ref().unwrap()[0],
        [30, 20, 10, 255],
        "BGRA bytes become RGBA"
    );
    assert_eq!(prim.indices, vec![0, 2, 1], "winding flipped");
    assert!(out.scene.root_rotation.is_none());
    assert_eq!(out.scene.materials[0].image, None);
    assert!(
        out.warnings
            .iter()
            .any(|w| w.contains("fixture_tex") && w.contains("none was found"))
    );
    // A LOD that does not exist exports nothing, with a warning.
    let opts = Options {
        lod: LodChoice::Index(3),
        ..Options::default()
    };
    let out = convert::convert("t", &[("t".into(), &drawable)], &res, &[], opts);
    assert!(out.scene.meshes.is_empty());
    assert!(out.warnings.iter().any(|w| w.contains("LOD 3")));
    let opts = Options {
        lod: LodChoice::All,
        ..Options::default()
    };
    let out = convert::convert("t", &[("t".into(), &drawable)], &res, &[], opts);
    assert_eq!(out.scene.meshes[0].name, "t_lod0");
}

#[test]
fn bad_geometry_is_skipped_not_fatal() {
    let res = model_resource(ModelSpec {
        indices: [0, 1, 9],
        ..ModelSpec::default()
    });
    let drawable = lf_model::Drawable::parse(&res).unwrap();
    let out = convert::convert(
        "t",
        &[("t".into(), &drawable)],
        &res,
        &[],
        Options::default(),
    );
    assert!(out.scene.meshes.is_empty());
    assert!(out.warnings.iter().any(|w| w.contains("index 9")));
}

#[test]
fn strips_and_fans() {
    let strip = convert::triangulate(Mode::TriangleStrip, &[0, 1, 2, 3, 3, 4]);
    // (0,1,2), (2,1,3) reversed odd, (2,3,3) and (3,3,4) degenerate.
    assert_eq!(strip, vec![0, 1, 2, 2, 1, 3]);
    let fan = convert::triangulate(Mode::TriangleFan, &[0, 1, 2, 3]);
    assert_eq!(fan, vec![0, 1, 2, 0, 2, 3]);
    assert_eq!(
        convert::triangulate(Mode::Triangles, &[5, 6, 7]),
        vec![5, 6, 7]
    );
    assert!(convert::triangulate(Mode::TriangleFan, &[]).is_empty());
    // A strip geometry keeps its mode unless winding is flipped.
    let res = model_resource(ModelSpec {
        primitive: 4,
        ..ModelSpec::default()
    });
    let drawable = lf_model::Drawable::parse(&res).unwrap();
    let out = convert::convert(
        "t",
        &[("t".into(), &drawable)],
        &res,
        &[],
        Options::default(),
    );
    assert_eq!(
        out.scene.meshes[0].primitives[0].mode,
        Some(Mode::TriangleStrip)
    );
}

#[test]
fn file_names_are_safe_and_unique() {
    assert_eq!(convert::sanitize("pack:/a b\\c.dds"), "pack__a_b_c.dds");
    assert_eq!(convert::sanitize("../up"), "_._up");
    assert_eq!(convert::sanitize(""), "unnamed");
    let (files, warnings) = convert::dictionary_to_pngs(&texture_dictionary());
    assert!(warnings.is_empty());
    assert_eq!(files[0].file_name, "fixture_tex.png");
}

/// A scratch folder under the system temporary directory, removed on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir =
            std::env::temp_dir().join(format!("lf-viewer-test-{}-{label}-{n}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        TempDir(dir)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn run(list: &[&str]) -> (Result<(), CliError>, String) {
    let mut log = Vec::new();
    let r = cli::run(&args(list), &mut log);
    (r, String::from_utf8(log).unwrap())
}

#[test]
fn command_line_end_to_end() {
    let tmp = TempDir::new("cli");
    let (msys, mgfx) = model_segments(ModelSpec::default());
    let (tsys, tgfx) = texture_segments();
    let wdr = tmp.0.join("thing.wdr");
    let wtd = tmp.0.join("thing.wtd");
    std::fs::write(&wdr, rsc_file(TYPE_DRAWABLE, MODEL_FLAGS, &msys, &mgfx)).unwrap();
    std::fs::write(&wtd, rsc_file(TYPE_TEXTURE, TEX_FLAGS, &tsys, &tgfx)).unwrap();
    let out = tmp.0.join("out");
    let (wdr_s, wtd_s, out_s) = (
        wdr.to_str().unwrap(),
        wtd.to_str().unwrap(),
        out.to_str().unwrap(),
    );

    let (r, log) = run(&["model", wdr_s, "--textures", wtd_s, "--out", out_s]);
    r.unwrap();
    assert!(
        log.contains("1 meshes, 3 vertices, 1 triangles, 1 textures"),
        "{log}"
    );
    let json = std::fs::read_to_string(out.join("thing.gltf")).unwrap();
    let doc: Value = serde_json::from_str(&json).unwrap();
    let bin = std::fs::read(out.join("thing.bin")).unwrap();
    assert_eq!(doc["buffers"][0]["uri"], "thing.bin");
    assert_eq!(
        doc["buffers"][0]["byteLength"].as_u64().unwrap(),
        bin.len() as u64
    );
    assert_eq!(doc["images"][0]["uri"], "textures/fixture_tex.png");
    let png_bytes = std::fs::read(out.join("textures/fixture_tex.png")).unwrap();
    assert_eq!(png_header(&png_bytes).0, 4);

    // A second run refuses to overwrite; --force allows it.
    let (r, _) = run(&["model", wdr_s, "--textures", wtd_s, "--out", out_s]);
    assert!(matches!(r, Err(CliError::Exists(_))));
    let (r, _) = run(&["model", wdr_s, "--out", out_s, "--force", "--all-lods"]);
    r.unwrap();

    // The kind is guessed from the resource when the extension says nothing.
    let odd = tmp.0.join("thing.bin");
    std::fs::copy(&wdr, &odd).unwrap();
    let out2 = tmp.0.join("out2");
    let (r, _) = run(&[
        "model",
        odd.to_str().unwrap(),
        "--out",
        out2.to_str().unwrap(),
    ]);
    r.unwrap();
    assert!(out2.join("thing.gltf").exists());

    // The texture command writes every texture.
    let tex_out = tmp.0.join("tex");
    let (r, log) = run(&["texture", wtd_s, "--out", tex_out.to_str().unwrap()]);
    r.unwrap();
    assert!(log.contains("wrote 1 of 1 textures"));
    assert!(tex_out.join("fixture_tex.png").exists());

    // Inputs that are not model files are reported, not panicked on.
    let (r, _) = run(&["model", wtd_s, "--kind", "wdr", "--out", out_s, "--force"]);
    assert!(matches!(r, Err(CliError::Format { .. })));
    let (r, _) = run(&[
        "model",
        tmp.0.join("missing.wdr").to_str().unwrap(),
        "--out",
        out_s,
    ]);
    assert!(matches!(r, Err(CliError::Io { .. })));
}

#[test]
fn argument_parsing() {
    let (r, log) = run(&["--help"]);
    r.unwrap();
    assert!(log.contains("never commit it"));
    assert!(log.contains("bought"));
    assert_eq!(cli::parse_args(&[]).unwrap(), Command::Help);
    for bad in [
        &["frobnicate"][..],
        &["model", "a.wdr"],
        &["model", "--out", "o"],
        &["model", "a.wdr", "b.wdr", "--out", "o"],
        &["model", "a.wdr", "--out"],
        &["model", "a.wdr", "--out", "o", "--lod", "x"],
        &["model", "a.wdr", "--out", "o", "--kind", "obj"],
        &["texture", "a.wtd", "--out", "o", "--lod", "1"],
        &["model", "a.wdr", "--out", "o", "--wat"],
    ] {
        assert!(
            matches!(cli::parse_args(&args(bad)), Err(CliError::Usage(_))),
            "{bad:?}"
        );
    }
    match cli::parse_args(&args(&[
        "model",
        "a.wdr",
        "--out",
        "o",
        "--lod",
        "2",
        "--flip-winding",
        "--keep-z-up",
    ]))
    .unwrap()
    {
        Command::Model { options, force, .. } => {
            assert_eq!(options.lod, LodChoice::Index(2));
            assert!(options.flip_winding && !options.z_up_to_y_up && !force);
        }
        other => panic!("{other:?}"),
    }
}

/// The `info` and `batch` commands, on the same invented fixtures.
mod info_and_batch {
    use std::path::Path;

    use lf_model::drawable::PrimitiveType;
    use lf_model::{ElementType, ElementUsage};
    use lf_texture::{D3DFormat, TextureKind};

    use super::*;
    use crate::batch;
    use crate::cli::ModelKind;
    use crate::info::{self, FileInfo, FileKind, MaterialInfo, Totals};

    fn wdr_bytes(spec: ModelSpec) -> Vec<u8> {
        let (sys, gfx) = model_segments(spec);
        rsc_file(TYPE_DRAWABLE, MODEL_FLAGS, &sys, &gfx)
    }

    fn wtd_bytes() -> Vec<u8> {
        let (sys, gfx) = texture_segments();
        rsc_file(TYPE_TEXTURE, TEX_FLAGS, &sys, &gfx)
    }

    fn model_info(path: &str, bytes: &[u8]) -> info::ModelInfo {
        match info::inspect(Path::new(path), bytes, None).unwrap() {
            FileInfo::Model(m) => m,
            FileInfo::Textures(_) => panic!("read as textures"),
        }
    }

    /// The sorted names in a folder.
    fn listing(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn info_describes_a_drawable() {
        let m = model_info("thing.wdr", &wdr_bytes(ModelSpec::default()));
        assert_eq!(m.kind, ModelKind::Drawable);
        assert_eq!(m.resource_type, TYPE_DRAWABLE);
        assert_eq!((m.system_bytes, m.graphics_bytes), (MODEL_SYS_LEN, GFX_LEN));
        assert_eq!(m.fragment_children, None);
        let d = &m.drawables[0];
        assert_eq!(d.label, "thing");
        assert_eq!(d.bones, None);
        assert_eq!(d.lods.len(), 1);
        assert_eq!(d.lods[0].models, 1);
        let g = &d.lods[0].geometries[0];
        assert_eq!(g.primitive, PrimitiveType::TriangleList);
        assert_eq!((g.vertices, g.indices, g.triangles), (3, 3, 1));
        assert_eq!((g.stride, g.shader, g.palette), (36, Some(0), 0));
        let layout: Vec<(ElementUsage, ElementType)> =
            g.elements.iter().map(|e| (e.usage, e.kind)).collect();
        assert_eq!(
            layout,
            [
                (ElementUsage::Position, ElementType::F32x3),
                (ElementUsage::Normal, ElementType::F32x3),
                (ElementUsage::Color(0), ElementType::Color),
                (ElementUsage::TexCoord(0), ElementType::F32x2),
            ]
        );
        assert_eq!(
            d.materials,
            [MaterialInfo {
                name: "test_shader".into(),
                parameters: 1,
                textures: vec![PARAM_TEX_NAME.into()],
            }]
        );
        assert_eq!(d.embedded, None);
        assert_eq!(
            m.totals(),
            Totals {
                drawables: 1,
                lods: 1,
                models: 1,
                geometries: 1,
                vertices: 3,
                indices: 3,
                triangles: 1,
                materials: 1,
                embedded_textures: 0,
            }
        );
        // A strip of three indices draws one triangle too.
        let strip = model_info(
            "thing.wdr",
            &wdr_bytes(ModelSpec {
                primitive: 4,
                ..ModelSpec::default()
            }),
        );
        let g = &strip.drawables[0].lods[0].geometries[0];
        assert_eq!(
            (g.primitive, g.triangles),
            (PrimitiveType::TriangleStrip, 1)
        );
    }

    #[test]
    fn info_lists_embedded_and_standalone_textures() {
        let m = model_info(
            "thing.wdr",
            &wdr_bytes(ModelSpec {
                embedded: true,
                ..ModelSpec::default()
            }),
        );
        let Some(Ok(embedded)) = &m.drawables[0].embedded else {
            panic!("no embedded dictionary: {:?}", m.drawables[0].embedded);
        };
        let expected = info::TextureInfo {
            name: TEX_NAME.into(),
            format: D3DFormat::Dxt1,
            width: 4,
            height: 4,
            levels: 1,
            kind: TextureKind::Flat,
            readable_levels: 1,
            data_bytes: Some(8),
        };
        assert_eq!(embedded.textures, std::slice::from_ref(&expected));
        assert_eq!(m.totals().embedded_textures, 1);
        match info::inspect(Path::new("thing.wtd"), &wtd_bytes(), None).unwrap() {
            FileInfo::Textures(dict) => assert_eq!(dict.textures, [expected]),
            FileInfo::Model(_) => panic!("read as a model"),
        }
    }

    #[test]
    fn info_guesses_the_kind_and_reports_bad_files() {
        let wdr = wdr_bytes(ModelSpec::default());
        let wtd = wtd_bytes();
        // No extension: the header decides.
        assert!(matches!(
            info::inspect(Path::new("blob"), &wtd, None),
            Ok(FileInfo::Textures(_))
        ));
        assert!(matches!(
            info::inspect(Path::new("blob"), &wdr, None),
            Ok(FileInfo::Model(m)) if m.kind == ModelKind::Drawable
        ));
        // The override beats the extension.
        assert!(matches!(
            info::inspect(
                Path::new("thing.wtd"),
                &wdr,
                Some(FileKind::Model(ModelKind::Drawable))
            ),
            Ok(FileInfo::Model(_))
        ));
        assert!(info::inspect(Path::new("thing.wdr"), &wtd, None).is_err());
        assert!(info::inspect(Path::new("thing.wtd"), &wdr, None).is_err());
        assert!(info::inspect(Path::new("junk"), b"not a resource", None).is_err());
        assert_eq!(
            FileKind::parse("WTD"),
            Some(FileKind::Textures),
            "kinds ignore case"
        );
        assert_eq!(
            FileKind::parse("wft"),
            Some(FileKind::Model(ModelKind::Fragment))
        );
        assert_eq!(FileKind::parse("obj"), None);
    }

    #[test]
    fn info_command_prints_text_and_json_and_writes_nothing() {
        let tmp = TempDir::new("info");
        let wdr = tmp.0.join("thing.wdr");
        let wtd = tmp.0.join("thing.wtd");
        std::fs::write(&wdr, wdr_bytes(ModelSpec::default())).unwrap();
        std::fs::write(&wtd, wtd_bytes()).unwrap();
        let (wdr_s, wtd_s) = (wdr.to_str().unwrap(), wtd.to_str().unwrap());
        let before = listing(&tmp.0);

        let (r, text) = run(&["info", wdr_s]);
        r.unwrap();
        for needle in [
            ": drawable; resource type 0x6e; system segment 2048 bytes, graphics segment 256 bytes\n",
            "\ndrawable thing\n",
            "\n  skeleton: none\n",
            "\n  LOD 0: 1 model, 1 geometry, 3 vertices, 3 indices, 1 triangle\n",
            "\n    model 0 geometry 0: triangle list, 3 vertices, 3 indices, 1 triangle, stride 36, shader 0\n",
            "\n      vertex: position f32x3, normal f32x3, color0 color, texcoord0 f32x2\n",
            "\n  materials: 1\n    0 test_shader: 1 parameter; textures: fixture_tex\n",
            "\n  embedded textures: none\n",
            "\ntotals: 1 drawable, 1 LOD, 1 model, 1 geometry, 3 vertices, 3 indices, 1 triangle, 1 material, 0 embedded textures\n",
        ] {
            assert!(text.contains(needle), "{needle}\n{text}");
        }

        let (r, json) = run(&["info", wdr_s, "--json"]);
        r.unwrap();
        let doc: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(doc["type"], "drawable");
        assert_eq!(doc["resource_type"], 0x6e);
        let d = &doc["drawables"][0];
        assert_eq!(d["label"], "thing");
        assert_eq!(d["bones"], Value::Null);
        assert_eq!(d["embedded_textures"], Value::Null);
        let g = &d["lods"][0]["geometries"][0];
        assert_eq!(g["primitive"], "triangle list");
        assert_eq!(g["vertices"], 3);
        assert_eq!(g["shader"], 0);
        assert_eq!(g["elements"][3]["usage"], "texcoord0");
        assert_eq!(g["elements"][3]["offset"], 28);
        assert_eq!(d["materials"][0]["textures"][0], "fixture_tex");
        assert_eq!(doc["totals"]["triangles"], 1);

        let (r, json) = run(&["info", wtd_s, "--json"]);
        r.unwrap();
        let doc: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(doc["type"], "texture dictionary");
        let t = &doc["textures"][0];
        assert_eq!(t["name"], TEX_NAME);
        assert_eq!(t["format"], "DXT1");
        assert_eq!(t["format_code"], DXT1);
        assert_eq!(
            (t["width"].as_u64(), t["height"].as_u64()),
            (Some(4), Some(4))
        );
        assert_eq!(t["levels"], 1);
        assert_eq!(t["readable_levels"], 1);
        assert_eq!(t["data_bytes"], 8);
        assert_eq!(t["kind"], "flat");
        let (r, text) = run(&["info", wtd_s]);
        r.unwrap();
        assert!(
            text.contains(
                "texture dictionary, 1 texture\n  pack:/fixture_tex.dds: DXT1 4x4, 1 mip level, flat, 8 bytes, 1 of 1 readable\n"
            ),
            "{text}"
        );

        // Nothing was written next to the inputs.
        assert_eq!(listing(&tmp.0), before);
        let (r, _) = run(&["info", wtd_s, "--kind", "wdr"]);
        assert!(matches!(r, Err(CliError::Format { .. })));
        let missing = tmp.0.join("missing.wdr");
        let (r, _) = run(&["info", missing.to_str().unwrap()]);
        assert!(matches!(r, Err(CliError::Io { .. })));
    }

    #[test]
    fn info_and_batch_arguments() {
        for bad in [
            &["info"][..],
            &["info", "a.wdr", "--out", "o"],
            &["info", "a.wdr", "--force"],
            &["info", "a.wdr", "b.wdr"],
            &["info", "a.wdr", "--kind", "obj"],
            &["info", "a.wdr", "--kind"],
            &["info", "a.wdr", "--wat"],
            &["batch", "--out", "o"],
            &["batch", "in"],
            &["batch", "in", "--out", "o", "--kind", "wdr"],
            &["model", "a.wdr", "--out", "o", "--recursive"],
            &["texture", "a.wtd", "--out", "o", "--recursive"],
        ] {
            assert!(
                matches!(cli::parse_args(&args(bad)), Err(CliError::Usage(_))),
                "{bad:?}"
            );
        }
        assert_eq!(
            cli::parse_args(&args(&["info", "a.wtd", "--json", "--kind", "WDD"])).unwrap(),
            Command::Info {
                input: PathBuf::from("a.wtd"),
                kind: Some(FileKind::Model(ModelKind::Dictionary)),
                json: true,
            }
        );
        assert_eq!(
            cli::parse_args(&args(&["info", "a.wdr", "--help"])).unwrap(),
            Command::Help
        );
        match cli::parse_args(&args(&[
            "batch",
            "in",
            "--out",
            "o",
            "--recursive",
            "--textures",
            "t.wtd",
            "--all-lods",
            "--force",
        ]))
        .unwrap()
        {
            Command::Batch {
                input,
                textures,
                out,
                force,
                recursive,
                options,
            } => {
                assert_eq!(input, PathBuf::from("in"));
                assert_eq!(textures, [PathBuf::from("t.wtd")]);
                assert_eq!(out, PathBuf::from("o"));
                assert!(force && recursive);
                assert_eq!(options.lod, LodChoice::All);
            }
            other => panic!("{other:?}"),
        }
        let (r, log) = run(&["--help"]);
        r.unwrap();
        assert!(log.contains("lf-viewer info <FILE>"));
        assert!(log.contains("lf-viewer batch <FOLDER>"));
    }

    #[test]
    fn output_folders_are_sanitized_and_unique() {
        let inputs: Vec<(PathBuf, FileKind)> = [
            "a.wdr",
            "a.wtd",
            "A.wft",
            "a_wtd.wdr",
            "a.WTD",
            "sub/a.wdr",
            "b c.wdr",
        ]
        .iter()
        .map(|p| {
            let path = PathBuf::from(p);
            let kind = FileKind::from_extension(&path).unwrap();
            (path, kind)
        })
        .collect();
        let folders: Vec<PathBuf> = batch::output_folders(&inputs);
        let expected: Vec<PathBuf> = [
            "a",
            "a_wtd",
            "A_wft",
            "a_wtd_wdr",
            "a_wtd_2",
            "sub/a",
            "b_c",
        ]
        .iter()
        .map(PathBuf::from)
        .collect();
        assert_eq!(folders, expected);
    }

    #[test]
    fn batch_converts_a_folder_and_summarises() {
        let tmp = TempDir::new("batch");
        let input = tmp.0.join("in");
        std::fs::create_dir_all(input.join("sub")).unwrap();
        std::fs::write(input.join("thing.wdr"), wdr_bytes(ModelSpec::default())).unwrap();
        std::fs::write(input.join("thing.wtd"), wtd_bytes()).unwrap();
        std::fs::write(input.join("broken.wdr"), b"not a model").unwrap();
        std::fs::write(input.join("notes.txt"), b"x").unwrap();
        std::fs::write(input.join("README"), b"x").unwrap();
        std::fs::write(
            input.join("sub").join("other.wdr"),
            wdr_bytes(ModelSpec::default()),
        )
        .unwrap();
        let before = listing(&input);
        let out = tmp.0.join("out");
        let (in_s, out_s) = (input.to_str().unwrap(), out.to_str().unwrap());

        let (r, log) = run(&["batch", in_s, "--out", out_s]);
        r.unwrap();
        assert!(out.join("thing").join("thing.gltf").exists(), "{log}");
        assert!(out.join("thing").join("thing.bin").exists());
        assert!(out.join("thing_wtd").join("fixture_tex.png").exists());
        assert!(!out.join("sub").exists(), "subfolders need --recursive");
        assert_eq!(listing(&out), ["thing", "thing_wtd"]);
        for needle in [
            "[1/3] broken.wdr -> ",
            "\nskipped: ",
            "[2/3] thing.wdr -> ",
            "1 meshes, 3 vertices, 1 triangles, 0 textures",
            "[3/3] thing.wtd -> ",
            "wrote 1 of 1 textures",
            "\nbatch: 2 converted, 1 skipped, 2 ignored; output in ",
            "\nskipped:\n  broken.wdr: ",
            "\nignored (not .wdr, .wdd, .wft or .wtd): (none) 1, .txt 1\n",
        ] {
            assert!(log.contains(needle), "{needle}\n{log}");
        }
        assert_eq!(listing(&input), before, "the input folder is untouched");

        // Again: everything exists now, so nothing converts and the run fails.
        let (r, log) = run(&["batch", in_s, "--out", out_s]);
        assert!(matches!(r, Err(CliError::Empty(_))), "{r:?}");
        assert!(log.contains("already exists (use --force to overwrite)"));
        assert!(log.contains("batch: 0 converted, 3 skipped"));
        let (r, _) = run(&["batch", in_s, "--out", out_s, "--force"]);
        r.unwrap();

        // --recursive mirrors subfolders; --textures reaches every model.
        let out2 = tmp.0.join("out2");
        let wtd = input.join("thing.wtd");
        let (r, log) = run(&[
            "batch",
            in_s,
            "--out",
            out2.to_str().unwrap(),
            "--recursive",
            "--textures",
            wtd.to_str().unwrap(),
        ]);
        r.unwrap();
        assert!(log.contains("batch: 3 converted, 1 skipped"), "{log}");
        assert!(out2.join("sub").join("other").join("other.gltf").exists());
        assert!(
            out2.join("thing")
                .join("textures")
                .join("fixture_tex.png")
                .exists()
        );
    }

    #[test]
    fn batch_never_walks_its_own_output_and_reports_empty_folders() {
        let tmp = TempDir::new("batch-self");
        let input = tmp.0.join("in");
        std::fs::create_dir_all(&input).unwrap();
        std::fs::write(input.join("thing.wdr"), wdr_bytes(ModelSpec::default())).unwrap();
        let out = input.join("converted");
        let (in_s, out_s) = (input.to_str().unwrap(), out.to_str().unwrap());
        let mut log = Vec::new();
        let first =
            batch::run(&input, &[], &out, false, true, Options::default(), &mut log).unwrap();
        assert_eq!(first.converted.len(), 1);
        // The second run would see .gltf and .bin files if it walked `converted`.
        let second =
            batch::run(&input, &[], &out, true, true, Options::default(), &mut log).unwrap();
        assert!(second.ignored.is_empty(), "{:?}", second.ignored);
        assert_eq!(
            second.converted,
            [(PathBuf::from("thing.wdr"), PathBuf::from("thing"))]
        );
        let found = batch::find(&input, true, None).unwrap();
        assert!(
            found.ignored.contains_key(".gltf"),
            "without the exclusion it is walked"
        );

        let empty = tmp.0.join("empty");
        std::fs::create_dir_all(&empty).unwrap();
        let (r, _) = run(&["batch", empty.to_str().unwrap(), "--out", out_s]);
        match r {
            Err(CliError::Empty(msg)) => assert!(msg.contains("no .wdr, .wdd, .wft or .wtd files")),
            other => panic!("{other:?}"),
        }
        let (r, _) = run(&[
            "batch",
            tmp.0.join("nope").to_str().unwrap(),
            "--out",
            out_s,
        ]);
        assert!(matches!(r, Err(CliError::Io { .. })));
        let (r, _) = run(&[
            "batch",
            in_s,
            "--out",
            out_s,
            "--textures",
            tmp.0.join("nope.wtd").to_str().unwrap(),
        ]);
        assert!(matches!(r, Err(CliError::Io { .. })));
    }

    #[cfg(unix)]
    #[test]
    fn batch_reads_file_links_but_not_folder_links() {
        let tmp = TempDir::new("batch-links");
        let input = tmp.0.join("in");
        let elsewhere = tmp.0.join("elsewhere");
        std::fs::create_dir_all(&input).unwrap();
        std::fs::create_dir_all(&elsewhere).unwrap();
        std::fs::write(elsewhere.join("far.wdr"), wdr_bytes(ModelSpec::default())).unwrap();
        std::os::unix::fs::symlink(elsewhere.join("far.wdr"), input.join("near.wdr")).unwrap();
        std::os::unix::fs::symlink(&elsewhere, input.join("loop")).unwrap();
        std::os::unix::fs::symlink(&input, input.join("self")).unwrap();
        let found = batch::find(&input, true, None).unwrap();
        let names: Vec<PathBuf> = found.inputs.iter().map(|(p, _)| p.clone()).collect();
        assert_eq!(names, [PathBuf::from("near.wdr")]);
        assert!(found.unreadable.is_empty());
    }
}
