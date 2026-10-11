//! Generic fallback scan for text files without a typed parser yet.
//!
//! [`scan_text`] sniffs a file's comment style and field delimiter from
//! its own lines, then reports structural counts: data rows, blank and
//! comment lines, section markers, and the distribution of field widths.
//! The integration test routes every data file it cannot type through
//! here so nothing goes uncounted.

use crate::text::{CommentStyle, split_comment, split_csv, split_ws};
use crate::{Result, decode};
use std::collections::BTreeMap;

/// Structural summary of an untyped text file.
#[derive(Debug, Clone)]
pub struct Scan {
    /// Total physical lines.
    pub total_lines: usize,
    /// Blank lines.
    pub blank_lines: usize,
    /// Comment-only lines.
    pub comment_lines: usize,
    /// Data lines (non-blank after stripping).
    pub data_lines: usize,
    /// Comment style the sniffer settled on.
    pub comments: &'static str,
    /// Delimiter the sniffer settled on (`"comma"`, `"whitespace"`, `"mixed"`, `"none"`).
    pub delimiter: &'static str,
    /// Field-width histogram over data lines: width -> count.
    pub widths: BTreeMap<usize, usize>,
    /// Distinct bare-word lines that look like section markers (no
    /// delimiter, not a directive), capped at 64 entries.
    pub markers: Vec<String>,
    /// First data line, for eyeballing.
    pub first_row: Option<String>,
}

/// Sniff comment style: counts leading `//`, `#`, `;` lines.
fn sniff_comments(text: &str) -> (CommentStyle, &'static str) {
    let mut slashes = 0;
    let mut hash = 0;
    let mut semi = 0;
    for raw in text.lines().take(400) {
        let s = raw.trim();
        if s.starts_with("//") {
            slashes += 1;
        } else if s.starts_with('#') {
            hash += 1;
        } else if s.starts_with(';') {
            semi += 1;
        }
    }
    if slashes >= hash && slashes >= semi && slashes > 0 {
        (CommentStyle::SLASHES, "//")
    } else if semi > hash && semi > 0 {
        (CommentStyle::SEMICOLON_HASH, ";")
    } else if hash > 0 {
        (CommentStyle::HASH, "#")
    } else {
        (CommentStyle::NONE, "none")
    }
}

/// Scan a text file structurally. Binary input is an error (see
/// [`crate::ErrorKind::Binary`]).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn scan_text(file: &str, bytes: &[u8]) -> Result<Scan> {
    let text = decode(file, bytes)?;
    let (style, comments) = sniff_comments(&text);
    let mut scan = Scan {
        total_lines: 0,
        blank_lines: 0,
        comment_lines: 0,
        data_lines: 0,
        comments,
        delimiter: "none",
        widths: BTreeMap::new(),
        markers: Vec::new(),
        first_row: None,
    };
    let mut comma_rows = 0usize;
    let mut ws_rows = 0usize;
    for raw in text.lines() {
        scan.total_lines += 1;
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if line.trim().is_empty() {
            scan.blank_lines += 1;
            continue;
        }
        let (code, _) = split_comment(line, style);
        let code = code.trim();
        if code.is_empty() {
            scan.comment_lines += 1;
            continue;
        }
        scan.data_lines += 1;
        if scan.first_row.is_none() {
            scan.first_row = Some(code.chars().take(160).collect());
        }
        let has_comma = code.contains(',');
        let has_ws = code.contains([' ', '\t']);
        let width = if has_comma {
            comma_rows += 1;
            split_csv(code).len()
        } else if has_ws {
            ws_rows += 1;
            split_ws(code).len()
        } else {
            1
        };
        *scan.widths.entry(width).or_insert(0) += 1;
        if !has_comma
            && !has_ws
            && scan.markers.len() < 64
            && !scan.markers.contains(&code.to_string())
        {
            scan.markers.push(code.to_string());
        }
    }
    scan.delimiter = if comma_rows > 0 && ws_rows > 0 {
        "mixed"
    } else if comma_rows > 0 {
        "comma"
    } else if ws_rows > 0 {
        "whitespace"
    } else {
        "none"
    };
    Ok(scan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_mixed_file() {
        let src = "# c\r\n\r\nGROUP: IT_SHOP\r\nITEM: shlf4_cablft 1 0 100 0\r\n";
        let s = scan_text("furnitur.dat", src.as_bytes()).unwrap();
        assert_eq!(s.total_lines, 4);
        assert_eq!(s.comment_lines, 1);
        assert_eq!(s.data_lines, 2);
        assert_eq!(s.comments, "#");
    }

    #[test]
    fn binary_is_error() {
        assert!(scan_text("x.dat", b"a\x00b").is_err());
    }
}
