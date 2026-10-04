//! The RSC5 resource container: header, segment sizes, zlib payload.
//!
//! The container is shared by every binary engine object (textures, models,
//! web pages, effects); this module parses only the framing. Texture content
//! lives in [`crate::texture`].

use std::io::Read;

use crate::Error;

/// Magic word of a little-endian RSC version 5 file, read as u32.
pub const MAGIC: u32 = 0x0543_5352;

/// Codec field for deflate. These two bytes are also the zlib header
/// (`78 DA`) of the payload, so decompression starts at file offset 12.
pub const CODEC_DEFLATE: u16 = 0xDA78;

/// Resource type word for texture dictionaries.
pub const RESOURCE_TYPE_TEXTURE: u32 = 8;

/// Marker nibble (top four bits) of a system-segment pointer.
pub const MARKER_SYSTEM: u32 = 5;

/// Marker nibble of a graphics-segment pointer.
pub const MARKER_GRAPHICS: u32 = 6;

/// Largest single segment accepted, in bytes (1 GiB). Real files are a few
/// megabytes at most; the cap only stops corrupt flags words from asking for
/// absurd allocations.
pub const MAX_SEGMENT: u64 = 1024 * 1024 * 1024;

/// File header: magic, resource type, flags, codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// Magic word; must equal [`MAGIC`].
    pub magic: u32,
    /// Resource type; texture dictionaries use [`RESOURCE_TYPE_TEXTURE`].
    pub resource_type: u32,
    /// Flags word packing both expanded segment sizes.
    pub flags: u32,
    /// Codec field; must equal [`CODEC_DEFLATE`].
    pub codec: u16,
}

/// Expanded size of the system segment encoded in a flags word.
pub fn system_size(flags: u32) -> u64 {
    let base = u64::from(flags & 0x7FF);
    let shift = ((flags >> 11) & 0xF) + 8;
    base << shift
}

/// Expanded size of the graphics segment encoded in a flags word.
pub fn graphics_size(flags: u32) -> u64 {
    let base = u64::from((flags >> 15) & 0x7FF);
    let shift = ((flags >> 26) & 0xF) + 8;
    base << shift
}

/// Decode a system-segment pointer: zero stays zero, otherwise the marker
/// nibble must be [`MARKER_SYSTEM`] and the low 28 bits are the offset.
pub fn system_offset(what: &'static str, raw: u32) -> Result<u32, Error> {
    if raw == 0 {
        return Ok(0);
    }
    if raw >> 28 != MARKER_SYSTEM {
        return Err(Error::BadOffset { what, value: raw });
    }
    Ok(raw & 0x0FFF_FFFF)
}

/// Decode a graphics-segment pointer: zero stays zero, otherwise the marker
/// nibble must be [`MARKER_GRAPHICS`] and the low 28 bits are the offset.
pub fn graphics_offset(what: &'static str, raw: u32) -> Result<u32, Error> {
    if raw == 0 {
        return Ok(0);
    }
    if raw >> 28 != MARKER_GRAPHICS {
        return Err(Error::BadOffset { what, value: raw });
    }
    Ok(raw & 0x0FFF_FFFF)
}

/// An expanded resource: header plus both segments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    /// The parsed file header.
    pub header: Header,
    /// Structures: dictionary, records, names.
    pub system: Vec<u8>,
    /// Raw texel data.
    pub graphics: Vec<u8>,
}

impl Resource {
    /// Parse a whole file already in memory.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    ///
    /// # Panics
    ///
    /// Never panics: header reads are length-checked and use fixed-size slices.
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 14 {
            return Err(Error::TooShort { what: "RSC header" });
        }
        let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if magic != MAGIC {
            return Err(Error::BadMagic { found: magic });
        }
        let header = Header {
            magic,
            resource_type: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            flags: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            codec: u16::from_le_bytes(bytes[12..14].try_into().unwrap()),
        };
        if header.codec != CODEC_DEFLATE {
            return Err(Error::BadCodec {
                found: header.codec,
            });
        }
        let sys_len = system_size(header.flags);
        let gfx_len = graphics_size(header.flags);
        if sys_len > MAX_SEGMENT {
            return Err(Error::SegmentTooLarge {
                what: "system",
                size: sys_len,
            });
        }
        if gfx_len > MAX_SEGMENT {
            return Err(Error::SegmentTooLarge {
                what: "graphics",
                size: gfx_len,
            });
        }
        let total = sys_len + gfx_len;
        // The codec field doubles as the zlib header, so the stream starts
        // at offset 12, not 14.
        let decoder = flate2::read::ZlibDecoder::new(&bytes[12..]);
        let mut expanded = Vec::new();
        decoder
            .take(total.saturating_add(1))
            .read_to_end(&mut expanded)
            .map_err(|e| Error::Inflate(e.to_string()))?;
        if expanded.len() as u64 != total {
            return Err(Error::SizeMismatch {
                expected: total,
                got: expanded.len() as u64,
            });
        }
        let sys_len = usize::try_from(sys_len).map_err(|_| Error::SegmentTooLarge {
            what: "system",
            size: sys_len,
        })?;
        let graphics = expanded.split_off(sys_len);
        Ok(Resource {
            header,
            system: expanded,
            graphics,
        })
    }

    /// Read a whole stream, then parse it as [`Resource::parse`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Self::parse(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Build a minimal valid container in memory: no game bytes involved.
    fn make_container(sys: &[u8], gfx: &[u8]) -> Vec<u8> {
        // Pad segments up to a multiple of 256 so the flags word can
        // represent them exactly (shift field 0, base = len / 256).
        fn pad(v: &[u8]) -> Vec<u8> {
            let mut o = v.to_vec();
            while !o.len().is_multiple_of(256) {
                o.push(0);
            }
            o
        }
        let sys = pad(sys);
        let gfx = pad(gfx);
        let flags = ((sys.len() / 256) as u32) | (((gfx.len() / 256) as u32) << 15);
        // Best compression, so the zlib header is 78 DA like real files.
        let mut enc = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        enc.write_all(&sys).unwrap();
        enc.write_all(&gfx).unwrap();
        let payload = enc.finish().unwrap();
        assert_eq!(&payload[0..2], &[0x78, 0xDA]);
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC.to_le_bytes());
        out.extend_from_slice(&RESOURCE_TYPE_TEXTURE.to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
        // The codec field overlaps the zlib header: the full payload,
        // starting with 78 DA, follows the 12-byte header.
        out.extend_from_slice(&payload);
        out
    }

    #[test]
    fn round_trip() {
        let sys = vec![1u8; 100];
        let gfx = vec![2u8; 300];
        let file = make_container(&sys, &gfx);
        let res = Resource::parse(&file).unwrap();
        assert_eq!(res.header.resource_type, RESOURCE_TYPE_TEXTURE);
        assert_eq!(&res.system[..100], &sys[..]);
        assert_eq!(&res.graphics[..300], &gfx[..]);
        assert!(res.system.len().is_multiple_of(256));
    }

    #[test]
    fn rejects_bad_magic() {
        let mut file = make_container(&[0u8; 10], &[0u8; 10]);
        file[0] = 0x00;
        assert!(matches!(
            Resource::parse(&file),
            Err(Error::BadMagic { .. })
        ));
    }

    #[test]
    fn rejects_truncated() {
        assert!(matches!(
            Resource::parse(&[0u8; 5]),
            Err(Error::TooShort { .. })
        ));
    }

    #[test]
    fn offset_markers() {
        assert_eq!(system_offset("t", 0).unwrap(), 0);
        assert_eq!(system_offset("t", 0x5000_1234).unwrap(), 0x1234);
        assert!(system_offset("t", 0x6000_1234).is_err());
        assert_eq!(graphics_offset("t", 0x6000_00FF).unwrap(), 0xFF);
        assert!(graphics_offset("t", 0x5000_00FF).is_err());
    }

    #[test]
    fn flag_math() {
        // base 1, shift 0 -> 256 bytes each side.
        assert_eq!(system_size(1), 256);
        assert_eq!(graphics_size(1 << 15), 256);
        // base 3, shift 2 on the system side -> 3 * 2^10.
        assert_eq!(system_size(3 | (2 << 11)), 3 * 1024);
    }
}
