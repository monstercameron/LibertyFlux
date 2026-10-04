//! Shared line plumbing: comment stripping, field splitting, scalars.
//!
//! Every table parser builds on [`LogicalLines`]: an iterator over the
//! non-blank, comment-stripped lines of a file that remembers 1-based line
//! numbers for error reports.

use crate::{Error, ErrorKind, Result};

/// Which comment markers a file family uses.
///
/// Markers are recognised anywhere on a line (leading or trailing) except
/// where noted. Order matters only in that `//` is checked before `#` and
/// `;`, so a URL-like `a://b` truncates at the slashes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentStyle {
    /// Strip `//` to end of line.
    pub slashes: bool,
    /// Strip `#` to end of line.
    pub hash: bool,
    /// Strip `;` to end of line (only for `;`-led files; some families
    /// use `;` inside data, e.g. HUD debug item names).
    pub semicolon: bool,
}

impl CommentStyle {
    /// `#` comments (IDE files, carcols, groups, most hash files).
    pub const HASH: Self = Self {
        slashes: false,
        hash: true,
        semicolon: false,
    };
    /// `;` and `#` comments (handling, object, plants, numplate).
    pub const SEMICOLON_HASH: Self = Self {
        slashes: false,
        hash: true,
        semicolon: true,
    };
    /// `//` comments (popcycle, timecyc, CSV files, train cams).
    pub const SLASHES: Self = Self {
        slashes: true,
        hash: false,
        semicolon: false,
    };
    /// No comments (XML is handled by [`crate::xml`]).
    pub const NONE: Self = Self {
        slashes: false,
        hash: false,
        semicolon: false,
    };
}

/// Strip one line's comment. Returns `(code, Option<comment>)` where
/// `comment` is the stripped comment text without its marker, if any.
/// Callers that need group names from comment lines (ped groups) use it.
#[must_use]
pub fn split_comment(line: &str, style: CommentStyle) -> (&str, Option<&str>) {
    let bytes = line.as_bytes();
    let mut cut: Option<(usize, usize)> = None; // (byte index, marker len)
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if style.slashes && b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            cut = Some((i, 2));
            break;
        }
        if style.hash && b == b'#' {
            cut = Some((i, 1));
            break;
        }
        if style.semicolon && b == b';' {
            cut = Some((i, 1));
            break;
        }
        i += 1;
    }
    match cut {
        Some((at, len)) => (line[..at].trim_end(), Some(line[at + len..].trim())),
        None => (line, None),
    }
}

/// One logical line: stripped code, optional comment, 1-based number.
#[derive(Debug, Clone)]
pub struct LogicalLine {
    /// 1-based line number in the source file.
    pub num: usize,
    /// Code part with comments removed and whitespace trimmed.
    pub code: String,
    /// Comment text without its marker, if the line had one.
    pub comment: Option<String>,
}

/// Iterate a file's non-blank lines with comments stripped.
///
/// Blank lines and lines that are only a comment still yield a
/// [`LogicalLine`] with empty `code` when `keep_comments` is set, so group
/// parsers can read names out of comment lines; otherwise they are skipped.
pub fn logical_lines(
    text: &str,
    style: CommentStyle,
    keep_comments: bool,
) -> impl Iterator<Item = LogicalLine> + '_ {
    text.lines().enumerate().filter_map(move |(idx, raw)| {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        let (code, comment) = split_comment(line, style);
        let code = code.trim().to_string();
        if code.is_empty() && !keep_comments {
            return None;
        }
        if code.is_empty() && comment.is_none_or(str::is_empty) {
            return None;
        }
        Some(LogicalLine {
            num: idx + 1,
            code,
            comment: comment.map(std::string::ToString::to_string),
        })
    })
}

/// Split a record on commas, trimming ASCII whitespace around fields.
/// Empty fields are kept (a row like `a,,b` yields three fields) except a
/// single trailing empty field from a line-final comma, which is dropped:
/// IDE rows end every line with a comma.
#[must_use]
pub fn split_csv(code: &str) -> Vec<String> {
    let mut out: Vec<String> = code.split(',').map(|f| f.trim().to_string()).collect();
    if out.len() > 1 && out.last().is_some_and(std::string::String::is_empty) {
        out.pop();
    }
    out
}

/// Split a record on commas and/or whitespace runs, dropping empties.
/// Used for colour index rows where the shipped files occasionally miss a
/// comma (`10,1,1,133 25,1,1,133`).
#[must_use]
pub fn split_csv_ws(code: &str) -> Vec<String> {
    code.split(|c: char| c == ',' || c.is_whitespace())
        .filter(|f| !f.is_empty())
        .map(std::string::ToString::to_string)
        .collect()
}

/// Split a record on whitespace runs.
#[must_use]
pub fn split_ws(code: &str) -> Vec<String> {
    code.split_whitespace()
        .map(std::string::ToString::to_string)
        .collect()
}

/// Fetch field `i` or fail with [`ErrorKind::FieldCount`].
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn field<'a>(
    file: &str,
    num: usize,
    fields: &'a [String],
    i: usize,
    expected: Option<usize>,
) -> Result<&'a str> {
    fields
        .get(i)
        .map(std::string::String::as_str)
        .ok_or_else(|| {
            Error::new(
                file,
                num,
                ErrorKind::FieldCount {
                    expected,
                    found: fields.len(),
                },
            )
        })
}

/// Parse helpers. Each reports [`ErrorKind::BadScalar`] naming the field.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_f32(file: &str, num: usize, fields: &[String], i: usize) -> Result<f32> {
    let s = field(file, num, fields, i, None)?;
    // Some files write C float literals (`-1.0f` in train cam nodes).
    let s = s.strip_suffix(['f', 'F']).unwrap_or(s);
    s.parse::<f32>().map_err(|_| {
        Error::new(
            file,
            num,
            ErrorKind::BadScalar {
                field: i,
                want: "f32",
            },
        )
    })
}

/// Parse an `f32`, accepting integers too (`1` parses as `1.0`).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_f64(file: &str, num: usize, fields: &[String], i: usize) -> Result<f64> {
    let s = field(file, num, fields, i, None)?;
    let s = s.strip_suffix(['f', 'F']).unwrap_or(s);
    s.parse::<f64>().map_err(|_| {
        Error::new(
            file,
            num,
            ErrorKind::BadScalar {
                field: i,
                want: "f64",
            },
        )
    })
}

/// Parse an `i32` (plain decimal, optional leading `-`).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_i32(file: &str, num: usize, fields: &[String], i: usize) -> Result<i32> {
    let s = field(file, num, fields, i, None)?;
    s.parse::<i32>().map_err(|_| {
        Error::new(
            file,
            num,
            ErrorKind::BadScalar {
                field: i,
                want: "i32",
            },
        )
    })
}

/// Parse a `u32` (plain decimal).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_u32(file: &str, num: usize, fields: &[String], i: usize) -> Result<u32> {
    let s = field(file, num, fields, i, None)?;
    s.parse::<u32>().map_err(|_| {
        Error::new(
            file,
            num,
            ErrorKind::BadScalar {
                field: i,
                want: "u32",
            },
        )
    })
}

/// Parse a `u8` (0-255).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_u8(file: &str, num: usize, fields: &[String], i: usize) -> Result<u8> {
    let s = field(file, num, fields, i, None)?;
    s.parse::<u8>().map_err(|_| {
        Error::new(
            file,
            num,
            ErrorKind::BadScalar {
                field: i,
                want: "u8",
            },
        )
    })
}

/// Parse a hex `u32` without any `0x` prefix (handling model/handling flags).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_hex_u32(file: &str, num: usize, fields: &[String], i: usize) -> Result<u32> {
    let s = field(file, num, fields, i, None)?;
    u32::from_str_radix(s, 16).map_err(|_| {
        Error::new(
            file,
            num,
            ErrorKind::BadScalar {
                field: i,
                want: "hex u32",
            },
        )
    })
}

/// True unless the token is `-`, `null`, `NULL` or empty (the files' spellings of "none").
#[must_use]
pub fn is_some_token(s: &str) -> bool {
    !(s.is_empty() || s == "-" || s.eq_ignore_ascii_case("null"))
}

/// Heal the shipped files' missing-comma typos: drop empty fields and
/// split fields with interior whitespace in two. Returns the healed
/// fields plus whether anything changed. Callers check the width after.
#[must_use]
pub fn heal_comma_row(fields: &[String]) -> (Vec<String>, bool) {
    let mut out: Vec<String> = Vec::with_capacity(fields.len() + 2);
    let mut healed = false;
    for tok in fields {
        if tok.is_empty() {
            healed = true;
            continue;
        }
        let parts: Vec<&str> = tok.split_whitespace().collect();
        if parts.len() == 2 {
            healed = true;
            out.push(parts[0].to_string());
            out.push(parts[1].to_string());
        } else {
            out.push(tok.clone());
        }
    }
    (out, healed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_hash_comments() {
        let (code, comment) = split_comment("col  # palette", CommentStyle::HASH);
        assert_eq!(code, "col");
        assert_eq!(comment, Some("palette"));
    }

    #[test]
    fn strips_slash_comments() {
        let (code, comment) = split_comment("15 15 12 // peds", CommentStyle::SLASHES);
        assert_eq!(code, "15 15 12");
        assert_eq!(comment, Some("peds"));
    }

    #[test]
    fn semicolon_kept_when_not_enabled() {
        let (code, _) = split_comment(";DEBUG_ITEM 1", CommentStyle::HASH);
        assert_eq!(code, ";DEBUG_ITEM 1");
    }

    #[test]
    fn csv_drops_one_trailing_empty() {
        assert_eq!(split_csv("a, b, c,"), vec!["a", "b", "c"]);
        assert_eq!(split_csv("a,,c"), vec!["a", "", "c"]);
    }

    #[test]
    fn csv_ws_heals_missing_comma() {
        assert_eq!(
            split_csv_ws("10,1,1,133 25,1,1,133"),
            vec!["10", "1", "1", "133", "25", "1", "1", "133"]
        );
    }

    #[test]
    fn float_suffix_f_accepted() {
        let f = vec!["-1.0f".to_string()];
        assert_eq!(parse_f32("t", 1, &f, 0).unwrap(), -1.0);
    }
}
