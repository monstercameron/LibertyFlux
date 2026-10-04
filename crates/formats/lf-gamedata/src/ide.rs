//! `*.ide` item definition files.
//!
//! Grammar: `#` comments (leading and trailing), comma-separated rows,
//! sections opened by a bare word (`cars`, `peds`, `objs`, `txdp`, `weap`,
//! `hier`, `amat`, `2dfx`, `agrps`) and closed by `end`. Every row ends
//! with a comma.
//!
//! Sections and their columns (from the header comments in the shipped
//! files):
//!
//! * `cars` (vehicles.ide): 16 fields per row — model, texture dictionary,
//!   type, handling id, game name, two anim groups, frequency, max number,
//!   front/rear wheel radius, dirt level, swankness, lod multiplier,
//!   wheel id/scale, extras flags.
//! * `peds` (peds.ide): 16 fields — model, props, ped type, anim group,
//!   gesture/phone/facial/viseme groups, flags, anim file, blendshape,
//!   two radio stations, audio type, first/last voice. A few shipped rows
//!   have typos (missing or doubled commas); see [`PedRow::parse_lenient`].
//! * `objs`: model, txd, draw distance, flags.
//! * `txdp`: model, texture dictionary (parent txd link).
//! * `weap`: model, txd, anim, ... (weapon pickups).
//! * `hier`: id, model, parent, ... (cutscene hierarchy).
//! * `amat`: model, ... (anim materials? names only, meanings unknown).
//! * `2dfx`: 18 fields (attached effects).
//! * `agrps`: ped model, variation model, index (cop/medic variants).

use crate::text::{
    CommentStyle, field, heal_comma_row, is_some_token, logical_lines, parse_f32, parse_i32,
    split_csv,
};
use crate::{Error, ErrorKind, Result, decode};

/// One raw IDE section: name plus untyped rows.
#[derive(Debug, Clone)]
pub struct RawSection {
    /// Section name as written (`cars`, `peds`, ...).
    pub name: String,
    /// 1-based line where the section opened.
    pub line: usize,
    /// Rows; each row is the comma-split fields of one line.
    pub rows: Vec<RawRow>,
}

/// One raw row: fields plus its line number.
#[derive(Debug, Clone)]
pub struct RawRow {
    /// 1-based line number.
    pub line: usize,
    /// Comma-split, trimmed fields.
    pub fields: Vec<String>,
}

/// A parsed IDE file: sections in order.
#[derive(Debug, Clone, Default)]
pub struct IdeFile {
    /// Sections in file order.
    pub sections: Vec<RawSection>,
}

impl IdeFile {
    /// Find the first section with this name.
    #[must_use]
    pub fn section(&self, name: &str) -> Option<&RawSection> {
        self.sections.iter().find(|s| s.name == name)
    }
}

/// Parse any `*.ide` file into raw sections.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_ide(file: &str, bytes: &[u8]) -> Result<IdeFile> {
    let text = decode(file, bytes)?;
    let mut out = IdeFile::default();
    let mut current: Option<RawSection> = None;
    for l in logical_lines(&text, CommentStyle::HASH, false) {
        let code = l.code.trim();
        if code.eq_ignore_ascii_case("end") {
            match current.take() {
                Some(s) => out.sections.push(s),
                None => return Err(Error::new(file, l.num, ErrorKind::StrayEnd)),
            }
            continue;
        }
        if !code.contains(',') {
            if current.is_some() {
                // A bare word inside a section: only legal as `end`, handled
                // above. Anything else is a malformed row.
                return Err(Error::new(
                    file,
                    l.num,
                    ErrorKind::UnknownMarker {
                        marker: code.to_string(),
                    },
                ));
            }
            current = Some(RawSection {
                name: code.to_string(),
                line: l.num,
                rows: Vec::new(),
            });
            continue;
        }
        match current.as_mut() {
            Some(s) => s.rows.push(RawRow {
                line: l.num,
                fields: split_csv(code),
            }),
            None => return Err(Error::new(file, l.num, ErrorKind::StrayEnd)),
        }
    }
    // The episode files end without a closing `end`; the game evidently
    // tolerates it, so an open section at EOF is closed, not an error.
    if let Some(s) = current {
        out.sections.push(s);
    }
    Ok(out)
}

/// One `cars` row (16 fields). See module docs for column meanings.
#[derive(Debug, Clone)]
pub struct CarRow {
    /// Model name.
    pub model: String,
    /// Texture dictionary name.
    pub txd: String,
    /// Vehicle type (`car`, `bike`, `heli`, `boat`, `train`).
    pub vehicle_type: String,
    /// Handling id, links [`crate::handling`].
    pub handling_id: String,
    /// Game (GXT) name.
    pub game_name: String,
    /// Primary anim group.
    pub anims: String,
    /// Secondary anim group, or `NULL`.
    pub anims2: Option<String>,
    /// Spawn frequency.
    pub frequency: i32,
    /// Max number alive.
    pub max_num: i32,
    /// Front/rear wheel radius.
    pub wheel_radius: [f32; 2],
    /// Default dirt level 0.0-1.0.
    pub dirt_level: f32,
    /// Swankness (spawn class).
    pub swankness: i32,
    /// LOD multiplier.
    pub lod_mult: f32,
    /// Wheel id / Flags field (meaning partly unknown; small int).
    pub wheel_id: i32,
    /// `+`-separated extras flags, or `-` for none.
    pub extras: Vec<String>,
}

impl CarRow {
    /// Parse one raw row; needs exactly 16 fields.
    ///
    /// Missing-comma typos (a field joining two values with whitespace,
    /// as in six shipped `vehicles.ide` rows) are healed first; see
    /// [`PedRow::parse_lenient`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(file: &str, row: &RawRow) -> Result<Self> {
        let (f, _) = Self::parse_healed(file, row)?;
        Ok(f)
    }

    /// Parse one raw row, reporting whether typo healing applied.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse_healed(file: &str, row: &RawRow) -> Result<(Self, bool)> {
        let (f, healed) = heal_comma_row(&row.fields);
        if f.len() != 16 {
            return Err(Error::new(
                file,
                row.line,
                ErrorKind::FieldCount {
                    expected: Some(16),
                    found: row.fields.len(),
                },
            ));
        }
        let g =
            |i: usize| field(file, row.line, &f, i, Some(16)).map(std::string::ToString::to_string);
        let anims2 = g(6)?;
        Ok((
            Self {
                model: g(0)?,
                txd: g(1)?,
                vehicle_type: g(2)?,
                handling_id: g(3)?,
                game_name: g(4)?,
                anims: g(5)?,
                anims2: is_some_token(&anims2).then_some(anims2),
                frequency: parse_i32(file, row.line, &f, 7)?,
                max_num: parse_i32(file, row.line, &f, 8)?,
                wheel_radius: [
                    parse_f32(file, row.line, &f, 9)?,
                    parse_f32(file, row.line, &f, 10)?,
                ],
                dirt_level: parse_f32(file, row.line, &f, 11)?,
                swankness: parse_i32(file, row.line, &f, 12)?,
                lod_mult: parse_f32(file, row.line, &f, 13)?,
                wheel_id: parse_i32(file, row.line, &f, 14)?,
                extras: {
                    let e = g(15)?;
                    if e == "-" {
                        Vec::new()
                    } else {
                        e.split('+').map(std::string::ToString::to_string).collect()
                    }
                },
            },
            healed,
        ))
    }
}

/// One `peds` row (16 fields). See module docs for column meanings.
#[derive(Debug, Clone)]
pub struct PedRow {
    /// Model name.
    pub model: String,
    /// Props model, or `NULL`.
    pub props: Option<String>,
    /// Default ped type (`CIVMALE`, `CIVFEMALE`, `COP`, gangs, ...).
    pub ped_type: String,
    /// Movement anim group.
    pub anim_group: String,
    /// Gesture group.
    pub gesture_group: String,
    /// Gesture phone group.
    pub gesture_phone_group: String,
    /// Facial group.
    pub facial_group: String,
    /// Viseme group.
    pub viseme_group: String,
    /// Flags bitmask.
    pub flags: i32,
    /// Anim file.
    pub anim_file: String,
    /// Blendshape file, or `null`.
    pub blendshape: Option<String>,
    /// Two radio station ids.
    pub radio: [i32; 2],
    /// Audio type (`PED_TYPE_PLAYER` in all shipped rows).
    pub audio_type: String,
    /// First/last voice.
    pub voice: [String; 2],
}

impl PedRow {
    /// Parse one raw row leniently.
    ///
    /// The shipped `peds.ide` has a handful of typo rows: some join two
    /// fields with a space instead of a comma (14-15 fields), one has a
    /// doubled comma (17 fields). This parser heals exactly those cases:
    /// empty fields are dropped, and a field containing an interior space
    /// is split in two. Anything else with a wrong field count is an error.
    /// [`Self::healed`] reports whether healing changed the row.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse_lenient(file: &str, row: &RawRow) -> Result<(Self, bool)> {
        let (f, healed) = heal_comma_row(&row.fields);
        if f.len() != 16 {
            return Err(Error::new(
                file,
                row.line,
                ErrorKind::FieldCount {
                    expected: Some(16),
                    found: row.fields.len(),
                },
            ));
        }
        let n = row.line;
        let g = |i: usize| field(file, n, &f, i, Some(16)).map(std::string::ToString::to_string);
        let props = g(1)?;
        let blend = g(10)?;
        Ok((
            Self {
                model: g(0)?,
                props: is_some_token(&props).then_some(props),
                ped_type: g(2)?,
                anim_group: g(3)?,
                gesture_group: g(4)?,
                gesture_phone_group: g(5)?,
                facial_group: g(6)?,
                viseme_group: g(7)?,
                flags: parse_i32(file, n, &f, 8)?,
                anim_file: g(9)?,
                blendshape: is_some_token(&blend).then_some(blend),
                radio: [parse_i32(file, n, &f, 11)?, parse_i32(file, n, &f, 12)?],
                audio_type: g(13)?,
                voice: [g(14)?, g(15)?],
            },
            healed,
        ))
    }
}

/// One `objs` row (4 fields): model, txd, draw distance, flags.
#[derive(Debug, Clone)]
pub struct ObjRow {
    /// Model name.
    pub model: String,
    /// Texture dictionary.
    pub txd: String,
    /// Draw distance.
    pub draw_distance: f32,
    /// Flags.
    pub flags: i32,
}

impl ObjRow {
    /// Parse one raw row; needs exactly 4 fields.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(file: &str, row: &RawRow) -> Result<Self> {
        let f = &row.fields;
        if f.len() != 4 {
            return Err(Error::new(
                file,
                row.line,
                ErrorKind::FieldCount {
                    expected: Some(4),
                    found: f.len(),
                },
            ));
        }
        Ok(Self {
            model: field(file, row.line, f, 0, Some(4))?.to_string(),
            txd: field(file, row.line, f, 1, Some(4))?.to_string(),
            draw_distance: parse_f32(file, row.line, f, 2)?,
            flags: parse_i32(file, row.line, f, 3)?,
        })
    }
}

/// One `txdp` row (2 fields): model plus parent texture dictionary.
#[derive(Debug, Clone)]
pub struct TxdpRow {
    /// Model name.
    pub model: String,
    /// Parent texture dictionary.
    pub parent_txd: String,
}

impl TxdpRow {
    /// Parse one raw row; needs exactly 2 fields.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(file: &str, row: &RawRow) -> Result<Self> {
        let f = &row.fields;
        if f.len() != 2 {
            return Err(Error::new(
                file,
                row.line,
                ErrorKind::FieldCount {
                    expected: Some(2),
                    found: f.len(),
                },
            ));
        }
        Ok(Self {
            model: field(file, row.line, f, 0, Some(2))?.to_string(),
            parent_txd: field(file, row.line, f, 1, Some(2))?.to_string(),
        })
    }
}

/// One `agrps` row (3 fields): ped model, variation model, index.
#[derive(Debug, Clone)]
pub struct AgrpRow {
    /// Base ped model.
    pub ped: String,
    /// Variation model.
    pub variation: String,
    /// Variation index.
    pub index: i32,
}

impl AgrpRow {
    /// Parse one raw row; needs exactly 3 fields.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(file: &str, row: &RawRow) -> Result<Self> {
        let f = &row.fields;
        if f.len() != 3 {
            return Err(Error::new(
                file,
                row.line,
                ErrorKind::FieldCount {
                    expected: Some(3),
                    found: f.len(),
                },
            ));
        }
        Ok(Self {
            ped: field(file, row.line, f, 0, Some(3))?.to_string(),
            variation: field(file, row.line, f, 1, Some(3))?.to_string(),
            index: parse_i32(file, row.line, f, 2)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# comment\r\ncars\r\nadmiral, admiral, car, ADMIRAL, ADMIRAL, VEH@STD, NULL, 100, 999, 0.2229, 0.2229, 0, 2, 1.0, 0, -\r\nend\r\ntxdp\r\nmule, vehshare_truck\r\nend\r\n";

    #[test]
    fn parses_sections_and_car_row() {
        let ide = parse_ide("vehicles.ide", SAMPLE.as_bytes()).unwrap();
        assert_eq!(ide.sections.len(), 2);
        let cars = ide.section("cars").unwrap();
        assert_eq!(cars.rows.len(), 1);
        let car = CarRow::parse("vehicles.ide", &cars.rows[0]).unwrap();
        assert_eq!(car.model, "admiral");
        assert_eq!(car.anims2, None);
        assert!(car.extras.is_empty());
        let tx = ide.section("txdp").unwrap();
        let t = TxdpRow::parse("vehicles.ide", &tx.rows[0]).unwrap();
        assert_eq!(t.parent_txd, "vehshare_truck");
    }

    #[test]
    fn eof_closes_open_section() {
        // Episode files omit the final `end`; tolerate it like the game.
        let src = "cars\r\nadmiral, admiral\r\n";
        let ide = parse_ide("x.ide", src.as_bytes()).unwrap();
        assert_eq!(ide.sections.len(), 1);
        assert_eq!(ide.sections[0].rows.len(), 1);
    }

    #[test]
    fn ped_heals_missing_comma() {
        let row = RawRow {
            line: 1,
            fields: vec![
                "m".into(),
                "p".into(),
                "CIVMALE".into(),
                "move_m@generic GESTURES@MALE".into(),
                "GP".into(),
                "F".into(),
                "V".into(),
                "0".into(),
                "move_m@generic".into(),
                "n".into(),
                "-1".into(),
                "-1".into(),
                "PED_TYPE_PLAYER".into(),
                "A".into(),
                "B".into(),
            ],
        };
        // 15 fields with one joined pair -> heals to 16.
        let (ped, healed) = PedRow::parse_lenient("peds.ide", &row).unwrap();
        assert!(healed);
        assert_eq!(ped.anim_group, "move_m@generic");
    }
}
