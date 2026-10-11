//! Item definition (`.ide`) files.
//!
//! Each file contributes rows to global sections. [`IdeFile::parse`] keeps the
//! typed rows per section plus [`IdeFile::errors`] for rows that did not match
//! their section's shape, and [`IdeFile::unknown_sections`] for sections this
//! crate does not type (none are known in shipped files).

use crate::Error;
use crate::text::{TextFile, TextRecord, flatten_fields};

/// A parsed `.ide` file: typed rows per section plus failures.
#[derive(Debug, Clone, Default)]
pub struct IdeFile {
    /// Static object rows (`objs`), in file order.
    pub objs: Vec<ObjRecord>,
    /// Timed object rows (`tobj`).
    pub tobj: Vec<TimedObj>,
    /// Animated object rows (`anim`).
    pub anim: Vec<AnimObj>,
    /// Timed animated object rows (`tanm`).
    pub tanm: Vec<TimedAnimObj>,
    /// Vehicle rows (`cars`).
    pub cars: Vec<Cars>,
    /// Pedestrian rows (`peds`).
    pub peds: Vec<Peds>,
    /// Weapon rows (`weap`).
    pub weap: Vec<Weap>,
    /// Texture-dictionary parent rows (`txdp`).
    pub txdp: Vec<Txdp>,
    /// Asset material rows (`amat`).
    pub amat: Vec<Amat>,
    /// Asset group rows (`agrps`).
    pub agrps: Vec<Agrps>,
    /// Hierarchy rows (`hier`).
    pub hier: Vec<Hier>,
    /// Interior (`mlo`) lines in file order; order matters within the section.
    pub mlo: Vec<MloLine>,
    /// Attached effect rows (`2dfx`).
    pub fx: Vec<Effect>,
    /// Whether the (always empty in shipped files) `tree` section was present.
    pub has_tree: bool,
    /// Whether the (always empty in shipped files) `path` section was present.
    pub has_path: bool,
    /// Rows that failed to parse, with their errors; the rest of the file
    /// still parses.
    pub errors: Vec<RecordError>,
    /// Sections this crate does not type, with their raw record counts.
    pub unknown_sections: Vec<UnknownSection>,
}

/// A record that failed to parse.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordError {
    /// 1-based line number.
    pub line: usize,
    /// Section name.
    pub section: String,
    /// The failure.
    pub error: Error,
}

/// An untyped section preserved by name.
#[derive(Debug, Clone, PartialEq)]
pub struct UnknownSection {
    /// Section name as written.
    pub name: String,
    /// Number of raw records it held.
    pub records: usize,
}

/// Axis-aligned bounding box plus bounding sphere, shared by the object rows.
#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    /// Box minimum corner (x, y, z).
    pub box_min: [f32; 3],
    /// Box maximum corner (x, y, z).
    pub box_max: [f32; 3],
    /// Sphere centre (x, y, z).
    pub sphere_center: [f32; 3],
    /// Sphere radius.
    pub sphere_radius: f32,
}

/// A static object row: full 16-field shape or a legacy 4-field short row.
///
/// The three shipped short rows (test objects) carry only model, dictionary,
/// draw distance and flags.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjRecord {
    /// Full row with bounds and LOD model.
    Full(Objs),
    /// Legacy short row without bounds.
    Short(ObjsShort),
}

/// Full static object row (`objs`, 16 fields).
#[derive(Debug, Clone, PartialEq)]
pub struct Objs {
    /// Model name.
    pub model: String,
    /// Texture dictionary name.
    pub texture_dict: String,
    /// Draw distance.
    pub draw_distance: f32,
    /// Object flags bitmask.
    pub flags: u32,
    /// Second flags word; meaning uncertain.
    pub unknown_flags: u32,
    /// Bounding volumes.
    pub bounds: Bounds,
    /// LOD model name, or `null` when absent.
    pub lod_model: String,
}

/// Legacy short object row (`objs`, 4 fields).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjsShort {
    /// Model name.
    pub model: String,
    /// Texture dictionary name.
    pub texture_dict: String,
    /// Draw distance.
    pub draw_distance: f32,
    /// Object flags bitmask.
    pub flags: u32,
}

/// Timed object row (`tobj`, 17 fields): an [`Objs`] plus a time mask.
#[derive(Debug, Clone, PartialEq)]
pub struct TimedObj {
    /// The object fields.
    pub obj: Objs,
    /// Time-of-day visibility mask; exact encoding uncertain.
    pub time_flags: u32,
}

/// Animated object row (`anim`, 17 fields).
#[derive(Debug, Clone, PartialEq)]
pub struct AnimObj {
    /// Model name.
    pub model: String,
    /// Texture dictionary name.
    pub texture_dict: String,
    /// Third name field (usually the area name); purpose uncertain.
    pub dict: String,
    /// Draw distance.
    pub draw_distance: f32,
    /// Object flags bitmask.
    pub flags: u32,
    /// Second flags word; meaning uncertain.
    pub unknown_flags: u32,
    /// Bounding volumes.
    pub bounds: Bounds,
    /// LOD model name, or `null` when absent.
    pub lod_model: String,
}

/// Timed animated object row (`tanm`, 18 fields).
#[derive(Debug, Clone, PartialEq)]
pub struct TimedAnimObj {
    /// The animated object fields.
    pub obj: AnimObj,
    /// Time-of-day visibility mask; exact encoding uncertain.
    pub time_flags: u32,
}

/// Vehicle row (`cars`, 15-16 tokens).
///
/// Tokens after the last comma-separated field may be split across whitespace
/// (see crate docs); the parser accepts 15 tokens (no flags word) or 16.
#[derive(Debug, Clone, PartialEq)]
pub struct Cars {
    /// Model name.
    pub model: String,
    /// Texture dictionary name.
    pub texture_dict: String,
    /// Vehicle kind (`car`, `bike`, `heli`, `boat`).
    pub kind: String,
    /// Handling identifier, links to `handling.dat`.
    pub handling: String,
    /// In-game display name identifier.
    pub game_name: String,
    /// Primary animation set.
    pub anims: String,
    /// Secondary animation set, or `NULL`.
    pub anims_extra: String,
    /// Spawn frequency.
    pub frequency: i32,
    /// Integer after frequency (9 in most rows, 999 for rare spawns);
    /// meaning uncertain.
    pub unknown_count: i32,
    /// Front wheel radius, inferred from values.
    pub wheel_front: f32,
    /// Rear wheel radius, inferred from values.
    pub wheel_rear: f32,
    /// Small float (often 0, 0.3, 0.5, 1.0); meaning uncertain.
    pub unknown_float: f32,
    /// Small integer; meaning uncertain.
    pub unknown_int_a: i32,
    /// Float near 1.0-2.0; meaning uncertain.
    pub unknown_float_b: f32,
    /// Small integer; meaning uncertain.
    pub unknown_int_b: i32,
    /// Flags word (`noboot+ext_strong+...`) or `-`; absent on truncated rows.
    pub flags: Option<String>,
}

/// Pedestrian row (`peds`, 15-16 tokens).
///
/// The voice field is missing on rows truncated at the game's 256-character
/// line limit, so it is optional.
#[derive(Debug, Clone, PartialEq)]
pub struct Peds {
    /// Model name.
    pub model: String,
    /// Ped model name, or `null`.
    pub ped_model: String,
    /// Ped type string (`CIVMALE`, gang names, ...).
    pub ped_type: String,
    /// Movement animation set.
    pub movement: String,
    /// Gesture set.
    pub gestures: String,
    /// Phone gesture set.
    pub gestures_phone: String,
    /// Facial animation set.
    pub facials: String,
    /// Viseme set.
    pub visemes: String,
    /// Integer, always 0 in shipped files; meaning uncertain.
    pub unknown_zero: i32,
    /// Second movement set (usually repeats [`Peds::movement`]).
    pub movement_b: String,
    /// Usually `null`; meaning uncertain.
    pub unknown_null: String,
    /// First integer of a pair; meaning uncertain.
    pub unknown_a: i32,
    /// Second integer of a pair; meaning uncertain.
    pub unknown_b: i32,
    /// Pedestrian class (`PED_TYPE_PLAYER` in all shipped rows).
    pub ped_class: String,
    /// Radio/voice group prefix; meaning uncertain.
    pub voice_group: String,
    /// Voice identifier; absent on truncated rows.
    pub voice: Option<String>,
}

/// Weapon row (`weap`, 6 tokens).
#[derive(Debug, Clone, PartialEq)]
pub struct Weap {
    /// Model name.
    pub model: String,
    /// Texture dictionary name.
    pub texture_dict: String,
    /// Animation set.
    pub anim: String,
    /// First integer; meaning uncertain.
    pub unknown_a: i32,
    /// Second integer (30/50 look like ammo counts); meaning uncertain.
    pub unknown_b: i32,
    /// Third integer, always 0; meaning uncertain.
    pub unknown_c: i32,
}

/// Texture-dictionary parent row (`txdp`, 2 fields).
#[derive(Debug, Clone, PartialEq)]
pub struct Txdp {
    /// Child dictionary.
    pub child: String,
    /// Parent dictionary.
    pub parent: String,
}

/// Asset material row (`amat`, 3 fields).
#[derive(Debug, Clone, PartialEq)]
pub struct Amat {
    /// Model name.
    pub model: String,
    /// Integer, always 0; meaning uncertain.
    pub unknown: i32,
    /// Material/constant name.
    pub material: String,
}

/// Asset group row (`agrps`, 3 fields).
#[derive(Debug, Clone, PartialEq)]
pub struct Agrps {
    /// Model name.
    pub model: String,
    /// Variant name.
    pub variant: String,
    /// Variant index.
    pub index: i32,
}

/// Hierarchy row (`hier`, 4-5 tokens); field meanings are uncertain.
#[derive(Debug, Clone, PartialEq)]
pub struct Hier {
    /// First token (a name, sometimes a number).
    pub key: String,
    /// Second token.
    pub a: String,
    /// Third token.
    pub b: String,
    /// Fourth token when five are present.
    pub c: Option<String>,
    /// Trailing float radius.
    pub radius: f32,
}

/// One line of an `mlo` interior section, in file order.
#[derive(Debug, Clone, PartialEq)]
pub enum MloLine {
    /// 8-field interior header: name plus seven integers.
    Header {
        /// Interior name.
        name: String,
        /// Header integers; meanings uncertain.
        values: [i32; 7],
    },
    /// 11-field placed object: name, position, quaternion, two integers.
    Placed {
        /// Model name.
        name: String,
        /// Position (x, y, z).
        pos: [f32; 3],
        /// Rotation quaternion (x, y, z, w).
        rot: [f32; 4],
        /// First trailing integer; meaning uncertain.
        a: i32,
        /// Second trailing integer; meaning uncertain.
        b: i32,
    },
    /// Single-word marker (`mloroomstart`, `roomend`, `mloportalstart`,
    /// `mloend`).
    Marker(String),
    /// Any other line: leading name plus raw values (portal, limbo, room and
    /// numeric-list lines); layouts undocumented.
    Other {
        /// First field.
        name: String,
        /// Remaining raw fields.
        values: Vec<String>,
    },
}

/// Attached effect row (`2dfx`): model, position, kind, kind payload.
///
/// Twelve kinds appear in shipped files (0, 1, 2, 12, 14, 15, 17, 18, 19,
/// 21, 22, 23) with payloads from 5 to 103 fields; only the kind values are
/// established, not their meanings.
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

fn num<T>(rec: &TextRecord, section: &str, field: usize, text: &str) -> Result<T, Error>
where
    T: std::str::FromStr,
{
    text.parse::<T>().map_err(|_| Error::BadNumber {
        line: rec.line,
        section: section.to_string(),
        field,
        text: text.to_string(),
    })
}

fn count_err(rec: &TextRecord, section: &str, found: usize, expected: &str) -> Error {
    Error::FieldCount {
        line: rec.line,
        section: section.to_string(),
        found,
        expected: expected.to_string(),
    }
}

fn parse_bounds(f: &[String], rec: &TextRecord, section: &str, o: usize) -> Result<Bounds, Error> {
    let g = |i: usize| num::<f32>(rec, section, o + i, &f[o + i]);
    Ok(Bounds {
        box_min: [g(0)?, g(1)?, g(2)?],
        box_max: [g(3)?, g(4)?, g(5)?],
        sphere_center: [g(6)?, g(7)?, g(8)?],
        sphere_radius: g(9)?,
    })
}

fn parse_objs_body(f: &[String], rec: &TextRecord, section: &str) -> Result<(Objs, usize), Error> {
    if f.len() == 4 {
        return Err(count_err(rec, section, 4, "16"));
    }
    if f.len() != 16 {
        return Err(count_err(rec, section, f.len(), "16"));
    }
    Ok((
        Objs {
            model: f[0].clone(),
            texture_dict: f[1].clone(),
            draw_distance: num(rec, section, 2, &f[2])?,
            flags: num(rec, section, 3, &f[3])?,
            unknown_flags: num(rec, section, 4, &f[4])?,
            bounds: parse_bounds(f, rec, section, 5)?,
            lod_model: f[15].clone(),
        },
        16,
    ))
}

impl IdeFile {
    /// Parse an `.ide` file from a byte slice.
    ///
    /// Never fails on record content: rows that miss their shape land in
    /// [`IdeFile::errors`] and unknown sections in
    /// [`IdeFile::unknown_sections`]. Only invalid UTF-8 fails the whole
    /// parse.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    // Row dispatch over many section shapes; splitting would scatter the table.
    #[allow(clippy::too_many_lines)]
    pub fn parse(bytes: &[u8]) -> Result<IdeFile, Error> {
        let text = TextFile::parse(bytes)?;
        let mut out = IdeFile::default();
        for sec in &text.sections {
            let name = sec.name.to_ascii_lowercase();
            match name.as_str() {
                "objs" => {
                    for r in &sec.records {
                        match parse_objs_record(r) {
                            Ok(o) => out.objs.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "tobj" => {
                    for r in &sec.records {
                        match parse_tobj_record(r) {
                            Ok(o) => out.tobj.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "anim" => {
                    for r in &sec.records {
                        match parse_anim_record(r) {
                            Ok(o) => out.anim.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "tanm" => {
                    for r in &sec.records {
                        match parse_tanm_record(r) {
                            Ok(o) => out.tanm.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "cars" => {
                    for r in &sec.records {
                        match parse_cars_record(r) {
                            Ok(o) => out.cars.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "peds" => {
                    for r in &sec.records {
                        match parse_peds_record(r) {
                            Ok(o) => out.peds.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "weap" => {
                    for r in &sec.records {
                        match parse_weap_record(r) {
                            Ok(o) => out.weap.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "txdp" => {
                    for r in &sec.records {
                        match parse_txdp_record(r) {
                            Ok(o) => out.txdp.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "amat" => {
                    for r in &sec.records {
                        match parse_amat_record(r) {
                            Ok(o) => out.amat.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "agrps" => {
                    for r in &sec.records {
                        match parse_agrps_record(r) {
                            Ok(o) => out.agrps.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "hier" => {
                    for r in &sec.records {
                        match parse_hier_record(r) {
                            Ok(o) => out.hier.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "mlo" => {
                    for r in &sec.records {
                        match parse_mlo_record(r) {
                            Ok(o) => out.mlo.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "2dfx" => {
                    for r in &sec.records {
                        match parse_effect_record(r) {
                            Ok(o) => out.fx.push(o),
                            Err(e) => out.errors.push(err_of(sec, r, e)),
                        }
                    }
                }
                "tree" => {
                    out.has_tree = true;
                    for r in &sec.records {
                        out.errors.push(err_of(
                            sec,
                            r,
                            count_err(r, &sec.name, r.fields.len(), "0 (empty section)"),
                        ));
                    }
                }
                "path" => {
                    out.has_path = true;
                    for r in &sec.records {
                        out.errors.push(err_of(
                            sec,
                            r,
                            count_err(r, &sec.name, r.fields.len(), "0 (empty section)"),
                        ));
                    }
                }
                _ => out.unknown_sections.push(UnknownSection {
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
        self.objs.len()
            + self.tobj.len()
            + self.anim.len()
            + self.tanm.len()
            + self.cars.len()
            + self.peds.len()
            + self.weap.len()
            + self.txdp.len()
            + self.amat.len()
            + self.agrps.len()
            + self.hier.len()
            + self.mlo.len()
            + self.fx.len()
    }

    /// True when no typed records were parsed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn err_of(sec: &crate::text::TextSection, r: &TextRecord, error: Error) -> RecordError {
    RecordError {
        line: r.line,
        section: sec.name.clone(),
        error,
    }
}

fn parse_objs_record(r: &TextRecord) -> Result<ObjRecord, Error> {
    let f = &r.fields;
    if f.len() == 4 {
        return Ok(ObjRecord::Short(ObjsShort {
            model: f[0].clone(),
            texture_dict: f[1].clone(),
            draw_distance: num(r, "objs", 2, &f[2])?,
            flags: num(r, "objs", 3, &f[3])?,
        }));
    }
    let (obj, _) = parse_objs_body(f, r, "objs")?;
    Ok(ObjRecord::Full(obj))
}

fn parse_tobj_record(r: &TextRecord) -> Result<TimedObj, Error> {
    let f = &r.fields;
    if f.len() != 17 {
        return Err(count_err(r, "tobj", f.len(), "17"));
    }
    let (obj, _) = parse_objs_body(&f[..16], r, "tobj")?;
    Ok(TimedObj {
        obj,
        time_flags: num(r, "tobj", 16, &f[16])?,
    })
}

fn parse_anim_record(r: &TextRecord) -> Result<AnimObj, Error> {
    let f = &r.fields;
    if f.len() != 17 {
        return Err(count_err(r, "anim", f.len(), "17"));
    }
    Ok(AnimObj {
        model: f[0].clone(),
        texture_dict: f[1].clone(),
        dict: f[2].clone(),
        draw_distance: num(r, "anim", 3, &f[3])?,
        flags: num(r, "anim", 4, &f[4])?,
        unknown_flags: num(r, "anim", 5, &f[5])?,
        bounds: parse_bounds(f, r, "anim", 6)?,
        lod_model: f[16].clone(),
    })
}

fn parse_tanm_record(r: &TextRecord) -> Result<TimedAnimObj, Error> {
    let f = &r.fields;
    if f.len() != 18 {
        return Err(count_err(r, "tanm", f.len(), "18"));
    }
    let obj = parse_anim_record(&TextRecord {
        line: r.line,
        fields: f[..17].to_vec(),
    })?;
    Ok(TimedAnimObj {
        obj,
        time_flags: num(r, "tanm", 17, &f[17])?,
    })
}

fn parse_cars_record(r: &TextRecord) -> Result<Cars, Error> {
    let t = flatten_fields(&r.fields);
    if t.len() != 15 && t.len() != 16 {
        return Err(count_err(r, "cars", t.len(), "15 or 16"));
    }
    Ok(Cars {
        model: t[0].clone(),
        texture_dict: t[1].clone(),
        kind: t[2].clone(),
        handling: t[3].clone(),
        game_name: t[4].clone(),
        anims: t[5].clone(),
        anims_extra: t[6].clone(),
        frequency: num(r, "cars", 7, &t[7])?,
        unknown_count: num(r, "cars", 8, &t[8])?,
        wheel_front: num(r, "cars", 9, &t[9])?,
        wheel_rear: num(r, "cars", 10, &t[10])?,
        unknown_float: num(r, "cars", 11, &t[11])?,
        unknown_int_a: num(r, "cars", 12, &t[12])?,
        unknown_float_b: num(r, "cars", 13, &t[13])?,
        unknown_int_b: num(r, "cars", 14, &t[14])?,
        flags: t.get(15).cloned(),
    })
}

fn parse_peds_record(r: &TextRecord) -> Result<Peds, Error> {
    let t = flatten_fields(&r.fields);
    if t.len() != 15 && t.len() != 16 {
        return Err(count_err(r, "peds", t.len(), "15 or 16"));
    }
    Ok(Peds {
        model: t[0].clone(),
        ped_model: t[1].clone(),
        ped_type: t[2].clone(),
        movement: t[3].clone(),
        gestures: t[4].clone(),
        gestures_phone: t[5].clone(),
        facials: t[6].clone(),
        visemes: t[7].clone(),
        unknown_zero: num(r, "peds", 8, &t[8])?,
        movement_b: t[9].clone(),
        unknown_null: t[10].clone(),
        unknown_a: num(r, "peds", 11, &t[11])?,
        unknown_b: num(r, "peds", 12, &t[12])?,
        ped_class: t[13].clone(),
        voice_group: t[14].clone(),
        voice: t.get(15).cloned(),
    })
}

fn parse_weap_record(r: &TextRecord) -> Result<Weap, Error> {
    let t = flatten_fields(&r.fields);
    if t.len() != 6 {
        return Err(count_err(r, "weap", t.len(), "6"));
    }
    Ok(Weap {
        model: t[0].clone(),
        texture_dict: t[1].clone(),
        anim: t[2].clone(),
        unknown_a: num(r, "weap", 3, &t[3])?,
        unknown_b: num(r, "weap", 4, &t[4])?,
        unknown_c: num(r, "weap", 5, &t[5])?,
    })
}

fn parse_txdp_record(r: &TextRecord) -> Result<Txdp, Error> {
    let f = &r.fields;
    if f.len() != 2 {
        return Err(count_err(r, "txdp", f.len(), "2"));
    }
    Ok(Txdp {
        child: f[0].clone(),
        parent: f[1].clone(),
    })
}

fn parse_amat_record(r: &TextRecord) -> Result<Amat, Error> {
    // One shipped row misses the comma after the model name; split tokens.
    let t = flatten_fields(&r.fields);
    if t.len() != 3 {
        return Err(count_err(r, "amat", t.len(), "3"));
    }
    Ok(Amat {
        model: t[0].clone(),
        unknown: num(r, "amat", 1, &t[1])?,
        material: t[2].clone(),
    })
}

fn parse_agrps_record(r: &TextRecord) -> Result<Agrps, Error> {
    let f = &r.fields;
    if f.len() != 3 {
        return Err(count_err(r, "agrps", f.len(), "3"));
    }
    Ok(Agrps {
        model: f[0].clone(),
        variant: f[1].clone(),
        index: num(r, "agrps", 2, &f[2])?,
    })
}

fn parse_hier_record(r: &TextRecord) -> Result<Hier, Error> {
    let t = flatten_fields(&r.fields);
    if t.len() != 4 && t.len() != 5 {
        return Err(count_err(r, "hier", t.len(), "4 or 5"));
    }
    let (u1, u2, u3, radius) = if t.len() == 4 {
        (
            t[1].clone(),
            t[2].clone(),
            None,
            num::<f32>(r, "hier", 3, &t[3])?,
        )
    } else {
        (
            t[1].clone(),
            t[2].clone(),
            Some(t[3].clone()),
            num::<f32>(r, "hier", 4, &t[4])?,
        )
    };
    Ok(Hier {
        key: t[0].clone(),
        a: u1,
        b: u2,
        c: u3,
        radius,
    })
}

fn parse_mlo_record(r: &TextRecord) -> Result<MloLine, Error> {
    let f: Vec<String> = r.fields.iter().filter(|s| !s.is_empty()).cloned().collect();
    if f.len() == 1 {
        return Ok(MloLine::Marker(f[0].clone()));
    }
    if f.len() == 8 {
        let mut values = [0i32; 7];
        for (i, v) in values.iter_mut().enumerate() {
            *v = num(r, "mlo", i + 1, &f[i + 1])?;
        }
        return Ok(MloLine::Header {
            name: f[0].clone(),
            values,
        });
    }
    // Placed lines end with a comma, so comma-splitting yields 11 fields with
    // an empty tail; 10 carry values.
    if f.len() == 10 {
        let g = |i: usize| num::<f32>(r, "mlo", i, &f[i]);
        return Ok(MloLine::Placed {
            name: f[0].clone(),
            pos: [g(1)?, g(2)?, g(3)?],
            rot: [g(4)?, g(5)?, g(6)?, g(7)?],
            a: num(r, "mlo", 8, &f[8])?,
            b: num(r, "mlo", 9, &f[9])?,
        });
    }
    if f.is_empty() {
        return Err(count_err(r, "mlo", 0, "at least 1"));
    }
    Ok(MloLine::Other {
        name: f[0].clone(),
        values: f[1..].to_vec(),
    })
}

fn parse_effect_record(r: &TextRecord) -> Result<Effect, Error> {
    let f: Vec<String> = r.fields.iter().filter(|s| !s.is_empty()).cloned().collect();
    if f.len() < 6 {
        return Err(count_err(r, "2dfx", f.len(), "at least 6"));
    }
    Ok(Effect {
        model: f[0].clone(),
        pos: [
            num(r, "2dfx", 1, &f[1])?,
            num(r, "2dfx", 2, &f[2])?,
            num(r, "2dfx", 3, &f[3])?,
        ],
        kind: num(r, "2dfx", 4, &f[4])?,
        values: f[5..].to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = b"\
# test ide
objs
myprop, mytxd, 100, 0, 0, -1, -1, -1, 1, 1, 1, 0, 0, 0, 2, null
tiny, gen, 50, 0
end
tobj
tprop, ttxd, 30, 8, 0, -1, -1, -1, 1, 1, 1, 0, 0, 0, 2, null, 7
end
cars
mycar, mycar, car, MYCAR, MYCAR, VEH@STD, NULL, 100, 999, 0.3, 0.3, 0, 5, 1.0, 0 noboot
end
peds
myped, myped_p, CIVMALE, move_m@generic, GESTURES@MALE, GESTURES@M_PHONE, FACIALS@M_LO, visemes@m_lo, 0, move_m@generic, null, -1, -1, PED_TYPE_PLAYER, GROUPY, VOICE_X
end
weap
mygun, mygun, gun@rifle, 1, 30, 0
end
txdp
child, parent
end
amat
prop, 0, MAT_FOO
end
agrps
prop, prop_var, 1
end
hier
key, a, b, 9.5
end
mlo
MyRoom, 1, 2, 3, 4, 5, 6, 7
mloroomstart
chair, 1, 2, 3, 0, 0, 0, 1, 0, 384,
roomend
mloend
end
2dfx
lamp, 1, 2, 3, 0, 9, 8, 7
end
tree
end
path
end
";

    #[test]
    fn parses_sample() {
        let f = IdeFile::parse(SAMPLE).unwrap();
        assert_eq!(f.objs.len(), 2);
        assert!(matches!(f.objs[0], ObjRecord::Full(_)));
        assert!(matches!(f.objs[1], ObjRecord::Short(_)));
        assert_eq!(f.tobj.len(), 1);
        assert_eq!(f.tobj[0].time_flags, 7);
        assert_eq!(f.cars.len(), 1);
        assert_eq!(f.cars[0].flags.as_deref(), Some("noboot"));
        assert_eq!(f.peds.len(), 1);
        assert_eq!(f.peds[0].voice.as_deref(), Some("VOICE_X"));
        assert_eq!(f.weap.len(), 1);
        assert_eq!(f.txdp.len(), 1);
        assert_eq!(f.amat.len(), 1);
        assert_eq!(f.agrps.len(), 1);
        assert_eq!(f.hier.len(), 1);
        assert_eq!(f.mlo.len(), 5);
        assert!(matches!(f.mlo[0], MloLine::Header { .. }));
        assert!(matches!(f.mlo[1], MloLine::Marker(_)));
        assert!(matches!(f.mlo[2], MloLine::Placed { .. }));
        assert_eq!(f.fx.len(), 1);
        assert_eq!(f.fx[0].kind, 0);
        assert!(f.has_tree && f.has_path);
        assert!(f.errors.is_empty());
        assert!(f.unknown_sections.is_empty());
    }

    #[test]
    fn bad_row_collected_not_fatal() {
        let f = IdeFile::parse(b"objs\nnope, nope\nend\n").unwrap();
        assert!(f.objs.is_empty());
        assert_eq!(f.errors.len(), 1);
        assert_eq!(f.errors[0].line, 2);
    }

    #[test]
    fn unknown_section_preserved() {
        let f = IdeFile::parse(b"zzznew\na, b\nend\n").unwrap();
        assert_eq!(f.unknown_sections.len(), 1);
        assert_eq!(f.unknown_sections[0].records, 1);
    }

    #[test]
    fn cars_without_flags_ok() {
        let f = IdeFile::parse(
            b"cars\nm, m, car, H, G, V@S, NULL, 1, 1, 0.1, 0.1, 0, 1, 1.0, 0\nend\n",
        )
        .unwrap();
        assert_eq!(f.cars.len(), 1);
        assert_eq!(f.cars[0].flags, None);
    }

    #[test]
    fn peds_truncated_voice_ok() {
        let f = IdeFile::parse(
            b"peds\nm, p, CIVMALE, mv, g, gp, fa, vi, 0, mv, null, -1, -1, PED_TYPE_PLAYER, GRP\nend\n",
        )
        .unwrap();
        assert_eq!(f.peds.len(), 1);
        assert_eq!(f.peds[0].voice, None);
    }
}
