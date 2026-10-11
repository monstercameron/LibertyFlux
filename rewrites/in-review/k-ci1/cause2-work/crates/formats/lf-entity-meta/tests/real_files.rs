//! Real-file integration test: every vehicle/ped/weapon resource in the game.
//!
//! Opens the model archives (base game + both episodes) with `lf-archive`,
//! parses every `.wft`/`.wdr`/`.wdd` with `lf-model`, builds the meta layer
//! and checks it. Skipped unless `LIBERTYFLUX_GAME_DIR` points at the install (the
//! folder holding `GTAIV/`). Prints a census; fails on any parse error.

use lf_archive::{Archive, crypto};
use lf_entity_meta::{BoneKind, PedModel, VehicleLayout, WeaponMounts, classify_bone};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

fn archives_under(game: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for top in [
        "GTAIV/pc/models/cdimages",
        "GTAIV/TLAD/pc/models/cdimages",
        "GTAIV/TBoGT/pc/models/cdimages",
    ] {
        let dir = game.join(top);
        for name in [
            "vehicles.img",
            "weapons.img",
            "weapons_e1.img",
            "weapons_e2.img",
            "componentpeds.img",
            "pedprops.img",
            "playerped.rpf",
        ] {
            let p = dir.join(name);
            if p.exists() {
                out.push(p);
            }
        }
    }
    out
}

#[test]
// One linear census pass; splitting it would scatter the shared counters.
#[allow(clippy::too_many_lines)]
// Entry paths are lowercased before the extension checks, so the
// comparisons are already case-insensitive.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn all_entity_resources_parse() {
    let Some(game) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR unset: skipping real-file test");
        return;
    };
    let exe = game.join("GTAIV/GTAIV.exe");
    let key = match crypto::load_key_from_exe(&exe) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("no key ({e}): skipping real-file test");
            return;
        }
    };
    // Container cross-check state: first file of each kind also parsed
    // with lf-resource; segment sizes must agree with lf-model.
    let mut cross_checked: BTreeSet<&str> = BTreeSet::new();

    let mut n_archives = 0usize;
    let mut parsed: BTreeMap<&str, usize> = BTreeMap::new();
    let mut failures: Vec<String> = Vec::new();
    let mut fail_kinds: BTreeMap<String, usize> = BTreeMap::new();
    // Meta stats.
    let mut veh_seats: BTreeMap<usize, usize> = BTreeMap::new();
    let mut veh_kinds: BTreeMap<&str, usize> = BTreeMap::new();
    let mut veh_bones: BTreeSet<String> = BTreeSet::new();
    let mut veh_unclassified: BTreeMap<String, usize> = BTreeMap::new();
    let mut ped_standard = 0usize;
    let mut ped_total = 0usize;
    let mut ped_comps: BTreeMap<usize, usize> = BTreeMap::new();
    let mut ped_hashes: BTreeSet<u32> = BTreeSet::new();
    let mut weap_firearms = 0usize;
    let mut weap_total = 0usize;
    let mut player_draw: BTreeMap<String, usize> = BTreeMap::new();
    let mut player_tex: BTreeMap<String, usize> = BTreeMap::new();
    let mut wbs_files: Vec<String> = Vec::new();
    let mut mirror_ok = 0usize;
    let mut mirror_total = 0usize;
    let mut mirror_bad: Vec<String> = Vec::new();
    let mut unknown_kind: Vec<String> = Vec::new();
    let mut odd_peds: Vec<String> = Vec::new();
    let mut nocomp_peds: Vec<String> = Vec::new();

    for path in archives_under(&game) {
        n_archives += 1;
        let mut reader = BufReader::new(File::open(&path).unwrap());
        let arch = lf_archive::open(&mut reader, Some(&key)).unwrap();
        // Ped dictionaries by base name for joining with fragments.
        let mut dict_bytes: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for (i, e) in arch.entries().iter().enumerate() {
            if !e.is_file() {
                continue;
            }
            let low = e.path.to_lowercase();
            if low.ends_with(".wdd") {
                let mut r = BufReader::new(File::open(&path).unwrap());
                if let Ok(b) = arch.read_file(&mut r, i, Some(&key)) {
                    dict_bytes.insert(strip_ext(&low).to_string(), b);
                }
            }
        }
        for (i, e) in arch.entries().iter().enumerate() {
            if !e.is_file() {
                continue;
            }
            let low = e.path.to_lowercase();
            let ext = if low.ends_with(".wft") {
                "wft"
            } else if low.ends_with(".wdr") {
                "wdr"
            } else if low.ends_with(".wdd") {
                "wdd"
            } else if low.ends_with(".wtd") && is_player_archive(&path) {
                let base = low.rsplit('/').next().unwrap_or(&low);
                let pre = base.split('_').next().unwrap_or(base).to_string();
                *player_tex.entry(pre).or_insert(0) += 1;
                continue;
            } else if low.ends_with(".wbs") {
                wbs_files.push(format!("{}:{}", path.display(), e.path));
                continue;
            } else {
                continue;
            };
            let mut r = BufReader::new(File::open(&path).unwrap());
            let bytes = match arch.read_file(&mut r, i, Some(&key)) {
                Ok(b) => b,
                Err(err) => {
                    failures.push(format!("{}:{}: read: {}", path.display(), e.path, err));
                    *fail_kinds.entry("read".to_string()).or_insert(0) += 1;
                    continue;
                }
            };
            // Cross-check the container layer once per kind.
            if cross_checked.insert(ext) {
                let lr = lf_resource::Resource::parse(&bytes).expect("lf-resource parses it");
                let lm = lf_model::Resource::open(&bytes).expect("lf-model parses it");
                assert_eq!(lr.system().len(), lm.sys.len(), "system size agrees");
                assert_eq!(lr.graphics().len(), lm.gfx.len(), "graphics size agrees");
            }
            *parsed.entry(ext).or_insert(0) += 1;
            let res = lf_model::Resource::open(&bytes);
            let res = match res {
                Ok(r) => r,
                Err(err) => {
                    failures.push(format!("{}:{}: container: {}", path.display(), e.path, err));
                    *fail_kinds.entry("container".to_string()).or_insert(0) += 1;
                    continue;
                }
            };
            match ext {
                "wft" => {
                    let frag = match lf_model::Fragment::parse(&res) {
                        Ok(f) => f,
                        Err(err) => {
                            failures.push(format!(
                                "{}:{}: fragment: {}",
                                path.display(),
                                e.path,
                                err
                            ));
                            *fail_kinds.entry("fragment".to_string()).or_insert(0) += 1;
                            continue;
                        }
                    };
                    if is_ped_archive(&path) {
                        ped_total += 1;
                        let dict = dict_bytes
                            .get(strip_ext(&low))
                            .and_then(|b| lf_model::Resource::open(b).ok())
                            .and_then(|r| lf_model::DrawableDictionary::parse(&r).ok());
                        let ped = PedModel::from_parts(&frag, dict.as_ref());
                        if ped.rig.is_standard {
                            ped_standard += 1;
                        } else {
                            odd_peds.push(format!(
                                "{}:{} bones={} children={}",
                                path.display(),
                                e.path,
                                ped.rig.bone_count,
                                ped.rig.child_count
                            ));
                        }
                        if ped.component_count() == 0 {
                            nocomp_peds.push(format!("{}:{}", path.display(), e.path));
                        }
                        *ped_comps.entry(ped.component_count()).or_insert(0) += 1;
                        for c in &ped.components {
                            ped_hashes.insert(c.hash);
                        }
                    } else if is_weapon_archive(&path) {
                        // Breakable prop bags ship as fragments.
                        weap_total += 1;
                        let w = WeaponMounts::from_fragment(&frag);
                        if w.is_firearm() {
                            weap_firearms += 1;
                        }
                    } else {
                        let lay = VehicleLayout::from_fragment(&frag);
                        *veh_seats.entry(lay.seat_count()).or_insert(0) += 1;
                        *veh_kinds.entry(kind_name(&lay)).or_insert(0) += 1;
                        for m in lay.mounts().chain(lay.other.iter()) {
                            veh_bones.insert(m.name.clone());
                            if classify_bone(&m.name) == BoneKind::Other {
                                *veh_unclassified.entry(m.name.clone()).or_insert(0) += 1;
                            }
                        }
                        mirror_total += 1;
                        if seats_mirror(&lay) {
                            mirror_ok += 1;
                        } else {
                            let show = |n: &str| {
                                lay.seats
                                    .iter()
                                    .find(|m| m.name == n)
                                    .map(|m| format!("{:?}/{:?}", m.local, m.world))
                                    .unwrap_or_default()
                            };
                            mirror_bad.push(format!(
                                "{}:{} dside_f={} pside_f={}",
                                path.display(),
                                e.path,
                                show("seat_dside_f"),
                                show("seat_pside_f")
                            ));
                        }
                        if lay.kind() == lf_entity_meta::VehicleKind::Unknown {
                            unknown_kind.push(format!("{}:{}", path.display(), e.path));
                        }
                    }
                }
                "wdr" => {
                    let draw = match lf_model::Drawable::parse(&res) {
                        Ok(d) => d,
                        Err(err) => {
                            failures.push(format!(
                                "{}:{}: drawable: {}",
                                path.display(),
                                e.path,
                                err
                            ));
                            *fail_kinds.entry("drawable".to_string()).or_insert(0) += 1;
                            continue;
                        }
                    };
                    if is_player_archive(&path) {
                        let base = e.path.rsplit('/').next().unwrap_or(&e.path);
                        let pre = base.split('_').next().unwrap_or(base).to_lowercase();
                        *player_draw.entry(pre).or_insert(0) += 1;
                    } else {
                        weap_total += 1;
                        if WeaponMounts::from_drawable(&draw).is_firearm() {
                            weap_firearms += 1;
                        }
                    }
                }
                "wdd" => {
                    let dict = match lf_model::DrawableDictionary::parse(&res) {
                        Ok(d) => d,
                        Err(err) => {
                            failures.push(format!(
                                "{}:{}: dictionary: {}",
                                path.display(),
                                e.path,
                                err
                            ));
                            *fail_kinds.entry("dictionary".to_string()).or_insert(0) += 1;
                            continue;
                        }
                    };
                    if is_ped_archive(&path) {
                        for (i, d) in dict.entries.iter().enumerate() {
                            let _ = (i, d);
                        }
                        // Props and ped dicts counted via fragments already;
                        // record hashes for the census.
                        for h in &dict.hashes {
                            ped_hashes.insert(*h);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    println!("archives opened: {n_archives}");
    println!("parsed by kind: {parsed:?}");
    println!("distinct failures: {fail_kinds:?}");
    println!("vehicle seat-count distribution: {veh_seats:?}");
    println!("vehicle kind distribution: {veh_kinds:?}");
    println!("vehicle distinct bones: {}", veh_bones.len());
    println!("vehicle unclassified bones: {veh_unclassified:?}");
    println!("peds: {ped_total} fragments, {ped_standard} standard rigs");
    println!("ped component-count distribution: {ped_comps:?}");
    println!("ped distinct dict hashes: {}", ped_hashes.len());
    println!("weapons: {weap_total} files, {weap_firearms} firearms");
    println!("player drawable variants by component: {player_draw:?}");
    println!("player texture files by component: {player_tex:?}");
    println!("wbs files: {wbs_files:?}");
    println!("seat mirror check: {mirror_ok}/{mirror_total}");
    println!("non-mirroring: {mirror_bad:?}");
    println!("unknown-kind vehicles: {unknown_kind:?}");
    println!("non-standard peds: {odd_peds:?}");
    println!("peds without dict: {nocomp_peds:?}");
    for f in failures.iter().take(20) {
        println!("FAIL: {f}");
    }
    assert!(
        failures.is_empty(),
        "{} files failed, first: {:?}",
        failures.len(),
        failures.first()
    );
    assert!(
        n_archives >= 12,
        "expected base + episode archives, got {n_archives}"
    );
    assert!(parsed.values().sum::<usize>() > 1000);
}

fn strip_ext(name: &str) -> &str {
    match name.rfind('.') {
        Some(i) => &name[..i],
        None => name,
    }
}

fn file_name(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().to_lowercase()
}

fn is_ped_archive(path: &Path) -> bool {
    let n = file_name(path);
    n.contains("componentpeds") || n.contains("pedprops") || n.contains("playerped")
}

fn is_weapon_archive(path: &Path) -> bool {
    file_name(path).contains("weapon")
}

fn is_player_archive(path: &Path) -> bool {
    file_name(path).contains("playerped")
}

fn kind_name(lay: &VehicleLayout) -> &'static str {
    match lay.kind() {
        lf_entity_meta::VehicleKind::Car => "car",
        lf_entity_meta::VehicleKind::Bike => "bike",
        lf_entity_meta::VehicleKind::Boat => "boat",
        lf_entity_meta::VehicleKind::Helicopter => "heli",
        lf_entity_meta::VehicleKind::Unknown => "unknown",
    }
}

/// Do the front seat pair mirror in exactly one axis (validates the world
/// position convention)? Returns true when no world positions exist (vacuous).
fn seats_mirror(lay: &VehicleLayout) -> bool {
    let get = |n: &str| lay.seats.iter().find(|m| m.name == n).and_then(|m| m.world);
    let (Some(a), Some(b)) = (get("seat_dside_f"), get("seat_pside_f")) else {
        return true;
    };
    // Mirror axis: sum near zero while differing clearly.
    let mut mirrored = 0;
    for i in 0..3 {
        if (a[i] + b[i]).abs() < 0.05 && (a[i] - b[i]).abs() > 0.2 {
            mirrored += 1;
        }
    }
    mirrored == 1
}
