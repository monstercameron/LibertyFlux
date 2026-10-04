//! Read-only parser for texture dictionaries (WTD files) with pixel decoders.
//!
//! A texture dictionary is a named collection of GPU textures. On disk each
//! file is an RSC5 resource: a small header followed by one zlib stream that
//! expands into two segments, a system segment holding structures and a
//! graphics segment holding raw texel data. All integers are little-endian.
//!
//! # File layout, in words
//!
//! The file opens with a 12-byte header: a four-byte magic identifying the
//! RSC container and version, a four-byte resource type (texture
//! dictionaries use type 8), and a four-byte flags word. The flags word
//! packs the expanded sizes of both segments: each size is an 11-bit base
//! multiplied by a power of two chosen by a 4-bit shift field, plus a fixed
//! extra shift of 8, so sizes are always multiples of 256 bytes. The top two
//! flag bits are reserved and ignored here.
//!
//! Immediately after the flags come the two bytes `78 DA`, which are both the
//! codec field (deflate) and the standard zlib header of the compressed
//! payload. The payload decompresses to exactly system size plus graphics
//! size bytes: the first part is the system segment, the rest is graphics.
//!
//! # Pointers
//!
//! Structures inside the system segment refer to each other with tagged
//! 32-bit offsets: the top four bits are a marker and the low 28 bits are a
//! byte offset into one of the segments. Marker `5` addresses the system
//! segment, marker `6` the graphics segment. A zero word is a null pointer.
//!
//! # The dictionary (system offset 0)
//!
//! The dictionary header is 32 bytes: a vtable word, a block-map pointer
//! (always null in files seen so far), a parent word (always zero), a usage
//! count (always one), a pointer to the hash table, a count word plus a
//! capacity word (equal in all files seen), and a pointer to the texture
//! pointer list, again followed by count and capacity words. The hash table
//! holds one 32-bit name hash per
//! texture; the pointer list holds one system pointer per texture, each
//! aiming at a texture record. Entry `i` of both tables describes the same
//! texture. The hash is a Jenkins one-at-a-time hash over the texture title
//! (the stored name without its `pack:/` prefix and `.dds` suffix),
//! lowercased, with backslashes treated as slashes; see [`hash`].
//!
//! # Texture records (80 bytes each)
//!
//! Each record describes one Direct3D 9 style texture: vtable word,
//! block-map pointer (null in files), three unknown words (observed values
//! 1-or-0x10000, 0, 0), a pointer to the null-terminated name string, one
//! more unknown word (0), then width and height as 16-bit words, a 32-bit
//! Direct3D format code, a 16-bit stride, a kind byte (0 is a plain 2D
//! texture; 1 and 3 name cube and volume textures, neither observed), a mip
//! level count byte, six float words (observed 1, 1, 1, 0, 0, 0), previous
//! and next link pointers (next always null), a graphics pointer to the mip
//! data, and a final unknown word (0).
//!
//! Stride is the byte distance between two texel rows of mip level 0, i.e.
//! the level-0 data size divided by the height. Mip levels are stored largest
//! first; each level halves both dimensions down to a minimum of one.
//!
//! # Pixel formats
//!
//! Five format codes appear in shipped files: DXT1, DXT3, DXT5 (block
//! compressed, 4x4 texel blocks of 8 or 16 bytes), A8R8G8B8 (32-bit
//! BGRA bytes), and L8 (8-bit luminance). See [`format`] and [`decode`].
//!
//! # What is still unknown
//!
//! The meaning of the vtable words, the block-map structure, the unknown
//! words and float words in the texture record, and the link pointers has
//! not been established; they are exposed as raw values. The top two flag
//! bits are reserved. Cube and volume texture data layouts are unobserved.
//! Files inside encrypted archives were not reachable, so every statement
//! above rests on loose files only.
//!
//! # Example
//!
//! ```no_run
//! let bytes = std::fs::read("example.wtd")?;
//! let dict = lf_texture::Dictionary::parse(&bytes)?;
//! for entry in dict.entries() {
//!     println!("{} ({}x{}, {:?}, {} mips)",
//!         entry.name, entry.record.width, entry.record.height,
//!         entry.record.format, entry.record.levels);
//! }
//! # Ok::<(), lf_texture::Error>(())
//! ```

mod dds;
mod decode;
mod error;
mod format;
mod hash;
mod rsc;
mod texture;

pub use dds::to_dds;
pub use decode::{RgbaImage, decode_to_rgba8};
pub use error::Error;
pub use format::{D3DFormat, TextureKind, level_byte_size, level_dims};
pub use hash::{hash_title, title_of};
pub use rsc::{CODEC_DEFLATE, MAGIC as RSC_MAGIC, RESOURCE_TYPE_TEXTURE, Resource};
pub use texture::{Dictionary, Entry, TextureRecord};
