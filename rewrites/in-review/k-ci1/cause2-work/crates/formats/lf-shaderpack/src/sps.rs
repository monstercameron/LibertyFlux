//! Parser for the `.sps` material preset sidecars.
//!
//! A preset names a shader, then overrides material values in brace blocks:
//!
//! ```text
//! shader gta_default
//! __rage_drawbucket {
//!     int 1
//! }
//! ```
//!
//! Only a handful of override names exist in shipped files (draw-bucket
//! ids and a few specular terms). Notably, these names do not match any
//! `.fxc` parameter name: the engine resolves presets through a different
//! path, so this module keeps values as raw text rather than typing them.

/// One `name { type values }` override inside a preset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Override {
    /// Override name (`__rage_drawbucket`, `SpecularColor`, ...).
    pub name: String,
    /// Value type word (`int`, `float`, ...).
    pub type_name: String,
    /// The values as raw text, whitespace-trimmed.
    pub values: String,
}

/// A parsed material preset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preset {
    /// Shader name from the leading `shader <name>` line.
    pub shader: String,
    /// Overrides in file order.
    pub overrides: Vec<Override>,
}

/// Reasons a preset parse can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpsError {
    /// No `shader <name>` first line.
    MissingHeader,
    /// A block that never closes, or text in an unexpected shape.
    Malformed {
        /// 1-based line number where parsing gave up.
        line: usize,
    },
}

impl std::fmt::Display for SpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpsError::MissingHeader => write!(f, "preset has no leading shader line"),
            SpsError::Malformed { line } => write!(f, "malformed preset near line {line}"),
        }
    }
}

impl std::error::Error for SpsError {}

/// Parse `.sps` preset text.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_preset(text: &str) -> Result<Preset, SpsError> {
    let lines: Vec<&str> = text.lines().collect();
    let mut idx = 0;
    while idx < lines.len() && lines[idx].trim().is_empty() {
        idx += 1;
    }
    if idx >= lines.len() {
        return Err(SpsError::MissingHeader);
    }
    let header = lines[idx].trim();
    let shader = header
        .strip_prefix("shader")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or(SpsError::MissingHeader)?;
    idx += 1;
    let mut overrides = Vec::new();
    while idx < lines.len() {
        let line = lines[idx].trim();
        idx += 1;
        if line.is_empty() {
            continue;
        }
        let name = line
            .strip_suffix('{')
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let Some(name) = name else {
            return Err(SpsError::Malformed { line: idx });
        };
        let mut body = Vec::new();
        let mut closed = false;
        while idx < lines.len() {
            let inner = lines[idx].trim();
            idx += 1;
            if inner.starts_with('}') {
                closed = true;
                break;
            }
            if !inner.is_empty() {
                body.push(inner);
            }
        }
        if !closed {
            return Err(SpsError::Malformed { line: idx });
        }
        let joined = body.join(" ");
        let mut words = joined.split_whitespace();
        let type_name = words.next().unwrap_or("").to_owned();
        let values = words.collect::<Vec<_>>().join(" ");
        overrides.push(Override {
            name: name.to_owned(),
            type_name,
            values,
        });
    }
    Ok(Preset {
        shader: shader.to_owned(),
        overrides,
    })
}
