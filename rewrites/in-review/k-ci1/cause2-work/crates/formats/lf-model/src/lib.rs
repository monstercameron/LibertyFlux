//! Read-only parser for the model resources of GTA IV: drawables,
//! drawable dictionaries and fragments (files ending `.wdr`, `.wdd`, `.wft`).
//!
//! This crate is original work written from public format documentation
//! (the `GTAMods` wiki pages for WDR, WDD and WFT) and from inspecting the
//! game's own files. Open-source readers (`RageLib` inside `GTA4Unity` and
//! `SparkIV`, `BlenDR`'s model module, the gta4-webmap extractor) were read to
//! learn field layouts; no code was copied from them. See `format-spec.md`
//! next to this crate for what was verified against real files and what is
//! still unknown.
//!
//! # Container
//!
//! Every model file is an RSC5 resource: a 12-byte header (magic, resource
//! type id, size flags) followed by one zlib stream. Inflating the stream
//! yields two segments back to back: the system segment (structures,
//! pointers, names) and the graphics segment (vertex and index data). The
//! flags word encodes both segment sizes. See [`rsc5`].
//!
//! Resource type ids observed: drawable and drawable dictionary share one
//! id, fragment has its own. The dictionary header starts with a different
//! vtable value than the drawable header, which is how the two are told
//! apart once inflated.
//!
//! # Layout overview
//!
//! A drawable header holds, in order: a vtable slot, a block-map address, a
//! shader-group pointer, a skeleton pointer (zero when absent), three
//! bounding volumes (centre, minimum, maximum, each four floats), four
//! level-of-detail slots (each a pointer to a model collection, zero when
//! the LOD is absent), an absolute-maximum vector, LOD tuning words, and a
//! light-attributes collection. Most map models use one LOD; fragments use
//! three.
//!
//! A model holds a pointer collection of geometries, one bounding box per
//! geometry plus one for the whole model, and one shader index per
//! geometry. A geometry holds counts (vertices, indices, faces), a
//! primitive-type word, a vertex stride, an optional skinning matrix
//! palette, and pointers to a vertex buffer and an index buffer.
//!
//! A vertex buffer holds a vertex count, a stride, a pointer into the
//! graphics segment for the raw bytes, and a pointer to a vertex
//! declaration. The declaration packs sixteen 4-bit element types into one
//! 64-bit word plus a 16-bit usage mask; each set bit adds one element
//! (position, blend weights, blend indices, normal, two colours, eight
//! texture coordinates, tangent, binormal) in slot order. Element data
//! types are the Direct3D 9 declaration types: 16-bit and 32-bit float
//! vectors, packed bytes, packed colours and packed normals. See
//! [`vertex`] for the decoding rules.
//!
//! A shader group holds an optional embedded texture-dictionary pointer, a
//! pointer collection of shaders, twelve render-pass shader indices, and
//! two small index collections. Each shader holds parameter offset, type
//! and name-hash arrays plus a shader name string. Parameter payloads are
//! textures (with a name string), four-vectors, matrices or floats.
//!
//! A skeleton holds a bone count, bone-id mappings, and parallel arrays of
//! bones (names plus hierarchy links), parent indices and three transform
//! sets (default, inverse, global).
//!
//! A drawable dictionary is a name-hash collection plus a pointer
//! collection of drawables. A fragment is a main drawable (which carries
//! all render geometry) plus a list of child records with bone indices,
//! flags and physics pointers; the child records carry no models.
//!
//! # Pointer model
//!
//! Internal pointers are 32-bit tags: a zero word is null, otherwise the
//! top four bits select the segment (system or graphics) and the low 28
//! bits are a byte offset into that segment. All integers are
//! little-endian. Parsing never depends on pointer width: everything is
//! read from byte slices with explicit bounds checks, and malformed input
//! returns [`Error`] instead of panicking.
//!
//! # Example
//!
//! ```no_run
//! use lf_model::{Fragment, Drawable, DrawableDictionary, Resource};
//!
//! let bytes = std::fs::read("sample.wdr").unwrap();
//! let res = Resource::open(&bytes).unwrap();
//! let draw = Drawable::parse(&res).unwrap();
//! println!("lods: {}, verts: {}", draw.lods.len(), draw.vertex_count());
//! ```

mod cursor;
pub mod dictionary;
pub mod drawable;
pub mod fragment;
pub mod rsc5;
pub mod shader;
pub mod skeleton;
pub mod vertex;

pub use dictionary::DrawableDictionary;
pub use drawable::{Drawable, Geometry, LodGroup, Model};
pub use fragment::Fragment;
pub use rsc5::Resource;
pub use shader::{Shader, ShaderGroup, ShaderParam};
pub use skeleton::Skeleton;
pub use vertex::{ElementType, ElementUsage, VertexDecl, VertexElement};

use std::fmt;

/// Error returned for malformed or truncated input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Input is shorter than the structure being read.
    Truncated {
        /// Byte offset that was requested.
        offset: usize,
        /// Bytes available.
        len: usize,
    },
    /// Magic or version word does not match.
    BadMagic {
        /// What was found.
        found: u32,
    },
    /// A pointer word has an unexpected segment marker.
    BadPointer {
        /// Byte offset of the pointer word.
        offset: usize,
        /// The pointer value.
        value: u32,
    },
    /// A count or size is inconsistent or absurd.
    BadCount {
        /// What was being counted.
        what: &'static str,
        /// The offending value.
        value: u32,
    },
    /// The zlib payload failed to inflate.
    Decompress(String),
    /// A name string is not valid text.
    BadString {
        /// Byte offset of the string.
        offset: usize,
    },
    /// An enumerated value is outside the known range.
    BadEnum {
        /// What was being decoded.
        what: &'static str,
        /// The offending value.
        value: u32,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Truncated { offset, len } => {
                write!(f, "truncated input: offset {offset} past length {len}")
            }
            Error::BadMagic { found } => {
                write!(f, "bad magic: {found:#x}")
            }
            Error::BadPointer { offset, value } => {
                write!(f, "bad pointer {value:#x} at offset {offset:#x}")
            }
            Error::BadCount { what, value } => {
                write!(f, "bad {what} count: {value}")
            }
            Error::Decompress(msg) => write!(f, "decompression failed: {msg}"),
            Error::BadString { offset } => {
                write!(f, "bad string at offset {offset:#x}")
            }
            Error::BadEnum { what, value } => {
                write!(f, "bad {what} value: {value}")
            }
        }
    }
}

impl std::error::Error for Error {}
