//! The 14-byte RSC5 file header and the flags-word size formula.

use crate::{Error, Result};

/// Magic word of a little-endian PC resource: ASCII "RSC" plus byte 0x05.
pub const MAGIC: u32 = 0x0543_5352;
/// The same bytes read big-endian (console files). Detected, not supported.
const MAGIC_BIG_ENDIAN: u32 = 0x5253_4305;
/// Largest payload this crate will inflate (256 MiB; the largest real loose
/// file is about 11 MiB on disk and inflates to far less than this).
pub const MAX_PAYLOAD: u64 = 256 * 1024 * 1024;

/// Asset type stored in a resource (header word at offset 4).
///
/// The id differs per asset type. Only ids observed in the game's loose files
/// or named by public documentation get constants; anything else is still
/// representable via [`ResourceKind::from_raw`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceKind(u32);

impl ResourceKind {
    /// Generic resource (in-game web pages and similar).
    pub const GENERIC: ResourceKind = ResourceKind(0x01);
    /// Console texture dictionary (Xbox 360).
    pub const TEXTURE_XBOX: ResourceKind = ResourceKind(0x07);
    /// Texture dictionary (PC).
    pub const TEXTURE: ResourceKind = ResourceKind(0x08);
    /// Effects package, second variant.
    pub const PARTICLES2: ResourceKind = ResourceKind(0x1B);
    /// Collision bounds.
    pub const BOUNDS: ResourceKind = ResourceKind(0x20);
    /// Effects package.
    pub const PARTICLES: ResourceKind = ResourceKind(0x24);
    /// Console drawable (Xbox 360).
    pub const MODEL_XBOX: ResourceKind = ResourceKind(0x6D);
    /// Drawable / drawable dictionary (PC).
    pub const MODEL: ResourceKind = ResourceKind(0x6E);
    /// Fragment: breakable compound object (PC).
    pub const MODEL_FRAG: ResourceKind = ResourceKind(0x70);

    /// Wrap a raw type id, known or not.
    #[must_use]
    pub const fn from_raw(id: u32) -> ResourceKind {
        ResourceKind(id)
    }

    /// The raw type id.
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Short human name for known ids, `None` otherwise.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        match self.0 {
            0x01 => Some("generic"),
            0x07 => Some("texture-xbox"),
            0x08 => Some("texture"),
            0x1B => Some("particles2"),
            0x20 => Some("bounds"),
            0x24 => Some("particles"),
            0x6D => Some("model-xbox"),
            0x6E => Some("model"),
            0x70 => Some("model-frag"),
            _ => None,
        }
    }
}

impl std::fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name() {
            Some(n) => write!(f, "{n} ({:#04x})", self.0),
            None => write!(f, "unknown ({:#04x})", self.0),
        }
    }
}

/// Compression codec id (u16 at offset 12, overlapping the stream header).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Codec {
    /// zlib/deflate (`78 DA`): every PC file seen.
    Deflate,
    /// LZX: console builds only; detected so callers get a clear error.
    Lzx,
    /// Anything else; the raw id is preserved.
    Unknown(u16),
}

impl Codec {
    /// Codec id for zlib/deflate.
    pub const DEFLATE_ID: u16 = 0xDA78;
    /// Codec id for LZX.
    pub const LZX_ID: u16 = 0xF505;

    /// Classify a raw codec id.
    #[must_use]
    pub const fn from_raw(id: u16) -> Codec {
        match id {
            Codec::DEFLATE_ID => Codec::Deflate,
            Codec::LZX_ID => Codec::Lzx,
            other => Codec::Unknown(other),
        }
    }

    /// The raw codec id.
    #[must_use]
    pub const fn raw(self) -> u16 {
        match self {
            Codec::Deflate => Codec::DEFLATE_ID,
            Codec::Lzx => Codec::LZX_ID,
            Codec::Unknown(id) => id,
        }
    }
}

/// Parsed RSC5 file header (bytes 0-13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// Resource type id (offset 4).
    pub kind: ResourceKind,
    /// Raw flags word (offset 8); use [`Header::segment_sizes`] to decode.
    pub flags: u32,
    /// Compression codec (offset 12).
    pub codec: Codec,
}

impl Header {
    /// Header length in bytes: magic + kind + flags + codec id.
    pub const LEN: usize = 14;

    /// Parse the header at the start of `bytes` using explicit
    /// little-endian reads. Rejects short input, bad magic and big-endian
    /// files; any codec id parses (support is checked at inflate time).
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<Header> {
        if bytes.len() < Header::LEN {
            return Err(Error::TooShort { len: bytes.len() });
        }
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic == MAGIC_BIG_ENDIAN {
            return Err(Error::BigEndian);
        }
        if magic != MAGIC {
            return Err(Error::BadMagic { found: magic });
        }
        let kind =
            ResourceKind::from_raw(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]));
        let flags = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let codec = Codec::from_raw(u16::from_le_bytes([bytes[12], bytes[13]]));
        Ok(Header { kind, flags, codec })
    }

    /// Decode the flags word into `(system_bytes, graphics_bytes)`:
    /// `size = mantissa << (exponent + 8)` per segment. Checked against
    /// `usize` so hostile flags yield [`Error::TooLarge`], never wrap.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn segment_sizes(&self) -> Result<(usize, usize)> {
        let sys = decode_size(self.flags & 0x7FF, (self.flags >> 11) & 0xF)?;
        let gfx = decode_size((self.flags >> 15) & 0x7FF, (self.flags >> 26) & 0xF)?;
        Ok((sys, gfx))
    }

    /// Total inflated payload size (`system + graphics`), checked.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn total_size(&self) -> Result<usize> {
        let (sys, gfx) = self.segment_sizes()?;
        sys.checked_add(gfx).ok_or(Error::TooLarge {
            size: u64::from(u32::MAX) * 2,
        })
    }
}

/// One mantissa/exponent pair of the flags word, in units of 256 bytes.
fn decode_size(mantissa: u32, exponent: u32) -> Result<usize> {
    let shift = exponent + 8;
    let size = u64::from(mantissa) << shift;
    if size > MAX_PAYLOAD {
        return Err(Error::TooLarge { size });
    }
    usize::try_from(size).map_err(|_| Error::TooLarge { size })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hand_built_header() {
        // magic "RSC\x05", kind 8, flags built from (sys_a=1, sys_b=4,
        // gfx_a=3, gfx_b=0) plus top bit, codec 0xDA78.
        let flags: u32 = 1 | (4 << 11) | (3 << 15) | (1 << 31);
        let mut bytes = [0u8; 14];
        bytes[0..4].copy_from_slice(&MAGIC.to_le_bytes());
        bytes[4..8].copy_from_slice(&8u32.to_le_bytes());
        bytes[8..12].copy_from_slice(&flags.to_le_bytes());
        bytes[12..14].copy_from_slice(&Codec::DEFLATE_ID.to_le_bytes());
        let h = Header::parse(&bytes).unwrap();
        assert_eq!(h.kind, ResourceKind::TEXTURE);
        assert_eq!(h.codec, Codec::Deflate);
        assert_eq!(h.segment_sizes().unwrap(), (1 << 12, 3 << 8));
        assert_eq!(h.total_size().unwrap(), (1 << 12) + (3 << 8));
    }

    #[test]
    fn rejects_short_bad_and_big_endian() {
        assert!(matches!(
            Header::parse(&[0u8; 13]),
            Err(Error::TooShort { len: 13 })
        ));
        let mut bad = [0u8; 14];
        bad[0..4].copy_from_slice(&0xdead_beefu32.to_le_bytes());
        assert!(matches!(Header::parse(&bad), Err(Error::BadMagic { .. })));
        let mut be = [0u8; 14];
        be[0..4].copy_from_slice(&MAGIC_BIG_ENDIAN.to_le_bytes());
        assert!(matches!(Header::parse(&be), Err(Error::BigEndian)));
    }

    #[test]
    fn known_codec_ids() {
        assert_eq!(Codec::from_raw(0xDA78), Codec::Deflate);
        assert_eq!(Codec::from_raw(0xF505), Codec::Lzx);
        assert_eq!(Codec::from_raw(0x1234), Codec::Unknown(0x1234));
    }

    #[test]
    fn hostile_flags_do_not_overflow() {
        let h = Header {
            kind: ResourceKind::GENERIC,
            flags: 0xFFFF_FFFF,
            codec: Codec::Deflate,
        };
        assert!(matches!(h.segment_sizes(), Err(Error::TooLarge { .. })));
    }

    #[test]
    fn kind_names() {
        assert_eq!(ResourceKind::TEXTURE.name(), Some("texture"));
        assert_eq!(ResourceKind::from_raw(0x99).name(), None);
    }
}
