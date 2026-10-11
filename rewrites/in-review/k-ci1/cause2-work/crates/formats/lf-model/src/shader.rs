//! Shader groups and shader parameters.
//!
//! A shader group holds an optional embedded texture dictionary, a
//! collection of shaders, twelve render-pass shader indices, and two
//! small index collections (vertex-declaration usage flags and technique
//! indices).
//!
//! Each shader holds three parallel arrays (parameter offsets, parameter
//! types, parameter name hashes) plus a shader name and an effect-source
//! string. Parameter payloads are textures (carrying a texture name),
//! four-vectors, 4x4 or 4x3 matrices, or single floats.

use crate::Error;
use crate::cursor::Cursor;
use crate::drawable::read_ptr_list;
use crate::rsc5::Resource;

/// A shader group: all shaders used by one drawable.
#[derive(Debug, Clone)]
pub struct ShaderGroup {
    /// System offset of the embedded texture dictionary, if any.
    pub texture_dict: Option<usize>,
    /// Shaders in the group.
    pub shaders: Vec<Shader>,
    /// Twelve shader indices for different render passes.
    pub pass_indices: [u32; 12],
    /// Vertex-declaration usage flags per shader.
    pub usage_flags: Vec<u32>,
    /// Technique index per shader.
    pub techniques: Vec<u32>,
}

impl ShaderGroup {
    /// Parse a shader group at a system-segment offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize) -> Result<ShaderGroup, Error> {
        let mut c = Cursor::at(&res.sys, at)?;
        c.skip(4)?; // vtable
        let texture_dict = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let mut shaders = Vec::new();
        for off in read_ptr_list(res, c.pos())? {
            shaders.push(Shader::parse(res, off)?);
        }
        c.skip(8)?; // pointer collection inline part
        let mut pass_indices = [0u32; 12];
        for slot in &mut pass_indices {
            *slot = c.u32()?;
        }
        let usage_flags = read_u32_collection(res, c.pos())?;
        c.skip(8)?;
        let techniques = read_u32_collection(res, c.pos())?;
        Ok(ShaderGroup {
            texture_dict,
            shaders,
            pass_indices,
            usage_flags,
            techniques,
        })
    }
}

/// One shader: parameter payloads keyed by name hash, plus names.
#[derive(Debug, Clone)]
pub struct Shader {
    /// Effect hash from the file.
    pub hash: u32,
    /// Parameters as (name hash, payload) pairs.
    pub params: Vec<(u32, ShaderParam)>,
    /// Shader name string (for example a `gta_*` effect name).
    pub name: String,
    /// Effect source string.
    pub source: String,
}

impl Shader {
    /// Parse a shader at a system-segment offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize) -> Result<Shader, Error> {
        let mut c = Cursor::at(&res.sys, at)?;
        c.skip(8)?; // page-base part (vtable + block map)
        c.skip(2 + 1 + 1 + 2 + 2 + 4)?; // unknown header words
        let offsets_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        c.skip(4)?; // unknown
        let count = c.u32()? as usize;
        c.skip(4)?; // unknown
        let types_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let hash = c.u32()?;
        c.skip(8)?; // unknowns
        let names_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        c.skip(12)?; // unknowns
        let name = res.sys_ptr(c.pos())?.map(|o| res.cstr(o)).transpose()?;
        c.skip(4)?;
        let source = res.sys_ptr(c.pos())?.map(|o| res.cstr(o)).transpose()?;
        if count > 1024 {
            return Err(Error::BadCount {
                what: "shader parameter",
                value: u32::try_from(count).unwrap_or(u32::MAX),
            });
        }
        let mut params = Vec::with_capacity(count);
        if let (Some(oo), Some(to), Some(no)) = (offsets_off, types_off, names_off) {
            for i in 0..count {
                let po = res.sys_ptr(oo + 4 * i)?;
                let ty = res.sys.get(to + i).copied().unwrap_or(0xFF);
                let nm = res.u32_sys(no + 4 * i)?;
                // A parameter that fails to parse is kept as unknown
                // rather than failing the whole shader.
                let payload = po
                    .map(|o| ShaderParam::parse(res, o, ty))
                    .transpose()?
                    .unwrap_or(ShaderParam::Unknown { kind: ty });
                params.push((nm, payload));
            }
        }
        Ok(Shader {
            hash,
            params,
            name: name.unwrap_or_default(),
            source: source.unwrap_or_default(),
        })
    }
}

/// One shader parameter payload.
#[derive(Debug, Clone)]
pub enum ShaderParam {
    /// Texture reference with a texture name.
    Texture {
        /// Texture name string.
        name: String,
    },
    /// Four floats.
    Vector4([f32; 4]),
    /// Sixteen floats in file order.
    Matrix4x4([[f32; 4]; 4]),
    /// Twelve floats in file order.
    Matrix4x3([[f32; 4]; 3]),
    /// One float.
    Float(f32),
    /// Unrecognised parameter type; the raw type byte is kept.
    Unknown {
        /// Raw type byte from the file.
        kind: u8,
    },
}

impl ShaderParam {
    /// Parse a parameter payload of type `kind` at `at`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize, kind: u8) -> Result<ShaderParam, Error> {
        match kind {
            0 => {
                // Texture: vtable + unknowns, name pointer partway in.
                let name_off = res.sys_ptr(at + 20)?;
                let name = name_off
                    .map(|o| res.cstr(o))
                    .transpose()?
                    .unwrap_or_default();
                Ok(ShaderParam::Texture { name })
            }
            1 => {
                let mut c = Cursor::at(&res.sys, at)?;
                Ok(ShaderParam::Vector4(c.vec4()?))
            }
            4 => {
                let mut c = Cursor::at(&res.sys, at)?;
                Ok(ShaderParam::Matrix4x4(c.mat4()?))
            }
            8 => {
                let mut c = Cursor::at(&res.sys, at)?;
                Ok(ShaderParam::Matrix4x3([
                    [c.f32()?, c.f32()?, c.f32()?, c.f32()?],
                    [c.f32()?, c.f32()?, c.f32()?, c.f32()?],
                    [c.f32()?, c.f32()?, c.f32()?, c.f32()?],
                ]))
            }
            16 => {
                let mut c = Cursor::at(&res.sys, at)?;
                Ok(ShaderParam::Float(c.f32()?))
            }
            x => Ok(ShaderParam::Unknown { kind: x }),
        }
    }
}

/// Read a simple u32 collection (pointer + count + size words).
fn read_u32_collection(res: &Resource, at: usize) -> Result<Vec<u32>, Error> {
    let mut c = Cursor::at(&res.sys, at)?;
    let list = res.sys_ptr(c.pos())?.unwrap_or(0);
    c.skip(4)?;
    let count = c.u16()? as usize;
    if count > 4096 {
        return Err(Error::BadCount {
            what: "u32 collection",
            value: u32::try_from(count).unwrap_or(u32::MAX),
        });
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        out.push(res.u32_sys(list + 4 * i)?);
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // parsed values are compared bit for bit on purpose
mod tests {
    use super::*;

    #[test]
    fn param_payloads_parse_from_hand_built_bytes() {
        let sys = {
            let mut b = vec![0u8; 96];
            // Vector4 at 0.
            b[0..16].copy_from_slice(&[0, 0, 128, 63, 0, 0, 0, 64, 0, 0, 64, 64, 0, 0, 128, 64]);
            // Float at 16.
            b[16..20].copy_from_slice(&2.5f32.to_le_bytes());
            // Texture at 32: name pointer at +20 -> 64, name "duff" there.
            b[52..56].copy_from_slice(&0x5000_0040u32.to_le_bytes());
            b[64..69].copy_from_slice(b"duff\0");
            b
        };
        let res = Resource {
            kind: crate::rsc5::TYPE_DRAWABLE,
            flags: 0,
            sys,
            gfx: vec![],
        };
        assert!(matches!(
            ShaderParam::parse(&res, 0, 1).unwrap(),
            ShaderParam::Vector4([1.0, 2.0, 3.0, 4.0])
        ));
        assert!(matches!(
            ShaderParam::parse(&res, 16, 16).unwrap(),
            ShaderParam::Float(x) if x == 2.5
        ));
        match ShaderParam::parse(&res, 32, 0).unwrap() {
            ShaderParam::Texture { name } => assert_eq!(name, "duff"),
            other => panic!("wrong payload: {other:?}"),
        }
        assert!(matches!(
            ShaderParam::parse(&res, 0, 99).unwrap(),
            ShaderParam::Unknown { kind: 99 }
        ));
    }
}
