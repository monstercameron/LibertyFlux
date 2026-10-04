//! Surface tables: `materials.dat`, `procedural.dat`, `mtl_convert.txt`.
//!
//! `materials.dat` opens with a version line (`2.00`), then `#`-commented
//! rows of 17 whitespace fields: material name, FX group, audio group,
//! rage friction, rage elasticity, density, tyre grip, wet grip,
//! roughness, ped density, flammability, burn time, burn strength, three
//! flags (see-through, no-camera-collision, unknown) and a repeat of the
//! material name (column meanings from the in-file header comments).
//!
//! `procedural.dat` shares the version line and row shape with different
//! column use (see [`ProceduralRow`]); `mtl_convert.txt` maps old
//! materials to new materials, procedurals, room ids, population
//! multipliers and flags (see [`MtlConvertRow`]).

use crate::text::{CommentStyle, field, logical_lines, parse_f32, parse_i32, split_ws};
use crate::{Error, ErrorKind, Result, decode};

/// One `materials.dat` row (17 fields).
#[derive(Debug, Clone)]
pub struct MaterialRow {
    /// Material name.
    pub name: String,
    /// FX group the material maps to.
    pub fx_group: String,
    /// Audio group.
    pub audio_group: String,
    /// RAGE friction.
    pub friction: f32,
    /// RAGE elasticity.
    pub elasticity: f32,
    /// Material density.
    pub density: f32,
    /// Tyre grip override.
    pub tyre_grip: f32,
    /// Wet grip multiplier.
    pub wet_grip: f32,
    /// Roughness 0-3.
    pub roughness: i32,
    /// Ped density 0-3.
    pub ped_density: f32,
    /// Flammability 0.0-1.0.
    pub flammability: f32,
    /// Burn time in seconds.
    pub burn_time: f32,
    /// Burn strength 0.0-1.0.
    pub burn_strength: f32,
    /// Three flag ints.
    pub flags: [i32; 3],
    /// Trailing repeat of the material name.
    pub repeat: String,
}

/// One `procedural.dat` row: name plus numbers (column use undocumented).
#[derive(Debug, Clone)]
pub struct ProceduralRow {
    /// Material name.
    pub name: String,
    /// Remaining tokens.
    pub tokens: Vec<String>,
}

/// Parse a `materials.dat` file from bytes: version plus rows.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_materials(file: &str, bytes: &[u8]) -> Result<(f32, Vec<MaterialRow>)> {
    let text = decode(file, bytes)?;
    let mut lines = logical_lines(&text, CommentStyle::HASH, false);
    let first = lines.next().ok_or_else(|| {
        Error::whole_file(
            file,
            ErrorKind::BadHeader {
                want: "version line",
            },
        )
    })?;
    let version = first
        .code
        .trim()
        .parse::<f32>()
        .map_err(|_| Error::new(file, first.num, ErrorKind::BadHeader { want: "version" }))?;
    let mut out = Vec::new();
    for l in lines {
        let f = split_ws(&l.code);
        if f.len() != 17 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: Some(17),
                    found: f.len(),
                },
            ));
        }
        let n = l.num;
        let g = |i: usize| field(file, n, &f, i, Some(17)).map(std::string::ToString::to_string);
        out.push(MaterialRow {
            name: g(0)?,
            fx_group: g(1)?,
            audio_group: g(2)?,
            friction: parse_f32(file, n, &f, 3)?,
            elasticity: parse_f32(file, n, &f, 4)?,
            density: parse_f32(file, n, &f, 5)?,
            tyre_grip: parse_f32(file, n, &f, 6)?,
            wet_grip: parse_f32(file, n, &f, 7)?,
            roughness: parse_i32(file, n, &f, 8)?,
            ped_density: parse_f32(file, n, &f, 9)?,
            flammability: parse_f32(file, n, &f, 10)?,
            burn_time: parse_f32(file, n, &f, 11)?,
            burn_strength: parse_f32(file, n, &f, 12)?,
            flags: [
                parse_i32(file, n, &f, 13)?,
                parse_i32(file, n, &f, 14)?,
                parse_i32(file, n, &f, 15)?,
            ],
            repeat: g(16)?,
        });
    }
    Ok((version, out))
}

/// Parse a `procedural.dat` file from bytes: version plus token rows.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_procedural(file: &str, bytes: &[u8]) -> Result<(f32, Vec<ProceduralRow>)> {
    let text = decode(file, bytes)?;
    let mut lines = logical_lines(&text, CommentStyle::HASH, false);
    let first = lines.next().ok_or_else(|| {
        Error::whole_file(
            file,
            ErrorKind::BadHeader {
                want: "version line",
            },
        )
    })?;
    let version = first
        .code
        .trim()
        .parse::<f32>()
        .map_err(|_| Error::new(file, first.num, ErrorKind::BadHeader { want: "version" }))?;
    let mut out = Vec::new();
    for l in lines {
        let f = split_ws(&l.code);
        if f.is_empty() {
            continue;
        }
        out.push(ProceduralRow {
            name: f[0].clone(),
            tokens: f[1..].to_vec(),
        });
    }
    Ok((version, out))
}

/// One `mtl_convert.txt` row: old material, new material, procedural
/// tokens, room id, population multiplier, flags. Column names from the
/// in-file header comment.
#[derive(Debug, Clone)]
pub struct MtlConvertRow {
    /// Old material name.
    pub old: String,
    /// New material name.
    pub new: String,
    /// Procedural tokens between the new material and the room id:
    /// usually one (`-`, `##` or a name), sometimes two (`##` plus a
    /// name). `##` has unknown meaning.
    pub procs: Vec<String>,
    /// Room id, if the row carries the numeric tail.
    pub room_id: Option<i32>,
    /// Population multiplier, if present.
    pub pop_mult: Option<i32>,
    /// Flags, if present.
    pub flags: Option<i32>,
}

/// Parse an `mtl_convert.txt` file from bytes.
///
/// `#` starts a comment only at line start here: `##` appears as a data
/// value (unknown meaning) in the procedural column.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_mtl_convert(file: &str, bytes: &[u8]) -> Result<Vec<MtlConvertRow>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::NONE, false) {
        if l.code.starts_with('#') {
            continue;
        }
        let f = split_ws(&l.code);
        if f.len() < 3 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: Some(6),
                    found: f.len(),
                },
            ));
        }
        let n = l.num;
        // Two shipped rows omit the numeric tail; take it only when the
        // last three tokens parse as integers.
        let has_tail = f.len() >= 6
            && f[f.len() - 3].parse::<i32>().is_ok()
            && f[f.len() - 2].parse::<i32>().is_ok()
            && f[f.len() - 1].parse::<i32>().is_ok();
        let tail = if has_tail { f.len() - 3 } else { f.len() };
        out.push(MtlConvertRow {
            old: field(file, n, &f, 0, None)?.to_string(),
            new: field(file, n, &f, 1, None)?.to_string(),
            procs: f[2..tail].to_vec(),
            room_id: has_tail.then(|| parse_i32(file, n, &f, tail)).transpose()?,
            pop_mult: has_tail
                .then(|| parse_i32(file, n, &f, tail + 1))
                .transpose()?,
            flags: has_tail
                .then(|| parse_i32(file, n, &f, tail + 2))
                .transpose()?,
        });
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // parsed values are compared bit for bit on purpose
mod tests {
    use super::*;

    #[test]
    fn parses_material_row() {
        let src = "2.00\r\n# c\r\nCONCRETE CONCRETE DEFAULT 1.0 0.1 1800.0 1.00 -0.10 0 0.0 0.0 6.0 0.7 0 0 0 CONCRETE\r\n";
        let (v, rows) = parse_materials("materials.dat", src.as_bytes()).unwrap();
        assert_eq!(v, 2.0);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].density, 1800.0);
    }

    #[test]
    fn parses_convert_row() {
        let src = "DEFAULT default - 0 7 0\r\n# comment\r\nGRASS_SHORT_LUSH grass_short ## EXT_GRASS_X 0 7 0\r\n";
        let p = parse_mtl_convert("mtl_convert.txt", src.as_bytes()).unwrap();
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].old, "DEFAULT");
        assert_eq!(p[0].procs, vec!["-"]);
        assert_eq!(p[0].pop_mult, Some(7));
        assert_eq!(p[1].procs, vec!["##", "EXT_GRASS_X"]);
    }
}
