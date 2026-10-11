//! Error type for [`crate`] resource parsing.

use std::fmt;

/// Every way parsing an RSC5 resource can fail.
///
/// All variants carry the offending values; none panic. Malformed input
/// always surfaces here rather than aborting.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Fewer bytes than the 14-byte header.
    TooShort {
        /// Bytes available.
        len: usize,
    },
    /// First word is not the RSC5 magic (and not the big-endian form).
    BadMagic {
        /// The word read at offset 0.
        found: u32,
    },
    /// Big-endian (console) file: magic bytes read as `0x52534305`.
    /// Only little-endian PC files are supported.
    BigEndian,
    /// Compression codec id is not one this crate can inflate.
    UnsupportedCodec {
        /// The codec id read at offset 12.
        codec: u16,
    },
    /// The zlib stream failed to inflate.
    Decompress {
        /// Underlying I/O error message.
        message: String,
    },
    /// The inflated payload is not exactly system + graphics bytes.
    LengthMismatch {
        /// Size the flags word promises.
        expected: usize,
        /// Bytes actually produced.
        actual: usize,
    },
    /// The flags word (or a checked computation on it) overflows `usize`,
    /// or the promised payload exceeds the safety cap.
    TooLarge {
        /// Promised payload size in bytes.
        size: u64,
    },
    /// A pointer has a null or unexpected segment tag for the operation.
    BadPointer {
        /// The raw pointer value.
        value: u32,
    },
    /// A pointer plus length runs past the end of its segment.
    OutOfBounds {
        /// The raw pointer value.
        value: u32,
        /// Bytes requested.
        len: usize,
        /// Bytes available in the segment.
        segment_len: usize,
    },
    /// Underlying read error when parsing from a stream.
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TooShort { len } => {
                write!(f, "file too short for RSC5 header: {len} bytes")
            }
            Error::BadMagic { found } => {
                write!(f, "not an RSC5 resource (magic {found:#010x})")
            }
            Error::BigEndian => {
                write!(f, "big-endian (console) resource; only PC files supported")
            }
            Error::UnsupportedCodec { codec } => {
                write!(f, "unsupported compression codec {codec:#06x}")
            }
            Error::Decompress { message } => {
                write!(f, "zlib decompression failed: {message}")
            }
            Error::LengthMismatch { expected, actual } => {
                write!(f, "inflated {actual} bytes but flags promise {expected}")
            }
            Error::TooLarge { size } => {
                write!(f, "promised payload too large: {size} bytes")
            }
            Error::BadPointer { value } => {
                write!(f, "bad resource pointer {value:#010x}")
            }
            Error::OutOfBounds {
                value,
                len,
                segment_len,
            } => write!(
                f,
                "pointer {value:#010x} + {len} bytes exceeds segment ({segment_len} bytes)"
            ),
            Error::Io(e) => write!(f, "read error: {e}"),
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
