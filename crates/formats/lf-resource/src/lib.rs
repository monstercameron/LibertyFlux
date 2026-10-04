//! Reader for the RSC5 resource container: the wrapper around nearly every
//! binary asset in Grand Theft Auto IV (models, textures, collision,
//! navigation meshes, web pages, effects packages and more), both as loose
//! files and as archive members.
//!
//! This crate is the foundation that asset-specific readers build on. It
//! covers the container only: the file header, the flags word that encodes
//! the two segment sizes, zlib decompression, splitting the payload into the
//! system (CPU) and graphics (data) segments, the tagged pointer
//! representation used inside resources, and the `pgBase` block-map slot. It
//! deliberately does not parse any asset type.
//!
//! # Format description
//!
//! All multi-byte integers are little-endian on PC. All offsets below are
//! file offsets unless stated otherwise.
//!
//! ## File layout
//!
//! ```text
//! offset  size  content
//! 0       4     magic: ASCII "RSC" followed by byte 0x05 (u32 0x05435352)
//! 4       4     resource type id (see [`ResourceKind`]); differs per asset type
//! 8       4     flags: encodes the system and graphics segment sizes (below)
//! 12      2     compression codec id; 0xDA78 (zlib) on every PC file seen
//! 12      ...   compressed payload: one zlib stream running to end of file
//! ```
//!
//! Note the overlap: the codec id at bytes 12-13 doubles as the zlib stream
//! header (`78 DA`). Console builds use an LZX codec id (0xF505) instead; this
//! crate detects it and reports [`Error::UnsupportedCodec`].
//!
//! ## Flags word (offset 8)
//!
//! The flags word stores each segment size as a mantissa/exponent pair with a
//! 256-byte unit:
//!
//! ```text
//! bits    width  meaning
//! 0-10    11     system mantissa  (sys_a)
//! 11-14   4      system exponent  (sys_b)
//! 15-25   11     graphics mantissa (gfx_a)
//! 26-29   4      graphics exponent (gfx_b)
//! 30-31   2      reserved (top bit set on every PC file seen; meaning unknown)
//! ```
//!
//! ```text
//! system_size   = sys_a << (sys_b + 8)
//! graphics_size = gfx_a << (gfx_b + 8)
//! ```
//!
//! The zlib stream always inflates to exactly `system_size + graphics_size`
//! bytes: the system segment first, then the graphics segment. Every loose
//! PC resource verified so far matches this exactly.
//!
//! ## Pointers inside resources
//!
//! Loaded resources are addressed with 32-bit tagged pointers: the top 4 bits
//! select the segment and the low 28 bits are the byte offset within it (see
//! [`Pointer`]):
//!
//! ```text
//! value         meaning
//! 0x00000000  null pointer
//! 0x5xxxxxxx  offset into the system (CPU) segment
//! 0x6xxxxxxx  offset into the graphics (data) segment
//! ```
//!
//! Two notes for asset readers. First, vtable slots at the start of objects
//! are raw executable addresses, not tagged pointers; only fields documented
//! as offsets use the tagged form. Second, a few data-segment references pack
//! small flag bits into the low bits of the offset; [`Pointer::offset`] keeps
//! them and it is the asset reader's job to mask them per its own layout.
//!
//! ## The `pgBase` block-map slot
//!
//! Paged engine objects derive from `rage::pgBase`, whose first two fields
//! are a vtable slot and a block-map pointer (a system-segment [`Pointer`]).
//! In every shipped PC file examined the slot is either absent (null or the
//! `0xCDCDCDCD` debug fill, as in effects packages) or reserved but
//! unpopulated (first word zero, remainder debug fill, as in texture and
//! drawable dictionaries); the engine evidently builds the map at load time.
//! See [`BlockMap`] and [`BlockMapState`].
//!
//! ## What is still unknown
//!
//! - The meaning of the top two flags bits (bit 31 is set on all PC files).
//! - The populated `BlockMap` entry layout: no shipped PC file has one, so
//!   [`BlockMap`] exposes the raw prefix words for a future lane to decode.
//! - Whether any PC file uses LZX compression (none found; all use zlib).
//! - Big-endian (console) files are detected and rejected, not parsed.
//!
//! ## Sources
//!
//! Layout cross-checked against the public `GTAMods` wiki WDR page (pointer
//! tags), the `RageLib` reader in SparkIV/GTA4Unity (header fields, flags
//! formula, codec ids, `pgBase` layout; GPL, read for layout only, no code
//! copied), and VIRUXE's `rage-formats` RSC7 reader (Unlicense; lineage
//! reference for the segment design). Every behavioural claim above was then
//! verified against the game's own loose files; see the integration test.
//!
//! [`Error::UnsupportedCodec`]: crate::Error::UnsupportedCodec

mod blockmap;
mod error;
mod header;
mod pointer;
mod resource;

pub use blockmap::{BlockMap, BlockMapState, PgBase, is_debug_fill};
pub use error::Error;
pub use header::{Codec, Header, ResourceKind};
pub use pointer::{Pointer, Segment};
pub use resource::Resource;

/// Result type used throughout this crate.
pub type Result<T> = std::result::Result<T, Error>;
