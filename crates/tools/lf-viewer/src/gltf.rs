//! A glTF 2.0 writer for static meshes: a `.gltf` JSON document plus one
//! `.bin` buffer, with PNG images referenced by relative URI.
//!
//! Layout of the output:
//!
//! - One scene with one root node. The root carries the axis conversion
//!   (see [`Scene::root_rotation`]) so vertex data stays exactly as the
//!   source stored it; every mesh hangs off the root as a child node.
//! - Every vertex attribute and index list gets its own tightly packed
//!   buffer view, each starting on a 4-byte boundary as the specification
//!   requires. Positions carry the `min` and `max` the specification makes
//!   mandatory.
//! - Indices are written as unsigned 16-bit when every index fits, else
//!   unsigned 32-bit.
//! - Materials are metal-free, fully rough base-colour materials, double
//!   sided (the source's winding convention is not confirmed, so culling
//!   would risk hiding faces), with alpha masking when their texture has
//!   transparent texels.

use lf_math::Quat;

use crate::json::Json;

/// glTF component type: unsigned byte.
const UNSIGNED_BYTE: i64 = 5121;
/// glTF component type: unsigned short.
const UNSIGNED_SHORT: i64 = 5123;
/// glTF component type: unsigned int.
const UNSIGNED_INT: i64 = 5125;
/// glTF component type: float.
const FLOAT: i64 = 5126;
/// Buffer view target for vertex data.
const ARRAY_BUFFER: i64 = 34_962;
/// Buffer view target for index data.
const ELEMENT_ARRAY_BUFFER: i64 = 34_963;
/// Sampler filter: linear.
const LINEAR: i64 = 9729;
/// Sampler filter: linear, mipmaps linear.
const LINEAR_MIPMAP_LINEAR: i64 = 9987;
/// Sampler wrap: repeat.
const REPEAT: i64 = 10_497;
/// Alpha cutoff for masked materials (the glTF default).
const ALPHA_CUTOFF: f32 = 0.5;

/// How a primitive's indices form shapes (glTF `mode` values).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Points (0).
    Points,
    /// Separate line segments (1).
    Lines,
    /// A connected line strip (3).
    LineStrip,
    /// Separate triangles (4).
    Triangles,
    /// A triangle strip (5).
    TriangleStrip,
    /// A triangle fan (6).
    TriangleFan,
}

impl Mode {
    /// The glTF `mode` number.
    #[must_use]
    pub fn code(self) -> i64 {
        match self {
            Mode::Points => 0,
            Mode::Lines => 1,
            Mode::LineStrip => 3,
            Mode::Triangles => 4,
            Mode::TriangleStrip => 5,
            Mode::TriangleFan => 6,
        }
    }
}

/// One draw: vertex attributes, indices, mode and material.
#[derive(Debug, Clone, Default)]
pub struct Primitive {
    /// Positions; required, all finite.
    pub positions: Vec<[f32; 3]>,
    /// Unit normals, one per position.
    pub normals: Option<Vec<[f32; 3]>>,
    /// First texture coordinate set, one per position (v down, as in glTF).
    pub uvs: Option<Vec<[f32; 2]>>,
    /// Vertex colours as RGBA bytes, one per position.
    pub colors: Option<Vec<[u8; 4]>>,
    /// Indices into the attributes; all below `positions.len()`.
    pub indices: Vec<u32>,
    /// How the indices form shapes.
    pub mode: Option<Mode>,
    /// Index into [`Scene::materials`].
    pub material: Option<usize>,
}

/// A named mesh: one or more primitives.
#[derive(Debug, Clone, Default)]
pub struct Mesh {
    /// Node and mesh name.
    pub name: String,
    /// The draws.
    pub primitives: Vec<Primitive>,
}

/// A material: name, optional base-colour image, alpha masking.
#[derive(Debug, Clone, Default)]
pub struct Material {
    /// Material name (the source shader's name).
    pub name: String,
    /// Index into [`Scene::images`] of the base-colour texture.
    pub image: Option<usize>,
    /// True to cut out texels whose alpha is below one half.
    pub alpha_mask: bool,
}

/// An image file referenced by relative URI.
#[derive(Debug, Clone, Default)]
pub struct Image {
    /// Display name (the source texture's name).
    pub name: String,
    /// Relative URI of the PNG file, already safe to use unescaped.
    pub uri: String,
}

/// Everything one glTF file holds.
#[derive(Debug, Clone, Default)]
pub struct Scene {
    /// Name of the root node.
    pub root_name: String,
    /// Rotation applied at the root node: converts the source's axes to
    /// glTF's (+Y up). `None` leaves the axes as stored.
    pub root_rotation: Option<Quat>,
    /// Meshes, one child node each.
    pub meshes: Vec<Mesh>,
    /// Materials.
    pub materials: Vec<Material>,
    /// Images.
    pub images: Vec<Image>,
}

/// Appends data to the binary buffer and records views and accessors.
struct BufferWriter {
    bin: Vec<u8>,
    views: Vec<Json>,
    accessors: Vec<Json>,
}

impl BufferWriter {
    /// Appends `bytes` as a new buffer view (4-byte aligned) and returns its
    /// index.
    fn view(&mut self, bytes: &[u8], target: i64) -> usize {
        while !self.bin.len().is_multiple_of(4) {
            self.bin.push(0);
        }
        let offset = self.bin.len();
        self.bin.extend_from_slice(bytes);
        self.views.push(Json::obj([
            ("buffer", Json::Int(0)),
            ("byteOffset", Json::uint(offset)),
            ("byteLength", Json::uint(bytes.len())),
            ("target", Json::Int(target)),
        ]));
        self.views.len() - 1
    }

    /// Adds an accessor over a whole view and returns its index.
    fn accessor(
        &mut self,
        view: usize,
        component: i64,
        count: usize,
        kind: &str,
        extra: Vec<(String, Json)>,
    ) -> usize {
        let mut acc = Json::obj([
            ("bufferView", Json::uint(view)),
            ("componentType", Json::Int(component)),
            ("count", Json::uint(count)),
            ("type", Json::str(kind)),
        ]);
        for (k, v) in extra {
            acc.push(&k, v);
        }
        self.accessors.push(acc);
        self.accessors.len() - 1
    }

    fn floats<const N: usize>(
        &mut self,
        data: &[[f32; N]],
        kind: &str,
        extra: Vec<(String, Json)>,
    ) -> usize {
        let bytes: Vec<u8> = data
            .iter()
            .flatten()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let view = self.view(&bytes, ARRAY_BUFFER);
        self.accessor(view, FLOAT, data.len(), kind, extra)
    }
}

/// Component-wise minimum and maximum of a non-empty position list.
fn bounds(positions: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for p in positions {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    (lo, hi)
}

/// Writes one primitive's buffers and returns its JSON.
fn write_primitive(w: &mut BufferWriter, p: &Primitive) -> Json {
    let (lo, hi) = bounds(&p.positions);
    let pos = w.floats(
        &p.positions,
        "VEC3",
        vec![
            ("min".into(), Json::nums(&lo)),
            ("max".into(), Json::nums(&hi)),
        ],
    );
    let mut attributes = Json::obj([("POSITION", Json::uint(pos))]);
    if let Some(normals) = &p.normals {
        attributes.push("NORMAL", Json::uint(w.floats(normals, "VEC3", Vec::new())));
    }
    if let Some(uvs) = &p.uvs {
        attributes.push("TEXCOORD_0", Json::uint(w.floats(uvs, "VEC2", Vec::new())));
    }
    if let Some(colors) = &p.colors {
        let bytes: Vec<u8> = colors.iter().flatten().copied().collect();
        let view = w.view(&bytes, ARRAY_BUFFER);
        let acc = w.accessor(
            view,
            UNSIGNED_BYTE,
            colors.len(),
            "VEC4",
            vec![("normalized".into(), Json::Bool(true))],
        );
        attributes.push("COLOR_0", Json::uint(acc));
    }
    let wide = p.indices.iter().any(|&i| i > u32::from(u16::MAX));
    let (bytes, component): (Vec<u8>, i64) = if wide {
        (
            p.indices.iter().flat_map(|i| i.to_le_bytes()).collect(),
            UNSIGNED_INT,
        )
    } else {
        (
            p.indices
                .iter()
                .flat_map(|&i| u16::try_from(i).unwrap_or(u16::MAX).to_le_bytes())
                .collect(),
            UNSIGNED_SHORT,
        )
    };
    let view = w.view(&bytes, ELEMENT_ARRAY_BUFFER);
    let indices = w.accessor(view, component, p.indices.len(), "SCALAR", Vec::new());
    let mut prim = Json::obj([("attributes", attributes), ("indices", Json::uint(indices))]);
    prim.push("mode", Json::Int(p.mode.unwrap_or(Mode::Triangles).code()));
    if let Some(m) = p.material {
        prim.push("material", Json::uint(m));
    }
    prim
}

/// Builds the glTF JSON and the binary buffer. `bin_uri` is the relative URI
/// the JSON uses for the buffer (the `.bin` file name).
#[must_use]
pub fn build(scene: &Scene, bin_uri: &str) -> (String, Vec<u8>) {
    let mut w = BufferWriter {
        bin: Vec::new(),
        views: Vec::new(),
        accessors: Vec::new(),
    };
    let mut meshes = Vec::new();
    let mut children = Vec::new();
    let mut nodes = vec![Json::Null]; // the root, filled in below
    for mesh in &scene.meshes {
        let prims: Vec<Json> = mesh
            .primitives
            .iter()
            .map(|p| write_primitive(&mut w, p))
            .collect();
        meshes.push(Json::obj([
            ("name", Json::str(&mesh.name)),
            ("primitives", Json::Arr(prims)),
        ]));
        children.push(Json::uint(nodes.len()));
        nodes.push(Json::obj([
            ("name", Json::str(&mesh.name)),
            ("mesh", Json::uint(meshes.len() - 1)),
        ]));
    }
    while !w.bin.len().is_multiple_of(4) {
        w.bin.push(0);
    }
    let mut root = Json::obj([("name", Json::str(&scene.root_name))]);
    if let Some(q) = scene.root_rotation {
        root.push("rotation", Json::nums(&[q.x, q.y, q.z, q.w]));
    }
    root.push("children", Json::Arr(children));
    nodes[0] = root;

    let materials: Vec<Json> = scene.materials.iter().map(material_json).collect();

    let mut doc = Json::obj([
        (
            "asset",
            Json::obj([
                ("version", Json::str("2.0")),
                ("generator", Json::str("lf-viewer (LibertyFlux)")),
            ]),
        ),
        ("scene", Json::Int(0)),
        (
            "scenes",
            Json::Arr(vec![Json::obj([("nodes", Json::Arr(vec![Json::Int(0)]))])]),
        ),
        ("nodes", Json::Arr(nodes)),
    ]);
    if !meshes.is_empty() {
        doc.push("meshes", Json::Arr(meshes));
        doc.push("accessors", Json::Arr(w.accessors));
        doc.push("bufferViews", Json::Arr(w.views));
        doc.push(
            "buffers",
            Json::Arr(vec![Json::obj([
                ("byteLength", Json::uint(w.bin.len())),
                ("uri", Json::str(bin_uri)),
            ])]),
        );
    }
    if !materials.is_empty() {
        doc.push("materials", Json::Arr(materials));
    }
    push_images(&mut doc, &scene.images);
    (doc.to_json_string(), w.bin)
}

/// One material's JSON.
fn material_json(m: &Material) -> Json {
    let mut pbr = Json::obj([
        ("metallicFactor", Json::Num(0.0)),
        ("roughnessFactor", Json::Num(1.0)),
    ]);
    if let Some(image) = m.image {
        pbr.push(
            "baseColorTexture",
            Json::obj([("index", Json::uint(image))]),
        );
    }
    let mut mat = Json::obj([
        ("name", Json::str(&m.name)),
        ("pbrMetallicRoughness", pbr),
        ("doubleSided", Json::Bool(true)),
    ]);
    if m.alpha_mask && m.image.is_some() {
        mat.push("alphaMode", Json::str("MASK"));
        mat.push("alphaCutoff", Json::Num(ALPHA_CUTOFF));
    }
    mat
}

/// Adds the textures, images and the one shared sampler, if there are
/// images. Textures are index-aligned with images.
fn push_images(doc: &mut Json, images: &[Image]) {
    if images.is_empty() {
        return;
    }
    let textures: Vec<Json> = (0..images.len())
        .map(|i| Json::obj([("sampler", Json::Int(0)), ("source", Json::uint(i))]))
        .collect();
    let images: Vec<Json> = images
        .iter()
        .map(|im| Json::obj([("name", Json::str(&im.name)), ("uri", Json::str(&im.uri))]))
        .collect();
    doc.push("textures", Json::Arr(textures));
    doc.push("images", Json::Arr(images));
    doc.push(
        "samplers",
        Json::Arr(vec![Json::obj([
            ("magFilter", Json::Int(LINEAR)),
            ("minFilter", Json::Int(LINEAR_MIPMAP_LINEAR)),
            ("wrapS", Json::Int(REPEAT)),
            ("wrapT", Json::Int(REPEAT)),
        ])]),
    );
}

// Tests compare exactly representable floats and index small fixtures.
#[cfg(test)]
#[allow(clippy::float_cmp, clippy::cast_possible_truncation)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn triangle() -> Primitive {
        Primitive {
            positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 2.0, -1.0]],
            normals: Some(vec![[0.0, 0.0, 1.0]; 3]),
            uvs: Some(vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]),
            colors: Some(vec![[255, 0, 0, 255]; 3]),
            indices: vec![0, 1, 2],
            mode: Some(Mode::Triangles),
            material: Some(0),
        }
    }

    fn f32_at(bin: &[u8], at: usize) -> f32 {
        f32::from_le_bytes(bin[at..at + 4].try_into().unwrap())
    }

    #[test]
    fn document_structure_and_buffer_contents() {
        let scene = Scene {
            root_name: "root".into(),
            root_rotation: Some(Quat::from_rotation_x(-core::f32::consts::FRAC_PI_2)),
            meshes: vec![Mesh {
                name: "m".into(),
                primitives: vec![triangle()],
            }],
            materials: vec![Material {
                name: "mat".into(),
                image: Some(0),
                alpha_mask: true,
            }],
            images: vec![Image {
                name: "tex".into(),
                uri: "textures/tex.png".into(),
            }],
        };
        let (json, bin) = build(&scene, "out.bin");
        let doc: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(doc["asset"]["version"], "2.0");
        assert_eq!(
            doc["buffers"][0]["byteLength"].as_u64().unwrap() as usize,
            bin.len()
        );
        assert_eq!(doc["buffers"][0]["uri"], "out.bin");
        assert_eq!(bin.len() % 4, 0);
        assert_eq!(doc["nodes"][0]["children"][0], 1);
        assert_eq!(doc["nodes"][1]["mesh"], 0);
        let rot = &doc["nodes"][0]["rotation"];
        assert!((rot[0].as_f64().unwrap() + std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
        let prim = &doc["meshes"][0]["primitives"][0];
        assert_eq!(prim["mode"], 4);
        assert_eq!(prim["material"], 0);
        let pos = &doc["accessors"][prim["attributes"]["POSITION"].as_u64().unwrap() as usize];
        assert_eq!(pos["count"], 3);
        let nums = |v: &Value| -> Vec<f64> {
            v.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_f64().unwrap())
                .collect()
        };
        assert_eq!(nums(&pos["min"]), vec![0.0, 0.0, -1.0]);
        assert_eq!(nums(&pos["max"]), vec![1.0, 2.0, 0.0]);
        // Every view starts 4-byte aligned and lies inside the buffer.
        for view in doc["bufferViews"].as_array().unwrap() {
            let off = view["byteOffset"].as_u64().unwrap() as usize;
            let len = view["byteLength"].as_u64().unwrap() as usize;
            assert_eq!(off % 4, 0);
            assert!(off + len <= bin.len());
        }
        // The position view holds the floats in order.
        let pv = &doc["bufferViews"][pos["bufferView"].as_u64().unwrap() as usize];
        let off = pv["byteOffset"].as_u64().unwrap() as usize;
        assert_eq!(f32_at(&bin, off + 4 * 3), 1.0);
        assert_eq!(f32_at(&bin, off + 4 * 7), 2.0);
        let idx = &doc["accessors"][prim["indices"].as_u64().unwrap() as usize];
        assert_eq!(idx["componentType"], UNSIGNED_SHORT);
        let col = &doc["accessors"][prim["attributes"]["COLOR_0"].as_u64().unwrap() as usize];
        assert_eq!(col["normalized"], true);
        assert_eq!(doc["materials"][0]["alphaMode"], "MASK");
        assert_eq!(
            doc["materials"][0]["pbrMetallicRoughness"]["baseColorTexture"]["index"],
            0
        );
        assert_eq!(doc["textures"][0]["source"], 0);
        assert_eq!(doc["images"][0]["uri"], "textures/tex.png");
    }

    #[test]
    fn wide_indices_and_empty_scenes() {
        let mut p = triangle();
        p.positions.resize(70_000, [0.0; 3]);
        p.normals = None;
        p.uvs = None;
        p.colors = None;
        p.indices = vec![0, 1, 69_999];
        let scene = Scene {
            meshes: vec![Mesh {
                name: "big".into(),
                primitives: vec![p],
            }],
            ..Scene::default()
        };
        let (json, _) = build(&scene, "b.bin");
        let doc: Value = serde_json::from_str(&json).unwrap();
        let prim = &doc["meshes"][0]["primitives"][0];
        assert_eq!(
            doc["accessors"][prim["indices"].as_u64().unwrap() as usize]["componentType"],
            UNSIGNED_INT
        );
        assert!(prim["attributes"].get("NORMAL").is_none());
        let (json, bin) = build(&Scene::default(), "e.bin");
        let doc: Value = serde_json::from_str(&json).unwrap();
        assert!(doc.get("buffers").is_none());
        assert!(bin.is_empty());
    }
}
