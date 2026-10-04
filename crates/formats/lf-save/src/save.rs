//! A parsed save game file.

use std::io::Read;

use crate::blocks::{MAX_BLOCKS, parse_at};
use crate::checksum::compute_checksum;
use crate::{Block, BlockKind, Checksum, EndBlock, Error, Header, Result};
use crate::{END_LEN, END_MAGIC, HEADER_LEN};

/// Largest file [`SaveFile::read_from`] will load (16 MiB; real saves are
/// about 2 MiB). Guards against hostile streams, not a format limit.
pub const MAX_FILE: u64 = 16 * 1024 * 1024;

/// What a block payload looks like to the container sniffers.
///
/// Save blocks are documented as raw per-system state, so `Unknown` is the
/// expected outcome; the other variants exist so a future lane that finds an
/// embedded container gets a precise answer instead of a guess. Detection
/// reuses the sibling crates rather than reimplementing their magics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payload {
    /// No known container magic at the payload start.
    Unknown,
    /// An RSC5 resource header (see `lf-resource`).
    Resource {
        /// Raw resource type id.
        kind: u32,
        /// Raw codec id.
        codec: u16,
    },
    /// An RPF archive header (see `lf-archive`).
    Rpf,
    /// An open (unencrypted) IMG archive header (see `lf-archive`).
    Img,
}

/// A parsed save game file: owned bytes plus the decoded structure.
///
/// The parser is lenient about the trailer on purpose: public documentation
/// places a checksum dword after the last data block and the end block after
/// that, but neither the order nor the presence of the two has been confirmed
/// against a real file, so any combination parses and the result reports
/// exactly what was found. Blocks, however, must frame correctly: a `BLOCK`
/// magic with an impossible or overrunning size is an error.
#[derive(Debug, Clone)]
pub struct SaveFile {
    data: Vec<u8>,
    header: Header,
    blocks: Vec<Block>,
    checksum: Option<Checksum>,
    end: Option<EndBlock>,
    trailing: usize,
}

impl SaveFile {
    /// Parse a whole save game file from memory.
    ///
    /// Reads the header, then blocks until the bytes stop framing as blocks,
    /// then the checksum dword and end block when present. Everything past
    /// the end block (or past the checksum, or past the blocks when neither
    /// trailer part is present) is kept as trailing bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<SaveFile> {
        let header = Header::parse(bytes)?;
        let mut blocks = Vec::new();
        let mut cursor = HEADER_LEN;
        while blocks.len() < MAX_BLOCKS {
            match parse_at(bytes, cursor, blocks.len())? {
                Some(block) => {
                    cursor += block.total_len as usize;
                    blocks.push(block);
                }
                None => break,
            }
        }
        if blocks.len() == MAX_BLOCKS {
            return Err(Error::TooLarge {
                what: "block count",
                size: MAX_BLOCKS as u64,
            });
        }

        // Trailer: an optional checksum dword, then an optional end block.
        // The end block announces itself with its magic; anything else in
        // the first four trailer bytes is taken as the checksum.
        let mut checksum = None;
        let rest = &bytes[cursor..];
        if !rest.starts_with(END_MAGIC) && rest.len() >= 4 {
            let stored = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
            checksum = Some(Checksum {
                offset: cursor as u64,
                stored,
            });
            cursor += 4;
        }
        let rest = &bytes[cursor..];
        let mut end = None;
        if rest.starts_with(END_MAGIC) && rest.len() >= END_LEN {
            end = Some(EndBlock::parse(rest, cursor)?);
            cursor += END_LEN;
        }
        let trailing = bytes.len() - cursor;

        Ok(SaveFile {
            data: bytes.to_vec(),
            header,
            blocks,
            checksum,
            end,
            trailing,
        })
    }

    /// Read a whole save game file from a stream, then parse it.
    ///
    /// At most [`MAX_FILE`] bytes are read; anything beyond that is an
    /// error rather than a silent truncation.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn read_from<R: Read>(reader: R) -> Result<SaveFile> {
        let mut data = Vec::new();
        reader
            .take(MAX_FILE + 1)
            .read_to_end(&mut data)
            .map_err(Error::Io)?;
        if data.len() as u64 > MAX_FILE {
            return Err(Error::TooLarge {
                what: "save file",
                size: data.len() as u64,
            });
        }
        SaveFile::parse(&data)
    }

    /// Total file length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// True when the file holds no bytes (never true for a parsed file,
    /// which always contains at least the header).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// The parsed file header.
    #[must_use]
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// The data blocks in file order.
    #[must_use]
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// The block with the given positional identity, if the file holds it.
    #[must_use]
    pub fn block(&self, kind: BlockKind) -> Option<&Block> {
        self.blocks.get(kind.index()).filter(|b| b.kind == kind)
    }

    /// Raw payload bytes of a block (the bytes after its 9-byte header).
    ///
    /// # Panics
    ///
    /// Panics when `block` did not come from this file. Blocks borrowed
    /// from [`SaveFile::blocks`] or [`SaveFile::block`] always qualify.
    #[must_use]
    pub fn payload(&self, block: &Block) -> &[u8] {
        let start =
            usize::try_from(block.offset).unwrap_or(usize::MAX) + crate::BLOCK_HEADER_LEN as usize;
        &self.data[start..start + block.payload_len()]
    }

    /// The stored checksum dword, if the trailer holds one.
    #[must_use]
    pub fn checksum(&self) -> Option<&Checksum> {
        self.checksum.as_ref()
    }

    /// Recompute the checksum over all bytes preceding the stored dword and
    /// compare. Returns `None` when the file holds no checksum.
    ///
    /// See the [`crate::checksum`] module: the summing rule itself is
    /// documented but unverified, so a mismatch is a finding, not proof of
    /// corruption.
    #[must_use]
    pub fn verify_checksum(&self) -> Option<bool> {
        self.checksum.map(|c| {
            compute_checksum(&self.data[..usize::try_from(c.offset).unwrap_or(usize::MAX)])
                == c.stored
        })
    }

    /// The end block, if the trailer holds one.
    #[must_use]
    pub fn end(&self) -> Option<&EndBlock> {
        self.end.as_ref()
    }

    /// Opaque end-block payload: the 0x124 bytes after its magic and word.
    /// Returns `None` when the file holds no end block.
    #[must_use]
    pub fn end_payload(&self) -> Option<&[u8]> {
        self.end.map(|e| {
            let start = usize::try_from(e.offset).unwrap_or(usize::MAX) + 8;
            &self.data[start..start + END_LEN - 8]
        })
    }

    /// Bytes past the parsed structure (normally empty).
    #[must_use]
    pub fn trailing_bytes(&self) -> &[u8] {
        &self.data[self.data.len() - self.trailing..]
    }

    /// Sniff a block payload for a known embedded container.
    ///
    /// Tries the RSC5 resource header first (via `lf-resource`), then the
    /// archive magics (via `lf-archive`, without a key, so only plaintext
    /// headers can match). Documented saves hold raw state in every block,
    /// so expect [`Payload::Unknown`].
    #[must_use]
    pub fn classify(&self, block: &Block) -> Payload {
        let payload = self.payload(block);
        if let Ok(header) = lf_resource::Header::parse(payload) {
            return Payload::Resource {
                kind: header.kind.raw(),
                codec: header.codec.raw(),
            };
        }
        match lf_archive::detect(payload, None) {
            Some(lf_archive::Kind::Rpf) => Payload::Rpf,
            Some(lf_archive::Kind::Img) => Payload::Img,
            Some(lf_archive::Kind::EncryptedImg) | None => Payload::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::tests::header_bytes;
    use crate::{BLOCK_MAGIC, SAVE_MAGIC};

    fn end_bytes(word: u32) -> [u8; END_LEN] {
        let mut b = [0u8; END_LEN];
        b[0..4].copy_from_slice(END_MAGIC);
        b[4..8].copy_from_slice(&word.to_le_bytes());
        b
    }

    fn block_bytes(payload: &[u8]) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(BLOCK_MAGIC);
        b.extend_from_slice(&(payload.len() as u32 + crate::BLOCK_HEADER_LEN).to_le_bytes());
        b.extend_from_slice(payload);
        b
    }

    /// Minimal hand-built save: header, two blocks, checksum, end block.
    fn full_save() -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&header_bytes(57, 0, "Mission"));
        v.extend_from_slice(&block_bytes(&[1u8; 10]));
        v.extend_from_slice(&block_bytes(&[2u8; 20]));
        let sum = compute_checksum(&v);
        v.extend_from_slice(&sum.to_le_bytes());
        v.extend_from_slice(&end_bytes(0x128));
        v
    }

    #[test]
    fn parses_full_hand_built_save() {
        let bytes = full_save();
        let save = SaveFile::parse(&bytes).unwrap();
        assert_eq!(save.header().version, 57);
        assert_eq!(save.header().mission(), "Mission");
        assert_eq!(save.blocks().len(), 2);
        assert_eq!(save.blocks()[0].kind, BlockKind::SimpleVars);
        assert_eq!(save.blocks()[1].kind, BlockKind::PlayerInfo);
        assert_eq!(save.payload(&save.blocks()[0]), &[1u8; 10]);
        assert_eq!(save.block(BlockKind::SimpleVars).unwrap().index, 0);
        assert!(save.block(BlockKind::Stats).is_none());
        assert_eq!(save.verify_checksum(), Some(true));
        assert_eq!(save.end().unwrap().word, 0x128);
        assert_eq!(save.end_payload().unwrap().len(), END_LEN - 8);
        assert!(save.trailing_bytes().is_empty());
    }

    #[test]
    fn checksum_mismatch_is_reported_not_fatal() {
        let mut bytes = full_save();
        let at = HEADER_LEN + 19 + 29;
        bytes[at] ^= 0xFF;
        let save = SaveFile::parse(&bytes).unwrap();
        assert_eq!(save.verify_checksum(), Some(false));
    }

    #[test]
    fn missing_trailer_parts_parse() {
        // Header plus blocks only.
        let mut v = Vec::new();
        v.extend_from_slice(&header_bytes(57, 0, ""));
        v.extend_from_slice(&block_bytes(&[7u8; 5]));
        let save = SaveFile::parse(&v).unwrap();
        assert_eq!(save.blocks().len(), 1);
        assert_eq!(save.checksum(), None);
        assert_eq!(save.end(), None);
        assert_eq!(save.verify_checksum(), None);
        assert!(save.trailing_bytes().is_empty());

        // End block without a checksum dword.
        v.extend_from_slice(&end_bytes(0x128));
        let save = SaveFile::parse(&v).unwrap();
        assert_eq!(save.checksum(), None);
        assert!(save.end().is_some());
    }

    #[test]
    fn trailing_bytes_are_kept() {
        let mut bytes = full_save();
        bytes.extend_from_slice(&[9u8; 6]);
        let save = SaveFile::parse(&bytes).unwrap();
        assert_eq!(save.trailing_bytes(), &[9u8; 6]);
    }

    #[test]
    fn corrupt_block_is_an_error() {
        let mut v = Vec::new();
        v.extend_from_slice(&header_bytes(57, 0, ""));
        v.extend_from_slice(BLOCK_MAGIC);
        v.extend_from_slice(&1000u32.to_le_bytes());
        assert!(matches!(
            SaveFile::parse(&v),
            Err(Error::BlockOverrun { .. })
        ));
    }

    #[test]
    fn classify_known_magics() {
        // Fake RSC5 header: magic + kind + flags + zlib codec id.
        let mut rsc = vec![0x52, 0x53, 0x43, 0x05];
        rsc.extend_from_slice(&8u32.to_le_bytes());
        rsc.extend_from_slice(&0u32.to_le_bytes());
        rsc.extend_from_slice(&0xDA78u16.to_le_bytes());
        let mut v = Vec::new();
        v.extend_from_slice(&header_bytes(57, 0, ""));
        v.extend_from_slice(&block_bytes(&rsc));
        v.extend_from_slice(&block_bytes(b"RPF3rest-of-fake-header-payload"));
        v.extend_from_slice(&block_bytes(&[0u8; 30]));
        let save = SaveFile::parse(&v).unwrap();
        assert_eq!(
            save.classify(&save.blocks()[0]),
            Payload::Resource {
                kind: 8,
                codec: 0xDA78
            }
        );
        assert_eq!(save.classify(&save.blocks()[1]), Payload::Rpf);
        assert_eq!(save.classify(&save.blocks()[2]), Payload::Unknown);
    }

    #[test]
    fn read_from_caps_huge_streams() {
        let big = std::io::repeat(0x41).take(MAX_FILE + 1);
        assert!(matches!(
            SaveFile::read_from(big),
            Err(Error::TooLarge { .. })
        ));
    }

    #[test]
    fn header_magic_uses_save_constant() {
        assert_eq!(SAVE_MAGIC, b"SAVE");
    }
}
