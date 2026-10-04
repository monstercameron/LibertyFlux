//! Effect tables: `effects/*.dat`.
//!
//! Every effects file opens with a version line (`1.0` / `1.00`), then a
//! `*_TABLE_START` marker line, then whitespace-separated rows. Column
//! meanings come from the two-line `#` header above the marker in each
//! file:
//!
//! * `explosionFx.dat`: type, damage at centre/edge, network player/ped
//!   modifiers, end radius, init speed, decay, force, ragdoll modifier,
//!   directed width/life, ground/air FX names, FX scale, fire counts and
//!   ranges, camera shake, weapon type, infinite flag, six light params,
//!   collision type (28 values after the name).
//! * `bloodFx.dat`, `entityFx.dat`, `materialFx.dat`, `pedFx.dat`,
//!   `vehicleFx.dat`, `weaponFx.dat`: target name plus numeric effect ids
//!   per damage/attack class (kept as tokens; per-column roles differ per
//!   file and are documented in `format-spec.md`).
//!
//! [`FxTable`] keeps rows structurally; [`ExplosionRow`] types the
//! explosion table fully.

use crate::text::{CommentStyle, field, logical_lines, parse_f32, parse_i32, split_ws};
use crate::{Error, ErrorKind, Result, decode};

/// One generic effect row: name plus raw tokens.
#[derive(Debug, Clone)]
pub struct FxRow {
    /// 1-based line number.
    pub line: usize,
    /// Row name (first token).
    pub name: String,
    /// Remaining tokens.
    pub tokens: Vec<String>,
}

/// One effects file: version, table marker, rows.
#[derive(Debug, Clone)]
pub struct FxTable {
    /// Version from the first line.
    pub version: f32,
    /// Table marker (`EXPLOSIONFX_TABLE_START`, ...).
    pub marker: String,
    /// Rows in file order.
    pub rows: Vec<FxRow>,
    /// Closing marker (`EXPLOSIONFX_TABLE_END`), if the file has one.
    pub end_marker: Option<String>,
}

/// Parse any `effects/*.dat` file structurally.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_fx_table(file: &str, bytes: &[u8]) -> Result<FxTable> {
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
    let marker = lines.next().ok_or_else(|| {
        Error::whole_file(
            file,
            ErrorKind::BadHeader {
                want: "TABLE_START marker",
            },
        )
    })?;
    if marker.code.split_whitespace().count() != 1 {
        return Err(Error::new(
            file,
            marker.num,
            ErrorKind::BadHeader {
                want: "TABLE_START marker",
            },
        ));
    }
    let mut rows = Vec::new();
    let mut end_marker = None;
    for l in lines {
        let f = split_ws(&l.code);
        if f.is_empty() {
            continue;
        }
        if f.len() == 1 && f[0].ends_with("_TABLE_END") {
            end_marker = Some(f[0].clone());
            continue;
        }
        rows.push(FxRow {
            line: l.num,
            name: f[0].clone(),
            tokens: f[1..].to_vec(),
        });
    }
    Ok(FxTable {
        version,
        marker: marker.code,
        rows,
        end_marker,
    })
}

/// One `explosionFx.dat` row (name + 30 values).
#[derive(Debug, Clone)]
pub struct ExplosionRow {
    /// Explosion type name.
    pub name: String,
    /// Damage at centre and edge.
    pub damage: [f32; 2],
    /// Network player/ped damage modifiers.
    pub network_mod: [f32; 2],
    /// End radius.
    pub end_radius: f32,
    /// Initial speed.
    pub init_speed: f32,
    /// Decay factor.
    pub decay_factor: f32,
    /// Force factor.
    pub force_factor: f32,
    /// Ragdoll modifier.
    pub ragdoll_modifier: f32,
    /// Directed width and life.
    pub directed: [f32; 2],
    /// Ground and air FX names.
    pub fx_names: [String; 2],
    /// FX scale.
    pub fx_scale: f32,
    /// Fire count min/max.
    pub fire_count: [i32; 2],
    /// Fire range min/max.
    pub fire_range: [f32; 2],
    /// Camera shake.
    pub cam_shake: f32,
    /// Weapon type name.
    pub weapon_type: String,
    /// Infinite flag.
    pub infinite: i32,
    /// Light parameters (6 values, roles undocumented).
    pub lights: [f32; 6],
    /// Collision type (1 ground plane, 2 ground bound, 3 vehicle up).
    pub coln_type: i32,
}

/// Type one explosion row; needs name + 28 tokens.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn explosion_row(file: &str, row: &FxRow) -> Result<ExplosionRow> {
    let t = &row.tokens;
    if t.len() != 28 {
        return Err(Error::new(
            file,
            row.line,
            ErrorKind::FieldCount {
                expected: Some(29),
                found: t.len() + 1,
            },
        ));
    }
    // Reuse the scalar helpers by borrowing tokens as fields.
    let owned: Vec<String> = t.clone();
    let n = row.line;
    let fl = |i: usize| parse_f32(file, n, &owned, i);
    let ii = |i: usize| parse_i32(file, n, &owned, i);
    let g = |i: usize| field(file, n, &owned, i, Some(28)).map(std::string::ToString::to_string);
    Ok(ExplosionRow {
        name: row.name.clone(),
        damage: [fl(0)?, fl(1)?],
        network_mod: [fl(2)?, fl(3)?],
        end_radius: fl(4)?,
        init_speed: fl(5)?,
        decay_factor: fl(6)?,
        force_factor: fl(7)?,
        ragdoll_modifier: fl(8)?,
        directed: [fl(9)?, fl(10)?],
        fx_names: [g(11)?, g(12)?],
        fx_scale: fl(13)?,
        fire_count: [ii(14)?, ii(15)?],
        fire_range: [fl(16)?, fl(17)?],
        cam_shake: fl(18)?,
        weapon_type: g(19)?,
        infinite: ii(20)?,
        lights: [fl(21)?, fl(22)?, fl(23)?, fl(24)?, fl(25)?, fl(26)?],
        coln_type: ii(27)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_explosion_table() {
        let src = "1.0\r\n# header\r\nEXPLOSIONFX_TABLE_START\r\nGRENADE 500.0 75.0 2.0 1.0 7.0 50.0 -1.0 30.0 0.0016 0.0 0.0 exp_grenade exp_grenade_air 1.0 0 0 0.0 0.0 0.6 GRENADE 0 1.0 0.3 0.0 250.0 24.0 0.3 2\r\n";
        let t = parse_fx_table("explosionFx.dat", src.as_bytes()).unwrap();
        assert_eq!(t.version, 1.0);
        assert_eq!(t.rows.len(), 1);
        let e = explosion_row("explosionFx.dat", &t.rows[0]).unwrap();
        assert_eq!(e.end_radius, 7.0);
        assert_eq!(e.coln_type, 2);
    }
}
