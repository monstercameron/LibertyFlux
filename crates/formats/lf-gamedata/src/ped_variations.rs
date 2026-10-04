//! Ped clothing tables: `pedVariations.dat` and `pedProps.dat`.
//!
//! Grammar: `#` comments, comma-separated rows, blocks opened by a
//! `MODEL, count` header line and closed by `end`. The count states how
//! many rows follow; the parser checks it.
//!
//! * `pedVariations.dat` rows (12 fields): slot, geometry id, bulk, job,
//!   sunny, wet, mat1, res1a, res1b, res2a, res2b, audio id (column names
//!   from the repeated in-file header comment).
//! * `pedProps.dat` rows (8 fields): slot, number, hot/cold weather,
//!   expensive, cheap, skiing, special1.

use crate::text::{CommentStyle, field, logical_lines, parse_i32, split_csv};
use crate::{Error, ErrorKind, Result, decode};

/// One variation block: a model plus its clothing rows.
#[derive(Debug, Clone)]
pub struct VariationBlock {
    /// Model name.
    pub model: String,
    /// Rows in file order.
    pub rows: Vec<VariationRow>,
}

/// One `pedVariations.dat` row (12 fields).
#[derive(Debug, Clone)]
pub struct VariationRow {
    /// Clothing slot (`head`, `uppr`, `lowr`, `feet`, `suse`, ...).
    pub slot: String,
    /// Geometry id.
    pub geometry: i32,
    /// Bulk value.
    pub bulk: i32,
    /// Job value.
    pub job: i32,
    /// Sunny value.
    pub sunny: i32,
    /// Wet value.
    pub wet: i32,
    /// Material 1.
    pub mat1: i32,
    /// Reserved fields (always -1 in shipped files).
    pub res: [i32; 4],
    /// Audio id.
    pub audio_id: i32,
}

/// One `pedProps.dat` block.
#[derive(Debug, Clone)]
pub struct PropBlock {
    /// Model name.
    pub model: String,
    /// Rows in file order.
    pub rows: Vec<PropRow>,
}

/// One `pedProps.dat` row (8 fields).
#[derive(Debug, Clone)]
pub struct PropRow {
    /// Prop slot (`head`, `eyes`, `hip`, ...).
    pub slot: String,
    /// Prop number.
    pub number: i32,
    /// Hot weather value.
    pub hot_weather: i32,
    /// Cold weather value.
    pub cold_weather: i32,
    /// Expensive value.
    pub expensive: i32,
    /// Cheap value.
    pub cheap: i32,
    /// Skiing value.
    pub skiing: i32,
    /// Special1 value.
    pub special1: i32,
}

fn variation_row(file: &str, num: usize, f: &[String]) -> Result<VariationRow> {
    if f.len() != 12 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: Some(12),
                found: f.len(),
            },
        ));
    }
    Ok(VariationRow {
        slot: field(file, num, f, 0, Some(12))?.to_string(),
        geometry: parse_i32(file, num, f, 1)?,
        bulk: parse_i32(file, num, f, 2)?,
        job: parse_i32(file, num, f, 3)?,
        sunny: parse_i32(file, num, f, 4)?,
        wet: parse_i32(file, num, f, 5)?,
        mat1: parse_i32(file, num, f, 6)?,
        res: [
            parse_i32(file, num, f, 7)?,
            parse_i32(file, num, f, 8)?,
            parse_i32(file, num, f, 9)?,
            parse_i32(file, num, f, 10)?,
        ],
        audio_id: parse_i32(file, num, f, 11)?,
    })
}

fn prop_row(file: &str, num: usize, f: &[String]) -> Result<PropRow> {
    if f.len() != 8 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: Some(8),
                found: f.len(),
            },
        ));
    }
    Ok(PropRow {
        slot: field(file, num, f, 0, Some(8))?.to_string(),
        number: parse_i32(file, num, f, 1)?,
        hot_weather: parse_i32(file, num, f, 2)?,
        cold_weather: parse_i32(file, num, f, 3)?,
        expensive: parse_i32(file, num, f, 4)?,
        cheap: parse_i32(file, num, f, 5)?,
        skiing: parse_i32(file, num, f, 6)?,
        special1: parse_i32(file, num, f, 7)?,
    })
}

/// Shared block driver: `MODEL, count ... end`.
fn blocks<T, F>(file: &str, text: &str, mut row: F) -> Result<Vec<(String, Vec<T>)>>
where
    F: FnMut(&str, usize, &[String]) -> Result<T>,
{
    // Open block state: model name, declared row count, header line, rows.
    struct Open<T> {
        model: String,
        want: usize,
        line: usize,
        rows: Vec<T>,
    }
    let mut out: Vec<(String, Vec<T>)> = Vec::new();
    let mut current: Option<Open<T>> = None;
    for l in logical_lines(text, CommentStyle::HASH, false) {
        if l.code.eq_ignore_ascii_case("end") {
            match current.take() {
                Some(o) => {
                    if o.rows.len() != o.want {
                        return Err(Error::new(
                            file,
                            o.line,
                            ErrorKind::FieldCount {
                                expected: Some(o.want),
                                found: o.rows.len(),
                            },
                        ));
                    }
                    out.push((o.model, o.rows));
                }
                None => return Err(Error::new(file, l.num, ErrorKind::StrayEnd)),
            }
            continue;
        }
        let f = split_csv(&l.code);
        if let Some(o) = current.as_mut() {
            o.rows.push(row(file, l.num, &f)?);
        } else {
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
            let want_raw = parse_i32(file, l.num, &f, 1)?;
            let want = usize::try_from(want_raw).map_err(|_| {
                Error::new(
                    file,
                    l.num,
                    ErrorKind::BadScalar {
                        field: 1,
                        want: "non-negative i32",
                    },
                )
            })?;
            current = Some(Open {
                model: f[0].clone(),
                want,
                line: l.num,
                rows: Vec::new(),
            });
        }
    }
    if let Some(o) = current {
        return Err(Error::whole_file(
            file,
            ErrorKind::Unterminated { section: o.model },
        ));
    }
    Ok(out)
}

/// Parse a `pedVariations.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_ped_variations(file: &str, bytes: &[u8]) -> Result<Vec<VariationBlock>> {
    let text = decode(file, bytes)?;
    Ok(blocks(file, &text, variation_row)?
        .into_iter()
        .map(|(model, rows)| VariationBlock { model, rows })
        .collect())
}

/// Parse a `pedProps.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_ped_props(file: &str, bytes: &[u8]) -> Result<Vec<PropBlock>> {
    let text = decode(file, bytes)?;
    Ok(blocks(file, &text, prop_row)?
        .into_iter()
        .map(|(model, rows)| PropBlock { model, rows })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_variation_block() {
        let src = "F_M_Business_01, 2\r\n#slot geometry\r\nfeet, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, 3\r\nhead, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, 10\r\nend\r\n";
        let b = parse_ped_variations("pedVariations.dat", src.as_bytes()).unwrap();
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].rows.len(), 2);
        assert_eq!(b[0].rows[1].audio_id, 10);
    }

    #[test]
    fn wrong_count_is_error() {
        let src = "M, 2\r\nhead, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, 0\r\nend\r\n";
        let e = parse_ped_variations("pedVariations.dat", src.as_bytes()).unwrap_err();
        assert!(matches!(e.kind, ErrorKind::FieldCount { .. }));
    }

    #[test]
    fn parses_prop_block() {
        let src = "M_Y_Uptown_01_p, 1\r\nhead, 0, 0, 0, 0, 0, 0, 0\r\nend\r\n";
        let b = parse_ped_props("pedProps.dat", src.as_bytes()).unwrap();
        assert_eq!(b[0].rows[0].slot, "head");
    }
}
