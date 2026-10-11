//! Drawables: LOD groups, models and geometries.
//!
//! A drawable header (at system offset 0 of a `.wdr` resource) holds a
//! shader group, a skeleton, bounding volumes, and up to four LOD slots.
//! Each LOD slot points at a model collection; each model points at
//! geometries; each geometry points at one vertex buffer and one index
//! buffer. Vertex bytes live in the graphics segment.
//!
//! Primitive types follow the Direct3D 9 enumeration starting at zero:
//! point list, line list, line strip, triangle list, triangle strip,
//! triangle fan. Every geometry in the verification sample is a triangle
//! list.

use crate::Error;
use crate::cursor::Cursor;
use crate::rsc5::Resource;
use crate::shader::ShaderGroup;
use crate::skeleton::Skeleton;
use crate::vertex::{DecodedVertex, VertexDecl, decode_vertex};

/// How indices in a geometry are assembled into primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    /// Unconnected points.
    PointList,
    /// Unconnected line segments.
    LineList,
    /// Connected line strip.
    LineStrip,
    /// Unconnected triangles.
    TriangleList,
    /// Connected triangle strip.
    TriangleStrip,
    /// Connected triangle fan.
    TriangleFan,
}

impl PrimitiveType {
    /// Decode the file value.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn from_file(v: u16) -> Result<PrimitiveType, Error> {
        match v {
            0 => Ok(PrimitiveType::PointList),
            1 => Ok(PrimitiveType::LineList),
            2 => Ok(PrimitiveType::LineStrip),
            3 => Ok(PrimitiveType::TriangleList),
            4 => Ok(PrimitiveType::TriangleStrip),
            5 => Ok(PrimitiveType::TriangleFan),
            x => Err(Error::BadEnum {
                what: "primitive type",
                value: u32::from(x),
            }),
        }
    }
}

/// A drawable: shader group, skeleton, bounds and LOD groups.
#[derive(Debug, Clone)]
pub struct Drawable {
    /// Vtable slot from the file, kept for identification.
    pub vtable: u32,
    /// Shader group, or `None` when the file has no shaders.
    pub shaders: Option<ShaderGroup>,
    /// Skeleton, or `None` when the file has none.
    pub skeleton: Option<Skeleton>,
    /// Bounding-sphere centre (x, y, z, radius).
    pub center: [f32; 4],
    /// Minimum corner of the bounding box.
    pub bounds_min: [f32; 4],
    /// Maximum corner of the bounding box.
    pub bounds_max: [f32; 4],
    /// Absolute maximum vector (usually all 9999).
    pub abs_max: [f32; 4],
    /// Present LOD groups in slot order.
    pub lods: Vec<LodGroup>,
}

impl Drawable {
    /// Parse the drawable at the start of the system segment.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource) -> Result<Drawable, Error> {
        Self::parse_at(res, 0)
    }

    /// Parse a drawable at a system-segment offset (used for dictionary
    /// entries and fragment children).
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse_at(res: &Resource, at: usize) -> Result<Drawable, Error> {
        let mut c = Cursor::at(&res.sys, at)?;
        let vtable = c.u32()?;
        c.skip(4)?; // block-map address
        let sg = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let sk = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let center = c.vec4()?;
        let bounds_min = c.vec4()?;
        let bounds_max = c.vec4()?;
        let mut lod_slots = [None; 4];
        for slot in &mut lod_slots {
            *slot = res.sys_ptr(c.pos())?;
            c.skip(4)?;
        }
        let abs_max = c.vec4()?;
        let shaders = sg.map(|o| ShaderGroup::parse(res, o)).transpose()?;
        let skeleton = sk.map(|o| Skeleton::parse(res, o)).transpose()?;
        let mut lods = Vec::new();
        for slot in lod_slots.into_iter().flatten() {
            lods.push(LodGroup::parse(res, slot)?);
        }
        Ok(Drawable {
            vtable,
            shaders,
            skeleton,
            center,
            bounds_min,
            bounds_max,
            abs_max,
            lods,
        })
    }

    /// Iterate over all models in all LODs.
    pub fn models(&self) -> impl Iterator<Item = &Model> {
        self.lods.iter().flat_map(|l| l.models.iter())
    }

    /// Iterate over all geometries in all LODs.
    pub fn geometries(&self) -> impl Iterator<Item = &Geometry> {
        self.models().flat_map(|m| m.geometries.iter())
    }

    /// Total vertex count over all geometries.
    #[must_use]
    pub fn vertex_count(&self) -> u32 {
        self.geometries().map(|g| u32::from(g.vertex_count)).sum()
    }

    /// Total index count over all geometries.
    #[must_use]
    pub fn index_count(&self) -> u32 {
        self.geometries().map(|g| g.index_count).sum()
    }
}

/// One level of detail: a collection of models.
#[derive(Debug, Clone)]
pub struct LodGroup {
    /// Models in this LOD.
    pub models: Vec<Model>,
}

impl LodGroup {
    /// Parse a model collection (pointer collection) at `at`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize) -> Result<LodGroup, Error> {
        let mut models = Vec::new();
        for off in read_ptr_list(res, at)? {
            models.push(Model::parse(res, off)?);
        }
        Ok(LodGroup { models })
    }
}

/// One model: geometries plus per-geometry bounds and shader mapping.
#[derive(Debug, Clone)]
pub struct Model {
    /// Vtable slot from the file.
    pub vtable: u32,
    /// Geometries of this model.
    pub geometries: Vec<Geometry>,
    /// Bounding boxes: one min/max pair per geometry plus one pair for
    /// the whole model (min then max, each four floats).
    pub bounds: Vec<[f32; 4]>,
    /// Shader index per geometry into the drawable's shader group.
    pub shader_map: Vec<u16>,
    /// Matrix count byte; meaning partly unknown.
    pub matrix_count: u8,
    /// Flags byte; meaning unknown.
    pub flags: u8,
    /// Type byte; meaning unknown.
    pub kind: u8,
    /// Matrix index byte; meaning unknown.
    pub matrix_index: u8,
    /// Render mask byte; meaning unknown.
    pub render_mask: u8,
    /// Skin flag byte; meaning unknown.
    pub skin_flag: u8,
}

impl Model {
    /// Parse a model at a system-segment offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize) -> Result<Model, Error> {
        let mut c = Cursor::at(&res.sys, at)?;
        let vtable = c.u32()?;
        let geo_coll = c.pos();
        c.skip(8)?; // pointer collection (offset + count + size)
        let bounds_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let map_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let matrix_count = c.u8()?;
        let flags = c.u8()?;
        let kind = c.u8()?;
        let matrix_index = c.u8()?;
        let render_mask = c.u8()?;
        let skin_flag = c.u8()?;
        let _geo_count = c.u16()?;
        let mut geometries = Vec::new();
        for off in read_ptr_list(res, geo_coll)? {
            geometries.push(Geometry::parse(res, off)?);
        }
        let mut bounds = Vec::new();
        if let Some(o) = bounds_off {
            // One min/max pair per geometry plus one pair for the model.
            let mut bc = Cursor::at(&res.sys, o)?;
            for _ in 0..(geometries.len() + 1) * 2 {
                bounds.push(bc.vec4()?);
            }
        }
        let mut shader_map = Vec::new();
        if let Some(o) = map_off {
            let mut mc = Cursor::at(&res.sys, o)?;
            for _ in 0..geometries.len() {
                shader_map.push(mc.u16()?);
            }
        }
        Ok(Model {
            vtable,
            geometries,
            bounds,
            shader_map,
            matrix_count,
            flags,
            kind,
            matrix_index,
            render_mask,
            skin_flag,
        })
    }
}

/// One geometry: a single draw call's buffers and counts.
#[derive(Debug, Clone)]
pub struct Geometry {
    /// Vtable slot from the file.
    pub vtable: u32,
    /// Number of indices.
    pub index_count: u32,
    /// Number of faces.
    pub face_count: u32,
    /// Number of vertices.
    pub vertex_count: u16,
    /// How indices assemble into primitives.
    pub primitive: PrimitiveType,
    /// Vertex stride in bytes.
    pub stride: u16,
    /// Skinning matrix palette (bone indices), empty when unskinned.
    pub matrix_palette: Vec<u16>,
    /// Vertex declaration decoded from the vertex buffer.
    pub declaration: VertexDecl,
    /// Byte offset of the vertex bytes in the graphics segment.
    pub vertex_data: usize,
    /// Byte offset of the index bytes in the graphics segment.
    pub index_data: usize,
}

impl Geometry {
    /// Parse a geometry at a system-segment offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize) -> Result<Geometry, Error> {
        let mut c = Cursor::at(&res.sys, at)?;
        let vtable = c.u32()?;
        c.skip(8)?; // unknowns 1-2
        let vb = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        c.skip(12)?; // unknowns 3-5
        let ib = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        c.skip(12)?; // unknowns 6-8
        let index_count = c.u32()?;
        let face_count = c.u32()?;
        let vertex_count = c.u16()?;
        let primitive = PrimitiveType::from_file(c.u16()?)?;
        let palette_ptr = c.u32()?;
        let stride = c.u16()?;
        let matrix_count = c.u16()?;
        let vb = vb.ok_or(Error::BadCount {
            what: "vertex buffer",
            value: 0,
        })?;
        let ib = ib.ok_or(Error::BadCount {
            what: "index buffer",
            value: 0,
        })?;
        let (vb_count, vertex_data, vb_stride, declaration) = parse_vertex_buffer(res, vb)?;
        let (ib_count, index_data) = parse_index_buffer(res, ib)?;
        if u32::from(vb_count) != u32::from(vertex_count) || ib_count != index_count {
            return Err(Error::BadCount {
                what: "buffer length",
                value: index_count,
            });
        }
        let _ = vb_stride;
        let mut matrix_palette = Vec::new();
        if matrix_count > 0 && palette_ptr != 0 {
            if palette_ptr >> 28 != 5 {
                return Err(Error::BadPointer {
                    offset: c.pos() - 8,
                    value: palette_ptr,
                });
            }
            let mut pc = Cursor::at(&res.sys, (palette_ptr & 0x0FFF_FFFF) as usize)?;
            for _ in 0..matrix_count {
                matrix_palette.push(pc.u16()?);
            }
        }
        Ok(Geometry {
            vtable,
            index_count,
            face_count,
            vertex_count,
            primitive,
            stride,
            matrix_palette,
            declaration,
            vertex_data,
            index_data,
        })
    }

    /// Read all indices as u16 values.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    ///
    /// # Panics
    ///
    /// Never panics: chunks are exactly two bytes, so the conversion cannot fail.
    pub fn indices(&self, res: &Resource) -> Result<Vec<u16>, Error> {
        let len = self.index_count as usize * 2;
        let end = self.index_data + len;
        let src = res.gfx.get(self.index_data..end).ok_or(Error::Truncated {
            offset: end,
            len: res.gfx.len(),
        })?;
        Ok(src
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes(c.try_into().unwrap()))
            .collect())
    }

    /// Decode all vertices into positions, normals, colours and uvs.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn vertices(&self, res: &Resource) -> Result<Vec<DecodedVertex>, Error> {
        let stride = usize::from(self.stride);
        let mut out = Vec::with_capacity(usize::from(self.vertex_count));
        for i in 0..usize::from(self.vertex_count) {
            let start = self.vertex_data + i * stride;
            let src = res.gfx.get(start..start + stride).ok_or(Error::Truncated {
                offset: start + stride,
                len: res.gfx.len(),
            })?;
            out.push(decode_vertex(&self.declaration, src)?);
        }
        Ok(out)
    }

    /// Number of triangles this geometry draws (triangle lists and
    /// strips only; other primitive types return `None`).
    #[must_use]
    pub fn triangle_count(&self) -> Option<u32> {
        match self.primitive {
            PrimitiveType::TriangleList => Some(self.index_count / 3),
            PrimitiveType::TriangleStrip => Some(self.index_count.saturating_sub(2)),
            _ => None,
        }
    }
}

/// Parse a vertex buffer header: returns count, data offset, stride and
/// the decoded declaration.
fn parse_vertex_buffer(res: &Resource, at: usize) -> Result<(u16, usize, u32, VertexDecl), Error> {
    let mut c = Cursor::at(&res.sys, at)?;
    c.skip(4)?; // vtable
    let count = c.u16()?;
    c.skip(2)?; // lock/align byte pair
    let data = res.gfx_ptr(c.pos())?.unwrap_or(0);
    c.skip(4)?;
    let stride = c.u32()?;
    let decl = res.sys_ptr(c.pos())?.unwrap_or(0);
    c.skip(4)?;
    let declaration = VertexDecl::parse(&res.sys, decl)?;
    Ok((count, data, stride, declaration))
}

/// Parse an index buffer header: returns count and data offset.
fn parse_index_buffer(res: &Resource, at: usize) -> Result<(u32, usize), Error> {
    let mut c = Cursor::at(&res.sys, at)?;
    c.skip(4)?; // vtable
    let count = c.u32()?;
    let data = res.gfx_ptr(c.pos())?.unwrap_or(0);
    Ok((count, data))
}

/// Read a pointer collection: a system pointer to a list of system
/// pointers followed by count and size words. Returns the item offsets.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn read_ptr_list(res: &Resource, at: usize) -> Result<Vec<usize>, Error> {
    let mut c = Cursor::at(&res.sys, at)?;
    let list = res.sys_ptr(c.pos())?.unwrap_or(0);
    c.skip(4)?;
    let count = c.u16()?;
    let _size = c.u16()?;
    if count > 4096 {
        return Err(Error::BadCount {
            what: "pointer collection",
            value: u32::from(count),
        });
    }
    let mut out = Vec::with_capacity(usize::from(count));
    for i in 0..usize::from(count) {
        let off = res.sys_ptr(list + 4 * i)?.ok_or(Error::BadCount {
            what: "pointer collection entry",
            value: u32::try_from(i).unwrap_or(u32::MAX),
        })?;
        out.push(off);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_values_follow_direct3d_order() {
        assert_eq!(
            PrimitiveType::from_file(3).unwrap(),
            PrimitiveType::TriangleList
        );
        assert!(PrimitiveType::from_file(6).is_err());
    }

    #[test]
    fn ptr_list_parses_hand_built_collection() {
        // Collection at 0: list ptr -> 16, count 2. Items at 32 and 48.
        let mut sys = vec![0u8; 64];
        sys[0..4].copy_from_slice(&0x5000_0010u32.to_le_bytes());
        sys[4..6].copy_from_slice(&2u16.to_le_bytes());
        sys[16..20].copy_from_slice(&0x5000_0020u32.to_le_bytes());
        sys[20..24].copy_from_slice(&0x5000_0030u32.to_le_bytes());
        let res = Resource {
            kind: crate::rsc5::TYPE_DRAWABLE,
            flags: 0,
            sys,
            gfx: vec![],
        };
        assert_eq!(read_ptr_list(&res, 0).unwrap(), vec![0x20, 0x30]);
    }

    #[test]
    fn triangle_counts_for_lists_and_strips() {
        let geo = Geometry {
            vtable: 0,
            index_count: 9,
            face_count: 3,
            vertex_count: 9,
            primitive: PrimitiveType::TriangleList,
            stride: 12,
            matrix_palette: vec![],
            declaration: VertexDecl {
                usage_mask: 0,
                stride: 12,
                alt_decoder: 0,
                decl_type: 0,
                elements: vec![],
            },
            vertex_data: 0,
            index_data: 0,
        };
        assert_eq!(geo.triangle_count(), Some(3));
        let mut strip = geo;
        strip.primitive = PrimitiveType::TriangleStrip;
        assert_eq!(strip.triangle_count(), Some(7));
    }
}
