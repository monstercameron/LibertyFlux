//! A decompressed resource: header plus the system and graphics segments,
//! with typed helpers to resolve pointers into them.

use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::blockmap::{BlockMap, PgBase};
use crate::header::MAX_PAYLOAD;
use crate::{Codec, Error, Header, Pointer, Result, Segment};

/// A fully parsed RSC5 resource: the header and the two decompressed
/// segments. Asset readers resolve [`Pointer`]s through [`Resource::slice`]
/// and the typed readers below.
#[derive(Debug, Clone)]
pub struct Resource {
    header: Header,
    system: Vec<u8>,
    graphics: Vec<u8>,
}

impl Resource {
    /// Parse a whole resource file from memory: header, zlib inflate, split
    /// into segments. The payload must inflate to exactly the size the flags
    /// word promises; anything else is [`Error::LengthMismatch`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<Resource> {
        let header = Header::parse(bytes)?;
        if header.codec != Codec::Deflate {
            return Err(Error::UnsupportedCodec {
                codec: header.codec.raw(),
            });
        }
        // The codec id at bytes 12-13 doubles as the zlib header, so the
        // stream starts at byte 12.
        let payload = bytes
            .get(12..)
            .ok_or(Error::TooShort { len: bytes.len() })?;
        let (sys_len, gfx_len) = header.segment_sizes()?;
        let total = header.total_size()?;
        let decoder = ZlibDecoder::new(payload);
        // Bounded: hostile flags cannot balloon the allocation.
        let max = usize::try_from(MAX_PAYLOAD).unwrap_or(usize::MAX);
        let cap = total.saturating_add(1).min(max.saturating_add(1));
        let mut inflated = Vec::new();
        decoder
            .take(cap as u64)
            .read_to_end(&mut inflated)
            .map_err(|e| Error::Decompress {
                message: e.to_string(),
            })?;
        if inflated.len() != total {
            // Distinguish truncation from a lying flags word is impossible;
            // report what we can measure.
            if inflated.len() > total {
                return Err(Error::LengthMismatch {
                    expected: total,
                    actual: inflated.len(),
                });
            }
            // A short read means the stream ended early (or the cap bit, in
            // which case the flags word already failed its own check).
            return Err(Error::LengthMismatch {
                expected: total,
                actual: inflated.len(),
            });
        }
        let graphics = inflated.split_off(sys_len);
        debug_assert_eq!(graphics.len(), gfx_len);
        Ok(Resource {
            header,
            system: inflated,
            graphics,
        })
    }

    /// Parse a resource from a stream. Reads to end of stream, then parses
    /// as [`Resource::parse`]; the stream is the whole file.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse_reader<R: Read>(mut reader: R) -> Result<Resource> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Resource::parse(&bytes)
    }

    /// The parsed file header.
    #[must_use]
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// The system (CPU) segment.
    #[must_use]
    pub fn system(&self) -> &[u8] {
        &self.system
    }

    /// The graphics (data) segment.
    #[must_use]
    pub fn graphics(&self) -> &[u8] {
        &self.graphics
    }

    /// Both segments in order: system first, then graphics.
    #[must_use]
    pub fn segments(&self) -> [(Segment, &[u8]); 2] {
        [
            (Segment::System, &self.system),
            (Segment::Graphics, &self.graphics),
        ]
    }

    /// Resolve `ptr` into the segment it tags and borrow `len` bytes there.
    /// Null, wrongly-tagged and out-of-bounds pointers are errors, never
    /// panics.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn slice(&self, ptr: Pointer, len: usize) -> Result<&[u8]> {
        let seg = ptr
            .segment()
            .ok_or(Error::BadPointer { value: ptr.raw() })?;
        let bytes = match seg {
            Segment::System => &self.system,
            Segment::Graphics => &self.graphics,
        };
        let off = ptr.offset();
        let end = off.checked_add(len).ok_or(Error::OutOfBounds {
            value: ptr.raw(),
            len,
            segment_len: bytes.len(),
        })?;
        bytes.get(off..end).ok_or(Error::OutOfBounds {
            value: ptr.raw(),
            len,
            segment_len: bytes.len(),
        })
    }

    /// Read a little-endian u16 at `ptr`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn read_u16(&self, ptr: Pointer) -> Result<u16> {
        let b = self.slice(ptr, 2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    /// Read a little-endian u32 at `ptr`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn read_u32(&self, ptr: Pointer) -> Result<u32> {
        let b = self.slice(ptr, 4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Read a tagged [`Pointer`] stored at `ptr`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn read_pointer(&self, ptr: Pointer) -> Result<Pointer> {
        self.read_u32(ptr).map(Pointer::new)
    }

    /// Read a NUL-terminated string at `ptr`, up to `max_len` bytes. The
    /// bytes must be valid UTF-8; names in resources are ASCII.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn read_cstring(&self, ptr: Pointer, max_len: usize) -> Result<&str> {
        let seg = ptr
            .segment()
            .ok_or(Error::BadPointer { value: ptr.raw() })?;
        let bytes = match seg {
            Segment::System => &self.system,
            Segment::Graphics => &self.graphics,
        };
        let off = ptr.offset();
        let avail = bytes.len().saturating_sub(off);
        let scan = avail.min(max_len);
        let window = bytes.get(off..off + scan).ok_or(Error::OutOfBounds {
            value: ptr.raw(),
            len: 1,
            segment_len: bytes.len(),
        })?;
        let end = window
            .iter()
            .position(|&b| b == 0)
            .ok_or(Error::OutOfBounds {
                value: ptr.raw(),
                len: scan,
                segment_len: bytes.len(),
            })?;
        std::str::from_utf8(&window[..end]).map_err(|_| Error::BadPointer { value: ptr.raw() })
    }

    /// Read the `pgBase` prefix (vtable slot + block-map pointer) at the
    /// start of the system segment. Only valid for asset types rooted in a
    /// paged object; generic resources carry other data there.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn pg_base(&self) -> Result<PgBase> {
        if self.system.len() < PgBase::LEN {
            return Err(Error::OutOfBounds {
                value: Pointer::new(0x5000_0000).raw(),
                len: PgBase::LEN,
                segment_len: self.system.len(),
            });
        }
        Ok(PgBase {
            vtable: u32::from_le_bytes([
                self.system[0],
                self.system[1],
                self.system[2],
                self.system[3],
            ]),
            block_map: Pointer::new(u32::from_le_bytes([
                self.system[4],
                self.system[5],
                self.system[6],
                self.system[7],
            ])),
        })
    }

    /// Read the block map via the `pgBase` slot at the system segment start.
    /// See [`BlockMap`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn block_map(&self) -> Result<BlockMap> {
        BlockMap::read(self)
    }

    /// Heuristic scan: every 4-byte aligned little-endian u32 in `segment`
    /// that is a correctly-tagged pointer landing inside its own target
    /// segment, yielded as `(offset_in_segment, pointer)`.
    ///
    /// This is a tooling aid for inspecting unknown assets, not a parser:
    /// floats, hashes and texture bytes can coincidentally look like
    /// pointers. Asset readers must follow documented fields instead.
    pub fn scan_pointers(&self, segment: Segment) -> impl Iterator<Item = (usize, Pointer)> + '_ {
        let bytes = match segment {
            Segment::System => &self.system,
            Segment::Graphics => &self.graphics,
        };
        bytes.chunks_exact(4).enumerate().filter_map(|(i, w)| {
            let ptr = Pointer::new(u32::from_le_bytes([w[0], w[1], w[2], w[3]]));
            let target = match ptr.segment()? {
                Segment::System => &self.system,
                Segment::Graphics => &self.graphics,
            };
            (ptr.offset() < target.len()).then_some((i * 4, ptr))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ResourceKind;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write;

    /// Build a complete resource file in memory: header + zlib of
    /// (system ++ graphics). All bytes are constructed here.
    fn build_file(sys: &[u8], gfx: &[u8], kind: u32) -> Vec<u8> {
        assert!(sys.len().is_multiple_of(256) && gfx.len().is_multiple_of(256));
        // sys_b = gfx_b = 0, so mantissa = len / 256.
        let flags: u32 = ((sys.len() / 256) as u32) | (((gfx.len() / 256) as u32) << 15);
        // Best compression, like the game's files (stream header 78 DA).
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
        enc.write_all(sys).unwrap();
        enc.write_all(gfx).unwrap();
        let payload = enc.finish().unwrap();
        let mut file = Vec::with_capacity(12 + payload.len());
        file.extend_from_slice(&crate::header::MAGIC.to_le_bytes());
        file.extend_from_slice(&kind.to_le_bytes());
        file.extend_from_slice(&flags.to_le_bytes());
        file.extend_from_slice(&payload);
        file
    }

    #[test]
    fn round_trips_hand_built_file() {
        let mut sys = vec![0u8; 512];
        sys[0..4].copy_from_slice(&0x1234_5678u32.to_le_bytes()); // vtable-ish
        sys[4..8].copy_from_slice(&0x5000_0100u32.to_le_bytes()); // blockmap ptr
        sys[0x100..0x104].copy_from_slice(&0u32.to_le_bytes());
        let mut gfx = vec![0u8; 256];
        gfx[0..4].copy_from_slice(&0x6000_0010u32.to_le_bytes());
        // nul-terminated name at sys+0x108
        sys[0x108..0x10e].copy_from_slice(b"hello\0");

        let file = build_file(&sys, &gfx, 8);
        let res = Resource::parse(&file).unwrap();
        assert_eq!(res.header().kind, ResourceKind::TEXTURE);
        assert_eq!(res.system(), sys.as_slice());
        assert_eq!(res.graphics(), gfx.as_slice());

        let p = Pointer::new(0x5000_0108);
        assert_eq!(res.read_cstring(p, 64).unwrap(), "hello");
        assert_eq!(
            res.read_u32(Pointer::new(0x5000_0000)).unwrap(),
            0x1234_5678
        );
        assert_eq!(
            res.read_pointer(Pointer::new(0x5000_0004)).unwrap(),
            Pointer::new(0x5000_0100)
        );
        assert_eq!(res.slice(Pointer::new(0x6000_0000), 4).unwrap(), &gfx[0..4]);

        // Null and mistagged pointers are errors.
        assert!(matches!(
            res.slice(Pointer::NULL, 1),
            Err(Error::BadPointer { .. })
        ));
        assert!(matches!(
            res.slice(Pointer::new(0x7000_0000), 1),
            Err(Error::BadPointer { .. })
        ));
        // Past the end is an error, not a panic.
        assert!(matches!(
            res.slice(Pointer::new(0x5000_01FF), 4),
            Err(Error::OutOfBounds { .. })
        ));
    }

    #[test]
    fn rejects_lzx_and_truncated_payloads() {
        let sys = vec![7u8; 256];
        let gfx = vec![9u8; 256];
        let mut file = build_file(&sys, &gfx, 1);
        // Claim LZX: codec bytes become the LZX id. (Stream bytes are
        // untouched; parse must fail before touching them.)
        file[12..14].copy_from_slice(&Codec::LZX_ID.to_le_bytes());
        assert!(matches!(
            Resource::parse(&file),
            Err(Error::UnsupportedCodec { codec: 0xF505 })
        ));

        // Truncated stream inflates short -> LengthMismatch.
        let mut file = build_file(&sys, &gfx, 1);
        file.truncate(file.len() - 4);
        assert!(matches!(
            Resource::parse(&file),
            Err(Error::LengthMismatch { .. }) | Err(Error::Decompress { .. })
        ));
    }

    #[test]
    fn scan_finds_only_plausible_pointers() {
        let mut sys = vec![0u8; 256];
        sys[0..4].copy_from_slice(&0x5000_0010u32.to_le_bytes()); // valid -> sys
        sys[8..12].copy_from_slice(&0x6000_0010u32.to_le_bytes()); // valid -> gfx
        sys[12..16].copy_from_slice(&0x7000_0010u32.to_le_bytes()); // bad tag
        sys[16..20].copy_from_slice(&0x5000_FFF0u32.to_le_bytes()); // oob
        let gfx = vec![0u8; 256];
        let file = build_file(&sys, &gfx, 1);
        let res = Resource::parse(&file).unwrap();
        let found: Vec<(usize, Pointer)> = res.scan_pointers(Segment::System).collect();
        assert_eq!(
            found,
            vec![
                (0, Pointer::new(0x5000_0010)),
                (8, Pointer::new(0x6000_0010)),
            ]
        );
    }
}
