//! Conversion from the format readers' types to a glTF [`Scene`] and PNG
//! files.
//!
//! What is carried over, and the assumptions behind it (labelled as in the
//! devlog):
//!
//! - Geometry: positions, normals, the first texture-coordinate set and,
//!   on request, the diffuse vertex colour, exactly as `lf-model` decodes
//!   them. Indices and primitive types map one to one (glTF has strips and
//!   fans too). Verified against hand-built inputs only.
//! - Axes: the game's world is taken to be +Z up (Inferred, see `lf-math`),
//!   glTF is +Y up, so the root node rotates -90 degrees about X unless
//!   asked not to. Vertex data is not touched.
//! - Winding: which way the game's front faces wind is Unknown, so
//!   materials are double sided and `--flip-winding` reverses triangles for
//!   viewers that cull anyway.
//! - Texture coordinates: Direct3D 9 and glTF both put v = 0 at the top of
//!   the image, so they are written unchanged (Verified for the two
//!   specifications; whether the game's shaders transform them is Unknown).
//! - Materials: one per shader of the drawable's shader group, named after
//!   the shader. The base-colour texture is the shader's first texture
//!   parameter that resolves to a texture in the available dictionaries
//!   (Inferred: parameter name hashes are not mapped to names yet, and the
//!   first texture is usually the diffuse map in the files read so far).
//! - Vertex colours are Direct3D packed colours, bytes in blue, green, red,
//!   alpha order in memory (from `lf-model`'s notes); they are reordered to
//!   RGBA. They are left out by default because the game appears to use
//!   them for baked lighting rather than tint (Inferred), which would darken
//!   the model in a viewer.
//! - Skeletons, skinning, LOD distances, collision and every shader
//!   parameter other than the first texture are not exported.

use std::collections::HashMap;

use lf_model::drawable::PrimitiveType;
use lf_model::{Drawable, ElementUsage, Geometry, ShaderParam};
use lf_texture::{CODEC_DEFLATE, Dictionary, RESOURCE_TYPE_TEXTURE, RSC_MAGIC};

use crate::gltf::{Image, Material, Mesh, Mode, Primitive, Scene};
use crate::png;

/// Which levels of detail to export.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LodChoice {
    /// One LOD by position among the present ones (0 is the most detailed).
    Index(usize),
    /// Every present LOD, each as its own mesh.
    All,
}

/// Conversion settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
    /// Which LODs to export.
    pub lod: LodChoice,
    /// Reverse every triangle's winding (strips and fans become lists).
    pub flip_winding: bool,
    /// Export diffuse vertex colours as `COLOR_0`.
    pub vertex_colors: bool,
    /// Rotate +Z up to glTF's +Y up at the root node.
    pub z_up_to_y_up: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            lod: LodChoice::Index(0),
            flip_winding: false,
            vertex_colors: false,
            z_up_to_y_up: true,
        }
    }
}

/// A PNG to write, with the file name it should get (no directories).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextureFile {
    /// Safe file name, unique within one conversion, ending `.png`.
    pub file_name: String,
    /// The encoded PNG.
    pub png: Vec<u8>,
}

/// The result of converting one model file.
#[derive(Debug, Clone, Default)]
pub struct Converted {
    /// The glTF scene (image URIs point into `textures/`).
    pub scene: Scene,
    /// PNG files to write under `textures/`.
    pub textures: Vec<TextureFile>,
    /// Problems that did not stop the conversion, one sentence each.
    pub warnings: Vec<String>,
    /// Vertices exported.
    pub vertices: usize,
    /// Triangles exported (triangle primitives only).
    pub triangles: usize,
}

/// The folder, relative to the `.gltf` file, that holds the PNG files.
pub const TEXTURE_DIR: &str = "textures";

/// Turns any text into a safe file-name stem: ASCII letters, digits, `.`,
/// `_` and `-` are kept, everything else becomes `_`; leading dots are
/// replaced so the result is never hidden or a relative path component;
/// empty input becomes `unnamed`.
#[must_use]
pub fn sanitize(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if out.starts_with('.') {
        out.replace_range(0..1, "_");
    }
    if out.is_empty() {
        out.push_str("unnamed");
    }
    out
}

/// Hands out unique file names: a repeated stem gets `_2`, `_3`, ...
#[derive(Debug, Default)]
struct NameSet {
    used: HashMap<String, usize>,
}

impl NameSet {
    fn unique(&mut self, stem: &str) -> String {
        let key = stem.to_ascii_lowercase();
        let n = self.used.entry(key).or_insert(0);
        *n += 1;
        if *n == 1 {
            stem.to_string()
        } else {
            format!("{stem}_{n}")
        }
    }
}

/// Reads a texture dictionary embedded in a model resource.
///
/// A drawable's shader group may point at a texture dictionary inside the
/// model's own system segment. `lf-texture` reads a dictionary only at
/// system offset 0, so this builds a texture resource from the model's
/// segments with the 32-byte dictionary header copied to offset 0. Every
/// pointer in the dictionary is an absolute segment offset, so the copy
/// keeps them valid; the bytes overwritten at offset 0 are the drawable
/// header, which no texture structure can overlap. That the embedded
/// structure is laid out like a standalone dictionary is Inferred from the
/// public format notes and needs a local check against real files.
///
/// # Errors
///
/// Returns the texture reader's error when the bytes are not a dictionary.
pub fn embedded_dictionary(
    model: &lf_model::Resource,
    offset: usize,
) -> Result<Dictionary, lf_texture::Error> {
    let end = offset
        .checked_add(DICTIONARY_HEADER_LEN)
        .filter(|&e| e <= model.sys.len())
        .ok_or(lf_texture::Error::TooShort {
            what: "embedded dictionary header",
        })?;
    let mut system = model.sys.clone();
    system.copy_within(offset..end, 0);
    // `lf-texture` does not export its header type by name, so start from an
    // empty texture resource parsed from bytes and swap the segments in.
    let mut resource = lf_texture::Resource::parse(&empty_texture_resource())?;
    resource.header.flags = model.flags;
    resource.system = system;
    resource.graphics.clone_from(&model.gfx);
    Dictionary::from_resource(resource)
}

/// Size of a texture dictionary header in bytes (see `lf-texture`).
const DICTIONARY_HEADER_LEN: usize = 32;

/// A valid texture resource file with two empty segments: the 12-byte
/// header (flags 0 encode both sizes as 0), then a zlib stream whose two
/// header bytes are the codec field `lf-texture` requires.
fn empty_texture_resource() -> Vec<u8> {
    let mut file = Vec::with_capacity(24);
    file.extend_from_slice(&RSC_MAGIC.to_le_bytes());
    file.extend_from_slice(&RESOURCE_TYPE_TEXTURE.to_le_bytes());
    file.extend_from_slice(&0u32.to_le_bytes());
    file.extend_from_slice(&CODEC_DEFLATE.to_le_bytes());
    // One final, empty stored block, then the Adler-32 of no data (1).
    file.extend_from_slice(&[1, 0, 0, 0xFF, 0xFF]);
    file.extend_from_slice(&png::adler32(&[]).to_be_bytes());
    file
}

/// Rewrites strip or fan indices as a triangle list with consistent
/// winding, dropping degenerate triangles. List input is returned as is.
#[must_use]
pub fn triangulate(mode: Mode, indices: &[u32]) -> Vec<u32> {
    let mut out = Vec::new();
    match mode {
        Mode::TriangleStrip => {
            for (i, w) in indices.windows(3).enumerate() {
                // Every second strip triangle is stored reversed.
                let tri = if i % 2 == 0 {
                    [w[0], w[1], w[2]]
                } else {
                    [w[1], w[0], w[2]]
                };
                if tri[0] != tri[1] && tri[1] != tri[2] && tri[0] != tri[2] {
                    out.extend_from_slice(&tri);
                }
            }
        }
        Mode::TriangleFan => {
            if let Some((&hub, rest)) = indices.split_first() {
                for w in rest.windows(2) {
                    if hub != w[0] && w[0] != w[1] && hub != w[1] {
                        out.extend_from_slice(&[hub, w[0], w[1]]);
                    }
                }
            }
        }
        _ => out.extend_from_slice(indices),
    }
    out
}

/// The glTF mode of a source primitive type.
fn mode_of(primitive: PrimitiveType) -> Mode {
    match primitive {
        PrimitiveType::PointList => Mode::Points,
        PrimitiveType::LineList => Mode::Lines,
        PrimitiveType::LineStrip => Mode::LineStrip,
        PrimitiveType::TriangleList => Mode::Triangles,
        PrimitiveType::TriangleStrip => Mode::TriangleStrip,
        PrimitiveType::TriangleFan => Mode::TriangleFan,
    }
}

/// One converted geometry.
struct GeometryOut {
    primitive: Primitive,
    /// Triangles it draws (0 for points and lines).
    triangles: usize,
    /// Normals that were zero or non-finite and were replaced by +Z.
    degenerate_normals: usize,
}

/// Converts one geometry. Errors are warnings for the caller: the geometry
/// is skipped and the rest of the model still converts.
fn convert_geometry(
    geo: &Geometry,
    res: &lf_model::Resource,
    opts: &Options,
) -> Result<GeometryOut, String> {
    let verts = geo
        .vertices(res)
        .map_err(|e| format!("vertex data unreadable: {e}"))?;
    let raw_indices = geo
        .indices(res)
        .map_err(|e| format!("index data unreadable: {e}"))?;
    if verts.is_empty() {
        return Err("no vertices".into());
    }
    if let Some(bad) = raw_indices.iter().find(|&&i| usize::from(i) >= verts.len()) {
        return Err(format!("index {bad} is past the {} vertices", verts.len()));
    }
    if verts.iter().any(|v| !v.pos.iter().all(|c| c.is_finite())) {
        return Err("non-finite vertex position".into());
    }
    let mut degenerate_normals = 0usize;
    let decl = &geo.declaration;
    let normals = decl.find(ElementUsage::Normal).map(|_| {
        verts
            .iter()
            .map(|v| {
                let n = lf_math::Vec3::from_array(v.normal);
                if let Some(u) = n.try_normalize() {
                    u.to_array()
                } else {
                    degenerate_normals += 1;
                    [0.0, 0.0, 1.0]
                }
            })
            .collect()
    });
    let uvs = decl
        .find(ElementUsage::TexCoord(0))
        .map(|_| verts.iter().map(|v| v.uv).collect());
    let colors = (opts.vertex_colors && decl.find(ElementUsage::Color(0)).is_some()).then(|| {
        verts
            .iter()
            .map(|v| {
                let [b, g, r, a] = v.diffuse.to_le_bytes();
                [r, g, b, a]
            })
            .collect()
    });
    let mut mode = mode_of(geo.primitive);
    let mut indices: Vec<u32> = raw_indices.iter().map(|&i| u32::from(i)).collect();
    if opts.flip_winding
        && matches!(
            mode,
            Mode::Triangles | Mode::TriangleStrip | Mode::TriangleFan
        )
    {
        indices = triangulate(mode, &indices);
        mode = Mode::Triangles;
        for tri in indices.chunks_exact_mut(3) {
            tri.swap(1, 2);
        }
    }
    let triangles = match mode {
        Mode::Triangles => indices.len() / 3,
        Mode::TriangleStrip | Mode::TriangleFan => indices.len().saturating_sub(2),
        _ => 0,
    };
    let primitive = Primitive {
        positions: verts.iter().map(|v| v.pos).collect(),
        normals,
        uvs,
        colors,
        indices,
        mode: Some(mode),
        material: None,
    };
    Ok(GeometryOut {
        primitive,
        triangles,
        degenerate_normals,
    })
}

/// Texture sources for one drawable: its embedded dictionary, then the
/// dictionaries the user passed, searched in that order.
struct Sources<'a> {
    embedded: Option<&'a Dictionary>,
    embedded_tag: String,
    external: &'a [(String, Dictionary)],
}

impl<'a> Sources<'a> {
    /// Finds a texture by name: `(source tag, dictionary, entry)`.
    fn find(&self, name: &str) -> Option<(String, &'a Dictionary, &'a lf_texture::Entry)> {
        if let Some(dict) = self.embedded
            && let Some(entry) = dict.find(name)
        {
            return Some((self.embedded_tag.clone(), dict, entry));
        }
        self.external
            .iter()
            .find_map(|(tag, dict)| dict.find(name).map(|e| (tag.clone(), dict, e)))
    }
}

/// Decodes mip level 0 of a texture and encodes it as PNG. Returns the PNG
/// and whether any texel is less than fully opaque.
///
/// # Errors
///
/// Returns a sentence describing why the texture could not be decoded.
pub fn texture_to_png(
    dict: &Dictionary,
    entry: &lf_texture::Entry,
) -> Result<(Vec<u8>, bool), String> {
    let image = dict
        .decode_rgba8(entry, 0)
        .map_err(|e| format!("texture {} could not be decoded: {e}", entry.name))?;
    let has_alpha = image.pixels.chunks_exact(4).any(|p| p[3] < u8::MAX);
    let png = png::encode_rgba8(image.width, image.height, &image.pixels)
        .map_err(|e| format!("texture {} could not be encoded: {e}", entry.name))?;
    Ok((png, has_alpha))
}

/// Converts every texture of a dictionary to PNG (the `texture` command).
/// Returns the files and one warning per texture that failed.
#[must_use]
pub fn dictionary_to_pngs(dict: &Dictionary) -> (Vec<TextureFile>, Vec<String>) {
    let mut names = NameSet::default();
    let mut files = Vec::new();
    let mut warnings = Vec::new();
    for entry in dict.entries() {
        match texture_to_png(dict, entry) {
            Ok((png, _)) => {
                let stem = names.unique(&sanitize(lf_texture::title_of(&entry.name)));
                files.push(TextureFile {
                    file_name: format!("{stem}.png"),
                    png,
                });
            }
            Err(w) => warnings.push(w),
        }
    }
    (files, warnings)
}

/// State shared while converting the drawables of one file.
struct Builder<'a> {
    opts: Options,
    external: &'a [(String, Dictionary)],
    out: Converted,
    names: NameSet,
    /// (source tag, texture name) -> (image index, has alpha).
    images: HashMap<(String, String), (usize, bool)>,
}

impl Builder<'_> {
    /// Resolves a material's texture, writing its PNG the first time.
    fn image_for(&mut self, sources: &Sources<'_>, wanted: &str) -> Option<(usize, bool)> {
        let (tag, dict, entry) = sources.find(wanted)?;
        let key = (tag, entry.name.clone());
        if let Some(&found) = self.images.get(&key) {
            return Some(found);
        }
        match texture_to_png(dict, entry) {
            Ok((png, has_alpha)) => {
                let stem = self
                    .names
                    .unique(&sanitize(lf_texture::title_of(&entry.name)));
                let file_name = format!("{stem}.png");
                self.out.scene.images.push(Image {
                    name: entry.name.clone(),
                    uri: format!("{TEXTURE_DIR}/{file_name}"),
                });
                self.out.textures.push(TextureFile { file_name, png });
                let found = (self.out.scene.images.len() - 1, has_alpha);
                self.images.insert(key, found);
                Some(found)
            }
            Err(w) => {
                self.out.warnings.push(w);
                None
            }
        }
    }

    /// Adds one material per shader of the drawable, in shader order, and
    /// returns the material index of each shader.
    fn add_materials(
        &mut self,
        label: &str,
        drawable: &Drawable,
        res: &lf_model::Resource,
    ) -> Vec<usize> {
        let Some(group) = &drawable.shaders else {
            return Vec::new();
        };
        // The embedded dictionary, if the shader group has one.
        let embedded = group.texture_dict.and_then(|offset| {
            embedded_dictionary(res, offset)
                .map_err(|e| {
                    self.out.warnings.push(format!(
                        "{label}: embedded texture dictionary unreadable: {e}"
                    ));
                })
                .ok()
        });
        let sources = Sources {
            embedded: embedded.as_ref(),
            embedded_tag: format!("embedded:{label}"),
            external: self.external,
        };
        let mut material_of_shader = Vec::new();
        for shader in &group.shaders {
            let textures: Vec<&str> = shader
                .params
                .iter()
                .filter_map(|(_, p)| match p {
                    ShaderParam::Texture { name } if !name.is_empty() => Some(name.as_str()),
                    _ => None,
                })
                .collect();
            let chosen = textures
                .iter()
                .find_map(|name| self.image_for(&sources, name));
            if chosen.is_none() && !textures.is_empty() {
                self.out.warnings.push(format!(
                    "{label}: shader {} uses {} but none was found in the texture dictionaries given",
                    shader.name,
                    textures.join(", ")
                ));
            }
            self.out.scene.materials.push(Material {
                name: if shader.name.is_empty() {
                    format!("{label}_shader")
                } else {
                    shader.name.clone()
                },
                image: chosen.map(|c| c.0),
                alpha_mask: chosen.is_some_and(|c| c.1),
            });
            material_of_shader.push(self.out.scene.materials.len() - 1);
        }
        material_of_shader
    }

    /// Adds one drawable's materials and meshes.
    fn add_drawable(&mut self, label: &str, drawable: &Drawable, res: &lf_model::Resource) {
        let material_of_shader = self.add_materials(label, drawable, res);
        let lods: Vec<(usize, &lf_model::LodGroup)> = match self.opts.lod {
            LodChoice::All => drawable.lods.iter().enumerate().collect(),
            LodChoice::Index(i) => {
                if let Some(l) = drawable.lods.get(i) {
                    vec![(i, l)]
                } else {
                    self.out.warnings.push(format!(
                        "{label}: LOD {i} requested but the drawable has {} LODs",
                        drawable.lods.len()
                    ));
                    Vec::new()
                }
            }
        };
        for (lod_index, lod) in lods {
            let mut mesh = Mesh {
                name: if drawable.lods.len() > 1 || self.opts.lod == LodChoice::All {
                    format!("{label}_lod{lod_index}")
                } else {
                    label.to_string()
                },
                primitives: Vec::new(),
            };
            for (model_index, model) in lod.models.iter().enumerate() {
                for (geo_index, geo) in model.geometries.iter().enumerate() {
                    match convert_geometry(geo, res, &self.opts) {
                        Ok(GeometryOut {
                            primitive: mut prim,
                            triangles,
                            degenerate_normals,
                        }) => {
                            if degenerate_normals > 0 {
                                self.out.warnings.push(format!(
                                    "{label}: LOD {lod_index} model {model_index} geometry {geo_index}: \
                                     {degenerate_normals} zero-length normals replaced by +Z"
                                ));
                            }
                            prim.material = model
                                .shader_map
                                .get(geo_index)
                                .and_then(|&s| material_of_shader.get(usize::from(s)))
                                .copied();
                            self.out.vertices += prim.positions.len();
                            self.out.triangles += triangles;
                            mesh.primitives.push(prim);
                        }
                        Err(w) => self.out.warnings.push(format!(
                            "{label}: LOD {lod_index} model {model_index} geometry {geo_index} skipped: {w}"
                        )),
                    }
                }
            }
            if !mesh.primitives.is_empty() {
                self.out.scene.meshes.push(mesh);
            }
        }
    }
}

/// Converts drawables (one from a `.wdr` or `.wft`, several from a `.wdd`)
/// that live in one model resource. Each entry is `(label, drawable)`;
/// labels name the meshes. `dictionaries` are `(tag, dictionary)` pairs
/// searched in order after each drawable's embedded dictionary.
#[must_use]
pub fn convert(
    root_name: &str,
    drawables: &[(String, &Drawable)],
    res: &lf_model::Resource,
    dictionaries: &[(String, Dictionary)],
    opts: Options,
) -> Converted {
    let mut b = Builder {
        opts,
        external: dictionaries,
        out: Converted::default(),
        names: NameSet::default(),
        images: HashMap::new(),
    };
    b.out.scene.root_name = root_name.to_string();
    if opts.z_up_to_y_up {
        b.out.scene.root_rotation = Some(lf_math::Quat::from_rotation_x(
            -core::f32::consts::FRAC_PI_2,
        ));
    }
    for (label, drawable) in drawables {
        b.add_drawable(label, drawable, res);
    }
    b.out
}
