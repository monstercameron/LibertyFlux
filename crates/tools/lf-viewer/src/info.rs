//! The `info` command: what a model or texture dictionary file holds, read
//! without converting or writing anything.
//!
//! For a model: per drawable, its bounds, skeleton size, levels of detail
//! with their models and geometries (primitive type, vertex, index and
//! triangle counts, stride, shader index, skinning palette size, vertex
//! declaration), its materials (shader name, parameter count, the texture
//! names it refers to) and its embedded texture dictionary. For a texture
//! dictionary: per texture, its format, size, mip levels, kind, data size
//! and how many levels lie inside the file. Counts come straight from the
//! format readers' headers; vertex and texel data are not decoded, so the
//! command is quick on large files. Triangles are counted the way the
//! `model` command counts them: a list draws one per three indices, a
//! strip or fan one per index after the second, points and lines none.
//!
//! The text and the JSON describe the user's own game files: they are for
//! the user's screen and scripts, not for the repository.

use std::fmt::Write as _;
use std::path::Path;

use lf_model::drawable::PrimitiveType;
use lf_model::{Drawable, DrawableDictionary, ElementType, ElementUsage, Fragment, Resource};
use lf_model::{ShaderParam, VertexElement};
use lf_texture::{D3DFormat, Dictionary, RESOURCE_TYPE_TEXTURE, RSC_MAGIC, TextureKind};

use crate::cli::{ModelKind, guess_kind};
use crate::convert;
use crate::json::Json;

/// What kind of file to read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    /// A model file of one of the three kinds.
    Model(ModelKind),
    /// A texture dictionary (`.wtd`).
    Textures,
}

impl FileKind {
    /// `wdr`, `wdd`, `wft` or `wtd`, in any case; `None` for anything else.
    #[must_use]
    pub fn parse(s: &str) -> Option<FileKind> {
        Some(match s.to_ascii_lowercase().as_str() {
            "wdr" => FileKind::Model(ModelKind::Drawable),
            "wdd" => FileKind::Model(ModelKind::Dictionary),
            "wft" => FileKind::Model(ModelKind::Fragment),
            "wtd" => FileKind::Textures,
            _ => return None,
        })
    }

    /// The kind a file's extension names, if it names one.
    #[must_use]
    pub fn from_extension(path: &Path) -> Option<FileKind> {
        path.extension()
            .and_then(|e| FileKind::parse(&e.to_string_lossy()))
    }
}

/// One texture of a dictionary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextureInfo {
    /// Stored name (for example `pack:/body.dds`).
    pub name: String,
    /// Pixel format.
    pub format: D3DFormat,
    /// Width of mip level 0 in texels.
    pub width: u16,
    /// Height of mip level 0 in texels.
    pub height: u16,
    /// Mip levels the record declares.
    pub levels: u8,
    /// Flat, cube or volume.
    pub kind: TextureKind,
    /// Mip levels whose bytes lie inside the file's graphics segment.
    pub readable_levels: u8,
    /// Bytes of all mip levels together, when the format is known.
    pub data_bytes: Option<u64>,
}

/// A texture dictionary's textures, in file order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DictionaryInfo {
    /// The textures.
    pub textures: Vec<TextureInfo>,
}

/// One geometry (one draw call).
#[derive(Debug, Clone, PartialEq)]
pub struct GeometryInfo {
    /// Index of its model within the level of detail.
    pub model: usize,
    /// Index within its model.
    pub index: usize,
    /// How the indices form primitives.
    pub primitive: PrimitiveType,
    /// Vertices.
    pub vertices: u16,
    /// Indices.
    pub indices: u32,
    /// Triangles drawn (see the module documentation).
    pub triangles: usize,
    /// Vertex stride in bytes.
    pub stride: u16,
    /// Index of the shader (material) it draws with, from the model's
    /// shader map.
    pub shader: Option<u16>,
    /// Bones in its skinning palette (0 when unskinned).
    pub palette: usize,
    /// The vertex declaration's elements, in vertex order.
    pub elements: Vec<VertexElement>,
}

/// One level of detail.
#[derive(Debug, Clone, PartialEq)]
pub struct LodInfo {
    /// Position among the present levels (0 is the most detailed).
    pub index: usize,
    /// Models in it.
    pub models: usize,
    /// Every geometry of every model.
    pub geometries: Vec<GeometryInfo>,
}

/// One shader of a drawable's shader group, which the converter turns
/// into one material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialInfo {
    /// Shader name.
    pub name: String,
    /// Parameters of any type.
    pub parameters: usize,
    /// Names of the textures its texture parameters refer to.
    pub textures: Vec<String>,
}

/// One drawable.
#[derive(Debug, Clone, PartialEq)]
pub struct DrawableInfo {
    /// The name the converter gives its meshes.
    pub label: String,
    /// Minimum corner of the bounding box.
    pub bounds_min: [f32; 3],
    /// Maximum corner of the bounding box.
    pub bounds_max: [f32; 3],
    /// Bounding sphere: centre and radius.
    pub sphere: [f32; 4],
    /// Bones of its skeleton, if it has one.
    pub bones: Option<usize>,
    /// Its levels of detail.
    pub lods: Vec<LodInfo>,
    /// Its shaders, in shader-group order.
    pub materials: Vec<MaterialInfo>,
    /// Its embedded texture dictionary: `None` when it has none, an error
    /// message when the dictionary cannot be read.
    pub embedded: Option<Result<DictionaryInfo, String>>,
}

/// Totals over every drawable of a model file.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Totals {
    /// Drawables.
    pub drawables: usize,
    /// Levels of detail.
    pub lods: usize,
    /// Models.
    pub models: usize,
    /// Geometries.
    pub geometries: usize,
    /// Vertices.
    pub vertices: usize,
    /// Indices.
    pub indices: usize,
    /// Triangles.
    pub triangles: usize,
    /// Materials (shaders).
    pub materials: usize,
    /// Textures in embedded dictionaries.
    pub embedded_textures: usize,
}

/// A model file.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelInfo {
    /// How the file was read.
    pub kind: ModelKind,
    /// Resource type from the file header.
    pub resource_type: u32,
    /// Size of the system segment (structures) in bytes.
    pub system_bytes: usize,
    /// Size of the graphics segment (vertex and index data) in bytes.
    pub graphics_bytes: usize,
    /// Child records of a fragment; `None` for other kinds.
    pub fragment_children: Option<usize>,
    /// The drawables: one, or one per dictionary entry.
    pub drawables: Vec<DrawableInfo>,
}

impl ModelInfo {
    /// Totals over every drawable.
    #[must_use]
    pub fn totals(&self) -> Totals {
        let mut t = Totals {
            drawables: self.drawables.len(),
            ..Totals::default()
        };
        for d in &self.drawables {
            t.lods += d.lods.len();
            t.materials += d.materials.len();
            if let Some(Ok(dict)) = &d.embedded {
                t.embedded_textures += dict.textures.len();
            }
            for lod in &d.lods {
                t.models += lod.models;
                t.geometries += lod.geometries.len();
                for g in &lod.geometries {
                    t.vertices += usize::from(g.vertices);
                    t.indices += g.indices as usize;
                    t.triangles += g.triangles;
                }
            }
        }
        t
    }
}

/// What a file holds.
#[derive(Debug, Clone, PartialEq)]
pub enum FileInfo {
    /// A model file.
    Model(ModelInfo),
    /// A texture dictionary.
    Textures(DictionaryInfo),
}

/// Reads a file's contents. `kind` overrides the guess, which is made from
/// the extension, then from the resource type in the header (a drawable
/// file whose drawable does not parse is read as a drawable dictionary, as
/// the `model` command does).
///
/// # Errors
///
/// A sentence saying why the file could not be read as the kind chosen.
pub fn inspect(path: &Path, bytes: &[u8], kind: Option<FileKind>) -> Result<FileInfo, String> {
    let label = stem_of(path);
    match kind.or_else(|| FileKind::from_extension(path)) {
        Some(FileKind::Textures) => textures(bytes),
        Some(FileKind::Model(k)) => {
            let res = Resource::open(bytes).map_err(|e| e.to_string())?;
            describe_model(&res, k, &label).map(FileInfo::Model)
        }
        None if is_texture_resource(bytes) => textures(bytes),
        None => {
            let res = Resource::open(bytes).map_err(|e| e.to_string())?;
            let k = guess_kind(path, &res);
            describe_model(&res, k, &label).map(FileInfo::Model)
        }
    }
}

fn textures(bytes: &[u8]) -> Result<FileInfo, String> {
    let dict = Dictionary::parse(bytes).map_err(|e| e.to_string())?;
    Ok(FileInfo::Textures(describe_dictionary(&dict)))
}

/// The converter's name stem for a file.
fn stem_of(path: &Path) -> String {
    convert::sanitize(
        &path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default(),
    )
}

/// Whether the header names a texture dictionary resource.
fn is_texture_resource(bytes: &[u8]) -> bool {
    let word = |at: usize| {
        bytes
            .get(at..at + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    word(0) == Some(RSC_MAGIC) && word(4) == Some(RESOURCE_TYPE_TEXTURE)
}

/// Describes every texture of a dictionary (headers only, nothing decoded).
#[must_use]
pub fn describe_dictionary(dict: &Dictionary) -> DictionaryInfo {
    let textures = dict
        .entries()
        .iter()
        .map(|e| {
            let r = &e.record;
            let readable =
                (0..r.levels).fold(0u8, |n, l| n + u8::from(dict.level_data(e, l).is_ok()));
            let data_bytes = (0..r.levels).try_fold(0u64, |sum, l| {
                lf_texture::level_byte_size(r.format, r.width, r.height, l)
                    .ok()
                    .and_then(|n| sum.checked_add(n))
            });
            TextureInfo {
                name: e.name.clone(),
                format: r.format,
                width: r.width,
                height: r.height,
                levels: r.levels,
                kind: r.kind,
                readable_levels: readable,
                data_bytes,
            }
        })
        .collect();
    DictionaryInfo { textures }
}

/// Describes a model resource read as `kind`; `label` names its drawables
/// as the converter would.
///
/// # Errors
///
/// The model reader's message when the structures do not parse.
pub fn describe_model(res: &Resource, kind: ModelKind, label: &str) -> Result<ModelInfo, String> {
    let (drawables, fragment_children) = match kind {
        ModelKind::Drawable => {
            let d = Drawable::parse(res).map_err(|e| e.to_string())?;
            (vec![describe_drawable(label, &d, res)], None)
        }
        ModelKind::Fragment => {
            let f = Fragment::parse(res).map_err(|e| e.to_string())?;
            (
                vec![describe_drawable(label, &f.drawable, res)],
                Some(f.children.len()),
            )
        }
        ModelKind::Dictionary => {
            let dict = DrawableDictionary::parse(res).map_err(|e| e.to_string())?;
            let drawables = dict
                .entries
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    let hash = dict.hashes.get(i).copied().unwrap_or(0);
                    describe_drawable(&format!("{label}_{hash:08x}"), d, res)
                })
                .collect();
            (drawables, None)
        }
    };
    Ok(ModelInfo {
        kind,
        resource_type: res.kind,
        system_bytes: res.sys.len(),
        graphics_bytes: res.gfx.len(),
        fragment_children,
        drawables,
    })
}

fn describe_drawable(label: &str, d: &Drawable, res: &Resource) -> DrawableInfo {
    let lods = d
        .lods
        .iter()
        .enumerate()
        .map(|(index, lod)| LodInfo {
            index,
            models: lod.models.len(),
            geometries: lod
                .models
                .iter()
                .enumerate()
                .flat_map(|(m, model)| {
                    model
                        .geometries
                        .iter()
                        .enumerate()
                        .map(move |(g, geo)| GeometryInfo {
                            model: m,
                            index: g,
                            primitive: geo.primitive,
                            vertices: geo.vertex_count,
                            indices: geo.index_count,
                            triangles: triangles(geo.primitive, geo.index_count as usize),
                            stride: geo.stride,
                            shader: model.shader_map.get(g).copied(),
                            palette: geo.matrix_palette.len(),
                            elements: geo.declaration.elements.clone(),
                        })
                })
                .collect(),
        })
        .collect();
    let (materials, embedded) = match &d.shaders {
        None => (Vec::new(), None),
        Some(group) => {
            let materials = group
                .shaders
                .iter()
                .map(|s| MaterialInfo {
                    name: s.name.clone(),
                    parameters: s.params.len(),
                    textures: s
                        .params
                        .iter()
                        .filter_map(|(_, p)| match p {
                            ShaderParam::Texture { name } if !name.is_empty() => Some(name.clone()),
                            _ => None,
                        })
                        .collect(),
                })
                .collect();
            let embedded = group.texture_dict.map(|offset| {
                convert::embedded_dictionary(res, offset)
                    .map(|dict| describe_dictionary(&dict))
                    .map_err(|e| e.to_string())
            });
            (materials, embedded)
        }
    };
    DrawableInfo {
        label: label.to_string(),
        bounds_min: [d.bounds_min[0], d.bounds_min[1], d.bounds_min[2]],
        bounds_max: [d.bounds_max[0], d.bounds_max[1], d.bounds_max[2]],
        sphere: d.center,
        bones: d.skeleton.as_ref().map(|s| s.bones.len()),
        lods,
        materials,
        embedded,
    }
}

/// Triangles a primitive draws from `indices` indices.
fn triangles(primitive: PrimitiveType, indices: usize) -> usize {
    match primitive {
        PrimitiveType::TriangleList => indices / 3,
        PrimitiveType::TriangleStrip | PrimitiveType::TriangleFan => indices.saturating_sub(2),
        PrimitiveType::PointList | PrimitiveType::LineList | PrimitiveType::LineStrip => 0,
    }
}

fn model_kind_name(kind: ModelKind) -> &'static str {
    match kind {
        ModelKind::Drawable => "drawable",
        ModelKind::Dictionary => "drawable dictionary",
        ModelKind::Fragment => "fragment",
    }
}

fn primitive_name(p: PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::PointList => "point list",
        PrimitiveType::LineList => "line list",
        PrimitiveType::LineStrip => "line strip",
        PrimitiveType::TriangleList => "triangle list",
        PrimitiveType::TriangleStrip => "triangle strip",
        PrimitiveType::TriangleFan => "triangle fan",
    }
}

fn usage_name(u: ElementUsage) -> String {
    match u {
        ElementUsage::Position => "position".to_string(),
        ElementUsage::BlendWeight => "blendweight".to_string(),
        ElementUsage::BlendIndices => "blendindices".to_string(),
        ElementUsage::Normal => "normal".to_string(),
        ElementUsage::Color(n) => format!("color{n}"),
        ElementUsage::TexCoord(n) => format!("texcoord{n}"),
        ElementUsage::Tangent => "tangent".to_string(),
        ElementUsage::Binormal => "binormal".to_string(),
    }
}

fn element_type_name(t: ElementType) -> String {
    match t {
        ElementType::F16x1 => "f16x1".to_string(),
        ElementType::F16x2 => "f16x2".to_string(),
        ElementType::F16x3 => "f16x3".to_string(),
        ElementType::F16x4 => "f16x4".to_string(),
        ElementType::F32x1 => "f32x1".to_string(),
        ElementType::F32x2 => "f32x2".to_string(),
        ElementType::F32x3 => "f32x3".to_string(),
        ElementType::F32x4 => "f32x4".to_string(),
        ElementType::UByte4 => "ubyte4".to_string(),
        ElementType::Color => "color".to_string(),
        ElementType::Dec3N => "dec3n".to_string(),
        ElementType::Reserved(n) => format!("reserved{n}"),
    }
}

fn texture_kind_name(k: TextureKind) -> String {
    match k {
        TextureKind::Flat => "flat".to_string(),
        TextureKind::Cube => "cube".to_string(),
        TextureKind::Volume => "volume".to_string(),
        TextureKind::Unknown(n) => format!("unknown kind {n}"),
    }
}

/// `1 geometry`, `3 geometries`.
fn count(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

fn vec_text(v: &[f32]) -> String {
    let parts: Vec<String> = v.iter().map(f32::to_string).collect();
    format!("({})", parts.join(", "))
}

impl FileInfo {
    /// A human-readable description; `name` is the file as the user gave it.
    #[must_use]
    pub fn to_text(&self, name: &str) -> String {
        let mut out = String::new();
        match self {
            FileInfo::Textures(dict) => {
                let _ = writeln!(
                    out,
                    "{name}: texture dictionary, {}",
                    count(dict.textures.len(), "texture", "textures")
                );
                dictionary_text(&mut out, dict, "  ");
            }
            FileInfo::Model(m) => model_text(&mut out, name, m),
        }
        out
    }

    /// The same description as JSON; `name` is the file as the user gave it.
    #[must_use]
    pub fn to_json(&self, name: &str) -> Json {
        match self {
            FileInfo::Textures(dict) => {
                let mut j = Json::obj([
                    ("file", Json::str(name)),
                    ("type", Json::str("texture dictionary")),
                ]);
                j.push("textures", textures_json(dict));
                j
            }
            FileInfo::Model(m) => model_json(name, m),
        }
    }
}

fn dictionary_text(out: &mut String, dict: &DictionaryInfo, indent: &str) {
    for t in &dict.textures {
        let bytes = match t.data_bytes {
            None => "unknown size".to_string(),
            Some(1) => "1 byte".to_string(),
            Some(b) => format!("{b} bytes"),
        };
        let _ = writeln!(
            out,
            "{indent}{}: {} {}x{}, {}, {}, {bytes}, {} of {} readable",
            t.name,
            t.format,
            t.width,
            t.height,
            count(usize::from(t.levels), "mip level", "mip levels"),
            texture_kind_name(t.kind),
            t.readable_levels,
            t.levels
        );
    }
}

fn model_text(out: &mut String, name: &str, m: &ModelInfo) {
    let _ = writeln!(
        out,
        "{name}: {}; resource type {:#x}; system segment {}, graphics segment {}",
        model_kind_name(m.kind),
        m.resource_type,
        count(m.system_bytes, "byte", "bytes"),
        count(m.graphics_bytes, "byte", "bytes")
    );
    if let Some(n) = m.fragment_children {
        let _ = writeln!(out, "fragment children: {n}");
    }
    for d in &m.drawables {
        drawable_text(out, d);
    }
    let t = m.totals();
    let _ = writeln!(
        out,
        "totals: {}, {}, {}, {}, {}, {}, {}, {}, {}",
        count(t.drawables, "drawable", "drawables"),
        count(t.lods, "LOD", "LODs"),
        count(t.models, "model", "models"),
        count(t.geometries, "geometry", "geometries"),
        count(t.vertices, "vertex", "vertices"),
        count(t.indices, "index", "indices"),
        count(t.triangles, "triangle", "triangles"),
        count(t.materials, "material", "materials"),
        count(t.embedded_textures, "embedded texture", "embedded textures")
    );
}

fn drawable_text(out: &mut String, d: &DrawableInfo) {
    let _ = writeln!(out, "drawable {}", d.label);
    let _ = writeln!(
        out,
        "  bounds: min {}, max {}; sphere centre {}, radius {}",
        vec_text(&d.bounds_min),
        vec_text(&d.bounds_max),
        vec_text(&d.sphere[..3]),
        d.sphere[3]
    );
    let _ = writeln!(
        out,
        "  skeleton: {}",
        d.bones
            .map_or_else(|| "none".to_string(), |n| count(n, "bone", "bones"))
    );
    for lod in &d.lods {
        let vertices: usize = lod.geometries.iter().map(|g| usize::from(g.vertices)).sum();
        let indices: usize = lod.geometries.iter().map(|g| g.indices as usize).sum();
        let tris: usize = lod.geometries.iter().map(|g| g.triangles).sum();
        let _ = writeln!(
            out,
            "  LOD {}: {}, {}, {}, {}, {}",
            lod.index,
            count(lod.models, "model", "models"),
            count(lod.geometries.len(), "geometry", "geometries"),
            count(vertices, "vertex", "vertices"),
            count(indices, "index", "indices"),
            count(tris, "triangle", "triangles")
        );
        for g in &lod.geometries {
            geometry_text(out, g);
        }
    }
    let _ = writeln!(out, "  materials: {}", d.materials.len());
    for (i, mat) in d.materials.iter().enumerate() {
        let textures = if mat.textures.is_empty() {
            "no textures".to_string()
        } else {
            format!("textures: {}", mat.textures.join(", "))
        };
        let _ = writeln!(
            out,
            "    {i} {}: {}; {textures}",
            mat.name,
            count(mat.parameters, "parameter", "parameters")
        );
    }
    match &d.embedded {
        None => {
            let _ = writeln!(out, "  embedded textures: none");
        }
        Some(Err(e)) => {
            let _ = writeln!(out, "  embedded textures: unreadable ({e})");
        }
        Some(Ok(dict)) => {
            let _ = writeln!(out, "  embedded textures: {}", dict.textures.len());
            dictionary_text(out, dict, "    ");
        }
    }
}

fn geometry_text(out: &mut String, g: &GeometryInfo) {
    let shader = g
        .shader
        .map_or_else(|| "no shader".to_string(), |s| format!("shader {s}"));
    let palette = if g.palette > 0 {
        format!(", skinned ({})", count(g.palette, "bone", "bones"))
    } else {
        String::new()
    };
    let _ = writeln!(
        out,
        "    model {} geometry {}: {}, {}, {}, {}, stride {}, {shader}{palette}",
        g.model,
        g.index,
        primitive_name(g.primitive),
        count(usize::from(g.vertices), "vertex", "vertices"),
        count(g.indices as usize, "index", "indices"),
        count(g.triangles, "triangle", "triangles"),
        g.stride
    );
    let elements: Vec<String> = g
        .elements
        .iter()
        .map(|e| format!("{} {}", usage_name(e.usage), element_type_name(e.kind)))
        .collect();
    let _ = writeln!(out, "      vertex: {}", elements.join(", "));
}

fn textures_json(dict: &DictionaryInfo) -> Json {
    Json::Arr(
        dict.textures
            .iter()
            .map(|t| {
                Json::obj([
                    ("name", Json::str(&t.name)),
                    ("format", Json::str(t.format.name())),
                    ("format_code", Json::Int(i64::from(t.format.code()))),
                    ("width", Json::Int(i64::from(t.width))),
                    ("height", Json::Int(i64::from(t.height))),
                    ("levels", Json::Int(i64::from(t.levels))),
                    ("kind", Json::str(texture_kind_name(t.kind))),
                    ("readable_levels", Json::Int(i64::from(t.readable_levels))),
                    (
                        "data_bytes",
                        t.data_bytes.map_or(Json::Null, |b| {
                            Json::Int(i64::try_from(b).unwrap_or(i64::MAX))
                        }),
                    ),
                ])
            })
            .collect(),
    )
}

fn model_json(name: &str, m: &ModelInfo) -> Json {
    let t = m.totals();
    Json::obj([
        ("file", Json::str(name)),
        ("type", Json::str(model_kind_name(m.kind))),
        ("resource_type", Json::Int(i64::from(m.resource_type))),
        ("system_bytes", Json::uint(m.system_bytes)),
        ("graphics_bytes", Json::uint(m.graphics_bytes)),
        (
            "fragment_children",
            m.fragment_children.map_or(Json::Null, Json::uint),
        ),
        (
            "drawables",
            Json::Arr(m.drawables.iter().map(drawable_json).collect()),
        ),
        (
            "totals",
            Json::obj([
                ("drawables", Json::uint(t.drawables)),
                ("lods", Json::uint(t.lods)),
                ("models", Json::uint(t.models)),
                ("geometries", Json::uint(t.geometries)),
                ("vertices", Json::uint(t.vertices)),
                ("indices", Json::uint(t.indices)),
                ("triangles", Json::uint(t.triangles)),
                ("materials", Json::uint(t.materials)),
                ("embedded_textures", Json::uint(t.embedded_textures)),
            ]),
        ),
    ])
}

fn drawable_json(d: &DrawableInfo) -> Json {
    let lods = d
        .lods
        .iter()
        .map(|lod| {
            Json::obj([
                ("index", Json::uint(lod.index)),
                ("models", Json::uint(lod.models)),
                (
                    "geometries",
                    Json::Arr(lod.geometries.iter().map(geometry_json).collect()),
                ),
            ])
        })
        .collect();
    let materials = d
        .materials
        .iter()
        .map(|mat| {
            Json::obj([
                ("name", Json::str(&mat.name)),
                ("parameters", Json::uint(mat.parameters)),
                (
                    "textures",
                    Json::Arr(mat.textures.iter().map(Json::str).collect()),
                ),
            ])
        })
        .collect();
    let embedded = match &d.embedded {
        None => Json::Null,
        Some(Err(e)) => Json::obj([("error", Json::str(e))]),
        Some(Ok(dict)) => Json::obj([("textures", textures_json(dict))]),
    };
    Json::obj([
        ("label", Json::str(&d.label)),
        (
            "bounds",
            Json::obj([
                ("min", Json::nums(&d.bounds_min)),
                ("max", Json::nums(&d.bounds_max)),
                ("sphere", Json::nums(&d.sphere)),
            ]),
        ),
        ("bones", d.bones.map_or(Json::Null, Json::uint)),
        ("lods", Json::Arr(lods)),
        ("materials", Json::Arr(materials)),
        ("embedded_textures", embedded),
    ])
}

fn geometry_json(g: &GeometryInfo) -> Json {
    let elements = g
        .elements
        .iter()
        .map(|e| {
            Json::obj([
                ("usage", Json::str(usage_name(e.usage))),
                ("type", Json::str(element_type_name(e.kind))),
                ("offset", Json::uint(e.offset)),
            ])
        })
        .collect();
    Json::obj([
        ("model", Json::uint(g.model)),
        ("geometry", Json::uint(g.index)),
        ("primitive", Json::str(primitive_name(g.primitive))),
        ("vertices", Json::Int(i64::from(g.vertices))),
        ("indices", Json::Int(i64::from(g.indices))),
        ("triangles", Json::uint(g.triangles)),
        ("stride", Json::Int(i64::from(g.stride))),
        (
            "shader",
            g.shader.map_or(Json::Null, |s| Json::Int(i64::from(s))),
        ),
        ("palette", Json::uint(g.palette)),
        ("elements", Json::Arr(elements)),
    ])
}
