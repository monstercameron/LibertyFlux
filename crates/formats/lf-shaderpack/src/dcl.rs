//! Parser for the `.dcl` vertex declaration sidecars.
//!
//! Each shader has one `.dcl` file whose stem matches the container name.
//! The body is one decimal bitmask citing the engine's vertex channel
//! header, plus a comment repeating the channel names, for example
//! `89 ; ...` followed by `; position diffuse texcoord0 normal`. The mask
//! bit assignment itself lives in engine code we have not recovered, so
//! the mask is exposed raw alongside the comment's channel words.

/// A parsed vertex declaration: bitmask plus channel words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexDecl {
    /// The declaration bitmask (observed values include 1, 89, 217, 16473).
    pub mask: u32,
    /// Channel words from the trailing comment, if any.
    pub channels: Vec<String>,
}

/// Reasons a `.dcl` parse can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DclError {
    /// No text at all.
    Empty,
    /// The leading field is not a decimal integer.
    BadMask(String),
}

impl std::fmt::Display for DclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DclError::Empty => write!(f, "empty vertex declaration"),
            DclError::BadMask(s) => write!(f, "bad declaration mask {s:?}"),
        }
    }
}

impl std::error::Error for DclError {}

/// Parse `.dcl` text: a leading decimal mask, then `;` comments whose last
/// line may list channel names.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_dcl(text: &str) -> Result<VertexDecl, DclError> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let first = lines.next().ok_or(DclError::Empty)?;
    let mask_text = first.split(';').next().unwrap_or("").trim();
    let mask: u32 = mask_text
        .parse()
        .map_err(|_| DclError::BadMask(mask_text.to_owned()))?;
    // Channel words come from the last comment in the file; every shipped
    // declaration carries them on its second line.
    let mut channels = Vec::new();
    for line in text.lines() {
        if let Some((_, comment)) = line.split_once(';') {
            let words: Vec<String> = comment.split_whitespace().map(str::to_owned).collect();
            if !words.is_empty() && !words.iter().any(|w| w.contains('/') || w.contains('.')) {
                channels = words;
            }
        }
    }
    Ok(VertexDecl { mask, channels })
}
