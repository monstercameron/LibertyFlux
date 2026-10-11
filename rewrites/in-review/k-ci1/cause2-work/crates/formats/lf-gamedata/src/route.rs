//! File router: pick a parser from a file name.
//!
//! [`parse_file`] maps a data file name (case-insensitive, `.dat`/`.csv`/
//! `.ide`/`.xml`/`.txt` under the game's data folders) to its typed parser
//! and returns a [`Parsed`] value. Files with no typed parser yet fall
//! back to a structural [`crate::scan::Scan`]; inputs containing NUL bytes
//! report [`Parsed::Binary`]. Later tools can drive the whole corpus
//! through this one function.

use crate::{
    Error, Result, bracket, carcols, effects, groups, handling, ide, load_list, materials, misc,
    object, ped_personality, ped_variations, popcycle, relations, scan, timecyc, water,
    weapon_info, xml,
};

/// The typed outcome of parsing one file.
#[derive(Debug)]
pub enum Parsed {
    /// `handling.dat`.
    Handling(handling::HandlingData),
    /// Any `*.ide` file: raw sections plus typed-row outcomes.
    Ide(IdeOutcome),
    /// `carcols.dat`.
    Carcols(carcols::Carcols),
    /// `cargrp.dat` / `pedgrp.dat`.
    Groups(Vec<groups::PopGroup>),
    /// `pedpersonality.dat`.
    PedPersonality(Vec<ped_personality::PedPersonality>),
    /// `pedVariations.dat`.
    PedVariations(Vec<ped_variations::VariationBlock>),
    /// `pedProps.dat`.
    PedProps(Vec<ped_variations::PropBlock>),
    /// `WeaponInfo.xml`.
    WeaponInfo(weapon_info::WeaponInfo),
    /// `ThrownWeaponInfo.xml`.
    Thrown(Vec<weapon_info::ThrownObject>),
    /// `timecyc.dat`.
    Timecyc(Vec<timecyc::Weather>),
    /// `timecyclemodifiers*.dat`.
    Modifiers(Vec<timecyc::Modifier>),
    /// `water.dat`.
    Water(Vec<water::WaterQuad>),
    /// Load lists (`gta.dat`, `default.dat`, `images.txt`, ...).
    LoadList(Vec<load_list::Directive>),
    /// `popcycle.dat`.
    Popcycle(Vec<popcycle::PopZone>),
    /// `object.dat`.
    Object(object::ObjectOutcome),
    /// `materials.dat`: version plus rows.
    Materials(f32, Vec<materials::MaterialRow>),
    /// `procedural.dat`: version plus rows.
    Procedural(f32, Vec<materials::ProceduralRow>),
    /// `mtl_convert.txt`.
    MtlConvert(Vec<materials::MtlConvertRow>),
    /// `[SECTION]` files: sections plus typed HUD rows where applicable.
    Bracket(Vec<bracket::BracketSection>),
    /// `visualSettings.dat`.
    Visual(Vec<bracket::VisualSetting>),
    /// `ped.dat` / `Relationships.dat`.
    Relations(Vec<relations::RelationGroup>),
    /// `effects/*.dat` (structural) plus typed explosion rows if present.
    Fx(FxOutcome),
    /// Generic CSV files.
    Csv(misc::CsvTable),
    /// `vehOff.csv`.
    Vehoff(Vec<misc::VehoffRow>),
    /// `shorelines.dat`.
    Shorelines(Vec<misc::Shoreline>),
    /// `numplate.dat`.
    Numplate(Vec<[u8; 3]>),
    /// `stockshake.txt`.
    Stockshake(Vec<[f32; 10]>),
    /// `trainCamNodes.txt`.
    TrainCams(Vec<misc::TrainCam>),
    /// `KEY=value` files (`nav.dat`, `stream.ini`).
    KeyValue(Vec<(String, String)>),
    /// Other XML files: root element (counts come from walking it).
    Xml(xml::Element),
    /// No typed parser yet: structural scan.
    Scanned(scan::Scan),
    /// Not text (NUL byte found): binary file, out of scope.
    Binary,
}

/// Typed-row outcomes for one IDE file.
#[derive(Debug, Default)]
pub struct IdeOutcome {
    /// Raw sections in file order.
    pub file: ide::IdeFile,
    /// `cars` rows typed.
    pub cars: Vec<ide::CarRow>,
    /// `peds` rows typed, with healed-typo flags.
    pub peds: Vec<(ide::PedRow, bool)>,
    /// `objs` rows typed.
    pub objs: Vec<ide::ObjRow>,
    /// `txdp` rows typed.
    pub txdp: Vec<ide::TxdpRow>,
    /// `agrps` rows typed.
    pub agrps: Vec<ide::AgrpRow>,
    /// Row-level failures: (section, line, error).
    pub row_errors: Vec<(String, usize, Error)>,
}

/// Effects outcome: structural table plus typed explosion rows.
#[derive(Debug)]
pub struct FxOutcome {
    /// Structural table.
    pub table: effects::FxTable,
    /// Typed explosion rows (empty for non-explosion files).
    pub explosions: Vec<effects::ExplosionRow>,
}

impl Parsed {
    /// Short parser name for reports.
    #[must_use]
    pub fn parser(&self) -> &'static str {
        match self {
            Parsed::Handling(_) => "handling",
            Parsed::Ide(_) => "ide",
            Parsed::Carcols(_) => "carcols",
            Parsed::Groups(_) => "groups",
            Parsed::PedPersonality(_) => "ped_personality",
            Parsed::PedVariations(_) => "ped_variations",
            Parsed::PedProps(_) => "ped_props",
            Parsed::WeaponInfo(_) => "weapon_info",
            Parsed::Thrown(_) => "thrown",
            Parsed::Timecyc(_) => "timecyc",
            Parsed::Modifiers(_) => "modifiers",
            Parsed::Water(_) => "water",
            Parsed::LoadList(_) => "load_list",
            Parsed::Popcycle(_) => "popcycle",
            Parsed::Object(_) => "object",
            Parsed::Materials(_, _) => "materials",
            Parsed::Procedural(_, _) => "procedural",
            Parsed::MtlConvert(_) => "mtl_convert",
            Parsed::Bracket(_) => "bracket",
            Parsed::Visual(_) => "visual",
            Parsed::Relations(_) => "relations",
            Parsed::Fx(_) => "fx",
            Parsed::Csv(_) => "csv",
            Parsed::Vehoff(_) => "vehoff",
            Parsed::Shorelines(_) => "shorelines",
            Parsed::Numplate(_) => "numplate",
            Parsed::Stockshake(_) => "stockshake",
            Parsed::TrainCams(_) => "train_cams",
            Parsed::KeyValue(_) => "key_value",
            Parsed::Xml(_) => "xml",
            Parsed::Scanned(_) => "scan",
            Parsed::Binary => "binary",
        }
    }

    /// Record counts per table for reports: (table, count).
    #[must_use]
    pub fn counts(&self) -> Vec<(String, usize)> {
        let n = |s: &str, c: usize| (s.to_string(), c);
        match self {
            Parsed::Handling(h) => vec![
                n("cars", h.cars.len()),
                n("boats", h.boats.len()),
                n("bikes", h.bikes.len()),
                n("flying", h.flying.len()),
                n("anim_groups", h.anim_groups.len()),
            ],
            Parsed::Ide(o) => {
                let mut v = vec![
                    n("cars", o.cars.len()),
                    n("peds", o.peds.len()),
                    n("objs", o.objs.len()),
                    n("txdp", o.txdp.len()),
                    n("agrps", o.agrps.len()),
                ];
                for s in &o.file.sections {
                    if !["cars", "peds", "objs", "txdp", "agrps"].contains(&s.name.as_str()) {
                        v.push((format!("raw:{}", s.name), s.rows.len()));
                    }
                }
                v.push(n("row_errors", o.row_errors.len()));
                v
            }
            Parsed::Carcols(c) => vec![
                n("palette", c.palette.len()),
                n("car3", c.car3.len()),
                n("car4", c.car4.len()),
            ],
            Parsed::Groups(g) => vec![
                n("groups", g.len()),
                n("models", g.iter().map(|x| x.models.len()).sum()),
            ],
            Parsed::PedPersonality(p) => vec![n("rows", p.len())],
            Parsed::PedVariations(b) => vec![
                n("blocks", b.len()),
                n("rows", b.iter().map(|x| x.rows.len()).sum()),
            ],
            Parsed::PedProps(b) => vec![
                n("blocks", b.len()),
                n("rows", b.iter().map(|x| x.rows.len()).sum()),
            ],
            Parsed::WeaponInfo(w) => vec![n("weapons", w.weapons.len())],
            Parsed::Thrown(t) => vec![n("objects", t.len())],
            Parsed::Timecyc(w) => vec![
                n("weathers", w.len()),
                n("slots", w.iter().map(|x| x.slots.len()).sum()),
            ],
            Parsed::Modifiers(m) => vec![n("modifiers", m.len())],
            Parsed::Water(w) => vec![n("quads", w.len())],
            Parsed::LoadList(d) => vec![n("directives", d.len())],
            Parsed::Popcycle(z) => vec![
                n("zones", z.len()),
                n(
                    "rows",
                    z.iter().map(|x| x.weekday.len() + x.weekend.len()).sum(),
                ),
            ],
            Parsed::Object(o) => vec![n("rows", o.rows.len()), n("row_errors", o.row_errors.len())],
            Parsed::Materials(_, m) => vec![n("rows", m.len())],
            Parsed::Procedural(_, p) => vec![n("rows", p.len())],
            Parsed::MtlConvert(p) => vec![n("rows", p.len())],
            Parsed::Bracket(s) => vec![
                n("sections", s.len()),
                n("rows", s.iter().map(|x| x.rows.len()).sum()),
            ],
            Parsed::Visual(v) => vec![n("settings", v.len())],
            Parsed::Relations(g) => vec![
                n("groups", g.len()),
                n("relations", g.iter().map(|x| x.relations.len()).sum()),
            ],
            Parsed::Fx(f) => vec![
                n("rows", f.table.rows.len()),
                n("explosions", f.explosions.len()),
            ],
            Parsed::Csv(t) => vec![n("rows", t.rows.len())],
            Parsed::Vehoff(v) => vec![n("rows", v.len())],
            Parsed::Shorelines(s) => vec![
                n("segments", s.len()),
                n("points", s.iter().map(|x| x.points.len()).sum()),
            ],
            Parsed::Numplate(c) => vec![n("colours", c.len())],
            Parsed::Stockshake(s) => vec![n("rows", s.len())],
            Parsed::TrainCams(t) => vec![n("nodes", t.len())],
            Parsed::KeyValue(k) => vec![n("pairs", k.len())],
            Parsed::Xml(root) => vec![n("elements", count_elements(root))],
            Parsed::Scanned(s) => vec![n("data_lines", s.data_lines)],
            Parsed::Binary => vec![],
        }
    }
}

fn count_elements(root: &xml::Element) -> usize {
    1 + root.children.iter().map(count_elements).sum::<usize>()
}

fn parse_ide_typed(file: &str, bytes: &[u8]) -> Result<IdeOutcome> {
    let parsed = ide::parse_ide(file, bytes)?;
    let mut out = IdeOutcome {
        file: parsed,
        ..IdeOutcome::default()
    };
    // Clone section refs by index to satisfy the borrow checker cheaply.
    let sections: Vec<(String, Vec<ide::RawRow>)> = out
        .file
        .sections
        .iter()
        .map(|s| (s.name.clone(), s.rows.clone()))
        .collect();
    for (name, rows) in &sections {
        for row in rows {
            let r: Result<()> = match name.as_str() {
                "cars" => ide::CarRow::parse(file, row).map(|c| out.cars.push(c)),
                "peds" => ide::PedRow::parse_lenient(file, row).map(|p| out.peds.push(p)),
                "objs" => ide::ObjRow::parse(file, row).map(|o| out.objs.push(o)),
                "txdp" => ide::TxdpRow::parse(file, row).map(|t| out.txdp.push(t)),
                "agrps" => ide::AgrpRow::parse(file, row).map(|a| out.agrps.push(a)),
                _ => Ok(()), // weap/hier/amat/2dfx: raw rows only.
            };
            if let Err(e) = r {
                out.row_errors.push((name.clone(), row.line, e));
            }
        }
    }
    Ok(out)
}

/// Parse one file's bytes, routing by file name (case-insensitive).
///
/// `name` may be a bare file name or a relative path; only the final
/// component and its parent directory (for `effects/`, `materials/`,
/// `streaming/`) steer routing.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
// `base` is lower-cased before matching, so the extension comparisons are
// effectively case-insensitive; the length is an explicit routing table.
#[allow(
    clippy::case_sensitive_file_extension_comparisons,
    clippy::too_many_lines
)]
pub fn parse_file(name: &str, bytes: &[u8]) -> Result<Parsed> {
    if bytes.contains(&0) {
        return Ok(Parsed::Binary);
    }
    let lower = name.replace('\\', "/").to_lowercase();
    let base = lower.rsplit('/').next().unwrap_or_default();
    let parent = lower.rsplit('/').nth(1).unwrap_or_default().to_string();

    // Exact file names first.
    match base {
        "handling.dat" => return Ok(Parsed::Handling(handling::parse_handling(name, bytes)?)),
        "carcols.dat" => return Ok(Parsed::Carcols(carcols::parse_carcols(name, bytes)?)),
        "cargrp.dat" => return Ok(Parsed::Groups(groups::parse_cargrp(name, bytes)?)),
        "pedgrp.dat" => return Ok(Parsed::Groups(groups::parse_pedgrp(name, bytes)?)),
        "pedpersonality.dat" => {
            return Ok(Parsed::PedPersonality(
                ped_personality::parse_ped_personality(name, bytes)?,
            ));
        }
        "pedvariations.dat" => {
            return Ok(Parsed::PedVariations(ped_variations::parse_ped_variations(
                name, bytes,
            )?));
        }
        "pedprops.dat" => {
            return Ok(Parsed::PedProps(ped_variations::parse_ped_props(
                name, bytes,
            )?));
        }
        "weaponinfo.xml" => {
            return Ok(Parsed::WeaponInfo(weapon_info::parse_weapon_info(
                name, bytes,
            )?));
        }
        "thrownweaponinfo.xml" => {
            return Ok(Parsed::Thrown(weapon_info::parse_thrown_weapon_info(
                name, bytes,
            )?));
        }
        "timecyc.dat" => return Ok(Parsed::Timecyc(timecyc::parse_timecyc(name, bytes)?)),
        "water.dat" => return Ok(Parsed::Water(water::parse_water(name, bytes)?)),
        "popcycle.dat" => return Ok(Parsed::Popcycle(popcycle::parse_popcycle(name, bytes)?)),
        "object.dat" => return Ok(Parsed::Object(object::parse_object(name, bytes)?)),
        "materials.dat" => {
            let (v, rows) = materials::parse_materials(name, bytes)?;
            return Ok(Parsed::Materials(v, rows));
        }
        "procedural.dat" => {
            let (v, rows) = materials::parse_procedural(name, bytes)?;
            return Ok(Parsed::Procedural(v, rows));
        }
        "mtl_convert.txt" => {
            return Ok(Parsed::MtlConvert(materials::parse_mtl_convert(
                name, bytes,
            )?));
        }
        "hud.dat" => {
            let s = bracket::parse_bracket(name, bytes)?;
            for sec in &s {
                bracket::hud_items(name, sec)?;
            }
            return Ok(Parsed::Bracket(s));
        }
        "hudcolor.dat" => {
            let s = bracket::parse_bracket(name, bytes)?;
            for sec in &s {
                bracket::hud_colors(name, sec)?;
            }
            return Ok(Parsed::Bracket(s));
        }
        "visualsettings.dat" => {
            return Ok(Parsed::Visual(bracket::parse_visual_settings(name, bytes)?));
        }
        "ped.dat" | "relationships.dat" => {
            return Ok(Parsed::Relations(relations::parse_relations(name, bytes)?));
        }
        "vehoff.csv" => return Ok(Parsed::Vehoff(misc::parse_vehoff(name, bytes)?)),
        "shorelines.dat" => {
            return Ok(Parsed::Shorelines(misc::parse_shorelines(name, bytes)?));
        }
        "numplate.dat" => return Ok(Parsed::Numplate(misc::parse_numplate(name, bytes)?)),
        "stockshake.txt" => {
            return Ok(Parsed::Stockshake(misc::parse_stockshake(name, bytes)?));
        }
        "traincamnodes.txt" => {
            return Ok(Parsed::TrainCams(misc::parse_train_cams(name, bytes)?));
        }
        "nav.dat" | "stream.ini" => {
            return Ok(Parsed::KeyValue(misc::parse_key_value(name, bytes)?));
        }
        "gta.dat"
        | "default.dat"
        | "images.txt"
        | "cj_gta.dat"
        | "cj_images.txt"
        | "animviewer.dat"
        | "animviewer_images.txt"
        | "networktest.dat"
        | "e1_list.txt"
        | "e2_list.txt" => {
            return Ok(Parsed::LoadList(load_list::parse_load_list(name, bytes)?));
        }
        _ => {}
    }

    // Prefix / directory / extension rules.
    if base.starts_with("timecyclemodifiers") {
        return Ok(Parsed::Modifiers(timecyc::parse_modifiers(name, bytes)?));
    }
    if parent == "effects" && base.ends_with(".dat") {
        let table = effects::parse_fx_table(name, bytes)?;
        let mut explosions = Vec::new();
        if base == "explosionfx.dat" {
            for row in &table.rows {
                explosions.push(effects::explosion_row(name, row)?);
            }
        }
        return Ok(Parsed::Fx(FxOutcome { table, explosions }));
    }
    if base.ends_with(".ide") {
        return Ok(Parsed::Ide(parse_ide_typed(name, bytes)?));
    }
    if base == "frontend_menus.xml"
        || base == "leaderboards_data.xml"
        || base == "gtarainemitter.xml"
        || base == "gtarainrender.xml"
        || base == "gtastormemitter.xml"
        || base == "gtastormrender.xml"
    {
        return Ok(Parsed::Xml(xml::parse_xml(name, bytes)?));
    }
    if base == "version.txt" || base == "episodeversion.txt" {
        return Ok(Parsed::Bracket(bracket::parse_bracket(name, bytes)?));
    }
    // Note: `radiohud.dat` looks bracket-shaped but mixes in `+`-ruled
    // texture lists with no section header, and `scrollbars.dat` is
    // `*group` ticker text, so both stay on the scan path.
    if (base.starts_with("hud") && base.ends_with(".dat"))
        || (base.starts_with("frontend") && base.ends_with(".dat"))
        || (base.starts_with("fonts") && base.ends_with(".dat"))
        || (base.starts_with("radiologo") && base.ends_with(".dat"))
        || (base.starts_with("loadingscreens") && base.ends_with(".dat"))
    {
        return Ok(Parsed::Bracket(bracket::parse_bracket(name, bytes)?));
    }
    if base.ends_with(".csv") {
        let has_header = base == "songlist.csv";
        return Ok(Parsed::Csv(misc::parse_csv_table(name, bytes, has_header)?));
    }

    // Fallback: structural scan.
    Ok(Parsed::Scanned(scan::scan_text(name, bytes)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_by_name() {
        let p = parse_file("common/data/handling.dat", b"; only a comment\n").unwrap();
        assert_eq!(p.parser(), "handling");
        let p = parse_file("DATA/FRONTEND_PC.DAT", b"[HD]\nA B\n").unwrap();
        assert_eq!(p.parser(), "bracket");
        let p = parse_file("x/unknown.dat", b"hello world\n").unwrap();
        assert_eq!(p.parser(), "scan");
        let p = parse_file("x/thing.dat", b"a\x00b").unwrap();
        assert_eq!(p.parser(), "binary");
    }

    #[test]
    fn ide_row_errors_collected() {
        // Valid section shape, one bad cars row (too few fields).
        let src = "cars\nadmiral, admiral\nend\n";
        let p = parse_file("vehicles.ide", src.as_bytes()).unwrap();
        match p {
            Parsed::Ide(o) => assert_eq!(o.row_errors.len(), 1),
            _ => panic!("wrong variant"),
        }
    }
}
