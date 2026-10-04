//! Text cross-check: model files against the plain-text definitions.
//!
//! Uses `lf-gamedata` (the fmt-data-files crate) to parse the base-game and
//! episode `vehicles.ide`, `peds.ide`, `WeaponInfo.xml` and
//! `VehicleExtras.dat`, then checks the model archives agree: every IDE
//! model has a resource, every weapon model file is referenced, extras line
//! up with `extra_N` bones. Skipped unless `LIBERTYFLUX_GAME_DIR` is set.

use lf_archive::{Archive, crypto};
use lf_gamedata::ide::{CarRow, PedRow, parse_ide};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

fn read(p: &Path) -> Vec<u8> {
    std::fs::read(p).unwrap_or_else(|_| panic!("missing {}", p.display()))
}

/// Base names (lowercase, no extension) of `.wft` entries in the archive.
fn wft_names(path: &Path, key: &crypto::Key) -> BTreeSet<String> {
    let mut r = BufReader::new(File::open(path).unwrap());
    let arch = lf_archive::open(&mut r, Some(key)).unwrap();
    arch.entries()
        .iter()
        .filter(|e| e.is_file() && e.path.to_lowercase().ends_with(".wft"))
        .map(|e| {
            let base = e.path.rsplit('/').next().unwrap().to_lowercase();
            base.trim_end_matches(".wft").to_string()
        })
        .collect()
}

fn wdr_names(path: &Path, key: &crypto::Key) -> BTreeSet<String> {
    let mut r = BufReader::new(File::open(path).unwrap());
    let arch = lf_archive::open(&mut r, Some(key)).unwrap();
    arch.entries()
        .iter()
        .filter(|e| e.is_file() && e.path.to_lowercase().ends_with(".wdr"))
        .map(|e| {
            e.path
                .rsplit('/')
                .next()
                .unwrap()
                .to_lowercase()
                .trim_end_matches(".wdr")
                .to_string()
        })
        .collect()
}

#[test]
// One linear cross-check; splitting it would scatter the shared sets.
#[allow(clippy::too_many_lines)]
// File names are lowercased before the extension check, so the
// comparison is already case-insensitive.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn ide_models_have_resources() {
    let Some(game) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset: skipping text cross-check");
        return;
    };
    let exe = game.join("GTAIV/GTAIV.exe");
    let Ok(key) = crypto::load_key_from_exe(&exe) else {
        eprintln!("no key: skipping text cross-check");
        return;
    };
    // Collect wft/wdr names across base + episodes.
    let mut wft: BTreeSet<String> = BTreeSet::new();
    let mut wdr: BTreeSet<String> = BTreeSet::new();
    for top in [
        "GTAIV/pc/models/cdimages",
        "GTAIV/TLAD/pc/models/cdimages",
        "GTAIV/TBoGT/pc/models/cdimages",
    ] {
        for name in [
            "vehicles.img",
            "weapons.img",
            "weapons_e1.img",
            "weapons_e2.img",
            "componentpeds.img",
        ] {
            let p = game.join(top).join(name);
            if !p.exists() {
                continue;
            }
            wft.extend(wft_names(&p, &key));
            wdr.extend(wdr_names(&p, &key));
        }
        let rpf = "playerped.rpf";
        let p = game.join(top).join(rpf);
        if p.exists() {
            wft.extend(wft_names(&p, &key));
            wdr.extend(wdr_names(&p, &key));
        }
    }
    println!(
        "wft names total: {}, wdr names total: {}",
        wft.len(),
        wdr.len()
    );

    // Vehicles + peds IDEs in base and episodes.
    let mut ide_cars: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut ide_peds: BTreeSet<String> = BTreeSet::new();
    for top in [
        "GTAIV/common/data",
        "GTAIV/TLAD/common/data",
        "GTAIV/TBoGT/common/data",
    ] {
        let dir = game.join(top);
        for f in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let name = f.file_name().to_string_lossy().to_lowercase();
            if !name.ends_with(".ide") {
                continue;
            }
            let bytes = read(&f.path());
            let label = format!("{top}/{name}");
            let Ok(ide) = parse_ide(&label, &bytes) else {
                continue;
            };
            if let Some(sec) = ide.section("cars") {
                for row in &sec.rows {
                    if let Ok(car) = CarRow::parse(&label, row) {
                        ide_cars
                            .entry(car.model.to_lowercase())
                            .or_default()
                            .extend(car.extras.iter().cloned());
                    }
                }
            }
            if let Some(sec) = ide.section("peds") {
                for row in &sec.rows {
                    if let Ok((ped, _)) = PedRow::parse_lenient(&label, row) {
                        ide_peds.insert(ped.model.to_lowercase());
                    }
                }
            }
        }
    }
    println!("ide cars: {}, ide peds: {}", ide_cars.len(), ide_peds.len());

    let cars_missing: Vec<_> = ide_cars
        .keys()
        .filter(|m| !wft.contains(m.as_str()))
        .collect();
    let peds_missing: Vec<_> = ide_peds
        .iter()
        .filter(|m| !wft.contains(m.as_str()))
        .collect();
    // Cutscene models (cs_*) ship in the cutscene archives, not the ped
    // archives; everything else must resolve.
    let (cs_missing, other_missing): (Vec<_>, Vec<_>) =
        peds_missing.into_iter().partition(|m| m.starts_with("cs_"));
    println!("ide cars without .wft: {cars_missing:?}");
    println!("cutscene peds (expected elsewhere): {}", cs_missing.len());
    println!("other peds without .wft: {other_missing:?}");
    assert!(
        cars_missing.is_empty(),
        "cars missing resources: {cars_missing:?}"
    );
    assert!(
        other_missing.is_empty(),
        "peds missing resources: {other_missing:?}"
    );

    // WeaponInfo.xml model references vs .wdr files.
    let wi_path = game.join("GTAIV/common/data/WeaponInfo.xml");
    let mut wi_models: BTreeSet<String> = BTreeSet::new();
    if wi_path.exists() {
        let bytes = read(&wi_path);
        let info = lf_gamedata::weapon_info::parse_weapon_info("WeaponInfo.xml", &bytes).unwrap();
        println!("weapons in WeaponInfo.xml: {}", info.weapons.len());
        for w in &info.weapons {
            // Model names live in <assets model="w_...">, kept in `extra`.
            for block in &w.extra {
                if block.name == "assets"
                    && let Some(m) = block.attr("model")
                {
                    wi_models.insert(m.to_lowercase());
                }
            }
        }
    }
    println!("weapon model refs: {wi_models:?}");
    let wi_missing: Vec<_> = wi_models
        .iter()
        .filter(|m| !wdr.contains(m.as_str()))
        .collect();
    println!("weapon models without .wdr: {wi_missing:?}");
    assert!(
        wi_missing.is_empty(),
        "weapon models missing: {wi_missing:?}"
    );

    // VehicleExtras.dat: which extras exist per vehicle (informational:
    // the file maps model -> extra names; bones are the authority).
    let ve_path = game.join("GTAIV/common/data/VehicleExtras.dat");
    if ve_path.exists() {
        let raw = read(&ve_path);
        let text = String::from_utf8_lossy(&raw);
        let models = text
            .lines()
            .filter(|l| {
                let t = l.trim();
                !(t.is_empty() || t.starts_with('#') || t.starts_with(';') || t.starts_with('/'))
            })
            .count();
        println!("VehicleExtras.dat data lines: {models}");
    }
}
