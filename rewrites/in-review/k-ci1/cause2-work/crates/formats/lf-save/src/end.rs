//! The trailing end block (Games for Windows Live data).

use crate::{Error, Result};

/// End-block magic: `END` plus a NUL byte.
pub const END_MAGIC: &[u8; 4] = b"END\0";

/// End-block total length in bytes.
pub const END_LEN: usize = 0x16C;

/// Parsed end block: offset and the unknown word.
///
/// The remaining 0x124 bytes are opaque per public documentation; read them
/// with [`crate::SaveFile::end_payload`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndBlock {
    /// File offset of the `END` magic.
    pub offset: u64,
    /// Word at offset 4 of the block; documented as always 0x128.
    pub word: u32,
}

impl EndBlock {
    /// Parse the end block at the start of `bytes` (which must begin at
    /// file `offset`). Rejects short input and a bad magic.
    pub(crate) fn parse(bytes: &[u8], offset: usize) -> Result<EndBlock> {
        if bytes.len() < END_LEN {
            return Err(Error::TooShort {
                what: "end block",
                len: bytes.len(),
                need: END_LEN,
            });
        }
        if bytes[0..4] != *END_MAGIC {
            return Err(Error::BadMagic {
                what: "end block",
                found: bytes[0..4].to_vec(),
            });
        }
        let word = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        Ok(EndBlock {
            offset: offset as u64,
            word,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn end_bytes(word: u32) -> [u8; END_LEN] {
        let mut b = [0u8; END_LEN];
        b[0..4].copy_from_slice(END_MAGIC);
        b[4..8].copy_from_slice(&word.to_le_bytes());
        b
    }

    #[test]
    fn parses_hand_built_end() {
        let b = end_bytes(0x128);
        let e = EndBlock::parse(&b, 100).unwrap();
        assert_eq!(e.offset, 100);
        assert_eq!(e.word, 0x128);
    }

    #[test]
    fn rejects_short_and_bad_magic() {
        assert!(matches!(
            EndBlock::parse(&[0u8; END_LEN - 1], 0),
            Err(Error::TooShort { .. })
        ));
        let mut b = end_bytes(0x128);
        b[0] = b'X';
        assert!(matches!(
            EndBlock::parse(&b, 0),
            Err(Error::BadMagic { .. })
        ));
    }
}
