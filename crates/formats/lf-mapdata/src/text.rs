//! Shared parser for the sectioned text formats (`.ide`, `.ipl`).
//!
//! Both formats are a flat sequence of `name ... end` sections whose records
//! are comma-separated lines. This module preserves every record's raw fields;
//! the [`crate::ide`] and [`crate::ipl`] modules give them types.

use crate::Error;

/// A parsed sectioned text file: the section list in file order.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFile {
    /// Sections in the order they appeared, including empty ones.
    pub sections: Vec<TextSection>,
}

/// One `name ... end` section with its raw record lines.
#[derive(Debug, Clone, PartialEq)]
pub struct TextSection {
    /// Section name exactly as written (usually lower case).
    pub name: String,
    /// 1-based line number of the section header.
    pub line: usize,
    /// Records in file order; comments and blank lines are already removed.
    pub records: Vec<TextRecord>,
}

/// One record line: comma-separated fields plus its line number.
#[derive(Debug, Clone, PartialEq)]
pub struct TextRecord {
    /// 1-based line number in the source file.
    pub line: usize,
    /// Fields split on commas, trimmed, with surrounding quotes removed.
    /// A doubled comma yields an empty field, which typed parsers skip.
    pub fields: Vec<String>,
}

impl TextFile {
    /// Parse sectioned text from a byte slice.
    ///
    /// Returns an error only when the input is not valid UTF-8. Unknown
    /// sections are kept, not rejected, and a section open at end of input is
    /// closed silently (two shipped files lack the final `end`).
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    ///
    /// # Panics
    ///
    /// Never panics: the only unwrap takes an option that is always populated on this path.
    pub fn parse(bytes: &[u8]) -> Result<TextFile, Error> {
        let text = std::str::from_utf8(bytes).map_err(|e| Error::Utf8 {
            offset: Some(e.valid_up_to()),
        })?;
        let mut sections = Vec::new();
        let mut current: Option<TextSection> = None;
        for (idx, raw_line) in text.lines().enumerate() {
            let line_no = idx + 1;
            let line = strip_comment(raw_line).trim();
            if line.is_empty() {
                continue;
            }
            if let Some(sec) = current.as_mut() {
                if line.eq_ignore_ascii_case("end") {
                    sections.push(current.take().unwrap());
                } else {
                    sec.records.push(TextRecord {
                        line: line_no,
                        fields: split_fields(line),
                    });
                }
            } else {
                if line.eq_ignore_ascii_case("end") {
                    // Stray `end` outside a section; ignore it.
                    continue;
                }
                current = Some(TextSection {
                    name: line.to_string(),
                    line: line_no,
                    records: Vec::new(),
                });
            }
        }
        // Two shipped files end mid-section without a final `end`; tolerate it.
        if let Some(sec) = current {
            sections.push(sec);
        }
        Ok(TextFile { sections })
    }

    /// Find a section by case-insensitive name.
    #[must_use]
    pub fn section(&self, name: &str) -> Option<&TextSection> {
        self.sections
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(name))
    }
}

/// Cut a `#` comment (outside double quotes) and full-line `//` comments.
fn strip_comment(line: &str) -> &str {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") {
        return "";
    }
    let mut in_quotes = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_quotes = !in_quotes,
            '#' if !in_quotes => return &line[..i],
            _ => {}
        }
    }
    line
}

/// Split a record line on commas, keeping quoted spans together.
///
/// Each field is trimmed of whitespace and one layer of surrounding double
/// quotes is removed.
#[must_use]
pub fn split_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut buf = String::new();
    let mut in_quotes = false;
    for c in line.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                buf.push(c);
            }
            ',' if !in_quotes => {
                fields.push(clean_field(&buf));
                buf.clear();
            }
            _ => buf.push(c),
        }
    }
    fields.push(clean_field(&buf));
    fields
}

fn clean_field(raw: &str) -> String {
    let t = raw.trim();
    if t.len() >= 2 && t.starts_with('"') && t.ends_with('"') {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

/// Split comma-separated fields further on whitespace, dropping empties.
///
/// This matches the game's documented comma-to-space handling for rows where a
/// comma was left out (several shipped `cars`, `peds` and `weap` rows rely on
/// it). Sections whose names may contain spaces (IPL `blok`) must not use this.
#[must_use]
pub fn flatten_fields(fields: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for f in fields {
        for tok in f.split_whitespace() {
            out.push(tok.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_two_sections() {
        let f = TextFile::parse(b"objs\na, b, 1\nend\ntxdp\nx, y\nend\n").unwrap();
        assert_eq!(f.sections.len(), 2);
        assert_eq!(f.sections[0].name, "objs");
        assert_eq!(f.sections[0].records.len(), 1);
        assert_eq!(f.sections[0].records[0].fields, vec!["a", "b", "1"]);
        assert_eq!(f.section("TXDP").unwrap().records.len(), 1);
    }

    #[test]
    fn comments_and_blank_lines_ignored() {
        let f = TextFile::parse(
            b"# header\n\nobjs # inline\n  # full\na, b # tail\nend\n// c++ style\n",
        )
        .unwrap();
        assert_eq!(f.sections.len(), 1);
        assert_eq!(f.sections[0].records.len(), 1);
        assert_eq!(f.sections[0].records[0].fields, vec!["a", "b"]);
    }

    #[test]
    fn quoted_comma_kept() {
        let f = TextFile::parse(b"2dfx\nname, \"a,b\", 3\nend\n").unwrap();
        assert_eq!(f.sections[0].records[0].fields, vec!["name", "a,b", "3"]);
    }

    #[test]
    fn stray_end_ignored() {
        let f = TextFile::parse(b"objs\na\nend\nend\n").unwrap();
        assert_eq!(f.sections.len(), 1);
    }

    #[test]
    fn unterminated_final_section_tolerated() {
        // Two shipped files lack the final `end`; EOF closes the section.
        let f = TextFile::parse(b"objs\na\n").unwrap();
        assert_eq!(f.sections.len(), 1);
        assert_eq!(f.sections[0].records.len(), 1);
    }

    #[test]
    fn invalid_utf8_errors() {
        assert!(TextFile::parse(b"objs\n\xff\nend\n").is_err());
    }

    #[test]
    fn flatten_splits_merged_fields() {
        let f = vec!["a b".to_string(), "".to_string(), "c".to_string()];
        assert_eq!(flatten_fields(&f), vec!["a", "b", "c"]);
    }
}
