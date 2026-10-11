//! Miscellaneous tables: CSV files, vehicle offsets, shorelines.
//!
//! * Generic CSV ([`parse_csv_table`]): `songlist.csv`,
//!   `action_table.csv` (+ episode variants), `introLoc.csv`,
//!   `introSpline.csv`. `//` comments, comma fields, optional header.
//! * [`VehoffRow`]: `vehOff.csv` cinematic-camera offsets — keyword rows
//!   `VEHICLE <model> FIELD <field> POS <x y z> ANGLES <a b c> FOV <f>`.
//! * [`parse_shorelines`]: `shorelines.dat` — a segment count, then per
//!   segment a `points flags` line and that many `x y` lines.
//! * [`parse_numplate`]: `numplate.dat` — `;` comments, RGB triples.
//! * [`parse_stockshake`]: `stockshake.txt` — bare rows of 10 numbers.
//! * [`parse_train_cams`]: `trainCamNodes.txt` — `//` comments, rows of
//!   `id x y z detect` (detect may carry an `f` suffix).
//! * [`parse_key_value`]: `nav.dat` / `stream.ini` — `KEY=value` rows.

use crate::text::{CommentStyle, logical_lines, parse_f32, parse_i32, parse_u8, split_ws};
use crate::{Error, ErrorKind, Result, decode};

/// One generic CSV table.
#[derive(Debug, Clone)]
pub struct CsvTable {
    /// Header row, if the file has one.
    pub header: Option<Vec<String>>,
    /// Data rows.
    pub rows: Vec<CsvRow>,
}

/// One CSV row with its line number.
#[derive(Debug, Clone)]
pub struct CsvRow {
    /// 1-based line number.
    pub line: usize,
    /// Fields.
    pub fields: Vec<String>,
}

/// Parse a `//`-commented CSV file. When `has_header` is set the first
/// data row becomes the header instead of a row.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_csv_table(file: &str, bytes: &[u8], has_header: bool) -> Result<CsvTable> {
    let text = decode(file, bytes)?;
    let mut header = None;
    let mut rows = Vec::new();
    for l in logical_lines(&text, CommentStyle::SLASHES, false) {
        // CSV files here never quote fields; commas always separate.
        let fields: Vec<String> = l.code.split(',').map(|s| s.trim().to_string()).collect();
        if has_header && header.is_none() {
            header = Some(fields);
        } else {
            rows.push(CsvRow {
                line: l.num,
                fields,
            });
        }
    }
    Ok(CsvTable { header, rows })
}

/// One `vehOff.csv` row: a named camera offset on a vehicle.
#[derive(Debug, Clone)]
pub struct VehoffRow {
    /// Vehicle model.
    pub vehicle: String,
    /// Offset field name (`BOOT_OPEN`, `BONNET_FWD`, ...).
    pub field: String,
    /// Position offset.
    pub pos: [f32; 3],
    /// Angles.
    pub angles: [f32; 3],
    /// Field of view.
    pub fov: f32,
}

/// Parse a `vehOff.csv` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_vehoff(file: &str, bytes: &[u8]) -> Result<Vec<VehoffRow>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::SLASHES, false) {
        let f = split_ws(&l.code);
        // VEHICLE v FIELD f POS x y z ANGLES a b c FOV v = 14 tokens.
        if f.len() != 14 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: Some(14),
                    found: f.len(),
                },
            ));
        }
        let n = l.num;
        for (i, want) in [
            (0, "VEHICLE"),
            (2, "FIELD"),
            (4, "POS"),
            (8, "ANGLES"),
            (12, "FOV"),
        ] {
            if f[i] != want {
                return Err(Error::new(
                    file,
                    n,
                    ErrorKind::UnknownMarker {
                        marker: f[i].clone(),
                    },
                ));
            }
        }
        out.push(VehoffRow {
            vehicle: f[1].clone(),
            field: f[3].clone(),
            pos: [
                parse_f32(file, n, &f, 5)?,
                parse_f32(file, n, &f, 6)?,
                parse_f32(file, n, &f, 7)?,
            ],
            angles: [
                parse_f32(file, n, &f, 9)?,
                parse_f32(file, n, &f, 10)?,
                parse_f32(file, n, &f, 11)?,
            ],
            fov: parse_f32(file, n, &f, 13)?,
        });
    }
    Ok(out)
}

/// One shoreline segment: a polyline plus flags.
#[derive(Debug, Clone)]
pub struct Shoreline {
    /// Points in file order.
    pub points: Vec<[f32; 2]>,
    /// Flag value from the segment header.
    pub flags: i32,
}

/// Parse a `shorelines.dat` file: segment count, then segments.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_shorelines(file: &str, bytes: &[u8]) -> Result<Vec<Shoreline>> {
    let text = decode(file, bytes)?;
    let mut lines = logical_lines(&text, CommentStyle::NONE, false);
    let first = lines.next().ok_or_else(|| {
        Error::whole_file(
            file,
            ErrorKind::BadHeader {
                want: "segment count",
            },
        )
    })?;
    let count = first.code.trim().parse::<usize>().map_err(|_| {
        Error::new(
            file,
            first.num,
            ErrorKind::BadHeader {
                want: "segment count",
            },
        )
    })?;
    let mut out = Vec::with_capacity(count.min(10000));
    while let Some(l) = lines.next() {
        let f = split_ws(&l.code);
        if f.len() != 2 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: Some(2),
                    found: f.len(),
                },
            ));
        }
        let npoints = parse_i32(file, l.num, &f, 0)?;
        let flags = parse_i32(file, l.num, &f, 1)?;
        if npoints < 0 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::BadScalar {
                    field: 0,
                    want: "non-negative i32",
                },
            ));
        }
        let count = usize::try_from(npoints).map_err(|_| {
            Error::new(
                file,
                l.num,
                ErrorKind::BadScalar {
                    field: 0,
                    want: "non-negative i32",
                },
            )
        })?;
        // Cap the pre-allocation: the count is hostile input, and the loop
        // below stops at end of input regardless.
        let mut points = Vec::with_capacity(count.min(1024));
        for _ in 0..count {
            let pl = lines.next().ok_or_else(|| {
                Error::whole_file(
                    file,
                    ErrorKind::BadHeader {
                        want: "shoreline point",
                    },
                )
            })?;
            let pf = split_ws(&pl.code);
            if pf.len() != 2 {
                return Err(Error::new(
                    file,
                    pl.num,
                    ErrorKind::FieldCount {
                        expected: Some(2),
                        found: pf.len(),
                    },
                ));
            }
            points.push([
                parse_f32(file, pl.num, &pf, 0)?,
                parse_f32(file, pl.num, &pf, 1)?,
            ]);
        }
        out.push(Shoreline { points, flags });
    }
    if out.len() != count {
        return Err(Error::whole_file(
            file,
            ErrorKind::BadHeader {
                want: "declared segment count",
            },
        ));
    }
    Ok(out)
}

/// Parse a `numplate.dat` file: RGB triples under `;` comments.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_numplate(file: &str, bytes: &[u8]) -> Result<Vec<[u8; 3]>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::SEMICOLON_HASH, false) {
        let f = split_ws(&l.code);
        if f.len() != 3 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: Some(3),
                    found: f.len(),
                },
            ));
        }
        out.push([
            parse_u8(file, l.num, &f, 0)?,
            parse_u8(file, l.num, &f, 1)?,
            parse_u8(file, l.num, &f, 2)?,
        ]);
    }
    Ok(out)
}

/// Parse a `stockshake.txt` file: bare rows of 10 numbers.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_stockshake(file: &str, bytes: &[u8]) -> Result<Vec<[f32; 10]>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::NONE, false) {
        let f = split_ws(&l.code);
        if f.len() != 10 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: Some(10),
                    found: f.len(),
                },
            ));
        }
        let mut row = [0.0f32; 10];
        for (i, slot) in row.iter_mut().enumerate() {
            *slot = parse_f32(file, l.num, &f, i)?;
        }
        out.push(row);
    }
    Ok(out)
}

/// One train camera node: id, position, detect distance.
#[derive(Debug, Clone)]
pub struct TrainCam {
    /// Node id.
    pub id: String,
    /// Position.
    pub pos: [f32; 3],
    /// Detect distance.
    pub detect: f32,
}

/// Parse a `trainCamNodes.txt` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_train_cams(file: &str, bytes: &[u8]) -> Result<Vec<TrainCam>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::SLASHES, false) {
        let f = split_ws(&l.code);
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
        out.push(TrainCam {
            id: f[0].clone(),
            pos: [
                parse_f32(file, l.num, &f, 1)?,
                parse_f32(file, l.num, &f, 2)?,
                parse_f32(file, l.num, &f, 3)?,
            ],
            detect: parse_f32(file, l.num, &f, 4)?,
        });
    }
    Ok(out)
}

/// Parse a `KEY=value` file (`nav.dat`, `stream.ini`).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_key_value(file: &str, bytes: &[u8]) -> Result<Vec<(String, String)>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::HASH, false) {
        match l.code.split_once('=') {
            Some((k, v)) => out.push((k.trim().to_string(), v.trim().to_string())),
            None => {
                return Err(Error::new(
                    file,
                    l.num,
                    ErrorKind::BadHeader { want: "KEY=value" },
                ));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // parsed values are compared bit for bit on purpose
mod tests {
    use super::*;

    #[test]
    fn parses_csv_with_header() {
        let src = "Artist,Song,Station,\r\n0,Seryoga ,Vladivostok FM,RADIO_X\r\n";
        let t = parse_csv_table("songlist.csv", src.as_bytes(), true).unwrap();
        assert_eq!(t.header.as_ref().unwrap().len(), 4);
        assert_eq!(t.rows.len(), 1);
    }

    #[test]
    fn parses_vehoff_row() {
        let src = "VEHICLE admiral FIELD BOOT_OPEN POS 0.39 -2.01 -0.23 ANGLES 0.93 0.24 2.76 FOV 55.0\r\n";
        let v = parse_vehoff("vehOff.csv", src.as_bytes()).unwrap();
        assert_eq!(v[0].vehicle, "admiral");
        assert_eq!(v[0].fov, 55.0);
    }

    #[test]
    fn parses_shorelines() {
        let src = "1\r\n2 0\r\n-740.9 1192.9\r\n-734.2 1203.5\r\n";
        let s = parse_shorelines("shorelines.dat", src.as_bytes()).unwrap();
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].points.len(), 2);
    }

    #[test]
    fn parses_key_value() {
        let src = "# c\r\nSECTORS_PER_NAVMESH=2\r\n";
        let kv = parse_key_value("nav.dat", src.as_bytes()).unwrap();
        assert_eq!(
            kv,
            vec![("SECTORS_PER_NAVMESH".to_string(), "2".to_string())]
        );
    }
}
