//! Skeletons: bones, hierarchy and transforms.
//!
//! A skeleton holds a bone count, several tuning words, a collection of
//! bone-id mappings, and five parallel arrays: bones (names plus
//! hierarchy links), parent indices, and three transform sets (default,
//! inverse, global), each one 4x4 matrix per bone.

use crate::Error;
use crate::cursor::Cursor;
use crate::rsc5::Resource;

/// A skeleton: the bone hierarchy of a drawable or fragment.
#[derive(Debug, Clone)]
pub struct Skeleton {
    /// Bones in file order.
    pub bones: Vec<Bone>,
    /// Parent bone index per bone (-1 for the root).
    pub parents: Vec<i32>,
    /// Default (local) transform per bone.
    pub default_pose: Vec<[[f32; 4]; 4]>,
    /// Inverse of the default transform per bone.
    pub inverse_pose: Vec<[[f32; 4]; 4]>,
    /// Global (absolute) transform per bone.
    pub global_pose: Vec<[[f32; 4]; 4]>,
    /// Bone-id mapping entries as raw (id, index) pairs.
    pub id_mappings: Vec<(u16, u16)>,
}

impl Skeleton {
    /// Parse a skeleton at a system-segment offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize) -> Result<Skeleton, Error> {
        let mut c = Cursor::at(&res.sys, at)?;
        let bones_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let parents_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let default_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let inverse_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let global_off = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let count = c.u16()? as usize;
        c.skip(2 + 4 + 4)?; // tuning words
        let id_mappings = read_id_mappings(res, c.pos())?;
        c.skip(8)?; // simple collection inline part
        c.skip(4 + 4 + 4)?; // more tuning words
        c.skip(4 + 4 + 4)?; // small sub-structure
        if count > 4096 {
            return Err(Error::BadCount {
                what: "bone",
                value: u32::try_from(count).unwrap_or(u32::MAX),
            });
        }
        let mut bones = Vec::with_capacity(count);
        if let Some(o) = bones_off {
            // Bones are fixed 224-byte records.
            for i in 0..count {
                bones.push(Bone::parse(res, o + 224 * i, o + 224 * i)?);
            }
        }
        let mut parents = Vec::with_capacity(count);
        if let Some(o) = parents_off {
            let mut pc = Cursor::at(&res.sys, o)?;
            for _ in 0..count {
                parents.push(pc.i32()?);
            }
        }
        let default_pose = read_matrices(res, default_off, count)?;
        let inverse_pose = read_matrices(res, inverse_off, count)?;
        let global_pose = read_matrices(res, global_off, count)?;
        Ok(Skeleton {
            bones,
            parents,
            default_pose,
            inverse_pose,
            global_pose,
            id_mappings,
        })
    }
}

/// One bone: name, hierarchy links and joint data.
#[derive(Debug, Clone)]
pub struct Bone {
    /// System offset of this bone record.
    pub offset: usize,
    /// Bone name.
    pub name: String,
    /// Bone index in the skeleton.
    pub index: i16,
    /// Bone id used by animations.
    pub bone_id: i16,
    /// Degrees-of-freedom flags.
    pub dofs: i16,
    /// Local position (x, y, z, w).
    pub position: [f32; 4],
    /// Local rotation as euler angles (x, y, z, w).
    pub rotation_euler: [f32; 4],
    /// Local rotation as a quaternion (x, y, z, w).
    pub rotation_quat: [f32; 4],
    /// System offset of the parent bone record (0 for none).
    pub parent: usize,
    /// System offset of the first child bone record (0 for none).
    pub first_child: usize,
    /// System offset of the next sibling bone record (0 for none).
    pub next_sibling: usize,
}

impl Bone {
    /// Parse one 224-byte bone record at `at` (`base` is the record's
    /// own offset, used to resolve self-relative links).
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize, base: usize) -> Result<Bone, Error> {
        let _ = base;
        let mut c = Cursor::at(&res.sys, at)?;
        let name_ptr = res.sys_ptr(c.pos())?;
        c.skip(4)?;
        let dofs = c.i16()?;
        c.skip(2)?; // unknown, always 8 in the reference sample
        // Hierarchy links tolerate non-pointer values (treated as none),
        // matching how existing readers handle these slots.
        let next_sibling = loose_link(res, c.pos())?;
        c.skip(4)?;
        let first_child = loose_link(res, c.pos())?;
        c.skip(4)?;
        let parent = loose_link(res, c.pos())?;
        c.skip(4)?;
        let index = c.i16()?;
        let bone_id = c.i16()?;
        c.skip(2 + 2 + 4)?; // index copy, unknown, zero
        let position = c.vec4()?;
        let rotation_euler = c.vec4()?;
        let rotation_quat = c.vec4()?;
        let name = name_ptr
            .map(|o| res.cstr(o))
            .transpose()?
            .unwrap_or_default();
        Ok(Bone {
            offset: at,
            name,
            index,
            bone_id,
            dofs,
            position,
            rotation_euler,
            rotation_quat,
            parent,
            first_child,
            next_sibling,
        })
    }
}

/// Read a hierarchy link: null or a system offset, with any other
/// value treated as none.
fn loose_link(res: &Resource, at: usize) -> Result<usize, Error> {
    let v = res.u32_sys(at)?;
    if v == 0 || v >> 28 != 5 {
        return Ok(0);
    }
    Ok((v & 0x0FFF_FFFF) as usize)
}

/// Read one 4x4 matrix per bone from `off`.
fn read_matrices(
    res: &Resource,
    off: Option<usize>,
    count: usize,
) -> Result<Vec<[[f32; 4]; 4]>, Error> {
    let mut out = Vec::with_capacity(count);
    if let Some(o) = off {
        let mut c = Cursor::at(&res.sys, o)?;
        for _ in 0..count {
            out.push(c.mat4()?);
        }
    }
    Ok(out)
}

/// Read the bone-id mapping collection as raw pairs.
fn read_id_mappings(res: &Resource, at: usize) -> Result<Vec<(u16, u16)>, Error> {
    let mut c = Cursor::at(&res.sys, at)?;
    let list = res.sys_ptr(c.pos())?.unwrap_or(0);
    c.skip(4)?;
    let count = c.u16()? as usize;
    if count > 4096 {
        return Err(Error::BadCount {
            what: "bone-id mapping",
            value: u32::try_from(count).unwrap_or(u32::MAX),
        });
    }
    let mut out = Vec::with_capacity(count);
    let mut lc = Cursor::at(&res.sys, list)?;
    for _ in 0..count {
        out.push((lc.u16()?, lc.u16()?));
    }
    Ok(out)
}
