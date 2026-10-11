//! Minimal private reader for the resource container.
//!
//! This covers exactly what collision parsing needs: header, zlib inflate,
//! segment split, and system-pointer resolution. It is intentionally not
//! public: the project's shared container crate (lane `fmt-rsc5`) will take
//! over once integrated, at which point this module goes away.

use crate::error::Error;
use std::io::Read as _;

/// Magic word of a version-5 resource ("RSC\x05" as little-endian u32).
const MAGIC: u32 = 0x0543_5352;

/// Codec id whose little-endian bytes are the zlib header `78 DA`.
const CODEC_DEFLATE: u16 = 0xDA78;

/// Largest single segment this reader will allocate (256 MiB). The biggest
/// shipped collision segment is a few megabytes; anything near this cap is
/// garbage input with a hostile flags word.
const MAX_SEGMENT: u64 = 256 * 1024 * 1024;

/// Tag nibble of a system-segment pointer.
const TAG_SYSTEM: u32 = 5;

/// An opened container: the two inflated segments plus the resource type id.
pub(crate) struct Resource {
    /// Resource type id from the header (32 for bounds).
    pub(crate) kind: u32,
    /// Inflated system segment; every collision structure lives here.
    pub(crate) system: Vec<u8>,
}

/// Decode one packed segment size: 11-bit base, 4-bit shift.
fn unpack_size(flags: u32, base_shift: u32, exp_shift: u32) -> u64 {
    let base = u64::from((flags >> base_shift) & 0x7FF);
    let exp = u64::from((flags >> exp_shift) & 0xF) + 8;
    base << exp
}

/// Open and inflate a resource file.
pub(crate) fn open(data: &[u8]) -> Result<Resource, Error> {
    if data.len() < 14 {
        return Err(Error::Truncated {
            offset: 0,
            needed: 14,
            available: data.len(),
        });
    }
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    if magic != MAGIC {
        return Err(Error::BadMagic { found: Some(magic) });
    }
    let kind = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    let flags = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    let codec = u16::from_le_bytes([data[12], data[13]]);
    if codec != CODEC_DEFLATE {
        return Err(Error::UnsupportedCodec { codec });
    }
    let system_size = unpack_size(flags, 0, 11);
    let graphics_size = unpack_size(flags, 15, 26);
    if system_size > MAX_SEGMENT || graphics_size > MAX_SEGMENT {
        return Err(Error::BadSegmentSize {
            size: system_size.max(graphics_size),
        });
    }
    // The codec word is the zlib header, so the stream starts at offset 12.
    let mut decoder = flate2::read::ZlibDecoder::new(&data[12..]);
    let total =
        usize::try_from(system_size + graphics_size).map_err(|_| Error::BadSegmentSize {
            size: system_size + graphics_size,
        })?;
    let mut inflated = Vec::new();
    // Cap the bytes read so a hostile stream cannot balloon memory: the
    // header sizes bound the payload, plus one byte to detect overrun.
    let cap = total.saturating_add(1);
    let mut limited = (&mut decoder).take(cap as u64);
    limited
        .read_to_end(&mut inflated)
        .map_err(|e| Error::Decompress {
            message: e.to_string(),
        })?;
    if inflated.len() < total {
        return Err(Error::ShortPayload {
            expected: total,
            actual: inflated.len(),
        });
    }
    inflated.truncate(total);
    let system_len =
        usize::try_from(system_size).map_err(|_| Error::BadSegmentSize { size: system_size })?;
    let system = inflated[..system_len].to_vec();
    Ok(Resource { kind, system })
}

/// Resolve one stored system pointer word to a segment offset.
///
/// A zero word is null (`Ok(None)`). A word with a tag other than 5 is an
/// error. Range checking against the segment is left to the caller, which
/// knows how many bytes it wants there.
pub(crate) fn resolve(word: u32, offset: usize) -> Result<Option<usize>, Error> {
    if word == 0 {
        return Ok(None);
    }
    if word >> 28 != TAG_SYSTEM {
        return Err(Error::BadPointerTag {
            offset,
            value: word,
        });
    }
    Ok(Some((word & 0x0FFF_FFFF) as usize))
}

/// Little-endian cursor over a byte slice with bounds-checked reads.
///
/// All reads return [`Error::Truncated`] instead of panicking at the end of
/// the buffer.
#[derive(Clone)]
pub(crate) struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    /// New cursor at the start of `buf`.
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Cursor { buf, pos: 0 }
    }

    /// New cursor at byte `offset` of `buf`, failing if out of range.
    pub(crate) fn at(buf: &'a [u8], offset: usize) -> Result<Self, Error> {
        if offset > buf.len() {
            return Err(Error::Truncated {
                offset,
                needed: 1,
                available: buf.len(),
            });
        }
        Ok(Cursor { buf, pos: offset })
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self.pos.checked_add(n).ok_or(Error::Truncated {
            offset: self.pos,
            needed: n,
            available: self.buf.len(),
        })?;
        if end > self.buf.len() {
            return Err(Error::Truncated {
                offset: self.pos,
                needed: n,
                available: self.buf.len(),
            });
        }
        let out = &self.buf[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    /// Skip `n` bytes.
    pub(crate) fn skip(&mut self, n: usize) -> Result<(), Error> {
        self.take(n).map(|_| ())
    }

    /// Read one byte.
    pub(crate) fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }

    /// Read a little-endian u16.
    pub(crate) fn u16(&mut self) -> Result<u16, Error> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    /// Read a little-endian u32.
    pub(crate) fn u32(&mut self) -> Result<u32, Error> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Read a little-endian i32.
    pub(crate) fn i32(&mut self) -> Result<i32, Error> {
        let b = self.take(4)?;
        Ok(i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Read a little-endian f32.
    pub(crate) fn f32(&mut self) -> Result<f32, Error> {
        let b = self.take(4)?;
        Ok(f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Read a little-endian i16.
    pub(crate) fn i16(&mut self) -> Result<i16, Error> {
        let b = self.take(2)?;
        Ok(i16::from_le_bytes([b[0], b[1]]))
    }

    /// Read `n` raw bytes.
    pub(crate) fn bytes(&mut self, n: usize) -> Result<&'a [u8], Error> {
        self.take(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write as _;

    fn pack(sys: usize, gfx: usize) -> u32 {
        // Inverse of unpack_size for sizes that fit shift 0 (multiples of
        // 256 below 0x7FF * 256).
        assert!(sys.is_multiple_of(256) && sys / 256 <= 0x7FF);
        assert!(gfx.is_multiple_of(256) && gfx / 256 <= 0x7FF);
        ((sys / 256) | ((gfx / 256) << 15)) as u32
    }

    fn resource_bytes(system: &[u8]) -> Vec<u8> {
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
        enc.write_all(system).unwrap();
        let mut payload = enc.finish().unwrap();
        // The stored stream must start with the codec word 78 DA; the
        // encoder emits exactly that header for best compression, matching
        // the shipped files.
        assert_eq!(&payload[..2], &[0x78, 0xDA]);
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC.to_le_bytes());
        out.extend_from_slice(&32u32.to_le_bytes());
        out.extend_from_slice(&pack(system.len(), 0).to_le_bytes());
        // Codec word overlaps the stream: strip it from the payload copy.
        out.extend_from_slice(&payload[..2]);
        payload.drain(..2);
        out.extend_from_slice(&payload);
        // Re-check: bytes at 12.. are a complete zlib stream.
        assert_eq!(&out[12..14], &[0x78, 0xDA]);
        out
    }

    #[test]
    fn round_trip_header_and_sizes() {
        let system = vec![0xABu8; 512];
        let bytes = resource_bytes(&system);
        let res = open(&bytes).unwrap();
        assert_eq!(res.kind, 32);
        assert_eq!(res.system, system);
    }

    #[test]
    fn rejects_bad_magic() {
        let mut bytes = resource_bytes(&vec![0u8; 256]);
        bytes[0] = 0x00;
        assert!(matches!(open(&bytes), Err(Error::BadMagic { .. })));
        assert!(matches!(open(&bytes[..2]), Err(Error::Truncated { .. })));
    }

    #[test]
    fn rejects_bad_codec() {
        let mut bytes = resource_bytes(&vec![0u8; 256]);
        bytes[12] = 0x00;
        bytes[13] = 0x00;
        assert!(matches!(
            open(&bytes),
            Err(Error::UnsupportedCodec { codec: 0 })
        ));
    }

    #[test]
    fn resolve_tags() {
        assert_eq!(resolve(0, 0).unwrap(), None);
        assert_eq!(resolve(0x5000_1234, 7).unwrap(), Some(0x1234));
        assert!(matches!(
            resolve(0x6000_0000, 7),
            Err(Error::BadPointerTag { offset: 7, .. })
        ));
    }

    #[test]
    fn cursor_truncates_cleanly() {
        let buf = [1u8, 2, 3];
        let mut c = Cursor::new(&buf);
        assert_eq!(c.u16().unwrap(), 0x0201);
        assert!(matches!(c.u16(), Err(Error::Truncated { .. })));
    }
}
