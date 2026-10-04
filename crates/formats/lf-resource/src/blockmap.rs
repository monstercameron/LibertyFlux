//! The `pgBase` prefix and its block-map slot.
//!
//! Paged engine objects start with a vtable slot followed by a system-segment
//! pointer to the object's block map. In shipped PC files the slot is never
//! populated (see [`BlockMapState`]), so this module reads and classifies the
//! slot rather than decoding entries.

use crate::{Pointer, Resource, Result, Segment};

/// The `rage::pgBase` prefix at the start of the system segment: a raw
/// vtable address (not a tagged pointer) plus the block-map pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgBase {
    /// Raw vtable address at system+0.
    pub vtable: u32,
    /// Block-map pointer at system+4 (may be null or debug fill).
    pub block_map: Pointer,
}

impl PgBase {
    /// Bytes consumed by the prefix.
    pub const LEN: usize = 8;
}

/// What the block-map slot holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockMapState {
    /// No map in the file: the slot is null or not a system-segment pointer
    /// (effects packages leave the `0xCDCDCDCD` debug fill here). Normal for
    /// types that are not paged objects; not an error.
    Absent,
    /// The slot points into the system segment but the target is the
    /// reserved-but-empty shape: first word zero, remainder debug fill.
    /// Texture and drawable dictionaries ship this way; the engine builds
    /// the map at load time.
    Empty,
    /// The slot points at data that is neither zeroed nor debug fill. No
    /// shipped PC file shows this; [`BlockMap::prefix`] carries the first
    /// words for future decoding.
    Present,
}

/// The block-map slot of a resource: where it points, what state it is in,
/// and the first eight words at the target when there is one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockMap {
    /// Raw slot value from system+4.
    pub target: Pointer,
    /// Classification of the slot.
    pub state: BlockMapState,
    /// First eight words at the target. Meaningful for [`BlockMapState::Empty`]
    /// (the zero-plus-fill shape) and [`BlockMapState::Present`]; zeroed when
    /// [`BlockMapState::Absent`].
    pub prefix: [u32; 8],
}

impl BlockMap {
    /// Words captured into [`BlockMap::prefix`].
    pub const PREFIX_WORDS: usize = 8;

    /// Read the block-map slot via the `pgBase` prefix at the system segment
    /// start. Fails only if the system segment is shorter than the prefix or
    /// the slot is a system pointer landing out of bounds; every other shape
    /// classifies as [`BlockMapState::Absent`], [`BlockMapState::Empty`] or
    /// [`BlockMapState::Present`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn read(resource: &Resource) -> Result<BlockMap> {
        let pg = resource.pg_base()?;
        let target = pg.block_map;
        if target.is_null() || target.segment() != Some(Segment::System) {
            return Ok(BlockMap {
                target,
                state: BlockMapState::Absent,
                prefix: [0; 8],
            });
        }
        let bytes = resource.slice(target, BlockMap::PREFIX_WORDS * 4)?;
        let mut prefix = [0u32; 8];
        for (i, word) in prefix.iter_mut().enumerate() {
            let o = i * 4;
            *word = u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]]);
        }
        let state = if prefix[0] == 0 && prefix[1..].iter().all(|&w| w == Pointer::DEBUG_FILL) {
            BlockMapState::Empty
        } else {
            BlockMapState::Present
        };
        Ok(BlockMap {
            target,
            state,
            prefix,
        })
    }
}

/// True when `value` is the toolchain's unpopulated-slot marker.
#[must_use]
pub fn is_debug_fill(value: u32) -> bool {
    value == Pointer::DEBUG_FILL
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write;

    fn build_resource(sys: &[u8]) -> Resource {
        assert!(sys.len().is_multiple_of(256));
        let flags: u32 = (sys.len() / 256) as u32; // gfx empty
        // Best compression, like the game's files (stream header 78 DA).
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
        enc.write_all(sys).unwrap();
        let payload = enc.finish().unwrap();
        let mut file = Vec::new();
        file.extend_from_slice(&crate::header::MAGIC.to_le_bytes());
        file.extend_from_slice(&1u32.to_le_bytes());
        file.extend_from_slice(&flags.to_le_bytes());
        file.extend_from_slice(&payload);
        Resource::parse(&file).unwrap()
    }

    #[test]
    fn classifies_absent_empty_present() {
        // Absent: debug fill in the slot.
        let mut sys = vec![0u8; 256];
        sys[0..4].copy_from_slice(&0x1111_1111u32.to_le_bytes());
        sys[4..8].copy_from_slice(&Pointer::DEBUG_FILL.to_le_bytes());
        let bm = build_resource(&sys).block_map().unwrap();
        assert_eq!(bm.state, BlockMapState::Absent);

        // Absent: null slot.
        let mut sys = vec![0u8; 256];
        sys[4..8].copy_from_slice(&0u32.to_le_bytes());
        let bm = build_resource(&sys).block_map().unwrap();
        assert_eq!(bm.state, BlockMapState::Absent);

        // Empty: zero word then fill.
        let mut sys = vec![0u8; 256];
        sys[4..8].copy_from_slice(&0x5000_0020u32.to_le_bytes());
        sys[0x20..0x24].copy_from_slice(&0u32.to_le_bytes());
        for w in (0x24..0x40).step_by(4) {
            sys[w..w + 4].copy_from_slice(&Pointer::DEBUG_FILL.to_le_bytes());
        }
        let bm = build_resource(&sys).block_map().unwrap();
        assert_eq!(bm.state, BlockMapState::Empty);
        assert_eq!(bm.target, Pointer::new(0x5000_0020));

        // Present: anything else.
        let mut sys = vec![0u8; 256];
        sys[4..8].copy_from_slice(&0x5000_0020u32.to_le_bytes());
        sys[0x20..0x24].copy_from_slice(&0x1234_5678u32.to_le_bytes());
        let bm = build_resource(&sys).block_map().unwrap();
        assert_eq!(bm.state, BlockMapState::Present);
        assert_eq!(bm.prefix[0], 0x1234_5678);

        // Out-of-bounds system pointer is an error.
        let mut sys = vec![0u8; 256];
        sys[4..8].copy_from_slice(&0x5000_FFF0u32.to_le_bytes());
        let err = build_resource(&sys).block_map().unwrap_err();
        assert!(matches!(err, Error::OutOfBounds { .. }));
    }
}
