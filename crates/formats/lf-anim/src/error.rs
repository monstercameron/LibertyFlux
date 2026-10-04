//! Errors returned when parsing animation dictionaries.

use std::fmt;

/// Errors returned when parsing animation dictionaries.
///
/// Every variant carries the values needed to diagnose the file; malformed
/// input always yields an `Err`, never a panic.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The bytes are not a usable RSC5 resource.
    Resource(lf_resource::Error),
    /// The resource type id is not the generic id animation uses.
    UnexpectedKind {
        /// The type id found in the file.
        found: u32,
    },
    /// The resource carries a graphics segment; animation has none.
    UnexpectedGraphics {
        /// Graphics segment length in bytes.
        len: usize,
    },
    /// A header word did not hold an expected value.
    BadHeader {
        /// Byte offset of the word, for the error message.
        offset: usize,
        /// What was expected, for the error message.
        expected: &'static str,
        /// The value actually found.
        found: u32,
    },
    /// A count word was internally inconsistent or absurd.
    BadCount {
        /// Byte offset of the word, for the error message.
        offset: usize,
        /// The value actually found.
        found: u32,
    },
    /// A name was not valid text.
    BadName {
        /// Byte offset of the name, for the error message.
        offset: usize,
    },
    /// A channel tag has no decoder in this crate.
    UnknownCodec {
        /// The tag word found in the file.
        tag: u32,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Resource(e) => write!(f, "resource error: {e}"),
            Error::UnexpectedKind { found } => {
                write!(f, "not an animation dictionary: kind {found:#x}, want 0x1")
            }
            Error::UnexpectedGraphics { len } => {
                write!(f, "animation with graphics segment ({len} bytes)")
            }
            Error::BadHeader {
                offset,
                expected,
                found,
            } => {
                write!(
                    f,
                    "bad header word at {offset:#x}: want {expected}, found {found:#x}"
                )
            }
            Error::BadCount { offset, found } => {
                write!(f, "bad count at {offset:#x}: {found:#x}")
            }
            Error::BadName { offset } => {
                write!(f, "bad clip name at {offset:#x}")
            }
            Error::UnknownCodec { tag } => {
                write!(f, "channel codec {tag:#x} has no decoder")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Resource(e) => Some(e),
            _ => None,
        }
    }
}

impl From<lf_resource::Error> for Error {
    fn from(e: lf_resource::Error) -> Self {
        Error::Resource(e)
    }
}
