//! Drawable dictionaries: named collections of drawables.
//!
//! A dictionary header holds a vtable slot, a block-map address, a parent
//! word (always zero in files), a usage count (always one), a collection
//! of name hashes, and a pointer collection of drawables. Entry `i` of
//! the hash collection names entry `i` of the drawable collection.

use crate::Error;
use crate::cursor::Cursor;
use crate::drawable::{Drawable, read_ptr_list};
use crate::rsc5::Resource;

/// A drawable dictionary: name hashes plus drawables.
#[derive(Debug, Clone)]
pub struct DrawableDictionary {
    /// Vtable slot from the file.
    pub vtable: u32,
    /// Usage count from the file (always one observed).
    pub usage_count: u32,
    /// Name hash per entry.
    pub hashes: Vec<u32>,
    /// Drawables in entry order.
    pub entries: Vec<Drawable>,
}

impl DrawableDictionary {
    /// Parse the dictionary at the start of the system segment.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(res: &Resource) -> Result<DrawableDictionary, Error> {
        let mut c = Cursor::at(&res.sys, 0)?;
        let vtable = c.u32()?;
        c.skip(4)?; // block-map address
        let _parent = c.u32()?;
        let usage_count = c.u32()?;
        let list = res.sys_ptr(c.pos())?.unwrap_or(0);
        c.skip(4)?;
        let hash_count = c.u16()? as usize;
        let _hash_size = c.u16()?;
        if hash_count > 4096 {
            return Err(Error::BadCount {
                what: "dictionary hash",
                value: u32::try_from(hash_count).unwrap_or(u32::MAX),
            });
        }
        let mut hashes = Vec::with_capacity(hash_count);
        for i in 0..hash_count {
            hashes.push(res.u32_sys(list + 4 * i)?);
        }
        let mut entries = Vec::new();
        for off in read_ptr_list(res, c.pos())? {
            entries.push(Drawable::parse_at(res, off)?);
        }
        Ok(DrawableDictionary {
            vtable,
            usage_count,
            hashes,
            entries,
        })
    }

    /// Number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the dictionary has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
