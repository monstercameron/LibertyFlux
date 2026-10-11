//! Error type for save game container parsing.

use std::fmt;

/// Every way parsing a save game file can fail.
///
/// All variants carry the offending values; none panic. Malformed input
/// always surfaces here rather than aborting.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Fewer bytes than the structure being read.
    TooShort {
        /// What was being read, for the error message.
        what: &'static str,
        /// Bytes available.
        len: usize,
        /// Bytes required.
        need: usize,
    },
    /// A magic word did not match.
    BadMagic {
        /// What was being read, for the error message.
        what: &'static str,
        /// The bytes actually found.
        found: Vec<u8>,
    },
    /// A block's total size is smaller than its own 9-byte header.
    BadBlockSize {
        /// Zero-based block position in the file.
        index: usize,
        /// File offset of the block.
        offset: u64,
        /// The declared size.
        size: u32,
    },
    /// A block's declared range runs past the end of the file.
    BlockOverrun {
        /// Zero-based block position in the file.
        index: usize,
        /// File offset of the block.
        offset: u64,
        /// The declared size.
        size: u32,
        /// Total file length.
        file_len: usize,
    },
    /// A declared size exceeds the crate's sanity cap.
    TooLarge {
        /// What was too large, for the error message.
        what: &'static str,
        /// The declared size.
        size: u64,
    },
    /// An underlying I/O operation failed.
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TooShort { what, len, need } => {
                write!(f, "{what} too short: {len} bytes, need {need}")
            }
            Error::BadMagic { what, found } => {
                write!(f, "bad {what} magic: found {found:02X?}")
            }
            Error::BadBlockSize {
                index,
                offset,
                size,
            } => {
                write!(
                    f,
                    "block {index} at offset {offset} has impossible size {size}"
                )
            }
            Error::BlockOverrun {
                index,
                offset,
                size,
                file_len,
            } => {
                write!(
                    f,
                    "block {index} at offset {offset} with size {size} overruns file ({file_len} bytes)"
                )
            }
            Error::TooLarge { what, size } => {
                write!(f, "{what} too large: {size}")
            }
            Error::Io(e) => write!(f, "i/o error: {e}"),
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
