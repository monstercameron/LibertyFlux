//! Error type for [`crate`]: every failure is a value, never a panic.

use std::fmt;

/// Every way parsing or decoding can fail.
#[derive(Debug)]
pub enum Error {
    /// Input ended before a complete value could be read.
    TooShort {
        /// Which structure was being read.
        what: &'static str,
    },
    /// The RSC magic word did not match.
    BadMagic {
        /// The magic word found.
        found: u32,
    },
    /// The resource type is not a texture dictionary.
    BadType {
        /// The type word found.
        found: u32,
    },
    /// The codec field is not the deflate/zlib marker.
    BadCodec {
        /// The codec field found.
        found: u16,
    },
    /// zlib decompression failed.
    Inflate(String),
    /// Decompressed size differs from the sizes in the flags word.
    SizeMismatch {
        /// Expected system plus graphics bytes.
        expected: u64,
        /// Bytes actually produced.
        got: u64,
    },
    /// A segment would be larger than the sanity cap.
    SegmentTooLarge {
        /// Which segment.
        what: &'static str,
        /// Requested size in bytes.
        size: u64,
    },
    /// A tagged offset had the wrong marker nibble.
    BadOffset {
        /// Which pointer was being read.
        what: &'static str,
        /// The raw pointer value.
        value: u32,
    },
    /// An offset points outside its segment.
    OutOfBounds {
        /// Which structure was being read.
        what: &'static str,
        /// Byte offset requested.
        offset: u64,
        /// Segment length.
        len: usize,
    },
    /// A texture name is not valid UTF-8.
    InvalidName,
    /// A name string runs past the end of the system segment.
    UnterminatedName,
    /// Pixel data for an unknown format code was requested.
    UnknownFormat(u32),
    /// A mip level index is past the texture's level count.
    BadLevel {
        /// Level requested.
        level: u8,
        /// Levels stored.
        levels: u8,
    },
    /// Arithmetic overflow while sizing pixel data.
    Overflow,
    /// Reading from a stream failed.
    Io(std::io::Error),
    /// DDS export does not support this texture (cube/volume).
    DdsUnsupported(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TooShort { what } => write!(f, "input too short while reading {what}"),
            Error::BadMagic { found } => write!(f, "bad RSC magic: {found:#010x}"),
            Error::BadType { found } => write!(f, "not a texture dictionary (type {found})"),
            Error::BadCodec { found } => write!(f, "unsupported codec field {found:#06x}"),
            Error::Inflate(msg) => write!(f, "zlib decompression failed: {msg}"),
            Error::SizeMismatch { expected, got } => {
                write!(f, "decompressed {got} bytes, flags word says {expected}")
            }
            Error::SegmentTooLarge { what, size } => {
                write!(f, "{what} segment too large: {size} bytes")
            }
            Error::BadOffset { what, value } => {
                write!(f, "bad {what} pointer marker: {value:#010x}")
            }
            Error::OutOfBounds { what, offset, len } => {
                write!(f, "{what} at offset {offset} past segment end {len}")
            }
            Error::InvalidName => write!(f, "texture name is not valid UTF-8"),
            Error::UnterminatedName => {
                write!(f, "texture name runs past the segment end")
            }
            Error::UnknownFormat(code) => {
                write!(f, "unknown pixel format code {code:#010x}")
            }
            Error::BadLevel { level, levels } => {
                write!(f, "mip level {level} requested, texture has {levels}")
            }
            Error::Overflow => write!(f, "size arithmetic overflow"),
            Error::Io(e) => write!(f, "read error: {e}"),
            Error::DdsUnsupported(what) => write!(f, "DDS export does not support {what}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
