//! Read-only parser for the game's audio configuration and metadata tables.
//!
//! The audio engine is driven by two families of files that live next to the
//! sound banks:
//!
//! * Versioned metadata files (`effects.dat11`, `curves.dat12`,
//!   `categories.dat15`, `sounds.dat15`, `game.dat16`, plus per-episode
//!   variants): a shared container holding an object blob, an archive-name
//!   list, an object directory and relocation tables. See [`container`].
//! * Speech lookup files (`speech.dat` and per-episode variants): a flat
//!   voice/context/bank table with no version header. See [`speech`].
//!
//! Object bodies are decoded with the per-file schemas in [`schema`], which
//! name every object type and field. Decoding is lenient by design: bytes the
//! schema does not describe are returned as trailing bytes on the decoded
//! object instead of failing the parse, so a partially understood file still
//! yields everything known plus an explicit record of what is unknown.
//!
//! # Versioned container layout
//!
//! All integers are little-endian. All offsets are byte offsets.
//!
//! ```text
//! offset  size  field
//! 0x00    4     version suffix: 11 (effects), 12 (curves), 15 (categories
//!               and sounds) or 16 (game)
//! 0x04    4     object-blob size N in bytes
//! 0x08    N     object blob: packed objects, each preceded by one zero
//!               padding byte; see below
//! ...     4     archive block size M (counted after this word)
//! ...     4     archive count A
//! ...     4*A   archive-name offsets, relative to the start of the name heap
//! ...     rest  archive-name heap: null-terminated "ARCHIVE\\BANK" paths
//! ...     4     object count O
//! ...     4     sum of object-name lengths plus one per object
//! ...     var   O directory entries: u8 name length, name bytes (no
//!               terminator), u32 blob offset, u32 object size
//! ...     4     hash-relocation count H
//! ...     4*H   absolute file offsets of hashed cross-reference fields
//! ...     4     archive-relocation count R
//! ...     4*R   absolute file offsets of hashed archive-reference fields
//! ```
//!
//! The file ends exactly after the relocation tables. A blob offset of zero
//! with an object count of zero is legal (an empty episode patch file).
//!
//! # Object layout
//!
//! Each object in the blob starts with a u8 type id, then a u32 name offset
//! (running total of earlier object-name lengths plus one per object; it
//! mirrors the directory and carries no extra information), then the
//! file-wide header fields, then the type-specific fields. Field encoding:
//!
//! * fixed integers and little-endian floats;
//! * `hash`: u32 Jenkins hash of the target name (see [`hash`]);
//! * counted strings and arrays with a u8, u16 or u32 length prefix;
//! * optional bitfields: a u32 presence mask followed by the present fields
//!   in schema order (bit `i` guards the `i`-th child).
//!
//! # Speech layout
//!
//! ```text
//! offset  size  field
//! 0x00    4     variation-data blob size V
//! 0x04    V     variation-data blob (per-context byte runs)
//! ...     4     context count C
//! ...     14*C  contexts: u32 bank-name index, i32 variation-data offset
//!               (-1 when the context has no variations), u32 name hash,
//!               u8 context data, u8 variation count
//! ...     4     voice count
//! ...     10*   voices: u32 first-context byte offset (divide by 14 for the
//!               index), u32 name hash, u16 context count
//! ...     4     bank-name count S
//! ...     4*S   heap offsets, relative to the heap start
//! ...     rest  heap: null-terminated "ARCHIVE\\BANK" paths
//! ```
//!
//! Contexts are laid out voice by voice: a voice owns
//! `context count` entries starting at its first-context index.
//!
//! # Cross-references
//!
//! Hashes name objects in the same file (child sounds, track lists, category
//! parents) or well-known engine variables and curves. Archive hashes name
//! `ARCHIVE\BANK` paths from the file's archive table (versioned files) or
//! bank table (speech files). [`hash::name_hash`] resolves a candidate name
//! to its hash; files do not store the reverse mapping.
//!
//! # What is still unknown
//!
//! * Several game object types have fixed-size bodies whose fields are not
//!   understood (helicopter, melee, boat, footsteps, clothing); two more are
//!   variable-length with only the rough shape known (scripted reports look
//!   like a count plus 7-byte entries; train stations look like a struct plus
//!   a counted name).
//! * One curve object carries 12 trailing bytes past its schema, and the
//!   meaning of most header flag words is unknown.
//! * Speech context-data bytes, variation-data bytes and voice-name hashes
//!   have no public description; voices and contexts are anonymous hashes.
//!
//! # Example
//!
//! ```no_run
//! use lf_audio_config::{container::MetaFile, schema::Schema, decode::decode_object};
//!
//! let bytes = std::fs::read("sounds.dat15").unwrap();
//! let file = MetaFile::parse(&bytes).unwrap();
//! let schema = Schema::detect("sounds.dat15").unwrap();
//! for entry in file.objects() {
//!     let obj = decode_object(&schema, entry).unwrap();
//!     println!("{}: {:?}", entry.name(), obj.type_name);
//! }
//! ```

pub mod container;
pub mod decode;
pub mod hash;
pub mod schema;
pub mod speech;

use core::fmt;

/// What went wrong while parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// The input ended before the value was complete.
    Truncated,
    /// The input is well-formed bytes but not this format.
    Invalid,
}

/// A parse failure: where it happened, what kind, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    offset: u64,
    kind: ErrorKind,
    message: String,
}

impl Error {
    /// Build an error at byte `offset` with a static message.
    pub fn new(offset: u64, kind: ErrorKind, message: impl Into<String>) -> Error {
        Error {
            offset,
            kind,
            message: message.into(),
        }
    }

    /// Byte offset in the input where parsing failed.
    #[must_use]
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// The failure kind.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Human-readable detail; never contains game content.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "at byte {}: {:?}: {}",
            self.offset, self.kind, self.message
        )
    }
}

impl std::error::Error for Error {}

/// Little-endian cursor over a byte slice.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Cursor<'a> {
        Cursor { buf, pos: 0 }
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    fn need(&self, n: usize, what: &str) -> Result<(), Error> {
        if self.remaining() < n {
            Err(Error::new(self.pos as u64, ErrorKind::Truncated, what))
        } else {
            Ok(())
        }
    }

    pub(crate) fn u8(&mut self, what: &'static str) -> Result<u8, Error> {
        self.need(1, what)?;
        let v = self.buf[self.pos];
        self.pos += 1;
        Ok(v)
    }

    pub(crate) fn bytes(&mut self, n: usize, what: &'static str) -> Result<&'a [u8], Error> {
        self.need(n, what)?;
        let v = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(v)
    }

    pub(crate) fn u16(&mut self, what: &'static str) -> Result<u16, Error> {
        let b = self.bytes(2, what)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    pub(crate) fn u32(&mut self, what: &'static str) -> Result<u32, Error> {
        let b = self.bytes(4, what)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub(crate) fn i16(&mut self, what: &'static str) -> Result<i16, Error> {
        Ok(i16::from_ne_bytes(self.u16(what)?.to_ne_bytes()))
    }

    pub(crate) fn i32(&mut self, what: &'static str) -> Result<i32, Error> {
        Ok(i32::from_ne_bytes(self.u32(what)?.to_ne_bytes()))
    }

    pub(crate) fn i8(&mut self, what: &'static str) -> Result<i8, Error> {
        Ok(i8::from_ne_bytes(self.u8(what)?.to_ne_bytes()))
    }

    pub(crate) fn f32(&mut self, what: &'static str) -> Result<f32, Error> {
        Ok(f32::from_bits(self.u32(what)?))
    }
}
