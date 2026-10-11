//! Data blocks: framing and the positional block vocabulary.
//!
//! Blocks carry no name in the file; the [`BlockKind`] of a block is its
//! zero-based position among the blocks. The 32 documented names and their
//! order match both the public wiki and the executable's own contiguous
//! block-name table.

use crate::{Error, Result};

/// Block magic: the 5 ASCII bytes `BLOCK`.
pub const BLOCK_MAGIC: &[u8; 5] = b"BLOCK";

/// Block header length in bytes: 5 magic bytes plus the u32 total size.
pub const BLOCK_HEADER_LEN: u32 = 9;

/// Largest block position this crate will scan to. Real saves hold 32
/// blocks; the cap only stops hostile size fields from looping the parser.
pub const MAX_BLOCKS: usize = 4096;

/// Identity of a data block by its position in the file.
///
/// Variants 0-31 are the documented game systems; anything past them (only
/// seen in hand-built or corrupt files) is [`BlockKind::Unknown`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockKind {
    /// Block 0: simple scalar game state.
    SimpleVars,
    /// Block 1: player state.
    PlayerInfo,
    /// Block 2: downloadable-content flags.
    ExtraContent,
    /// Block 3: running scripts.
    Scripts,
    /// Block 4: garage contents.
    Garages,
    /// Block 5: game logic state.
    GameLogic,
    /// Block 6: pathfinding state.
    PathFind,
    /// Block 7: pickups.
    Pickups,
    /// Block 8: restart points.
    Restart,
    /// Block 9: radar blips.
    Radar,
    /// Block 10: zone states.
    Zones,
    /// Block 11: gang states.
    Gangs,
    /// Block 12: car generators.
    CarGenerators,
    /// Block 13: statistics.
    Stats,
    /// Block 14: IPL store.
    IplStore,
    /// Block 15: stunt jumps.
    StuntJumps,
    /// Block 16: radio state.
    Radio,
    /// Block 17: objects.
    Objects,
    /// Block 18: relationship sets.
    Relationships,
    /// Block 19: inventory.
    Inventory,
    /// Block 20: pools.
    Pools,
    /// Block 21: phone state.
    PhoneInfo,
    /// Block 22: audio script objects.
    AudioScriptObject,
    /// Block 23: set pieces.
    SetPieces,
    /// Block 24: streaming state.
    Streaming,
    /// Block 25: ped types.
    PedType,
    /// Block 26: tags.
    Tags,
    /// Block 27: shopping.
    Shopping,
    /// Block 28: gang wars.
    GangWars,
    /// Block 29: entry/exit points.
    EntryExits,
    /// Block 30: 3D markers.
    Markers3d,
    /// Block 31: vehicles.
    Vehicles,
    /// A block past the 32 documented positions. Holds its position.
    Unknown(usize),
}

/// Documented block names in file order.
const NAMES: [&str; 32] = [
    "SimpleVars",
    "PlayerInfo",
    "ExtraContent",
    "Scripts",
    "Garages",
    "GameLogic",
    "PathFind",
    "Pickups",
    "Restart",
    "Radar",
    "Zones",
    "Gangs",
    "CarGenerators",
    "Stats",
    "IplStore",
    "StuntJumps",
    "Radio",
    "Objects",
    "Relationships",
    "Inventory",
    "Pools",
    "PhoneInfo",
    "AudioScriptObject",
    "SetPieces",
    "Streaming",
    "PedType",
    "Tags",
    "Shopping",
    "GangWars",
    "EntryExits",
    "3dMarkers",
    "Vehicles",
];

impl BlockKind {
    /// Number of documented block positions.
    pub const COUNT: usize = 32;

    /// Map a zero-based block position to its kind.
    #[must_use]
    pub const fn from_index(index: usize) -> BlockKind {
        match index {
            0 => BlockKind::SimpleVars,
            1 => BlockKind::PlayerInfo,
            2 => BlockKind::ExtraContent,
            3 => BlockKind::Scripts,
            4 => BlockKind::Garages,
            5 => BlockKind::GameLogic,
            6 => BlockKind::PathFind,
            7 => BlockKind::Pickups,
            8 => BlockKind::Restart,
            9 => BlockKind::Radar,
            10 => BlockKind::Zones,
            11 => BlockKind::Gangs,
            12 => BlockKind::CarGenerators,
            13 => BlockKind::Stats,
            14 => BlockKind::IplStore,
            15 => BlockKind::StuntJumps,
            16 => BlockKind::Radio,
            17 => BlockKind::Objects,
            18 => BlockKind::Relationships,
            19 => BlockKind::Inventory,
            20 => BlockKind::Pools,
            21 => BlockKind::PhoneInfo,
            22 => BlockKind::AudioScriptObject,
            23 => BlockKind::SetPieces,
            24 => BlockKind::Streaming,
            25 => BlockKind::PedType,
            26 => BlockKind::Tags,
            27 => BlockKind::Shopping,
            28 => BlockKind::GangWars,
            29 => BlockKind::EntryExits,
            30 => BlockKind::Markers3d,
            31 => BlockKind::Vehicles,
            other => BlockKind::Unknown(other),
        }
    }

    /// Zero-based block position of this kind. Round-trips [`BlockKind::from_index`].
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            BlockKind::SimpleVars => 0,
            BlockKind::PlayerInfo => 1,
            BlockKind::ExtraContent => 2,
            BlockKind::Scripts => 3,
            BlockKind::Garages => 4,
            BlockKind::GameLogic => 5,
            BlockKind::PathFind => 6,
            BlockKind::Pickups => 7,
            BlockKind::Restart => 8,
            BlockKind::Radar => 9,
            BlockKind::Zones => 10,
            BlockKind::Gangs => 11,
            BlockKind::CarGenerators => 12,
            BlockKind::Stats => 13,
            BlockKind::IplStore => 14,
            BlockKind::StuntJumps => 15,
            BlockKind::Radio => 16,
            BlockKind::Objects => 17,
            BlockKind::Relationships => 18,
            BlockKind::Inventory => 19,
            BlockKind::Pools => 20,
            BlockKind::PhoneInfo => 21,
            BlockKind::AudioScriptObject => 22,
            BlockKind::SetPieces => 23,
            BlockKind::Streaming => 24,
            BlockKind::PedType => 25,
            BlockKind::Tags => 26,
            BlockKind::Shopping => 27,
            BlockKind::GangWars => 28,
            BlockKind::EntryExits => 29,
            BlockKind::Markers3d => 30,
            BlockKind::Vehicles => 31,
            BlockKind::Unknown(other) => other,
        }
    }

    /// Documented block name, or `None` past the 32 known positions.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        match self.index() {
            i if i < NAMES.len() => Some(NAMES[i]),
            _ => None,
        }
    }

    /// Total block size in bytes when public documentation fixes it.
    ///
    /// Only the first two blocks have documented fixed sizes (0xB9 and
    /// 0xD4); every other block varies per save. A mismatch against a real
    /// file is a warning, not proof of corruption: the documentation may
    /// simply be stale.
    #[must_use]
    pub const fn documented_len(self) -> Option<u32> {
        match self {
            BlockKind::SimpleVars => Some(0xB9),
            BlockKind::PlayerInfo => Some(0xD4),
            _ => None,
        }
    }
}

impl std::fmt::Display for BlockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name() {
            Some(n) => write!(f, "{n}"),
            None => write!(f, "unknown block {}", self.index()),
        }
    }
}

/// One data block: position, file range and total size.
///
/// The payload layout differs per game system and is out of scope; read the
/// raw bytes with [`crate::SaveFile::payload`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    /// Zero-based position among the blocks in the file.
    pub index: usize,
    /// Positional identity (see [`BlockKind`]).
    pub kind: BlockKind,
    /// File offset of the `BLOCK` magic.
    pub offset: u64,
    /// Total block size in bytes, including magic and size field.
    pub total_len: u32,
}

impl Block {
    /// Payload length in bytes (`total_len` minus the 9-byte header).
    #[must_use]
    pub fn payload_len(&self) -> usize {
        self.total_len.saturating_sub(BLOCK_HEADER_LEN) as usize
    }
}

/// Parse one block header at `offset` inside `bytes`.
///
/// Returns `Ok(None)` when fewer than 9 bytes remain (the trailer starts
/// here) and `Err` when the magic is present but the size is impossible or
/// the range overruns the file.
pub(crate) fn parse_at(bytes: &[u8], offset: usize, index: usize) -> Result<Option<Block>> {
    let rest = bytes.len().saturating_sub(offset);
    if rest < BLOCK_HEADER_LEN as usize {
        return Ok(None);
    }
    if bytes[offset..offset + 5] != *BLOCK_MAGIC {
        return Ok(None);
    }
    let size = u32::from_le_bytes([
        bytes[offset + 5],
        bytes[offset + 6],
        bytes[offset + 7],
        bytes[offset + 8],
    ]);
    if size < BLOCK_HEADER_LEN {
        return Err(Error::BadBlockSize {
            index,
            offset: offset as u64,
            size,
        });
    }
    if u64::from(size) > rest as u64 {
        return Err(Error::BlockOverrun {
            index,
            offset: offset as u64,
            size,
            file_len: bytes.len(),
        });
    }
    Ok(Some(Block {
        index,
        kind: BlockKind::from_index(index),
        offset: offset as u64,
        total_len: size,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_round_trip() {
        for (i, name) in NAMES.iter().enumerate() {
            let kind = BlockKind::from_index(i);
            assert_eq!(kind.index(), i);
            assert_eq!(kind.name(), Some(*name));
        }
        for i in NAMES.len()..45 {
            let kind = BlockKind::from_index(i);
            assert_eq!(kind.index(), i);
            assert_eq!(kind.name(), None);
        }
    }

    #[test]
    fn documented_lengths() {
        assert_eq!(BlockKind::SimpleVars.documented_len(), Some(0xB9));
        assert_eq!(BlockKind::PlayerInfo.documented_len(), Some(0xD4));
        assert_eq!(BlockKind::Stats.documented_len(), None);
        assert_eq!(BlockKind::Vehicles.documented_len(), None);
    }

    #[test]
    fn parses_hand_built_block() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(BLOCK_MAGIC);
        bytes.extend_from_slice(&20u32.to_le_bytes());
        bytes.extend_from_slice(&[0xAA; 11]);
        let b = parse_at(&bytes, 0, 3).unwrap().unwrap();
        assert_eq!(b.kind, BlockKind::Scripts);
        assert_eq!(b.total_len, 20);
        assert_eq!(b.payload_len(), 11);
    }

    #[test]
    fn rejects_bad_sizes() {
        // Size below the header length.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(BLOCK_MAGIC);
        bytes.extend_from_slice(&8u32.to_le_bytes());
        assert!(matches!(
            parse_at(&bytes, 0, 0),
            Err(Error::BadBlockSize { .. })
        ));
        // Declared range past end of file.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(BLOCK_MAGIC);
        bytes.extend_from_slice(&100u32.to_le_bytes());
        assert!(matches!(
            parse_at(&bytes, 0, 0),
            Err(Error::BlockOverrun { .. })
        ));
    }

    #[test]
    fn non_block_bytes_end_the_run() {
        assert!(parse_at(b"END\0rest", 0, 0).unwrap().is_none());
        assert!(parse_at(b"BLOC", 0, 0).unwrap().is_none());
    }
}
