//! Validation for the Direct3D 9 shader bytecode inside each program.
//!
//! The bytecode layout is public Microsoft documentation: a version token,
//! then instruction and comment tokens, then the end token `0x0000FFFF`.
//! A comment token carries `0xFFFE` in its low word and its dword length
//! in its high word; any other token is an instruction whose dword length
//! after the opcode sits in bits 27:24. One of the comment blobs usually
//! holds the constant table (`CTAB`) with the compiler version string.

use crate::Stage;

/// End-of-shader token.
pub const END: u32 = 0x0000_FFFF;

/// What a bytecode walk reports about one program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytecodeInfo {
    /// Vertex or pixel stage, from the version token's high word.
    pub stage: Stage,
    /// Shader model major version (3 in every shipped file).
    pub major: u8,
    /// Shader model minor version (0 in every shipped file).
    pub minor: u8,
    /// Instructions walked, not counting comments or the end token.
    pub instr_count: usize,
    /// Offset just past the end token; equals the blob length when the
    /// stored size is exact.
    pub end_offset: usize,
    /// Whether a `CTAB` constant-table blob is present.
    pub has_ctab: bool,
}

/// Reasons bytecode validation can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BytecodeError {
    /// Fewer than 4 bytes, or length not a multiple of 4.
    TooShort,
    /// First dword is not a recognised `vs_*`/`ps_*` version token.
    BadVersion(u32),
    /// A comment token runs past the end of the blob.
    TruncatedComment {
        /// Offset of the offending token.
        offset: usize,
    },
    /// An instruction runs past the end of the blob.
    TruncatedInstruction {
        /// Offset of the offending token.
        offset: usize,
    },
    /// Tokens ran out without an end token.
    MissingEnd,
}

impl std::fmt::Display for BytecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeError::TooShort => write!(f, "bytecode shorter than one token"),
            BytecodeError::BadVersion(v) => write!(f, "bad version token 0x{v:08x}"),
            BytecodeError::TruncatedComment { offset } => {
                write!(f, "comment token at offset {offset} runs past the end")
            }
            BytecodeError::TruncatedInstruction { offset } => {
                write!(f, "instruction at offset {offset} runs past the end")
            }
            BytecodeError::MissingEnd => write!(f, "bytecode has no end token"),
        }
    }
}

impl std::error::Error for BytecodeError {}

fn dword_at(blob: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        blob[offset],
        blob[offset + 1],
        blob[offset + 2],
        blob[offset + 3],
    ])
}

/// Walk a Direct3D 9 token stream to its end token.
///
/// Accepts any `vs_`/`ps_` major version 1-3; the container holds 3.0 only,
/// but the walk does not assume that. Returns the classification plus the
/// instruction count and end offset.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn validate(blob: &[u8]) -> Result<BytecodeInfo, BytecodeError> {
    if blob.len() < 4 || !blob.len().is_multiple_of(4) {
        return Err(BytecodeError::TooShort);
    }
    let version = dword_at(blob, 0);
    let stage = match version & 0xFFFF_0000 {
        0xFFFE_0000 => Stage::Vertex,
        0xFFFF_0000 => Stage::Pixel,
        _ => return Err(BytecodeError::BadVersion(version)),
    };
    let major = ((version >> 8) & 0xFF) as u8;
    let minor = (version & 0xFF) as u8;
    if !(1..=3).contains(&major) {
        return Err(BytecodeError::BadVersion(version));
    }
    let mut pos = 4usize;
    let mut instr_count = 0usize;
    while pos + 4 <= blob.len() {
        let token = dword_at(blob, pos);
        if token == END {
            return Ok(BytecodeInfo {
                stage,
                major,
                minor,
                instr_count,
                end_offset: pos + 4,
                has_ctab: find_fourcc(blob, *b"CTAB").is_some(),
            });
        }
        if token & 0xFFFF == 0xFFFE {
            let dwords = ((token >> 16) & 0xFFFF) as usize;
            let next = pos
                .saturating_add(4)
                .saturating_add(dwords.saturating_mul(4));
            if next > blob.len() {
                return Err(BytecodeError::TruncatedComment { offset: pos });
            }
            pos = next;
        } else {
            let params = ((token >> 24) & 0x0F) as usize;
            let next = pos.saturating_add(4 * (1 + params));
            if next > blob.len() {
                return Err(BytecodeError::TruncatedInstruction { offset: pos });
            }
            pos = next;
            instr_count += 1;
        }
    }
    Err(BytecodeError::MissingEnd)
}

/// Find a four-character code (such as `CTAB`) at 4-byte alignment.
/// Returns the byte offset of the tag, if present.
#[must_use]
pub fn find_fourcc(blob: &[u8], tag: [u8; 4]) -> Option<usize> {
    blob.chunks_exact(4).position(|w| w == tag).map(|i| i * 4)
}
