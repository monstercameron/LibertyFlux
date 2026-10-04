//! Parse errors. Every failure carries the byte offset (into the inflated
//! system segment unless stated) where the problem was found.

use core::fmt;

/// Reason a collision file could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The input ended before a complete value could be read.
    Truncated {
        /// Byte offset where more data was needed.
        offset: usize,
        /// How many bytes were needed there.
        needed: usize,
        /// How many bytes the buffer holds in total.
        available: usize,
    },
    /// The 4-byte file magic is not the resource magic.
    BadMagic {
        /// The magic word found, if the input was long enough to hold one.
        found: Option<u32>,
    },
    /// The compression codec id is not one this crate reads.
    UnsupportedCodec {
        /// The codec id found at file offset 12.
        codec: u16,
    },
    /// The zlib payload failed to inflate.
    Decompress {
        /// The underlying decoder message.
        message: String,
    },
    /// The inflated payload is shorter than the segment sizes in the header.
    ShortPayload {
        /// Bytes the header sizes require.
        expected: usize,
        /// Bytes the zlib stream produced.
        actual: usize,
    },
    /// The flags word decodes to a segment size this crate refuses (absurdly
    /// large, which would otherwise mean a huge allocation for garbage input).
    BadSegmentSize {
        /// The decoded size in bytes.
        size: u64,
    },
    /// A stored pointer has a segment tag this crate does not resolve.
    BadPointerTag {
        /// Byte offset of the pointer word.
        offset: usize,
        /// The raw pointer word.
        value: u32,
    },
    /// A stored pointer resolves outside its segment.
    PointerOutOfRange {
        /// Byte offset of the pointer word.
        offset: usize,
        /// The resolved byte offset.
        target: usize,
    },
    /// A count field is negative or unreasonably large.
    BadCount {
        /// Byte offset of the count field.
        offset: usize,
        /// The count found.
        value: i64,
    },
    /// The two dictionary counts disagree (hashes versus bounds).
    CountMismatch {
        /// Number of hashes.
        hashes: usize,
        /// Number of bounds.
        bounds: usize,
    },
    /// Neither a dictionary root nor a single-bound root was recognised.
    BadRoot,
    /// Composite nesting exceeds the recursion limit (shipped files never
    /// nest at all; this only triggers on corrupt or hostile input).
    TooDeeplyNested,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Truncated {
                offset,
                needed,
                available,
            } => write!(
                f,
                "truncated input at offset {offset}: needed {needed} bytes, have {available}"
            ),
            Error::BadMagic { found } => match found {
                Some(w) => write!(f, "bad magic: found {w:#010x}, expected RSC version 5"),
                None => write!(f, "bad magic: input shorter than 4 bytes"),
            },
            Error::UnsupportedCodec { codec } => {
                write!(f, "unsupported compression codec {codec:#06x}")
            }
            Error::Decompress { message } => write!(f, "zlib inflate failed: {message}"),
            Error::ShortPayload { expected, actual } => write!(
                f,
                "inflated payload too short: need {expected} bytes, got {actual}"
            ),
            Error::BadSegmentSize { size } => {
                write!(f, "refusing absurd segment size of {size} bytes")
            }
            Error::BadPointerTag { offset, value } => write!(
                f,
                "bad pointer tag at offset {offset}: word {value:#010x} (want tag 5 or null)"
            ),
            Error::PointerOutOfRange { offset, target } => write!(
                f,
                "pointer at offset {offset} resolves to {target}, outside the segment"
            ),
            Error::BadCount { offset, value } => {
                write!(f, "bad count at offset {offset}: {value}")
            }
            Error::CountMismatch { hashes, bounds } => write!(
                f,
                "dictionary count mismatch: {hashes} hashes but {bounds} bounds"
            ),
            Error::BadRoot => write!(f, "root is neither a bound dictionary nor a single bound"),
            Error::TooDeeplyNested => write!(f, "composite nesting exceeds the recursion limit"),
        }
    }
}

impl std::error::Error for Error {}
