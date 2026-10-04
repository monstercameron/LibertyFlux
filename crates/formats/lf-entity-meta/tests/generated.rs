//! Property and fuzz tests on generated vehicle, weapon and ped models.
//! Every byte is generated here (see `crates/formats/tests/support.rs`);
//! bone names come from the vocabulary documented in the crate. No game
//! files are needed.
//!
//! - Property: the bone classifier sorts every documented vehicle and
//!   weapon name into its documented kind; random fragments whose skeletons
//!   carry a random mix of those names yield vehicle layouts with matching
//!   seat, door, wheel, light, extra and siren counts and mount positions;
//!   random drawables yield the weapon mounts named in their skeletons.
//! - Fuzz: mutated fragment and drawable system segments, re-wrapped in a
//!   valid container, never panic in any of the three entry points.

// Fixture builders narrow random words and lengths into smaller fields on
// purpose, and the property checks compare floats that were written
// bit-exactly, so these pedantic lints do not apply here.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::format_push_string,
    clippy::format_collect,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::many_single_char_names,
    clippy::similar_names
)]

#[path = "../../tests/support.rs"]
mod support;

use lf_entity_meta::{BoneKind, LightKind, classify_bone, parse_ped, parse_vehicle, parse_weapon};
use support::{Buf, Rng, fuzz, rsc5};

const SYS: u32 = 0x5000_0000;
const TYPE_DRAWABLE: u32 = 0x6E;
const TYPE_FRAGMENT: u32 = 0x70;
const BONE_LEN: usize = 224;
const FRAG_MAIN_DRAWABLE: usize = 0xB4;
const FRAG_CHILD_COUNT: usize = 0x1F3;

const SEATS: [&str; 4] = [
    "seat_dside_f",
    "seat_pside_f",
    "seat_dside_r",
    "seat_pside_r",
];
const DOORS: [&str; 4] = [
    "door_dside_f",
    "door_pside_f",
    "door_dside_r",
    "door_pside_r",
];
const WHEELS: [&str; 4] = ["wheel_lf", "wheel_rf", "wheel_lr", "wheel_rr"];
const LIGHTS: [(&str, LightKind); 6] = [
    ("headlight_l", LightKind::Headlight),
    ("taillight_r", LightKind::Taillight),
    ("brakelight_m", LightKind::Brakelight),
    ("indicator_lf", LightKind::Indicator),
    ("reversinglight_l", LightKind::Reversing),
    ("interiorlight", LightKind::Interior),
];
const SIRENS: [&str; 3] = ["siren1", "siren2", "siren7"];
const OTHERS: [&str; 5] = ["chassis", "bodyshell", "bonnet", "misc_a", "wheelmesh_lf"];
const WEAPON_MOUNTS: [&str; 4] = ["gun_grip", "gun_muzzle", "gun_ejac", "sight_adjust"];

#[test]
fn classifier_matches_the_documented_vocabulary() {
    for n in SEATS {
        assert_eq!(classify_bone(n), BoneKind::Seat, "{n}");
    }
    for n in DOORS {
        assert_eq!(classify_bone(n), BoneKind::Door, "{n}");
    }
    for n in WHEELS {
        assert_eq!(classify_bone(n), BoneKind::Wheel, "{n}");
    }
    for (n, k) in LIGHTS {
        assert_eq!(classify_bone(n), BoneKind::Light(k), "{n}");
    }
    for i in 1..=10u8 {
        assert_eq!(classify_bone(&format!("extra_{i}")), BoneKind::Extra(i));
    }
    for n in SIRENS {
        assert_eq!(classify_bone(n), BoneKind::Siren, "{n}");
    }
    assert_eq!(classify_bone("siren_glass1"), BoneKind::SirenGlass);
    assert_eq!(classify_bone("wheelmesh_lf"), BoneKind::WheelMesh);
    for n in WEAPON_MOUNTS {
        assert_eq!(classify_bone(n), BoneKind::WeaponMount, "{n}");
    }
    // Random identifiers outside the vocabulary are never misfiled as a
    // seat, door or wheel, and classification never panics.
    let mut rng = Rng::for_test("entity classify");
    for _ in 0..2000 {
        let name = format!("zz{}", rng.ident(1, 20));
        assert!(!matches!(
            classify_bone(&name),
            BoneKind::Seat | BoneKind::Door | BoneKind::Wheel
        ));
    }
}

/// System segment writer for a drawable whose only content is a skeleton
/// (no shaders, no LODs), at `at`.
fn write_skeleton_drawable(sys: &mut Buf, at: usize, bones: &[(String, [f32; 3])]) {
    let alloc = |sys: &mut Buf, len: usize| {
        sys.align(16);
        let a = sys.len();
        sys.put(a, &vec![0; len.max(1)]);
        a
    };
    let n = bones.len();
    let names: Vec<usize> = bones
        .iter()
        .map(|(name, _)| {
            let a = alloc(sys, name.len() + 1);
            sys.put(a, name.as_bytes());
            a
        })
        .collect();
    let records = alloc(sys, n * BONE_LEN);
    for (i, (_, pos)) in bones.iter().enumerate() {
        let r = records + i * BONE_LEN;
        sys.put_u32(r, SYS | names[i] as u32)
            .put_u16(r + 20, i as u16)
            .put_u16(r + 22, i as u16);
        for (k, x) in pos.iter().enumerate() {
            sys.put_f32(r + 32 + 4 * k, *x);
        }
    }
    let parents = alloc(sys, n * 4);
    for i in 0..n {
        sys.put_u32(parents + 4 * i, (i as i32 - 1) as u32);
    }
    // Global pose: identity rotation, translation = bone position.
    let global = alloc(sys, n * 64);
    for (i, (_, pos)) in bones.iter().enumerate() {
        let m = global + i * 64;
        sys.put_f32(m, 1.0)
            .put_f32(m + 20, 1.0)
            .put_f32(m + 40, 1.0)
            .put_f32(m + 60, 1.0);
        for (k, x) in pos.iter().enumerate() {
            sys.put_f32(m + 48 + 4 * k, *x);
        }
    }
    let skel = alloc(sys, 64);
    sys.put_u32(skel, SYS | records as u32)
        .put_u32(skel + 4, SYS | parents as u32)
        .put_u32(skel + 16, SYS | global as u32)
        .put_u16(skel + 20, n as u16);
    sys.put_u32(at, 0x0069_5254)
        .put_u32(at + 12, SYS | skel as u32);
}

fn vehicle_fragment(bones: &[(String, [f32; 3])]) -> Vec<u8> {
    let mut sys = Buf::zeroed(0x200);
    sys.align(16);
    let main = sys.len();
    sys.put(main + 95, &[0]);
    write_skeleton_drawable(&mut sys, main, bones);
    sys.put_u32(FRAG_MAIN_DRAWABLE, SYS | main as u32)
        .put(FRAG_CHILD_COUNT, &[0]);
    sys.0
}

fn random_vehicle_bones(rng: &mut Rng) -> Vec<(String, [f32; 3])> {
    let mut names: Vec<String> = Vec::new();
    let pools: [&[&str]; 5] = [&SEATS, &DOORS, &WHEELS, &SIRENS, &OTHERS];
    for pool in pools {
        for n in pool {
            if rng.chance(1, 2) {
                names.push((*n).to_string());
            }
        }
    }
    for (n, _) in LIGHTS {
        if rng.chance(1, 2) {
            names.push(n.to_string());
        }
    }
    for i in 1..=10 {
        if rng.chance(1, 3) {
            names.push(format!("extra_{i}"));
        }
    }
    names
        .into_iter()
        .map(|n| {
            (
                n,
                [
                    rng.f32_in(-3.0, 3.0),
                    rng.f32_in(-3.0, 3.0),
                    rng.f32_in(-1.0, 2.0),
                ],
            )
        })
        .collect()
}

#[test]
fn vehicle_layout_counts_match_bones() {
    let mut rng = Rng::for_test("entity vehicles");
    for _ in 0..60 {
        let bones = random_vehicle_bones(&mut rng);
        let count = |pool: &[&str]| {
            bones
                .iter()
                .filter(|(n, _)| pool.contains(&n.as_str()))
                .count()
        };
        let lay = parse_vehicle(&rsc5(TYPE_FRAGMENT, &vehicle_fragment(&bones), &[]))
            .expect("generated fragment parses");
        assert_eq!(lay.bone_count, bones.len());
        assert_eq!(lay.seat_count(), count(&SEATS));
        assert_eq!(lay.door_count(), count(&DOORS));
        assert_eq!(lay.wheel_count(), count(&WHEELS));
        assert_eq!(lay.sirens.len(), count(&SIRENS));
        let lights: Vec<&str> = LIGHTS.iter().map(|(n, _)| *n).collect();
        assert_eq!(lay.lights.len(), count(&lights));
        let mut extras: Vec<u8> = bones
            .iter()
            .filter_map(|(n, _)| n.strip_prefix("extra_").and_then(|d| d.parse().ok()))
            .collect();
        extras.sort_unstable();
        let mut got = lay.extra_numbers();
        got.sort_unstable();
        assert_eq!(got, extras);
        for m in lay.mounts() {
            let (_, pos) = &bones[m.bone];
            assert_eq!(&m.local, pos, "{}", m.name);
            assert_eq!(m.world, Some(*pos), "{}", m.name);
        }
        let _ = lay.kind();
    }
}

#[test]
fn weapon_mounts_match_bones() {
    let mut rng = Rng::for_test("entity weapons");
    for _ in 0..60 {
        let mut bones: Vec<(String, [f32; 3])> = vec![("gun_root".into(), [0.0; 3])];
        for n in WEAPON_MOUNTS {
            if rng.chance(1, 2) {
                bones.push((n.to_string(), [rng.f32_in(-1.0, 1.0), 0.0, 0.0]));
            }
        }
        let mut sys = Buf::zeroed(96);
        write_skeleton_drawable(&mut sys, 0, &bones);
        let w = parse_weapon(&rsc5(TYPE_DRAWABLE, &sys.0, &[])).expect("generated drawable parses");
        assert_eq!(w.bone_count, bones.len());
        let named: Vec<&str> = w.mounts().map(|m| m.name.as_str()).collect();
        for (n, _) in bones.iter().skip(1) {
            assert!(named.contains(&n.as_str()), "{n} missing from {named:?}");
        }
        let has_grip = bones.iter().any(|(n, _)| n == "gun_grip");
        let has_muzzle = bones.iter().any(|(n, _)| n == "gun_muzzle");
        if !has_grip && !has_muzzle {
            assert!(!w.is_firearm());
        }
    }
}

#[test]
fn fuzz_entry_points() {
    let mut rng = Rng::for_test("entity fuzz");
    let seeds: Vec<Vec<u8>> = (0..3)
        .map(|_| vehicle_fragment(&random_vehicle_bones(&mut rng)))
        .collect();
    fuzz("entity fragment", &seeds, 2000, |sys| {
        let frag = rsc5(TYPE_FRAGMENT, sys, &[]);
        if let Ok(lay) = parse_vehicle(&frag) {
            let _ = (
                lay.extra_numbers(),
                lay.lights_of(LightKind::Headlight),
                lay.kind(),
            );
            let _ = lay
                .mounts()
                .map(lf_entity_meta::Mount::position)
                .collect::<Vec<_>>();
        }
        if let Ok(ped) = parse_ped(&frag, None) {
            let _ = (ped.component_count(), ped.rig.is_standard);
        }
        if let Ok(w) = parse_weapon(&rsc5(TYPE_DRAWABLE, sys, &[])) {
            let _ = (w.is_firearm(), w.mounts().count());
        }
    });
}
