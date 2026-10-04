//! The 0x110-byte file header.

use crate::{Error, Result};

/// File header length in bytes.
pub const HEADER_LEN: usize = 0x110;

/// Header magic at offset 0x0C: ASCII "SAVE".
pub const SAVE_MAGIC: &[u8; 4] = b"SAVE";

/// Save game version recorded in the game's own data files for this build.
///
/// This is an observed value, not a requirement: older saves carry older
/// versions and the parser accepts any version.
pub const VERSION_CE_12059: u32 = 57;

/// Parsed save game file header (bytes 0x00-0x10F).
#[derive(Debug, Clone)]
pub struct Header {
    /// Save game version (offset 0x00).
    pub version: u32,
    /// Save game size in bytes, as stored (offset 0x04).
    pub file_size: u32,
    /// Third header word (offset 0x08); documented as a global variables
    /// size but never confirmed.
    pub globals_size: u32,
    /// Last mission name as stored: 128 UTF-16LE code units (offset 0x10).
    /// Use [`Header::mission`] for the decoded text.
    pub mission_raw: [u16; 128],
}

impl Header {
    /// Parse the header at the start of `bytes` using explicit
    /// little-endian reads. Rejects short input and a bad `SAVE` magic.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<Header> {
        if bytes.len() < HEADER_LEN {
            return Err(Error::TooShort {
                what: "file header",
                len: bytes.len(),
                need: HEADER_LEN,
            });
        }
        if bytes[0x0C..0x0C + 4] != *SAVE_MAGIC {
            return Err(Error::BadMagic {
                what: "file header",
                found: bytes[0x0C..0x0C + 4].to_vec(),
            });
        }
        let word =
            |o: usize| u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]]);
        let mut mission_raw = [0u16; 128];
        for (i, slot) in mission_raw.iter_mut().enumerate() {
            let o = 0x10 + i * 2;
            *slot = u16::from_le_bytes([bytes[o], bytes[o + 1]]);
        }
        Ok(Header {
            version: word(0x00),
            file_size: word(0x04),
            globals_size: word(0x08),
            mission_raw,
        })
    }

    /// Decode the stored mission name as UTF-16LE, lossy.
    ///
    /// Trailing NUL padding is stripped. Unpaired surrogates become the
    /// replacement character rather than failing.
    #[must_use]
    pub fn mission(&self) -> String {
        let end = self
            .mission_raw
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.mission_raw.len());
        String::from_utf16_lossy(&self.mission_raw[..end])
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn header_bytes(version: u32, size: u32, mission: &str) -> [u8; HEADER_LEN] {
        let mut b = [0u8; HEADER_LEN];
        b[0x00..0x04].copy_from_slice(&version.to_le_bytes());
        b[0x04..0x08].copy_from_slice(&size.to_le_bytes());
        b[0x0C..0x10].copy_from_slice(SAVE_MAGIC);
        let units: Vec<u16> = mission.encode_utf16().take(128).collect();
        for (i, u) in units.iter().enumerate() {
            b[0x10 + i * 2..0x10 + i * 2 + 2].copy_from_slice(&u.to_le_bytes());
        }
        b
    }

    #[test]
    fn parses_hand_built_header() {
        let b = header_bytes(57, 1234, "Test Mission");
        let h = Header::parse(&b).unwrap();
        assert_eq!(h.version, 57);
        assert_eq!(h.file_size, 1234);
        assert_eq!(h.mission(), "Test Mission");
    }

    #[test]
    fn rejects_short_and_bad_magic() {
        assert!(matches!(
            Header::parse(&[0u8; 0x10F]),
            Err(Error::TooShort { .. })
        ));
        let mut b = header_bytes(57, 0, "");
        b[0x0C] = b'X';
        assert!(matches!(Header::parse(&b), Err(Error::BadMagic { .. })));
    }

    #[test]
    fn mission_decoding_is_lossy() {
        let mut b = header_bytes(57, 0, "");
        // Lone high surrogate at the start, then NUL.
        b[0x10..0x12].copy_from_slice(&0xD800u16.to_le_bytes());
        let h = Header::parse(&b).unwrap();
        assert_eq!(h.mission(), "\u{FFFD}");
    }
}
