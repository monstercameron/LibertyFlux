//! Vertex declarations and vertex data decoding.
//!
//! Each geometry's vertex buffer points at a 16-byte declaration: a usage
//! mask, the vertex stride in bytes, two decoder tag bytes, and a 64-bit
//! word packing sixteen 4-bit element types, one per usage slot. A set bit
//! in the mask means that slot's element is present; elements appear in the
//! vertex in slot order, tightly packed.
//!
//! Slot order: position, blend weights, blend indices, normal, two
//! colours, eight texture coordinates, tangent, binormal.
//!
//! Element data types are the Direct3D 9 declaration types: one- to
//! four-component 16-bit or 32-bit float vectors, four unsigned bytes,
//! packed colours, and packed 10-10-10-2 normals.

use crate::Error;
use crate::cursor::Cursor;

/// What a vertex element is used for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementUsage {
    /// Vertex position.
    Position,
    /// Skinning blend weights.
    BlendWeight,
    /// Skinning blend bone indices.
    BlendIndices,
    /// Vertex normal.
    Normal,
    /// Vertex colour; index 0 is diffuse, 1 is specular.
    Color(u8),
    /// Texture coordinates; the channel number.
    TexCoord(u8),
    /// Tangent vector.
    Tangent,
    /// Binormal vector.
    Binormal,
}

/// Storage type of a vertex element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    /// One 16-bit float.
    F16x1,
    /// Two 16-bit floats.
    F16x2,
    /// Three 16-bit floats.
    F16x3,
    /// Four 16-bit floats.
    F16x4,
    /// One 32-bit float.
    F32x1,
    /// Two 32-bit floats.
    F32x2,
    /// Three 32-bit floats.
    F32x3,
    /// Four 32-bit floats.
    F32x4,
    /// Four unsigned bytes.
    UByte4,
    /// Packed colour (four bytes).
    Color,
    /// Packed 10-10-10-2 signed normalised vector.
    Dec3N,
    /// Reserved type nibbles; never observed in the sample.
    Reserved(u8),
}

impl ElementType {
    /// Decode a 4-bit type nibble.
    #[must_use]
    pub fn from_nibble(n: u8) -> ElementType {
        match n {
            0 => ElementType::F16x1,
            1 => ElementType::F16x2,
            2 => ElementType::F16x3,
            3 => ElementType::F16x4,
            4 => ElementType::F32x1,
            5 => ElementType::F32x2,
            6 => ElementType::F32x3,
            7 => ElementType::F32x4,
            8 => ElementType::UByte4,
            9 => ElementType::Color,
            10 => ElementType::Dec3N,
            x => ElementType::Reserved(x),
        }
    }

    /// Size in bytes of one element of this type.
    #[must_use]
    pub fn size(&self) -> usize {
        match self {
            ElementType::F16x1 => 2,
            ElementType::F16x2
            | ElementType::F32x1
            | ElementType::UByte4
            | ElementType::Color
            | ElementType::Dec3N => 4,
            ElementType::F16x3 => 6,
            ElementType::F16x4 | ElementType::F32x2 => 8,
            ElementType::F32x3 => 12,
            ElementType::F32x4 => 16,
            ElementType::Reserved(_) => 0,
        }
    }

    /// Number of scalar components this type decodes to.
    #[must_use]
    pub fn components(&self) -> usize {
        match self {
            ElementType::F16x1 | ElementType::F32x1 => 1,
            ElementType::F16x2 | ElementType::F32x2 => 2,
            ElementType::F16x3 | ElementType::F32x3 => 3,
            ElementType::F16x4
            | ElementType::F32x4
            | ElementType::UByte4
            | ElementType::Color
            | ElementType::Dec3N => 4,
            ElementType::Reserved(_) => 0,
        }
    }
}

/// One present element of a vertex declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VertexElement {
    /// What the element is used for.
    pub usage: ElementUsage,
    /// How it is stored.
    pub kind: ElementType,
    /// Byte offset of the element within one vertex.
    pub offset: usize,
}

/// A decoded vertex declaration.
#[derive(Debug, Clone)]
pub struct VertexDecl {
    /// Raw usage mask from the file.
    pub usage_mask: u32,
    /// Stride in bytes from the declaration (should match the buffer).
    pub stride: u16,
    /// First decoder tag byte; meaning unknown.
    pub alt_decoder: u8,
    /// Second decoder tag byte; meaning unknown.
    pub decl_type: u8,
    /// Elements present, in vertex order.
    pub elements: Vec<VertexElement>,
}

impl VertexDecl {
    /// Parse a declaration from the system segment at `at`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(sys: &[u8], at: usize) -> Result<VertexDecl, Error> {
        let mut c = Cursor::at(sys, at)?;
        let usage_mask = c.u32()?;
        let stride = c.u16()?;
        let alt_decoder = c.u8()?;
        let decl_type = c.u8()?;
        let packed = c.u64()?;
        let mut elements = Vec::new();
        let mut offset = 0usize;
        for slot in 0..16u32 {
            if usage_mask & (1 << slot) == 0 {
                continue;
            }
            let kind =
                ElementType::from_nibble(u8::try_from((packed >> (4 * slot)) & 0xF).unwrap_or(0));
            let usage = match slot {
                0 => ElementUsage::Position,
                1 => ElementUsage::BlendWeight,
                2 => ElementUsage::BlendIndices,
                3 => ElementUsage::Normal,
                4 => ElementUsage::Color(0),
                5 => ElementUsage::Color(1),
                6..=13 => ElementUsage::TexCoord(u8::try_from(slot - 6).unwrap_or(u8::MAX)),
                14 => ElementUsage::Tangent,
                _ => ElementUsage::Binormal,
            };
            elements.push(VertexElement {
                usage,
                kind,
                offset,
            });
            offset += kind.size();
        }
        Ok(VertexDecl {
            usage_mask,
            stride,
            alt_decoder,
            decl_type,
            elements,
        })
    }

    /// Find the first element with the given usage.
    #[must_use]
    pub fn find(&self, usage: ElementUsage) -> Option<VertexElement> {
        self.elements.iter().copied().find(|e| e.usage == usage)
    }
}

/// One decoded vertex: positions, normals, colours and texture
/// coordinates as plain floats. Missing elements read as zero.
#[derive(Debug, Clone, Default)]
pub struct DecodedVertex {
    /// Position (x, y, z).
    pub pos: [f32; 3],
    /// Normal (x, y, z).
    pub normal: [f32; 3],
    /// Diffuse colour as stored (packed bytes).
    pub diffuse: u32,
    /// Specular colour as stored (packed bytes).
    pub specular: u32,
    /// First texture coordinate channel (u, v).
    pub uv: [f32; 2],
}

/// Decode one vertex from raw bytes using its declaration.
///
/// `bytes` must hold at least `stride` bytes starting at the vertex.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
///
/// # Panics
///
/// Never panics: every conversion is on an exactly-sized slice.
pub fn decode_vertex(decl: &VertexDecl, bytes: &[u8]) -> Result<DecodedVertex, Error> {
    let mut v = DecodedVertex::default();
    for e in &decl.elements {
        let end = e.offset + e.kind.size();
        let src = bytes.get(e.offset..end).ok_or(Error::Truncated {
            offset: end,
            len: bytes.len(),
        })?;
        match e.usage {
            ElementUsage::Position => {
                let c = decode_components(e.kind, src)?;
                v.pos = [get(&c, 0), get(&c, 1), get(&c, 2)];
            }
            ElementUsage::Normal => {
                let c = decode_components(e.kind, src)?;
                v.normal = [get(&c, 0), get(&c, 1), get(&c, 2)];
            }
            ElementUsage::Color(0) => v.diffuse = u32::from_le_bytes(src.try_into().unwrap()),
            ElementUsage::Color(1) => v.specular = u32::from_le_bytes(src.try_into().unwrap()),
            ElementUsage::TexCoord(0) => {
                let c = decode_components(e.kind, src)?;
                v.uv = [get(&c, 0), get(&c, 1)];
            }
            _ => {}
        }
    }
    Ok(v)
}

fn get(c: &[f32], i: usize) -> f32 {
    c.get(i).copied().unwrap_or(0.0)
}

/// Decode one element's raw bytes into float components.
fn decode_components(kind: ElementType, src: &[u8]) -> Result<[f32; 4], Error> {
    let mut out = [0.0f32; 4];
    match kind {
        ElementType::F32x1 | ElementType::F32x2 | ElementType::F32x3 | ElementType::F32x4 => {
            for (i, chunk) in src.chunks_exact(4).enumerate() {
                out[i] = f32::from_le_bytes(chunk.try_into().unwrap());
            }
        }
        ElementType::F16x1 | ElementType::F16x2 | ElementType::F16x3 | ElementType::F16x4 => {
            for (i, chunk) in src.chunks_exact(2).enumerate() {
                out[i] = half_to_f32(u16::from_le_bytes(chunk.try_into().unwrap()));
            }
        }
        ElementType::UByte4 | ElementType::Color => {
            // Kept as raw bytes scaled to 0..1; colour channel order is
            // the Direct3D packed order (blue, green, red, alpha).
            for (i, b) in src.iter().enumerate().take(4) {
                out[i] = f32::from(*b) / 255.0;
            }
        }
        ElementType::Dec3N => {
            let v = u32::from_le_bytes(src.try_into().unwrap());
            out[0] = dec3n_axis(v, 0);
            out[1] = dec3n_axis(v, 10);
            out[2] = dec3n_axis(v, 20);
            out[3] = f32::from(u8::try_from((v >> 30) & 3).unwrap_or(0));
        }
        ElementType::Reserved(x) => {
            return Err(Error::BadEnum {
                what: "vertex element type",
                value: u32::from(x),
            });
        }
    }
    Ok(out)
}

/// Convert an IEEE 754 binary16 value to f32.
#[must_use]
pub fn half_to_f32(h: u16) -> f32 {
    let sign = f32::from((h >> 15) & 1);
    let exp = (h >> 10) & 0x1F;
    let mant = f32::from(h & 0x3FF);
    let v = if exp == 0 {
        mant / 1024.0 * 2f32.powi(-14)
    } else if exp == 31 {
        if mant == 0.0 { f32::INFINITY } else { f32::NAN }
    } else {
        (1.0 + mant / 1024.0) * 2f32.powi(i32::from(exp) - 15)
    };
    if sign == 0.0 { v } else { -v }
}

/// Decode one 10-bit signed-normalised axis of a `Dec3N` word.
fn dec3n_axis(v: u32, shift: u32) -> f32 {
    let raw = i32::try_from((v >> shift) & 0x3FF).unwrap_or(0);
    let signed = (raw << 22) >> 22; // sign-extend 10 bits
    if signed == -512 {
        -1.0
    } else {
        f32::from(i16::try_from(signed).unwrap_or(0)) / 511.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decl_bytes(mask: u32, stride: u16, packed: u64) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&mask.to_le_bytes());
        b.extend_from_slice(&stride.to_le_bytes());
        b.push(0);
        b.push(0);
        b.extend_from_slice(&packed.to_le_bytes());
        b
    }

    #[test]
    fn decl_layout_matches_documented_slots() {
        // Position f32x3 (slot 0, nibble 6), normal f32x3 (slot 3),
        // colour (slot 4, nibble 9), tex0 f32x2 (slot 6, nibble 5).
        let packed = 6u64 | (6 << 12) | (9 << 16) | (5 << 24);
        let mask = (1 << 0) | (1 << 3) | (1 << 4) | (1 << 6);
        let b = decl_bytes(mask, 36, packed);
        let d = VertexDecl::parse(&b, 0).unwrap();
        assert_eq!(d.elements.len(), 4);
        assert_eq!(d.elements[0].usage, ElementUsage::Position);
        assert_eq!(d.elements[0].kind, ElementType::F32x3);
        assert_eq!(d.elements[0].offset, 0);
        assert_eq!(d.elements[1].usage, ElementUsage::Normal);
        assert_eq!(d.elements[1].offset, 12);
        assert_eq!(d.elements[2].usage, ElementUsage::Color(0));
        assert_eq!(d.elements[2].offset, 24);
        assert_eq!(d.elements[3].usage, ElementUsage::TexCoord(0));
        assert_eq!(d.elements[3].offset, 28);
    }

    #[test]
    fn decodes_mixed_float_vertex() {
        // Position f16x3 + normal f32x3 + tex0 f16x2 = 6+12+4 = 22 bytes.
        let packed = 2u64 | (6 << 12) | (1 << 24);
        let mask = (1 << 0) | (1 << 3) | (1 << 6);
        let b = decl_bytes(mask, 22, packed);
        let d = VertexDecl::parse(&b, 0).unwrap();
        let mut v = Vec::new();
        // 1.0, -2.0, 0.5 as binary16.
        v.extend_from_slice(&[0x00, 0x3C, 0x00, 0xC0, 0x00, 0x38]);
        v.extend_from_slice(&0.0f32.to_le_bytes());
        v.extend_from_slice(&1.0f32.to_le_bytes());
        v.extend_from_slice(&0.0f32.to_le_bytes());
        v.extend_from_slice(&[0x00, 0x3C, 0x00, 0x3C]); // uv 1,1
        let dv = decode_vertex(&d, &v).unwrap();
        assert_eq!(dv.pos, [1.0, -2.0, 0.5]);
        assert_eq!(dv.normal, [0.0, 1.0, 0.0]);
        assert_eq!(dv.uv, [1.0, 1.0]);
    }

    #[test]
    fn decodes_packed_normal_and_colour() {
        let packed = (10u64 << 12) | (9u64 << 16);
        let mask = (1 << 3) | (1 << 4);
        let b = decl_bytes(mask, 8, packed);
        let d = VertexDecl::parse(&b, 0).unwrap();
        // Dec3N +Z: z axis max (511 << 20).
        let mut v = Vec::new();
        v.extend_from_slice(&(511u32 << 20).to_le_bytes());
        v.extend_from_slice(&0xFF8040C0u32.to_le_bytes());
        let dv = decode_vertex(&d, &v).unwrap();
        assert!((dv.normal[2] - 1.0).abs() < 0.01);
        assert_eq!(dv.diffuse, 0xFF8040C0);
    }

    #[test]
    fn type_sizes_cover_all_nibbles() {
        let sizes = [2, 4, 6, 8, 4, 8, 12, 16, 4, 4, 4];
        for (n, want) in sizes.iter().enumerate() {
            assert_eq!(ElementType::from_nibble(n as u8).size(), *want);
        }
    }
}
