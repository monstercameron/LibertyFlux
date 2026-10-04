//! The texture dictionary: header, texture records, names, pixel access.
//!
//! [`Dictionary::parse`] takes a whole file, expands the RSC container and
//! indexes every texture. Pixel bytes stay borrowed from the graphics
//! segment; [`Dictionary::level_data`] slices them and
//! [`Dictionary::decode_rgba8`] decodes them through [`crate::decode`].

use std::io::Read;

use crate::Error;
use crate::decode::{RgbaImage, decode_to_rgba8};
use crate::format::{D3DFormat, TextureKind, level_byte_size, level_dims};
use crate::hash::{hash_title, title_of};
use crate::rsc::{self, RESOURCE_TYPE_TEXTURE, Resource};

/// Size of a texture record in bytes.
pub const RECORD_LEN: usize = 80;

/// Size of the dictionary header in bytes.
pub const HEADER_LEN: usize = 32;

/// Read helpers over a byte slice with bounds errors instead of panics.
fn u32_at(seg: &[u8], what: &'static str, off: u64) -> Result<u32, Error> {
    let off: usize = off.try_into().map_err(|_| Error::OutOfBounds {
        what,
        offset: off,
        len: seg.len(),
    })?;
    seg.get(off..off + 4)
        .map(|w| u32::from_le_bytes(w.try_into().unwrap()))
        .ok_or(Error::OutOfBounds {
            what,
            offset: off as u64,
            len: seg.len(),
        })
}

fn u16_at(seg: &[u8], what: &'static str, off: u64) -> Result<u16, Error> {
    let off: usize = off.try_into().map_err(|_| Error::OutOfBounds {
        what,
        offset: off,
        len: seg.len(),
    })?;
    seg.get(off..off + 2)
        .map(|w| u16::from_le_bytes(w.try_into().unwrap()))
        .ok_or(Error::OutOfBounds {
            what,
            offset: off as u64,
            len: seg.len(),
        })
}

fn byte_at(seg: &[u8], what: &'static str, off: u64) -> Result<u8, Error> {
    let off: usize = off.try_into().map_err(|_| Error::OutOfBounds {
        what,
        offset: off,
        len: seg.len(),
    })?;
    seg.get(off).copied().ok_or(Error::OutOfBounds {
        what,
        offset: off as u64,
        len: seg.len(),
    })
}

/// One 80-byte texture record (`grcTexturePC` in engine terms).
#[derive(Debug, Clone, PartialEq)]
pub struct TextureRecord {
    /// Vtable word; meaning unknown, kept verbatim.
    pub vtable: u32,
    /// Block-map pointer decoded to a system offset (0 in files seen).
    pub block_map: u32,
    /// First unknown word (observed 1 or 0x10000).
    pub unknown1: u32,
    /// Second unknown word (observed 0).
    pub unknown2: u32,
    /// Third unknown word (observed 0).
    pub unknown3: u32,
    /// System offset of the null-terminated name.
    pub name_offset: u32,
    /// Fourth unknown word (observed 0).
    pub unknown4: u32,
    /// Width in texels.
    pub width: u16,
    /// Height in texels.
    pub height: u16,
    /// Pixel format code.
    pub format: D3DFormat,
    /// Byte distance between two texel rows of mip level 0.
    pub stride: u16,
    /// Texture kind: flat, cube, or volume.
    pub kind: TextureKind,
    /// Mip level count; level 0 is the largest.
    pub levels: u8,
    /// Six float words (observed 1, 1, 1, 0, 0, 0); meaning unknown.
    pub floats: [f32; 6],
    /// Previous-link pointer decoded to a system offset.
    pub prev: u32,
    /// Next-link pointer decoded to a system offset (0 in files seen).
    pub next: u32,
    /// Graphics-segment offset of the mip data (levels back to back).
    pub data_offset: u32,
    /// Final unknown word (observed 0).
    pub unknown6: u32,
}

impl TextureRecord {
    /// Parse one record at a system-segment offset.
    fn parse(system: &[u8], at: u32) -> Result<Self, Error> {
        let base = u64::from(at);
        if base + RECORD_LEN as u64 > system.len() as u64 {
            return Err(Error::OutOfBounds {
                what: "texture record",
                offset: base,
                len: system.len(),
            });
        }
        let b = usize::try_from(base).map_err(|_| Error::OutOfBounds {
            what: "texture record",
            offset: base,
            len: system.len(),
        })?;
        let floats = [
            f32::from_le_bytes(system[b + 40..b + 44].try_into().unwrap()),
            f32::from_le_bytes(system[b + 44..b + 48].try_into().unwrap()),
            f32::from_le_bytes(system[b + 48..b + 52].try_into().unwrap()),
            f32::from_le_bytes(system[b + 52..b + 56].try_into().unwrap()),
            f32::from_le_bytes(system[b + 56..b + 60].try_into().unwrap()),
            f32::from_le_bytes(system[b + 60..b + 64].try_into().unwrap()),
        ];
        Ok(TextureRecord {
            vtable: u32_at(system, "texture record", base)?,
            block_map: rsc::system_offset(
                "texture block map",
                u32_at(system, "texture record", base + 4)?,
            )?,
            unknown1: u32_at(system, "texture record", base + 8)?,
            unknown2: u32_at(system, "texture record", base + 12)?,
            unknown3: u32_at(system, "texture record", base + 16)?,
            name_offset: rsc::system_offset(
                "texture name",
                u32_at(system, "texture record", base + 20)?,
            )?,
            unknown4: u32_at(system, "texture record", base + 24)?,
            width: u16_at(system, "texture record", base + 28)?,
            height: u16_at(system, "texture record", base + 30)?,
            format: D3DFormat::from_code(u32_at(system, "texture record", base + 32)?),
            stride: u16_at(system, "texture record", base + 36)?,
            kind: TextureKind::from_byte(byte_at(system, "texture record", base + 38)?),
            levels: byte_at(system, "texture record", base + 39)?,
            floats,
            prev: rsc::system_offset(
                "texture prev link",
                u32_at(system, "texture record", base + 64)?,
            )?,
            next: rsc::system_offset(
                "texture next link",
                u32_at(system, "texture record", base + 68)?,
            )?,
            data_offset: rsc::graphics_offset(
                "texture data",
                u32_at(system, "texture record", base + 72)?,
            )?,
            unknown6: u32_at(system, "texture record", base + 76)?,
        })
    }

    /// Byte offset of a mip level's data relative to the texture's data start.
    fn level_relative_offset(&self, level: u8) -> Result<u64, Error> {
        if level >= self.levels {
            return Err(Error::BadLevel {
                level,
                levels: self.levels,
            });
        }
        let mut off = 0u64;
        for l in 0..level {
            off = off
                .checked_add(level_byte_size(self.format, self.width, self.height, l)?)
                .ok_or(Error::Overflow)?;
        }
        Ok(off)
    }
}

/// One dictionary entry: hash, name, and record.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    /// Name hash from the hash table.
    pub hash: u32,
    /// Null-terminated name from the system segment.
    pub name: String,
    /// The parsed texture record.
    pub record: TextureRecord,
}

/// A parsed texture dictionary: the expanded resource plus an entry index.
#[derive(Debug, Clone)]
pub struct Dictionary {
    resource: Resource,
    entries: Vec<Entry>,
}

impl Dictionary {
    /// Parse a whole file already in memory.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let resource = Resource::parse(bytes)?;
        Self::from_resource(resource)
    }

    /// Read a whole stream, then parse it as [`Dictionary::parse`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let resource = Resource::parse_reader(reader)?;
        Self::from_resource(resource)
    }

    /// Index the textures of an already expanded resource.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    ///
    /// # Panics
    ///
    /// Never panics: header reads are length-checked and use fixed-size slices.
    pub fn from_resource(resource: Resource) -> Result<Self, Error> {
        if resource.header.resource_type != RESOURCE_TYPE_TEXTURE {
            return Err(Error::BadType {
                found: resource.header.resource_type,
            });
        }
        let system = &resource.system;
        if system.len() < HEADER_LEN {
            return Err(Error::TooShort {
                what: "dictionary header",
            });
        }
        let hash_table = rsc::system_offset(
            "hash table",
            u32::from_le_bytes(system[16..20].try_into().unwrap()),
        )?;
        let count = u16::from_le_bytes(system[20..22].try_into().unwrap());
        let texture_list = rsc::system_offset(
            "texture list",
            u32::from_le_bytes(system[24..28].try_into().unwrap()),
        )?;
        let count = usize::from(count);
        let mut entries = Vec::with_capacity(count.min(1 << 20));
        for i in 0..count {
            let hash = u32_at(system, "hash table", u64::from(hash_table) + (i as u64) * 4)?;
            let at = rsc::system_offset(
                "texture pointer",
                u32_at(
                    system,
                    "texture list",
                    u64::from(texture_list) + (i as u64) * 4,
                )?,
            )?;
            let record = TextureRecord::parse(system, at)?;
            let name = read_name(system, record.name_offset)?;
            entries.push(Entry { hash, name, record });
        }
        Ok(Dictionary { resource, entries })
    }

    /// The expanded RSC resource (both segments, header).
    #[must_use]
    pub fn resource(&self) -> &Resource {
        &self.resource
    }

    /// All entries in file order.
    #[must_use]
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// Number of textures.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True when the dictionary holds no textures.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Find a texture by name, as the engine does: hash the title and look
    /// it up, falling back to a case-insensitive linear scan.
    #[must_use]
    pub fn find(&self, name: &str) -> Option<&Entry> {
        let hash = hash_title(title_of(name));
        if let Some(entry) = self.entries.iter().find(|e| e.hash == hash) {
            return Some(entry);
        }
        self.entries
            .iter()
            .find(|e| e.name.eq_ignore_ascii_case(name))
    }

    /// Borrow the raw bytes of one mip level of an entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn level_data(&self, entry: &Entry, level: u8) -> Result<&[u8], Error> {
        let rel = entry.record.level_relative_offset(level)?;
        let size = level_byte_size(
            entry.record.format,
            entry.record.width,
            entry.record.height,
            level,
        )?;
        let start = u64::from(entry.record.data_offset)
            .checked_add(rel)
            .ok_or(Error::Overflow)?;
        let end = start.checked_add(size).ok_or(Error::Overflow)?;
        let gfx = &self.resource.graphics;
        if end > gfx.len() as u64 {
            return Err(Error::OutOfBounds {
                what: "mip data",
                offset: start,
                len: gfx.len(),
            });
        }
        let start = usize::try_from(start).map_err(|_| Error::Overflow)?;
        let end = usize::try_from(end).map_err(|_| Error::Overflow)?;
        Ok(&gfx[start..end])
    }

    /// Decode one mip level of an entry to RGBA8.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn decode_rgba8(&self, entry: &Entry, level: u8) -> Result<RgbaImage, Error> {
        let (w, h) = level_dims(entry.record.width, entry.record.height, level);
        let data = self.level_data(entry, level)?;
        let pixels = decode_to_rgba8(entry.record.format, w, h, data)?;
        Ok(RgbaImage {
            width: w,
            height: h,
            pixels,
        })
    }
}

/// Read a null-terminated UTF-8 string at a system offset.
fn read_name(system: &[u8], at: u32) -> Result<String, Error> {
    let start = at as usize;
    if start >= system.len() {
        return Err(Error::OutOfBounds {
            what: "texture name",
            offset: u64::from(at),
            len: system.len(),
        });
    }
    let end = system[start..]
        .iter()
        .position(|&b| b == 0)
        .ok_or(Error::UnterminatedName)?;
    std::str::from_utf8(&system[start..start + end])
        .map(std::string::ToString::to_string)
        .map_err(|_| Error::InvalidName)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        let seg = b"pack:/a.dds\x00trailing";
        assert_eq!(read_name(seg, 0).unwrap(), "pack:/a.dds");
        assert!(matches!(
            read_name(seg, 100),
            Err(Error::OutOfBounds { .. })
        ));
        assert!(matches!(
            read_name(b"no-nul-here", 0),
            Err(Error::UnterminatedName)
        ));
        assert!(matches!(
            read_name(b"\xff\xfe\x00", 0),
            Err(Error::InvalidName)
        ));
    }
}
