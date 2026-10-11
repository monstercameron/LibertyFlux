//! Readers for the navigation and vehicle-path formats: pedestrian
//! navigation meshes (`.wnv`), compiled vehicle path nodes (`.nod`), and the
//! text path graph (`paths*.ipl`) they are built from.
//!
//! All parsing is from byte slices with explicit little-endian reads. It is
//! pointer-width independent and platform independent, returns [`Error`] on
//! malformed input, and never panics on file contents.
//!
//! # Formats
//!
//! ## RSC5 resource container ([`rsc`])
//!
//! Every `.wnv` file is an RSC5 resource: a 12-byte header holding a magic
//! word, a resource type id and a flags word, followed by a zlib stream that
//! inflates to the tile payload. Observed type id for navmeshes is 1.
//!
//! ```text
//! offset  size  meaning
//! 0       4     magic bytes "RSC" + 0x05
//! 4       4     resource type id (1 for navigation meshes)
//! 8       4     flags word
//! 12      ...   zlib-compressed payload
//! ```
//!
//! ## Navigation mesh tile ([`wnv`])
//!
//! The world is covered by a dense 60 by 60 grid of 100 m tiles named
//! `sectors2x2_<sx>_<sy>.wnv`, where `<sx>` and `<sy>` are even game-sector
//! numbers (50 m sectors, two per tile). Tile world corner (metres, game
//! X/Y-up axes): `x = sx * 50 - 3000`, `y = sy * 50 - 3000`. Eight extra
//! small meshes cover boats. Inflated tiles are pool-sized (8 KiB to 128 KiB).
//!
//! Header (offsets into the inflated payload):
//!
//! ```text
//! offset  meaning
//! 0x00    constant 4x4 block: 1.0 on the diagonal, NaN down column 3
//! 0x40    tile size X in metres (float, always 100 for sector tiles)
//! 0x44    tile size Y in metres (float, always 100 for sector tiles)
//! 0x48    tile Z extent in metres (float, 0 for open-water corners)
//! 0x58    pointer to the vertex array (segment tag in the top nibble)
//! 0x60    pointer to the index array
//! 0x64    pointer to the per-corner edge array
//! 0x68    index count (u32)
//! 0x6c    pointer to the polygon array (always just after the header)
//! 0x70    pointer to a trailing region (contents not decoded)
//! 0x78    vertex count (u32)
//! 0x7c    polygon count (u32)
//! ```
//!
//! RAGE pointers carry a `0x5...` segment tag; masking it off gives the byte
//! offset into the inflated buffer.
//!
//! * Vertices are 6 bytes: three u16 values quantised over the tile box.
//!   Plan position is `q / 65535 * size`; height is `qz / 65535 * z_extent`
//!   plus a per-tile base that is not stored in any recognised header field.
//! * Indices are u16 into the vertex array. Every index is in range in all
//!   shipped tiles; most tiles reference every vertex.
//! * Polygons are 40 bytes. The u16 at offset +4 is the polygon's first index;
//!   the next polygon's first index (or the index count for the last polygon)
//!   ends it, giving 3 to 15 vertices. The remaining 38 bytes are exposed raw;
//!   their field breakdown is not established (see [`wnv::Polygon`]).
//! * The edge array sits directly after the polygon array and holds about
//!   one 8-byte record per index. Its neighbour encoding is not established;
//!   records and the raw region are exposed as-is (see [`wnv::EdgeRecord`]
//!   and [`wnv::Tile::edge_region`]).
//!
//! ## Vehicle path nodes ([`nod`])
//!
//! 64 `.nod` files per map (`nodes0.nod` .. `nodes63.nod`) tile the world in
//! 750 m squares in row-major order from (-3000, -3000). Each file holds a
//! node table followed by a link table:
//!
//! ```text
//! header:  u32 node count, u32 car-node count, u32 intersection-node
//!          count, u32 link count (16 bytes; car + intersection = nodes)
//! nodes:   32 bytes each (see [`nod::Node`])
//! links:   8 bytes each: u16 area, u16 node, u8 length in metres,
//!          u8 link flags, u16 link flags (see [`nod::Link`])
//! ```
//!
//! Node positions are three i16 values: X and Y in eighths of a metre, Z in
//! 128ths of a metre. Every node lies inside its own file's 750 m square.
//!
//! A node's links are the slice `links[link_id .. next_link_id]`, where the
//! end is the next node's `link_id` (or the link count for the last node).
//! Link ids are non-decreasing within a file and the slices partition the
//! link table exactly. Every link target `(area, node)` exists, and the graph
//! is fully symmetric. Note the public wiki's link-count nibble (node flags
//! bits 12-15) does not match this ownership rule in the shipped data and
//! must not be used as a count.
//!
//! ## Text path graph ([`ipl`])
//!
//! `paths.ipl` and `paths2.ipl` hold the road network as `vnod` (vehicle node)
//! and `link` text sections, `paths3.ipl` the boat lanes, `paths4.ipl` nodes
//! without links. The `.nod` files are the compiled form of the same network
//! (node counts agree to within a fraction of a percent; each undirected text
//! link becomes two directed node links).
//!
//! ## Sources
//!
//! Layouts were learned from the gta4-webmap project's prose notes and
//! extractor (navmesh tile arrays), the `GTAMods` wiki Paths page (node/link
//! tables), and inspection of the shipped files. All code here is an original
//! implementation; no third-party parser code was copied.

pub mod ipl;
pub mod nod;
pub mod rsc;
pub mod wnv;

use core::fmt;

/// Error returned when input bytes are not a readable file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Fewer bytes than the format needs at this offset.
    UnexpectedEnd {
        /// Byte offset that could not be read.
        offset: usize,
    },
    /// Wrong magic word.
    BadMagic,
    /// A count or size field disagrees with the buffer length.
    BadSize {
        /// What the size fields imply.
        expected: usize,
        /// What the buffer holds.
        actual: usize,
    },
    /// The zlib payload did not inflate.
    Inflate,
    /// An index is outside its table.
    OutOfRange {
        /// Which table was addressed.
        what: &'static str,
        /// The bad index.
        index: usize,
        /// Table length.
        count: usize,
    },
    /// A field holds a value the format forbids here.
    Invalid {
        /// Which field or rule.
        what: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedEnd { offset } => {
                write!(f, "unexpected end of input at offset {offset}")
            }
            Error::BadMagic => write!(f, "bad magic word"),
            Error::BadSize { expected, actual } => {
                write!(
                    f,
                    "size mismatch: fields say {expected} bytes, have {actual}"
                )
            }
            Error::Inflate => write!(f, "zlib payload did not inflate"),
            Error::OutOfRange { what, index, count } => {
                write!(f, "{what} index {index} out of range (count {count})")
            }
            Error::Invalid { what } => write!(f, "invalid {what}"),
        }
    }
}

impl std::error::Error for Error {}

/// Read a little-endian u16 at `offset`.
fn u16le(bytes: &[u8], offset: usize) -> Result<u16, Error> {
    bytes
        .get(offset..offset + 2)
        .and_then(|s| s.try_into().ok())
        .map(u16::from_le_bytes)
        .ok_or(Error::UnexpectedEnd { offset })
}

/// Read one byte at `offset`.
fn u8le(bytes: &[u8], offset: usize) -> Result<u8, Error> {
    bytes
        .get(offset)
        .copied()
        .ok_or(Error::UnexpectedEnd { offset })
}

/// Read a little-endian i16 at `offset`.
fn i16le(bytes: &[u8], offset: usize) -> Result<i16, Error> {
    Ok(i16::from_ne_bytes(u16le(bytes, offset)?.to_ne_bytes()))
}

/// Read a little-endian u32 at `offset`.
fn u32le(bytes: &[u8], offset: usize) -> Result<u32, Error> {
    bytes
        .get(offset..offset + 4)
        .and_then(|s| s.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(Error::UnexpectedEnd { offset })
}

/// Read a little-endian f32 at `offset`.
fn f32le(bytes: &[u8], offset: usize) -> Result<f32, Error> {
    Ok(f32::from_bits(u32le(bytes, offset)?))
}
