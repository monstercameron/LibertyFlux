//! Text item placement (`.ipl`) files.
//!
//! Every shipped file declares the same section list, but only `blok`,
//! `cull`, `occl`, `vnod`, `link` and `2dfx` ever carry records; the rest
//! (notably `inst`) are always empty because placement moved to [`.wpl`
//! files](crate::wpl). [`IplFile::parse`] keeps the typed rows, the set of
//! declared sections, and per-record [`errors`](IplFile::errors).

use crate::Error;
use crate::text::TextFile;

/// A parsed `.ipl` file.
#[derive(Debug, Clone, Default)]
pub struct IplFile {
    /// Source-file stamp rows (`blok`).
    pub blok: Vec<Blok>,
    /// Cull zone rows (`cull`); one exists in all shipped files.
    pub cull: Vec<Cull>,
    /// Occlusion volume rows (`occl`).
    pub occl: Vec<Occl>,
    /// Path node rows (`vnod`).
    pub vnod: Vec<Vnod>,
    /// Path link rows (`link`).
    pub link: Vec<Link>,
    /// Placed effect rows (`2dfx`).
    pub fx: Vec<Effect>,
    /// Every section name declared by the file, in file order (lower-cased),
    /// including empty ones.
    pub declared: Vec<String>,
    /// Rows that failed to parse, with their errors.
    pub errors: Vec<crate::ide::RecordError>,
    /// Sections this crate does not type, with their raw record counts.
    pub unknown_sections: Vec<crate::ide::UnknownSection>,
}

/// Source-file stamp row (`blok`, 6 fields).
///
/// Names may contain spaces, so fields are split on commas only. The matching
/// binary [`crate::wpl::Blok`] string carries an extra `unknown` word between
/// the name and the author.
#[derive(Debug, Clone, PartialEq)]
pub struct Blok {
    /// Source area name.
    pub name: String,
    /// Author name.
    pub author: String,
    /// Date stamp (`2007:00:00:00:00:00` style).
    pub date: String,
    /// Integer, always 128; meaning uncertain.
    pub unknown_128: i32,
    /// Integer, always 0; meaning uncertain.
    pub unknown_zero: i32,
    /// Trailing word, always `unknown`.
    pub trailing: String,
}

/// Cull zone row (`cull`, 11 numeric fields).
///
/// Only one shipped record exists, so field meanings are unknown; values are
/// kept in order.
#[derive(Debug, Clone, PartialEq)]
pub struct Cull {
    /// Raw fields in order.
    pub values: Vec<String>,
}

/// Occlusion volume row (`occl`, 10 fields).
#[derive(Debug, Clone, PartialEq)]
pub struct Occl {
    /// Position (x, y, z).
    pub pos: [f32; 3],
    /// Six further floats; meanings uncertain.
    pub values: [f32; 6],
    /// Trailing integer flags.
    pub flags: i32,
}

/// Path node row (`vnod`, 13-14 fields).
///
/// No public documentation exists. The position is followed by 10-11 numeric
/// fields whose types vary by column and by file: small integers, unsigned
/// 32-bit hashes, and float headings all occur, so each value keeps the
/// narrowest type that holds it.
#[derive(Debug, Clone, PartialEq)]
pub struct Vnod {
    /// Position (x, y, z).
    pub pos: [f32; 3],
    /// Remaining numeric fields (10 or 11 of them).
    pub values: Vec<VnodValue>,
}

/// One numeric tail field of a [`Vnod`] row.
#[derive(Debug, Clone, PartialEq)]
pub enum VnodValue {
    /// Signed integer field.
    Int(i32),
    /// Unsigned field (hashes above `i32::MAX`).
    Uint(u32),
    /// Float field (headings in some files).
    Float(f32),
}

/// Path link row (`link`, 6 integers).
///
/// No public documentation exists; values are kept in order.
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    /// The six link integers.
    pub values: [i32; 6],
}

/// Placed effect row (`2dfx`): model, position, kind, payload.
///
/// The shipped kind is always 13 with 7 or 11 payload fields; meanings are
/// unknown.
#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    /// Model the effect attaches to.
    pub model: String,
    /// Position (x, y, z).
    pub pos: [f32; 3],
    /// Effect kind id.
    pub kind: u32,
    /// Raw payload fields after the kind.
    pub values: Vec<String>,
}

fn num<T>(line: usize, section: &str, field: usize, text: &str) -> Result<T, Error>
where
    T: std::str::FromStr,
{
    text.parse::<T>().map_err(|_| Error::BadNumber {
        line,
        section: section.to_string(),
        field,
        text: text.to_string(),
    })
}

fn count_err(line: usize, section: &str, found: usize, expected: &str) -> Error {
    Error::FieldCount {
        line,
        section: section.to_string(),
        found,
        expected: expected.to_string(),
    }
}

impl IplFile {
    /// Parse an `.ipl` file from a byte slice.
    ///
    /// Record failures land in [`IplFile::errors`]; only invalid UTF-8 fails
    /// the whole parse.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<IplFile, Error> {
        let text = TextFile::parse(bytes)?;
        let mut out = IplFile::default();
        for sec in &text.sections {
            let name = sec.name.to_ascii_lowercase();
            out.declared.push(name.clone());
            match name.as_str() {
                "blok" => {
                    for r in &sec.records {
                        match parse_blok(r.line, &r.fields) {
                            Ok(b) => out.blok.push(b),
                            Err(e) => out.errors.push(crate::ide::RecordError {
                                line: r.line,
                                section: sec.name.clone(),
                                error: e,
                            }),
                        }
                    }
                }
                "cull" => {
                    for r in &sec.records {
                        match parse_cull(r.line, &r.fields) {
                            Ok(b) => out.cull.push(b),
                            Err(e) => out.errors.push(crate::ide::RecordError {
                                line: r.line,
                                section: sec.name.clone(),
                                error: e,
                            }),
                        }
                    }
                }
                "occl" => {
                    for r in &sec.records {
                        match parse_occl(r.line, &r.fields) {
                            Ok(b) => out.occl.push(b),
                            Err(e) => out.errors.push(crate::ide::RecordError {
                                line: r.line,
                                section: sec.name.clone(),
                                error: e,
                            }),
                        }
                    }
                }
                "vnod" => {
                    for r in &sec.records {
                        match parse_vnod(r.line, &r.fields) {
                            Ok(b) => out.vnod.push(b),
                            Err(e) => out.errors.push(crate::ide::RecordError {
                                line: r.line,
                                section: sec.name.clone(),
                                error: e,
                            }),
                        }
                    }
                }
                "link" => {
                    for r in &sec.records {
                        match parse_link(r.line, &r.fields) {
                            Ok(b) => out.link.push(b),
                            Err(e) => out.errors.push(crate::ide::RecordError {
                                line: r.line,
                                section: sec.name.clone(),
                                error: e,
                            }),
                        }
                    }
                }
                "2dfx" => {
                    for r in &sec.records {
                        match parse_effect(r.line, &r.fields) {
                            Ok(b) => out.fx.push(b),
                            Err(e) => out.errors.push(crate::ide::RecordError {
                                line: r.line,
                                section: sec.name.clone(),
                                error: e,
                            }),
                        }
                    }
                }
                "inst" | "path" | "grge" | "enex" | "pick" | "cars" | "jump" | "tcyc" | "auzo"
                | "mult" | "pnod" | "plnk" | "mlo+" | "lodm" | "slow" | "rtfx" => {
                    // Known but always empty in shipped files; keep any
                    // unexpected records as errors so they are noticed.
                    for r in &sec.records {
                        out.errors.push(crate::ide::RecordError {
                            line: r.line,
                            section: sec.name.clone(),
                            error: count_err(
                                r.line,
                                &sec.name,
                                r.fields.len(),
                                "0 (empty section)",
                            ),
                        });
                    }
                }
                _ => out.unknown_sections.push(crate::ide::UnknownSection {
                    name: sec.name.clone(),
                    records: sec.records.len(),
                }),
            }
        }
        Ok(out)
    }

    /// Total typed records across all sections.
    #[must_use]
    pub fn len(&self) -> usize {
        self.blok.len()
            + self.cull.len()
            + self.occl.len()
            + self.vnod.len()
            + self.link.len()
            + self.fx.len()
    }

    /// True when no typed records were parsed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn nonempty(fields: &[String]) -> Vec<String> {
    fields.iter().filter(|s| !s.is_empty()).cloned().collect()
}

fn parse_blok(line: usize, fields: &[String]) -> Result<Blok, Error> {
    // Blok names may contain spaces: use comma fields, not whitespace tokens.
    let f = nonempty(fields);
    if f.len() != 6 {
        return Err(count_err(line, "blok", f.len(), "6"));
    }
    Ok(Blok {
        name: f[0].clone(),
        author: f[1].clone(),
        date: f[2].clone(),
        unknown_128: num(line, "blok", 3, &f[3])?,
        unknown_zero: num(line, "blok", 4, &f[4])?,
        trailing: f[5].clone(),
    })
}

fn parse_cull(line: usize, fields: &[String]) -> Result<Cull, Error> {
    let f = nonempty(fields);
    if f.len() != 11 {
        return Err(count_err(line, "cull", f.len(), "11"));
    }
    Ok(Cull { values: f })
}

fn parse_occl(line: usize, fields: &[String]) -> Result<Occl, Error> {
    let f = nonempty(fields);
    if f.len() != 10 {
        return Err(count_err(line, "occl", f.len(), "10"));
    }
    let g = |i: usize| num::<f32>(line, "occl", i, &f[i]);
    Ok(Occl {
        pos: [g(0)?, g(1)?, g(2)?],
        values: [g(3)?, g(4)?, g(5)?, g(6)?, g(7)?, g(8)?],
        flags: num(line, "occl", 9, &f[9])?,
    })
}

fn parse_vnod(line: usize, fields: &[String]) -> Result<Vnod, Error> {
    let f = nonempty(fields);
    if f.len() != 13 && f.len() != 14 {
        return Err(count_err(line, "vnod", f.len(), "13 or 14"));
    }
    let mut values = Vec::with_capacity(f.len() - 3);
    for (i, v) in f.iter().enumerate().skip(3) {
        if let Ok(n) = v.parse::<i32>() {
            values.push(VnodValue::Int(n));
        } else if let Ok(n) = v.parse::<u32>() {
            values.push(VnodValue::Uint(n));
        } else if let Ok(n) = v.parse::<f32>() {
            values.push(VnodValue::Float(n));
        } else {
            return Err(Error::BadNumber {
                line,
                section: "vnod".to_string(),
                field: i,
                text: v.clone(),
            });
        }
    }
    Ok(Vnod {
        pos: [
            num(line, "vnod", 0, &f[0])?,
            num(line, "vnod", 1, &f[1])?,
            num(line, "vnod", 2, &f[2])?,
        ],
        values,
    })
}

fn parse_link(line: usize, fields: &[String]) -> Result<Link, Error> {
    let f = nonempty(fields);
    if f.len() != 6 {
        return Err(count_err(line, "link", f.len(), "6"));
    }
    let mut values = [0i32; 6];
    for (i, v) in values.iter_mut().enumerate() {
        *v = num(line, "link", i, &f[i])?;
    }
    Ok(Link { values })
}

fn parse_effect(line: usize, fields: &[String]) -> Result<Effect, Error> {
    let f = nonempty(fields);
    if f.len() < 6 {
        return Err(count_err(line, "2dfx", f.len(), "at least 6"));
    }
    Ok(Effect {
        model: f[0].clone(),
        pos: [
            num(line, "2dfx", 1, &f[1])?,
            num(line, "2dfx", 2, &f[2])?,
            num(line, "2dfx", 3, &f[3])?,
        ],
        kind: num(line, "2dfx", 4, &f[4])?,
        values: f[5..].to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample() {
        // Note: the blok name holds a space, exercising comma-only splitting.
        let data = b"inst\nend\nblok\nMy Area, ann, 2007:00:00:00:00:00, 128, 0, unknown\nend\ncull\n1, 2, 3, 0, 4, 3, 5, 0, 6, 8, 0\nend\noccl\n1, 2, 3, 0.1, 0.2, 0.3, 0, 0, 0, 0\nend\nvnod\n1, 2, 3, 0, 0, 0, 1, 0, 1, 5, 0, 0, 255\nend\nlink\n1, 0, 0, 2, 0, 0\nend\n2dfx\nworld, 1, 2, 3, 13, 0, 0, 0, -1, fxname, 0\nend\n";
        let f = IplFile::parse(data).unwrap();
        assert_eq!(f.blok.len(), 1);
        assert_eq!(f.blok[0].name, "My Area");
        assert_eq!(f.blok[0].author, "ann");
        assert_eq!(f.cull.len(), 1);
        assert_eq!(f.occl.len(), 1);
        assert_eq!(f.vnod.len(), 1);
        assert_eq!(f.vnod[0].values.len(), 10);
        assert_eq!(f.link.len(), 1);
        assert_eq!(f.fx.len(), 1);
        assert_eq!(f.fx[0].kind, 13);
        assert!(f.errors.is_empty());
        assert!(f.declared.contains(&"inst".to_string()));
    }

    #[test]
    fn vnod_accepts_13_and_14_fields() {
        let data = b"vnod\n1, 2, 3, 0, 0, 0, 1, 0, 1, 5, 0, 0, 0, 255\n1, 2, 3, 0, 0, 0, 1, 0, 1, 5, 0, 0, 255\nend\n";
        let f = IplFile::parse(data).unwrap();
        assert_eq!(f.vnod.len(), 2);
        assert_eq!(f.vnod[0].values.len(), 11);
        assert_eq!(f.vnod[1].values.len(), 10);
    }

    #[test]
    fn bad_row_collected_not_fatal() {
        let f = IplFile::parse(b"link\n1, 2\nend\n").unwrap();
        assert!(f.link.is_empty());
        assert_eq!(f.errors.len(), 1);
    }
}
