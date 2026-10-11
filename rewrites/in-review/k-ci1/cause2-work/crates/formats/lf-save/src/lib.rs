//! Save game container reader: header, data blocks, checksum and end block.
//!
//! This crate parses the container that wraps a Grand Theft Auto IV save game
//! file (slot files named `SGTA4xx`). It covers the framing only: the file
//! header, the sequence of `BLOCK` data blocks, the trailing checksum dword
//! and the `END` block. The inside of each data block has its own layout per
//! game system and is deliberately left as opaque bytes (see
//! [`SaveFile::payload`]).
//!
//! The implementation is written from public format documentation plus the
//! author's own inspection of the game executable's block-name table. It
//! contains no game data. No save files exist on the author's machine, so the
//! framing rules below are verified against documentation and executable
//! strings only, never against a real save; every unverified point is marked.
//!
//! # Format description
//!
//! All multi-byte integers are little-endian. Offsets are file offsets.
//!
//! ```text
//! offset  size   content
//! 0x000   0x110  file header (see [`Header`])
//! 0x110   ...    data blocks, back to back (see [`Block`])
//! ...     4      checksum dword (see [`checksum`]) -- UNVERIFIED position
//! ...     0x16C  end block (see [`EndBlock`]) -- UNVERIFIED position
//! ...     ...    trailing bytes, if any (normally none)
//! ```
//!
//! ## File header (0x110 bytes)
//!
//! ```text
//! offset  size  field
//! 0x00    4     save game version (u32; 57 in the game's own data files)
//! 0x04    4     save game size in bytes (u32)
//! 0x08    4     global variables size in bytes (u32, meaning uncertain)
//! 0x0C    4     magic: ASCII "SAVE"
//! 0x10    0x100 last mission name: 128 UTF-16LE code units, NUL padded
//! ```
//!
//! ## Data blocks
//!
//! Each block opens with the 5-byte magic `BLOCK` followed by a u32 total
//! size that includes the magic and the size field itself, then the payload:
//!
//! ```text
//! offset  size     field
//! 0x00    5        magic: ASCII "BLOCK"
//! 0x05    4        total block size in bytes, counting from offset 0
//! 0x09    size-9   payload (layout differs per block)
//! ```
//!
//! Blocks carry no name in the file; identity is positional. The game writes
//! 32 blocks in a fixed order (see [`BlockKind`]); the order and names below
//! match both the public wiki and a contiguous name table in the executable,
//! where the last thirteen entries carry a `NotDefined` prefix:
//!
//! ```text
//!  0 SimpleVars        8 Restart          16 Radio           24 Streaming
//!  1 PlayerInfo        9 Radar            17 Objects         25 PedType
//!  2 ExtraContent     10 Zones            18 Relationships   26 Tags
//!  3 Scripts          11 Gangs            19 Inventory       27 Shopping
//!  4 Garages          12 CarGenerators    20 Pools           28 GangWars
//!  5 GameLogic        13 Stats            21 PhoneInfo       29 EntryExits
//!  6 PathFind         14 IplStore         22 AudioScriptObj  30 3dMarkers
//!  7 Pickups          15 StuntJumps       23 SetPieces       31 Vehicles
//! ```
//!
//! Only two blocks have a documented fixed total size: block 0 is 0xB9 bytes
//! and block 1 is 0xD4 bytes (see [`BlockKind::documented_len`]). The rest
//! vary per save.
//!
//! ## Checksum
//!
//! Public documentation says a u32 checksum follows the last data block and
//! is the wrapping sum of every byte before it. The game reportedly ignores
//! the value on load. The exact normalisation of the size field before
//! summing (documented for the older Games for Windows Live layout) could not
//! be verified, so [`SaveFile::verify_checksum`] sums the stored bytes
//! as-is; see [`checksum`].
//!
//! ## End block (0x16C bytes)
//!
//! ```text
//! offset  size  field
//! 0x00    4     magic: "END" plus a NUL byte
//! 0x04    4     unknown word (documented as always 0x128)
//! 0x08    0x124 unknown bytes (Games for Windows Live data, per the wiki)
//! ```
//!
//! Whether Complete Edition saves still carry this block is UNVERIFIED; the
//! parser accepts files with and without it.
//!
//! ## What is still unknown
//!
//! - The relative order of the checksum dword and the end block, and whether
//!   either may be absent, has not been confirmed against a real file.
//! - Per-block payload layouts (32 separate formats) are out of scope.
//! - The meaning of the header's third word ("global variables size").
//! - Whether any block payload embeds compressed or container data; the
//!   [`SaveFile::classify`] helper sniffs for the archive and resource
//!   containers without assuming any block does.
//! - Slot files past the sixteen names the executable references, and the
//!   exact on-disk locations on every Windows version (see [`save_dirs`]).
//!
//! ## Sources
//!
//! Layout: the `GTAMods` wiki "Saves (GTA 4)" page (header table, block
//! framing, block names and order, checksum rule, end-block table). Block
//! names, order and count cross-checked against the executable's own
//! contiguous block-name table; slot names and the version-stamp mechanism
//! likewise. No parser code was copied from any source: no open-source GTA IV
//! save parser was found (only GTA: San Andreas and GTA V readers, whose
//! containers differ).
//!
//! [`checksum`]: crate::checksum

mod blocks;
mod checksum;
mod end;
mod error;
mod header;
mod paths;
mod save;

pub use blocks::{BLOCK_HEADER_LEN, BLOCK_MAGIC, Block, BlockKind};
pub use checksum::{Checksum, compute_checksum};
pub use end::{END_LEN, END_MAGIC, EndBlock};
pub use error::Error;
pub use header::{HEADER_LEN, Header, SAVE_MAGIC, VERSION_CE_12059};
pub use paths::{save_dirs, slot_name};
pub use save::{MAX_FILE, Payload, SaveFile};

/// Shorthand for `Result<T, lf_save::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
