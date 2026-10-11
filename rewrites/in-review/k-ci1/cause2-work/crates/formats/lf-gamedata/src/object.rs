//! `object.dat`: physics tuning for dynamic props.
//!
//! Grammar: `;` comments, comma-separated rows (17 fields). Column letters
//! from the in-file header: (A) object name, (B) mass kg, (C) turn mass,
//! (D) air resistance 0-1, (E) elasticity 0-1, (F) percent submerged,
//! (G) uproot limit, (H) collision damage multiplier, (I) collision
//! damage effect id, four small ints of unknown role, three floats of
//! unknown role (usually 0.0), and an explosion FX name.
//!
//! The header comments in the shipped file stop describing columns after
//! the damage effect; the trailing numbers are typed by shape only.

use crate::text::{
    CommentStyle, field, heal_comma_row, logical_lines, parse_f32, parse_i32, split_csv,
};
use crate::{Error, ErrorKind, Result, decode};

/// One object physics row.
#[derive(Debug, Clone)]
pub struct ObjectRow {
    /// Object name.
    pub name: String,
    /// Mass in kg (1-50000).
    pub mass: f32,
    /// Turn mass.
    pub turn_mass: f32,
    /// Air resistance, 0 (total) to 1 (none).
    pub air_resistance: f32,
    /// Elasticity, 0 (no bounce) to 1 (full bounce).
    pub elasticity: f32,
    /// Percent submerged, 10-120.
    pub percent_submerged: f32,
    /// Force needed to uproot.
    pub uproot_limit: f32,
    /// Collision damage multiplier.
    pub collision_damage_mult: f32,
    /// Collision damage effect id (0 none, 1 `change_model`, 20 smash,
    /// 21 `change_then_smash`, 200 breakable, 202 breakable-then-removed).
    pub collision_damage_effect: i32,
    /// Four unknown small ints (fields 9-12).
    pub unk_ints: [i32; 4],
    /// Three unknown floats (fields 13-15, usually 0.0).
    pub unk_floats: [f32; 3],
    /// Explosion FX name.
    pub explosion_fx: String,
    /// Extended tail on `dyn_*` rows (7 more values of unknown role);
    /// empty on standard 17-field rows.
    pub extra: Vec<String>,
}

/// Outcome of parsing one file: rows plus per-row failures.
///
/// One shipped row is truncated mid-record; it is reported here rather
/// than failing the file, mirroring [`crate::route::IdeOutcome`].
#[derive(Debug, Default)]
pub struct ObjectOutcome {
    /// Typed rows.
    pub rows: Vec<ObjectRow>,
    /// Failures: (line, error).
    pub row_errors: Vec<(usize, Error)>,
}

fn one_row(file: &str, num: usize, f: &[String]) -> Result<ObjectRow> {
    if f.len() < 17 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: Some(17),
                found: f.len(),
            },
        ));
    }
    Ok(ObjectRow {
        name: field(file, num, f, 0, Some(17))?.to_string(),
        mass: parse_f32(file, num, f, 1)?,
        turn_mass: parse_f32(file, num, f, 2)?,
        air_resistance: parse_f32(file, num, f, 3)?,
        elasticity: parse_f32(file, num, f, 4)?,
        percent_submerged: parse_f32(file, num, f, 5)?,
        uproot_limit: parse_f32(file, num, f, 6)?,
        collision_damage_mult: parse_f32(file, num, f, 7)?,
        collision_damage_effect: parse_i32(file, num, f, 8)?,
        unk_ints: [
            parse_i32(file, num, f, 9)?,
            parse_i32(file, num, f, 10)?,
            parse_i32(file, num, f, 11)?,
            parse_i32(file, num, f, 12)?,
        ],
        unk_floats: [
            parse_f32(file, num, f, 13)?,
            parse_f32(file, num, f, 14)?,
            parse_f32(file, num, f, 15)?,
        ],
        explosion_fx: field(file, num, f, 16, Some(17))?.to_string(),
        extra: f[17..].to_vec(),
    })
}

/// Parse an `object.dat` file from bytes.
///
/// Lines starting with `*` are banner comments (two shipped section
/// dividers use them instead of `;`).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_object(file: &str, bytes: &[u8]) -> Result<ObjectOutcome> {
    let text = decode(file, bytes)?;
    let mut out = ObjectOutcome::default();
    for l in logical_lines(&text, CommentStyle::SEMICOLON_HASH, false) {
        if l.code.starts_with('*') {
            continue;
        }
        // One shipped row misses a comma; heal it like IDE rows.
        // `dyn_*` rows carry a 7-value extended tail after the FX name.
        let (f, _) = heal_comma_row(&split_csv(&l.code));
        match one_row(file, l.num, &f) {
            Ok(row) => out.rows.push(row),
            Err(e) => out.row_errors.push((l.num, e)),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_row() {
        let src = "; c\r\ncardboardbox, 10.0, 10.0, 0.99, 0.03, 50.0, 0.0, 2.5, 20, 2, 1, 0, 2, 0, 0.0, 0.0, explosion_crate\r\n";
        let o = parse_object("object.dat", src.as_bytes()).unwrap();
        assert_eq!(o.rows.len(), 1);
        assert_eq!(o.rows[0].collision_damage_effect, 20);
        assert_eq!(o.rows[0].explosion_fx, "explosion_crate");
    }

    #[test]
    fn truncated_row_is_collected() {
        let src = "JUD_LAN 99999.0, 99999.0, 0.99,\r\n";
        let o = parse_object("object.dat", src.as_bytes()).unwrap();
        assert_eq!(o.rows.len(), 0);
        assert_eq!(o.row_errors.len(), 1);
    }
}
