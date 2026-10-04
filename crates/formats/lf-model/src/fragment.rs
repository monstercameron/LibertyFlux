//! Fragments: breakable compound objects (vehicles, peds).
//!
//! A fragment resource holds one main drawable plus a list of child
//! records. Verified against real vehicle files: all render geometry
//! lives in the main drawable (usually three LODs); the child records
//! carry break-off and physics data (a bone index, flags, a transform
//! node and physics pointers) and no models of their own.
//!
//! The fragment header is otherwise undocumented: only the offsets used
//! here (main drawable pointer, child count byte, child list pointer)
//! are established. Physics and vehicle-specific data past the child
//! list are left for the collision and vehicle lanes.

use crate::Error;
use crate::cursor::Cursor;
use crate::drawable::Drawable;
use crate::rsc5::Resource;

/// A fragment: main drawable plus child records.
#[derive(Debug, Clone)]
pub struct Fragment {
    /// Main drawable carrying all render geometry.
    pub drawable: Drawable,
    /// Child records in file order.
    pub children: Vec<FragChild>,
}

impl Fragment {
    /// Parse the fragment at the start of the system segment.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource) -> Result<Fragment, Error> {
        let main = res.sys_ptr(0xB4)?.ok_or(Error::BadCount {
            what: "fragment main drawable",
            value: 0,
        })?;
        let drawable = Drawable::parse_at(res, main)?;
        let count = res.sys.get(0x1F3).copied().unwrap_or(0) as usize;
        if count > 256 {
            return Err(Error::BadCount {
                what: "fragment child",
                value: u32::try_from(count).unwrap_or(u32::MAX),
            });
        }
        let list = res.sys_ptr(0xD4)?.unwrap_or(0);
        let mut children = Vec::with_capacity(count);
        for i in 0..count {
            let off = res.sys_ptr(list + 4 * i)?.ok_or(Error::BadCount {
                what: "fragment child entry",
                value: u32::try_from(i).unwrap_or(u32::MAX),
            })?;
            children.push(FragChild::parse(res, off)?);
        }
        Ok(Fragment { drawable, children })
    }
}

/// One fragment child record: break-off data for one part.
#[derive(Debug, Clone)]
pub struct FragChild {
    /// System offset of this child record.
    pub offset: usize,
    /// Index of the bone this part follows.
    pub bone_index: i16,
    /// Flags byte; meaning unknown.
    pub flags: u8,
    /// System offset of the child's transform node.
    pub node: Option<usize>,
    /// Vtable slot of the transform node, if present.
    pub node_vtable: Option<u32>,
    /// System offsets of the child's physics records.
    pub physics: Vec<usize>,
}

impl FragChild {
    /// Parse one child record at a system-segment offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource, at: usize) -> Result<FragChild, Error> {
        let mut c = Cursor::at(&res.sys, at)?;
        c.skip(0x0C)?;
        let flags = c.u8()?;
        c.skip(1)?;
        let bone_index = c.i16()?;
        let node = res.sys_ptr(at + 0x90)?;
        let node_vtable = node.map(|o| res.u32_sys(o)).transpose()?;
        // Four physics pointers follow the node pointer.
        let mut physics = Vec::new();
        for i in 0..4 {
            if let Some(o) = res.sys_ptr(at + 0x98 + 4 * i)? {
                physics.push(o);
            }
        }
        Ok(FragChild {
            offset: at,
            bone_index,
            flags,
            node,
            node_vtable,
            physics,
        })
    }
}
