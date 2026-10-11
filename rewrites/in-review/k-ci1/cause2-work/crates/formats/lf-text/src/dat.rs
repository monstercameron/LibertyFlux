//! Shared helpers for the `#`-commented front-end data files, plus the
//! parser for the `frontend*.dat` layout files.
//!
//! `frontend.dat`, `frontend_pc.dat` and `frontend_360.dat` hold per display
//! mode (`[HD]`, `[CRT]`) rows of a layout key followed by one or more
//! floating-point values.

use std::fmt;

/// One data line with its 1-based line number, comments stripped.
#[derive(Debug, Clone)]
pub struct DataLine {
    /// 1-based line number in the source file.
    pub line: usize,
    /// Line content without comments or surrounding whitespace.
    pub text: String,
}

/// Split a data file into significant lines.
///
/// Lines that are empty or start with `#` are dropped; anything from `#` or
/// `//` onwards is treated as a comment. Returns the lines with numbers so
/// callers can report errors precisely.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn data_file_lines(data: &[u8]) -> Result<Vec<DataLine>, DataError> {
    // Comment lines carry raw non-UTF-8 bytes in some files; decode lossily
    // so that comments never break parsing of the data.
    let text = String::from_utf8_lossy(data);
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let hash = line.find('#').unwrap_or(line.len());
        let slash = line.find("//").unwrap_or(line.len());
        let body = line[..hash.min(slash)].trim();
        if body.is_empty() {
            continue;
        }
        out.push(DataLine {
            line: index + 1,
            text: body.to_owned(),
        });
    }
    Ok(out)
}

/// Error for the plain data-file parsers.
#[derive(Debug, Clone, PartialEq)]
pub enum DataError {
    /// The input is not valid UTF-8.
    NotUtf8,
    /// A line does not match the expected shape.
    BadLine {
        /// 1-based line number.
        line: usize,
    },
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUtf8 => write!(f, "file is not valid UTF-8"),
            Self::BadLine { line } => write!(f, "malformed data on line {line}"),
        }
    }
}

impl std::error::Error for DataError {}

/// Name inside a `[NAME]` header, if the line is one.
pub fn section_name(line: &str) -> Option<&str> {
    let inner = line.strip_prefix('[')?.strip_suffix(']')?;
    if inner.is_empty() || inner.contains(['[', ']']) {
        return None;
    }
    Some(inner)
}

/// One layout section: a display mode plus its key/value rows.
#[derive(Debug, Clone)]
pub struct LayoutSection {
    /// Section name, e.g. `HD`.
    pub name: String,
    /// Rows in file order.
    pub values: Vec<LayoutValue>,
}

/// One layout row: a key plus floating-point values.
#[derive(Debug, Clone)]
pub struct LayoutValue {
    /// Layout key, e.g. `MID_background_opacity`.
    pub key: String,
    /// Values on the row (usually one or two).
    pub numbers: Vec<f32>,
}

/// A parsed `frontend*.dat` layout file.
#[derive(Debug, Clone)]
pub struct FrontendLayout {
    /// Sections in file order.
    pub sections: Vec<LayoutSection>,
}

impl FrontendLayout {
    /// Parse a `frontend*.dat` file from its bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &[u8]) -> Result<Self, DataError> {
        let mut sections = Vec::new();
        let mut current: Option<LayoutSection> = None;
        for item in data_file_lines(data)? {
            if let Some(name) = section_name(&item.text) {
                if let Some(done) = current.take() {
                    sections.push(done);
                }
                current = Some(LayoutSection {
                    name: name.to_owned(),
                    values: Vec::new(),
                });
                continue;
            }
            let section = current
                .as_mut()
                .ok_or(DataError::BadLine { line: item.line })?;
            let mut parts = item.text.split_whitespace();
            let key = parts.next().ok_or(DataError::BadLine { line: item.line })?;
            let mut numbers = Vec::new();
            for part in parts {
                numbers.push(
                    part.parse::<f32>()
                        .map_err(|_| DataError::BadLine { line: item.line })?,
                );
            }
            if numbers.is_empty() {
                return Err(DataError::BadLine { line: item.line });
            }
            section.values.push(LayoutValue {
                key: key.to_owned(),
                numbers,
            });
        }
        if let Some(done) = current.take() {
            sections.push(done);
        }
        Ok(Self { sections })
    }

    /// Find a section by name.
    #[must_use]
    pub fn section(&self, name: &str) -> Option<&LayoutSection> {
        self.sections.iter().find(|s| s.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_layout() {
        let input = b"# c\n[HD]\nMID_background_opacity 160.0 0.0\nFAD_main_fade_time 200\n";
        let file = FrontendLayout::parse(input).unwrap();
        assert_eq!(file.sections.len(), 1);
        let section = &file.sections[0];
        assert_eq!(section.name, "HD");
        assert_eq!(section.values.len(), 2);
        assert_eq!(section.values[0].key, "MID_background_opacity");
        assert_eq!(section.values[0].numbers, vec![160.0, 0.0]);
        assert!(file.section("HD").is_some());
        assert!(file.section("CRT").is_none());
    }

    #[test]
    fn rejects_row_before_section() {
        let err = FrontendLayout::parse(b"KEY 1.0\n").unwrap_err();
        assert!(matches!(err, DataError::BadLine { .. }));
    }
}
