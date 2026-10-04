//! Minimal private RSC5 container reader.
//!
//! The shared container reader is being written by the `fmt-rsc5` lane;
//! this module carries only what the model reader needs: header parsing,
//! zlib inflation, segment splitting and tagged-pointer resolution.
//!
//! Layout of the 12-byte header, all little-endian: magic word
//! (`RSC` plus version byte 5), resource type id, then a flags word that
//! encodes both inflated segment sizes. Each size is an 11-bit mantissa
//! shifted by a 4-bit exponent plus 8: the low half of the flags word is
//! the system segment, the high half the graphics segment.
//!
//! The bytes from offset 12 to end of file are one zlib stream. Inflated,
//! the system segment comes first, then the graphics segment.

use crate::Error;
use flate2::read::ZlibDecoder;
use std::io::Read;

/// Magic word of an RSC5 resource (`RSC\x05` read little-endian).
pub const MAGIC: u32 = 0x0543_5352;

/// Resource type id shared by drawables and drawable dictionaries.
pub const TYPE_DRAWABLE: u32 = 0x6e;
/// Resource type id of fragments.
pub const TYPE_FRAGMENT: u32 = 0x70;

/// Segment marker nibble for system-segment pointers.
const SYS_MARKER: u32 = 5;
/// Segment marker nibble for graphics-segment pointers.
const GFX_MARKER: u32 = 6;

/// An opened resource: the inflated system and graphics segments.
#[derive(Debug, Clone)]
pub struct Resource {
    /// Resource type id from the header.
    pub kind: u32,
    /// Raw flags word from the header.
    pub flags: u32,
    /// Inflated system segment (structures, pointers, names).
    pub sys: Vec<u8>,
    /// Inflated graphics segment (vertex and index bytes).
    pub gfx: Vec<u8>,
}

impl Resource {
    /// Open a resource from a whole file image.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    ///
    /// # Panics
    ///
    /// Never panics: header reads are length-checked and use fixed-size slices.
    pub fn open(data: &[u8]) -> Result<Resource, Error> {
        if data.len() < 12 {
            return Err(Error::Truncated {
                offset: 12,
                len: data.len(),
            });
        }
        let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
        if magic != MAGIC {
            return Err(Error::BadMagic { found: magic });
        }
        let kind = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let flags = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let sys_len = seg_len(flags, 0);
        let gfx_len = seg_len(flags, 15);
        let total = sys_len.saturating_add(gfx_len);
        if total > MAX_TOTAL {
            return Err(Error::BadCount {
                what: "inflated resource",
                value: u32::try_from(total).unwrap_or(u32::MAX),
            });
        }
        let sys_len = usize::try_from(sys_len).map_err(|_| Error::BadCount {
            what: "system segment",
            value: u32::try_from(sys_len).unwrap_or(u32::MAX),
        })?;
        let gfx_len = usize::try_from(gfx_len).map_err(|_| Error::BadCount {
            what: "graphics segment",
            value: u32::try_from(gfx_len).unwrap_or(u32::MAX),
        })?;
        let total = sys_len.saturating_add(gfx_len);
        let mut raw = Vec::new();
        ZlibDecoder::new(&data[12..])
            .take(total.saturating_add(1) as u64)
            .read_to_end(&mut raw)
            .map_err(|e| Error::Decompress(e.to_string()))?;
        if raw.len() < total {
            return Err(Error::Truncated {
                offset: total,
                len: raw.len(),
            });
        }
        let gfx = raw[sys_len..sys_len + gfx_len].to_vec();
        raw.truncate(sys_len);
        Ok(Resource {
            kind,
            flags,
            sys: raw,
            gfx,
        })
    }

    /// Resolve a system-segment pointer word into a byte offset.
    ///
    /// A zero word resolves to `None` (null). Any nonzero word must carry
    /// the system marker in its top nibble.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn sys_ptr(&self, at: usize) -> Result<Option<usize>, Error> {
        let v = self.u32_sys(at)?;
        if v == 0 {
            return Ok(None);
        }
        if v >> 28 != SYS_MARKER {
            return Err(Error::BadPointer {
                offset: at,
                value: v,
            });
        }
        Ok(Some((v & 0x0FFF_FFFF) as usize))
    }

    /// Resolve a graphics-segment pointer word into a byte offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn gfx_ptr(&self, at: usize) -> Result<Option<usize>, Error> {
        let v = self.u32_sys(at)?;
        if v == 0 {
            return Ok(None);
        }
        if v >> 28 != GFX_MARKER {
            return Err(Error::BadPointer {
                offset: at,
                value: v,
            });
        }
        Ok(Some((v & 0x0FFF_FFFF) as usize))
    }

    /// Read a little-endian u32 from the system segment.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    ///
    /// # Panics
    ///
    /// Never panics: the slice is exactly four bytes, so the conversion cannot fail.
    pub fn u32_sys(&self, at: usize) -> Result<u32, Error> {
        let end = at.checked_add(4).ok_or(Error::Truncated {
            offset: at,
            len: self.sys.len(),
        })?;
        self.sys
            .get(at..end)
            .ok_or(Error::Truncated {
                offset: end,
                len: self.sys.len(),
            })
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
    }

    /// Read a null-terminated string from the system segment.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn cstr(&self, at: usize) -> Result<String, Error> {
        let tail = self.sys.get(at..).ok_or(Error::Truncated {
            offset: at,
            len: self.sys.len(),
        })?;
        let end = tail
            .iter()
            .position(|&b| b == 0)
            .ok_or(Error::BadString { offset: at })?;
        std::str::from_utf8(&tail[..end])
            .map(std::string::ToString::to_string)
            .map_err(|_| Error::BadString { offset: at })
    }
}

/// Largest total inflated size accepted; bounds hostile size flags.
const MAX_TOTAL: u64 = 512 * 1024 * 1024;

/// Decode one segment size from the flags word.
///
/// `shift` is 0 for the system half, 15 for the graphics half.
fn seg_len(flags: u32, shift: u32) -> u64 {
    let mantissa = u64::from((flags >> shift) & 0x7FF);
    let exp = ((flags >> (shift + 11)) & 0xF) + 8;
    mantissa << exp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seg_sizes_match_documented_formula() {
        // Hand-built flags: sys mantissa 3, exp field 0 -> 3 << 8.
        assert_eq!(seg_len(0x0000_0003, 0), 3u64 << 8);
        // Gfx half: mantissa 1 at bit 15, exp field 2 -> 1 << 10.
        assert_eq!(seg_len(0x0800_8000, 15), 1u64 << 10);
    }

    #[test]
    fn rejects_short_input_and_bad_magic() {
        assert!(Resource::open(&[0u8; 11]).is_err());
        let mut bad = vec![0u8; 16];
        bad[0..4].copy_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
        assert!(matches!(Resource::open(&bad), Err(Error::BadMagic { .. })));
    }

    #[test]
    fn pointer_markers_are_checked() {
        let res = Resource {
            kind: TYPE_DRAWABLE,
            flags: 0,
            sys: vec![0u8; 16],
            gfx: vec![],
        };
        assert_eq!(res.sys_ptr(0).unwrap(), None);
        let mut res = res;
        res.sys[4..8].copy_from_slice(&0x5000_0010u32.to_le_bytes());
        assert_eq!(res.sys_ptr(4).unwrap(), Some(0x10));
        res.sys[8..12].copy_from_slice(&0x6000_0010u32.to_le_bytes());
        assert!(matches!(res.sys_ptr(8), Err(Error::BadPointer { .. })));
        assert_eq!(res.gfx_ptr(8).unwrap(), Some(0x10));
    }
}
