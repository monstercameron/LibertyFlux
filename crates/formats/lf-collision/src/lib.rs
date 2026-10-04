//! Reader for the game's collision bounds resources.
//!
//! This crate parses the two collision file kinds the game ships inside its
//! map archives: single-bound files (`.wbn`) and bound dictionaries (`.wbd`).
//! Both are resource-container files holding one serialised bound hierarchy
//! each. All integers are little-endian; all parsing is done from byte slices
//! with explicit bounds checks, so the crate is pointer-width and platform
//! independent.
//!
//! # Format overview
//!
//! The description below is written from public documentation and from this
//! crate's author inspecting the shipped files. Field names follow the ones
//! used by the open-source readers consulted (see the project report); the
//! code itself is an independent implementation.
//!
//! ## Resource container (minimal private reader)
//!
//! Every collision file starts with a 14-byte header:
//!
//! ```text
//! offset  size  meaning
//! 0       4     magic: the bytes "RSC" followed by version byte 5
//! 4       4     resource type id (32 for bounds)
//! 8       4     flags: packed system/graphics segment sizes (see below)
//! 12      2     compression codec id (0xDA78 means zlib deflate)
//! ```
//!
//! The codec id doubles as the zlib stream header: bytes `78 DA`, the
//! standard zlib header for default compression, are the little-endian codec
//! word. The zlib stream therefore starts at file offset 12 and inflates to
//! the system segment followed by the graphics segment. Collision files never
//! use the graphics segment (its decoded size is always zero).
//!
//! The flags word packs two segment sizes. Each size is an 11-bit base value
//! and a 4-bit shift: `size = base << (shift + 8)`. The low 11 bits and the
//! next 4 bits describe the system segment; bits 15..25 and bits 26..29
//! describe the graphics segment. Top bits are unused on the files seen.
//!
//! Inside the inflated system segment, a stored pointer is a 32-bit word. A
//! zero word is null. Otherwise the top nibble is a segment tag: `5` means
//! "offset into the system segment" and `6` means "offset into the graphics
//! segment"; the low 28 bits are the byte offset. Collision files only ever
//! use tag `5`.
//!
//! Note: this container reader is deliberately minimal and private to this
//! crate. The project's shared resource-container crate (lane `fmt-rsc5`)
//! will replace it once integrated.
//!
//! ## File roots
//!
//! A single-bound file holds one root bound. Its first 12 bytes are an opaque
//! vtable word, one auxiliary system pointer of unknown purpose, and the
//! system pointer to the root bound.
//!
//! A dictionary file holds a header of eight words: an opaque vtable word, a
//! block-map system pointer, two unknown words (always 0 and 1 on files
//! seen), a hash collection (pointer, count, duplicate count) and a bound
//! collection (pointer, count, duplicate count). The hash array holds one
//! unsigned 32-bit model-name hash per bound (hash algorithm unknown); the
//! bound array holds one system pointer per bound. All four counts are always
//! equal on the files seen.
//!
//! ## Bound base (128 bytes, every bound starts with this)
//!
//! ```text
//! offset  size  meaning
//! 0       4     opaque vtable word (a small id, constant per bound kind)
//! 4       1     bound kind byte (see [`BoundType`])
//! 5       1     flags byte (usually 1)
//! 6       2     part index (usually 1)
//! 8       4     bounding-sphere radius around the centroid (float)
//! 12      4     unknown float (small positive values on files seen)
//! 16      16    bounding-box maximum, three floats plus a NaN pad word
//! 32      16    bounding-box minimum, three floats plus a NaN pad word
//! 48      16    centroid offset, three floats plus a NaN pad word
//! 64      16    centre-of-gravity offset, three floats plus a NaN pad word
//! 80      16    volume distribution, three floats plus a NaN pad word
//! 96      32    reserved: not yet interpreted, kept as raw bytes
//! ```
//!
//! The NaN pad word is almost always the bit pattern `0x7F800001`; a handful
//! of composite bounds use `0x7FC00001` instead. Both are quiet NaNs.
//!
//! ## Bound kinds
//!
//! The kind byte selects one of the layouts below. Kinds marked "unobserved"
//! have engine classes behind them but no shipped file uses them, so their
//! tail layouts are unknown and this crate surfaces them as [`Bound::Unparsed`].
//!
//! - Sphere (0): base plus one 16-byte vector holding the radius three times
//!   plus a NaN pad. The value matches the base bounding radius.
//! - Capsule (1): base plus four 16-byte vectors: radius three times plus
//!   pad, cylinder length three times plus pad, then two all-zero vectors
//!   (pads NaN). The base bounding radius always equals radius + length / 2.
//! - Box (3): the full mesh layout below, always with exactly 8 vertices and
//!   6 quadrilateral polygons (a cuboid).
//! - Geometry (4): the mesh layout, plus a second vertex array ("shrunk"
//!   vertices) with the same element count.
//! - Curved geometry (5), grid (6), ribbon (7), surface (11): unobserved.
//! - BVH (10): the mesh layout with the tree flag set. No separate on-disk
//!   tree structure was found: BVH bounds differ from plain geometry bounds
//!   only by the flag byte and by the absence of the second vertex array, so
//!   the search tree is presumably built when the file loads.
//! - Composite (12): a group of child bounds with per-child transforms.
//!
//! Kind bytes 2, 8 and 9 never appear in shipped files and have no confirmed
//! meaning; they also surface as [`Bound::Unparsed`].
//!
//! ## Mesh layout (box, geometry, BVH; 80 bytes after the base)
//!
//! ```text
//! offset  size  meaning
//! 128     4     reserved (always zero on files seen)
//! 132     4     system pointer to the second vertex array, or null
//! 136     4     reserved (always zero)
//! 140     4     system pointer to the polygon array
//! 144     16    unquantise factors, three floats plus a NaN pad
//! 160     16    quantisation centre, three floats plus a NaN pad
//! 176     4     system pointer to the vertex array
//! 180     4     reserved (always zero)
//! 184     1     tree flag: 1 for BVH bounds, 0 otherwise (next 3 bytes padding)
//! 188     4     marker word, always 0xFFFFFFFF on files seen
//! 192     4     reserved (always zero)
//! 196     4     reserved (always zero)
//! 200     4     vertex count (signed)
//! 204     4     polygon count (signed)
//! ```
//!
//! Vertices are stored as three signed 16-bit integers each. The world
//! position is `centre + quantised * unquantise`, applied per component. The
//! second vertex array, when present, holds the same number of triples in the
//! same quantisation; its exact relationship to the main array is unknown
//! (the per-element difference is large, not a small inset).
//!
//! Each polygon is a 32-byte record: the face normal (three floats, always
//! unit length on files seen), a face-area float whose low bits carry the
//! material index (see [`Polygon::material_index`]), four unsigned 16-bit
//! vertex indices (the fourth is zero for triangles), and four unsigned
//! 16-bit neighbour-polygon indices (`0xFFFF` means no neighbour).
//!
//! Quadrilaterals dominate shipped files (about two thirds of all polygons).
//! Every vertex index seen is below its bound's vertex count, and every
//! vertex seen lies inside its bound's stated bounding box.
//!
//! ## Composite layout (after the base)
//!
//! Four system pointers (child-bound pointer array, current-matrix array,
//! last-matrix array, local min/max box array) followed by two unsigned
//! 16-bit counts (maximum and current bound count, always equal on files
//! seen). Each child matrix is 64 bytes: four 16-byte rows holding the three
//! basis vectors and the translation, each with a NaN pad word. Each local
//! box is 32 bytes: minimum and maximum vectors with NaN pads. Child bounds
//! are always leaf bounds in shipped files (never a nested composite).
//!
//! ## What is still unknown
//!
//! - The last 32 bytes of the bound base (margin data and reference count
//!   are the working guess, unconfirmed).
//! - The unknown float at base offset 12.
//! - The meaning of the flags byte and part index.
//! - The second ("shrunk") vertex array's exact purpose.
//! - The capsule's two trailing zero vectors.
//! - The auxiliary pointer at single-bound root offset 4 and the dictionary
//!   block-map pointer target (starts with a zero word, then padding).
//! - The model-name hash algorithm used by dictionaries.
//! - Tail layouts for kinds 2, 5, 6, 7, 8, 9 and 11 (no shipped samples).
//!
//! # Example
//!
//! ```no_run
//! let bytes = std::fs::read("sample.wbn").unwrap();
//! let file = lf_collision::parse(&bytes).unwrap();
//! println!("bounds: {}", file.all_bounds().len());
//! let report = file.validate();
//! assert!(report.is_clean());
//! ```

#![forbid(unsafe_code)]

mod bound;
mod error;
mod file;
mod rsc;

pub use bound::{
    Bound, BoundHeader, BoundType, CapsuleBound, CompositeBound, Matrix4, MeshBound, MeshKind,
    Polygon, SphereBound, UnparsedBound, Vec3,
};
pub use error::Error;
pub use file::{CollisionFile, ValidationReport, WbdEntry, WbdFile, WbnFile};

/// Parse one collision file (single-bound or dictionary) from memory.
///
/// The kind is detected from the root structure: a dictionary root carries
/// two collections with matching counts and valid bound pointers, otherwise
/// the bytes are read as a single-bound file. Returns an error describing the
/// first problem found; malformed input never panics.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse(data: &[u8]) -> Result<CollisionFile, Error> {
    file::parse(data)
}
