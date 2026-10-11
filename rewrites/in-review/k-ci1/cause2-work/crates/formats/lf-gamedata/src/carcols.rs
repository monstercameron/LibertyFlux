//! `carcols.dat`: vehicle colour palette and per-car paint options.
//!
//! Grammar: `#` comments (leading and trailing), comma-separated rows,
//! three sections closed by `end`:
//!
//! * `col`: the palette. Each row is `r, g, b, prefix, colour` where the
//!   last two name the police-scanner audio (`-`/`bright`/`light`/`dark`
//!   plus a colour word). A trailing `#` comment gives the palette index
//!   and the colour's name; the parser reads both.
//! * `car3`: per-car paint triples. Each row names a car then a flat list
//!   of palette indices in groups of three (one shipped row breaks the
//!   grouping with 16 indices; grouping is not enforced).
//! * `car4`: per-car paint quads, same shape in groups of four.
//!
//! The shipped `car4` rows occasionally miss a comma between groups, so
//! index lists are split on commas and whitespace alike.

use crate::text::{
    CommentStyle, LogicalLine, field, logical_lines, parse_u8, split_csv, split_csv_ws,
};
use crate::{Error, ErrorKind, Result, decode};

/// One palette entry.
#[derive(Debug, Clone)]
pub struct PaletteEntry {
    /// Palette index, from the row's trailing comment (if present).
    pub index: Option<u32>,
    /// Colour name, from the row's trailing comment (if present).
    pub name: Option<String>,
    /// Red, green, blue (0-255).
    pub rgb: [u8; 3],
    /// Scanner audio prefix (`-`, `bright`, `light`, `dark`).
    pub prefix: String,
    /// Scanner audio colour word.
    pub colour: String,
}

/// One car's paint options: palette indices in groups of three or four.
#[derive(Debug, Clone)]
pub struct CarPaint {
    /// Car model name.
    pub car: String,
    /// Flat index list; length is a multiple of the group size.
    pub indices: Vec<u8>,
}

/// Full contents of a `carcols.dat` file.
#[derive(Debug, Clone, Default)]
pub struct Carcols {
    /// Palette rows in file order.
    pub palette: Vec<PaletteEntry>,
    /// `car3` rows: triples.
    pub car3: Vec<CarPaint>,
    /// `car4` rows: quads.
    pub car4: Vec<CarPaint>,
}

fn palette_row(file: &str, l: &LogicalLine) -> Result<PaletteEntry> {
    let f = split_csv(&l.code);
    if f.len() != 5 {
        return Err(Error::new(
            file,
            l.num,
            ErrorKind::FieldCount {
                expected: Some(5),
                found: f.len(),
            },
        ));
    }
    let (index, name) = match &l.comment {
        Some(c) => {
            // Trailing comment shape: `0 black`, `1 black poly`.
            let mut parts = c.split_whitespace();
            let idx = parts.next().and_then(|s| s.parse::<u32>().ok());
            let rest: Vec<&str> = parts.collect();
            let nm = (!rest.is_empty()).then(|| rest.join(" "));
            (idx, nm)
        }
        None => (None, None),
    };
    Ok(PaletteEntry {
        index,
        name,
        rgb: [
            parse_u8(file, l.num, &f, 0)?,
            parse_u8(file, l.num, &f, 1)?,
            parse_u8(file, l.num, &f, 2)?,
        ],
        prefix: field(file, l.num, &f, 3, Some(5))?.to_string(),
        colour: field(file, l.num, &f, 4, Some(5))?.to_string(),
    })
}

fn paint_row(file: &str, l: &LogicalLine, group: usize) -> Result<CarPaint> {
    let toks = split_csv_ws(&l.code);
    // One shipped `car3` row carries 16 indices (four quads); grouping is
    // interpretive, so any non-empty index list parses.
    if toks.len() < 1 + group {
        return Err(Error::new(
            file,
            l.num,
            ErrorKind::FieldCount {
                expected: None,
                found: toks.len(),
            },
        ));
    }
    let mut indices = Vec::with_capacity(toks.len() - 1);
    for (i, t) in toks[1..].iter().enumerate() {
        // One shipped row writes `89.113` for `89,113`; split dotted
        // pairs that fail plain parsing.
        let parts: Vec<&str> = if t.parse::<u8>().is_ok() {
            vec![t.as_str()]
        } else {
            t.split('.').collect()
        };
        for part in parts {
            let v = part.parse::<u8>().map_err(|_| {
                Error::new(
                    file,
                    l.num,
                    ErrorKind::BadScalar {
                        field: i + 1,
                        want: "u8",
                    },
                )
            })?;
            indices.push(v);
        }
    }
    Ok(CarPaint {
        car: toks[0].clone(),
        indices,
    })
}

/// Parse a `carcols.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_carcols(file: &str, bytes: &[u8]) -> Result<Carcols> {
    let text = decode(file, bytes)?;
    let mut out = Carcols::default();
    let mut section: Option<String> = None;
    for l in logical_lines(&text, CommentStyle::HASH, true) {
        if l.code.is_empty() {
            continue;
        }
        if l.code.eq_ignore_ascii_case("end") {
            if section.take().is_none() {
                return Err(Error::new(file, l.num, ErrorKind::StrayEnd));
            }
            continue;
        }
        match section.as_deref() {
            None => {
                section = Some(l.code.clone());
            }
            Some("col") => out.palette.push(palette_row(file, &l)?),
            Some("car3") => out.car3.push(paint_row(file, &l, 3)?),
            Some("car4") => out.car4.push(paint_row(file, &l, 4)?),
            Some(other) => {
                return Err(Error::new(
                    file,
                    l.num,
                    ErrorKind::UnknownMarker {
                        marker: other.to_string(),
                    },
                ));
            }
        }
    }
    // Episode files omit the final `end`; tolerate it like the game.
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# colours\r\ncol\r\n10,10,10,-,black # 0 black\r\nend\r\ncar3\r\nbus, 53,8,53,\r\nend\r\ncar4\r\nfuto, 32,0,30,1,\r\nend\r\n";

    #[test]
    fn parses_all_sections() {
        let c = parse_carcols("carcols.dat", SAMPLE.as_bytes()).unwrap();
        assert_eq!(c.palette.len(), 1);
        assert_eq!(c.palette[0].index, Some(0));
        assert_eq!(c.palette[0].name.as_deref(), Some("black"));
        assert_eq!(c.car3.len(), 1);
        assert_eq!(c.car3[0].indices, vec![53, 8, 53]);
        assert_eq!(c.car4.len(), 1);
    }

    #[test]
    fn heals_missing_comma_between_groups() {
        let src = "car4\r\nsultan, 74,0,83,90 0,1,1,133,\r\nend\r\n";
        let c = parse_carcols("carcols.dat", src.as_bytes()).unwrap();
        assert_eq!(c.car4[0].indices.len(), 8);
    }
}
