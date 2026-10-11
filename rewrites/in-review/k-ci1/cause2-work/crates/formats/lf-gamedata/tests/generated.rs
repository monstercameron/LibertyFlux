//! Property and fuzz tests on generated gameplay data files. Every byte is
//! generated here (see `crates/formats/tests/support.rs`): names and numbers
//! are random, only the file shapes follow the formats. No game files are
//! needed.
//!
//! - Property: random `water.dat`, `timecyc.dat`, `trainCamNodes.txt`,
//!   `stockshake.txt`, `carcols.dat`, `ped.dat` relations, `cargrp.dat`,
//!   load lists, key-value files and XML trees, routed through
//!   [`parse_file`] by file name, parse back to the values written.
//! - Fuzz: one seed per routed parser (handling, IDE, carcols, groups, ped
//!   personality and variations, weapon XML, time cycle, water, popcycle,
//!   object, materials, bracket files, relations, effects tables, CSV, misc
//!   formats, load lists, XML, scan fallback), mutated, never panics.

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

use lf_gamedata::route::{Parsed, parse_file};
use support::{Rng, fuzz};

/// Numbers with an exact shortest decimal form, so text round-trips.
fn num(rng: &mut Rng) -> f32 {
    rng.below(200_000) as f32 / 100.0 - 1000.0
}

fn nums(rng: &mut Rng, n: usize) -> Vec<f32> {
    (0..n).map(|_| num(rng)).collect()
}

fn join(v: &[f32], sep: &str) -> String {
    v.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(sep)
}

#[test]
fn water_round_trip() {
    let mut rng = Rng::for_test("gamedata water");
    for _ in 0..50 {
        let quads: Vec<Vec<f32>> = (0..rng.below(8)).map(|_| nums(&mut rng, 28)).collect();
        let kinds: Vec<i32> = quads.iter().map(|_| rng.below(4) as i32).collect();
        let mut text = String::new();
        for (q, k) in quads.iter().zip(&kinds) {
            text.push_str(&format!("{} {k} 1\n", join(q, " ")));
        }
        let Parsed::Water(got) = parse_file("common/data/water.dat", text.as_bytes()).unwrap()
        else {
            panic!("water.dat routed elsewhere");
        };
        assert_eq!(got.len(), quads.len());
        for ((g, q), k) in got.iter().zip(&quads).zip(&kinds) {
            for (v, vert) in g.verts.iter().enumerate() {
                let b = v * 7;
                assert_eq!(vert.pos, [q[b], q[b + 1], q[b + 2]]);
                assert_eq!(vert.normal, [q[b + 3], q[b + 4], q[b + 5]]);
                assert_eq!(vert.foam, q[b + 6]);
            }
            assert_eq!((g.kind, g.flag), (*k, 1.0));
        }
    }
}

/// Values per `timecyc.dat` slot row.
const TIMECYC_VALUES: usize = 134;

fn random_timecyc(rng: &mut Rng) -> (String, Vec<(String, usize)>) {
    let mut text = String::from("// generated time cycle\n");
    let mut want = Vec::new();
    for w in 0..rng.range(1, 3) {
        let name = format!("WEATHER{w}{}", rng.ident(1, 6).to_uppercase());
        text.push_str(&format!("////////// {name}\n"));
        let slots = rng.range(1, 4);
        for s in 0..slots {
            text.push_str(&format!("//Slot{s}\n"));
            let row: Vec<String> = (0..TIMECYC_VALUES)
                .map(|_| rng.below(256).to_string())
                .collect();
            text.push_str(&row.join(" "));
            text.push('\n');
        }
        want.push((name, slots));
    }
    (text, want)
}

#[test]
fn timecyc_round_trip() {
    let mut rng = Rng::for_test("gamedata timecyc");
    for _ in 0..30 {
        let (text, want) = random_timecyc(&mut rng);
        let Parsed::Timecyc(got) = parse_file("timecyc.dat", text.as_bytes()).unwrap() else {
            panic!("timecyc.dat routed elsewhere");
        };
        let got: Vec<(String, usize)> = got
            .iter()
            .map(|w| (w.name.clone(), w.slots.len()))
            .collect();
        assert_eq!(got, want);
    }
}

#[test]
fn small_text_formats_round_trip() {
    let mut rng = Rng::for_test("gamedata small formats");
    for _ in 0..50 {
        // Train camera nodes: id plus four numbers.
        let cams: Vec<(String, Vec<f32>)> = (0..rng.below(6))
            .map(|i| (format!("cam{i}"), nums(&mut rng, 4)))
            .collect();
        let text: String = cams
            .iter()
            .map(|(id, v)| format!("{id} {}\n", join(v, " ")))
            .collect();
        let Parsed::TrainCams(got) = parse_file("trainCamNodes.txt", text.as_bytes()).unwrap()
        else {
            panic!("train cams routed elsewhere");
        };
        assert_eq!(got.len(), cams.len());
        for (g, (id, v)) in got.iter().zip(&cams) {
            assert_eq!(&g.id, id);
            assert_eq!(g.pos, [v[0], v[1], v[2]]);
            assert_eq!(g.detect, v[3]);
        }
        // Stock shake rows: ten numbers each.
        let rows: Vec<Vec<f32>> = (0..rng.below(6)).map(|_| nums(&mut rng, 10)).collect();
        let text: String = rows.iter().map(|r| format!("{}\n", join(r, " "))).collect();
        let Parsed::Stockshake(got) = parse_file("stockshake.txt", text.as_bytes()).unwrap() else {
            panic!("stockshake routed elsewhere");
        };
        assert_eq!(got.iter().map(|r| r.to_vec()).collect::<Vec<_>>(), rows);
        // Key-value files.
        let pairs: Vec<(String, String)> = (0..rng.below(6))
            .map(|i| {
                (
                    format!("{}_{i}", rng.ident(1, 10).to_uppercase()),
                    rng.below(1000).to_string(),
                )
            })
            .collect();
        let text: String = pairs.iter().map(|(k, v)| format!("{k}={v}\n")).collect();
        let Parsed::KeyValue(got) = parse_file("nav.dat", text.as_bytes()).unwrap() else {
            panic!("nav.dat routed elsewhere");
        };
        assert_eq!(got, pairs);
        // Load lists: keyword plus arguments.
        let dirs: Vec<(String, String)> = (0..rng.below(6))
            .map(|i| {
                (
                    format!("KEY{i}"),
                    format!("common:/data/{}.txt", rng.ident(1, 8)),
                )
            })
            .collect();
        let text: String = dirs.iter().map(|(k, a)| format!("{k} {a}\n")).collect();
        let Parsed::LoadList(got) = parse_file("gta.dat", text.as_bytes()).unwrap() else {
            panic!("gta.dat routed elsewhere");
        };
        assert_eq!(got.len(), dirs.len());
        for (g, (k, a)) in got.iter().zip(&dirs) {
            assert_eq!(&g.key, k);
            assert_eq!(g.args, vec![a.clone()]);
        }
    }
}

#[test]
fn carcols_groups_relations_round_trip() {
    let mut rng = Rng::for_test("gamedata carcols");
    for _ in 0..50 {
        let palette: Vec<[u8; 3]> = (0..rng.range(1, 6))
            .map(|_| {
                [
                    rng.below(256) as u8,
                    rng.below(256) as u8,
                    rng.below(256) as u8,
                ]
            })
            .collect();
        let cars: Vec<(String, Vec<u8>)> = (0..rng.below(5))
            .map(|i| {
                (
                    format!("car{i}{}", rng.ident(1, 5)),
                    (0..4).map(|_| rng.below(palette.len()) as u8).collect(),
                )
            })
            .collect();
        let mut text = String::from("# generated\ncol\n");
        for (i, c) in palette.iter().enumerate() {
            text.push_str(&format!(
                "{},{},{},-,colour{i} # {i} colour{i}\n",
                c[0], c[1], c[2]
            ));
        }
        text.push_str("end\ncar4\n");
        for (car, idx) in &cars {
            let idx: Vec<String> = idx.iter().map(ToString::to_string).collect();
            text.push_str(&format!("{car}, {},\n", idx.join(",")));
        }
        text.push_str("end\n");
        let Parsed::Carcols(got) = parse_file("carcols.dat", text.as_bytes()).unwrap() else {
            panic!("carcols routed elsewhere");
        };
        assert_eq!(
            got.palette.iter().map(|p| p.rgb).collect::<Vec<_>>(),
            palette
        );
        assert_eq!(got.car4.len(), cars.len());
        for (g, (car, idx)) in got.car4.iter().zip(&cars) {
            assert_eq!(&g.car, car);
            assert_eq!(&g.indices, idx);
        }

        // Car groups: comma-separated models, group named in the comment.
        let groups: Vec<Vec<String>> = (0..rng.range(1, 4))
            .map(|g| {
                (0..rng.range(1, 4))
                    .map(|m| format!("model{g}_{m}"))
                    .collect()
            })
            .collect();
        let text: String = groups
            .iter()
            .enumerate()
            .map(|(g, m)| format!("{}, # POPCYCLE_GROUP_{g}\n", m.join(", ")))
            .collect();
        let Parsed::Groups(got) = parse_file("cargrp.dat", text.as_bytes()).unwrap() else {
            panic!("cargrp routed elsewhere");
        };
        assert_eq!(
            got.iter().map(|g| g.models.clone()).collect::<Vec<_>>(),
            groups
        );

        // Relationship groups: a name line, then level lines with targets.
        let rel: Vec<(String, Vec<(String, Vec<String>)>)> = (0..rng.range(1, 4))
            .map(|g| {
                let levels = (0..rng.range(1, 3))
                    .map(|l| {
                        (
                            ["Hate", "Respect", "Like", "Dislike"][l].to_string(),
                            (0..rng.range(1, 3)).map(|t| format!("GROUP{t}")).collect(),
                        )
                    })
                    .collect();
                (format!("GROUP{g}"), levels)
            })
            .collect();
        let mut text = String::from("# Acquaintance\n");
        for (name, levels) in &rel {
            text.push_str(&format!("{name}\n"));
            for (level, targets) in levels {
                text.push_str(&format!("{level} {}\n", targets.join(" ")));
            }
        }
        let Parsed::Relations(got) = parse_file("relationships.dat", text.as_bytes()).unwrap()
        else {
            panic!("relationships routed elsewhere");
        };
        assert_eq!(got.len(), rel.len());
        for (g, (name, levels)) in got.iter().zip(&rel) {
            assert_eq!(&g.name, name);
            let gl: Vec<(String, Vec<String>)> = g
                .relations
                .iter()
                .map(|r| (r.level.clone(), r.targets.clone()))
                .collect();
            assert_eq!(&gl, levels);
        }
    }
}

/// A random XML tree and its text.
fn random_xml(rng: &mut Rng, depth: usize) -> (String, usize) {
    let name = rng.ident(1, 8);
    let attrs: String = (0..rng.below(3))
        .map(|i| format!(" a{i}=\"{}\"", rng.below(100)))
        .collect();
    if depth == 0 || rng.chance(1, 3) {
        return (format!("<{name}{attrs}/>"), 1);
    }
    let mut body = String::new();
    let mut count = 1;
    for _ in 0..rng.below(4) {
        let (child, n) = random_xml(rng, depth - 1);
        body.push_str(&child);
        count += n;
    }
    (format!("<{name}{attrs}>{body}</{name}>"), count)
}

#[test]
fn xml_round_trip() {
    let mut rng = Rng::for_test("gamedata xml");
    for _ in 0..60 {
        let (text, count) = random_xml(&mut rng, 4);
        let text = format!("<?xml version=\"1.0\"?>\n<!-- generated -->\n{text}\n");
        let parsed = parse_file("leaderboards_data.xml", text.as_bytes()).unwrap();
        assert_eq!(parsed.parser(), "xml");
        assert_eq!(parsed.counts(), vec![("elements".to_string(), count)]);
    }
}

/// One seed per routed parser: (file name, contents). Names and numbers
/// are invented; only the shapes follow the formats.
fn seeds(rng: &mut Rng) -> Vec<(&'static str, String)> {
    // Small integers parse as every numeric field type.
    let row = |rng: &mut Rng, n: usize| {
        (0..n)
            .map(|_| rng.below(10).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    };
    vec![
        ("handling.dat", format!("; c\nCARA {}\n% BOATA {}\n! BIKEA {}\n$ HELIA {}\n", row(rng, 36), row(rng, 19), row(rng, 15), row(rng, 22))),
        ("vehicles.ide", "# c\ncars\ncara, cara, car, CARA, CARA, VEH@STD, NULL, 100, 999, 0.2, 0.2, 0, 2, 1.0, 0, -\nend\ntxdp\ncara, sharedtxd\nend\npeds\npeda, pedb\nend\n".to_string()),
        ("carcols.dat", "col\n1,2,3,-,red # 0 red\nend\ncar3\nbusa, 1,2,3,\nend\ncar4\ncara, 0,1,2,3,\nend\n".to_string()),
        ("cargrp.dat", "cara, carb, # POPCYCLE_GROUP_A\ncarc, # POPCYCLE_GROUP_B\n".to_string()),
        ("pedgrp.dat", "# POPCYCLE_GROUP_A\npeda\npedb\n# POPCYCLE_GROUP_B\npedc\n".to_string()),
        ("pedpersonality.dat", "pa, M, 30, 3, 5, 1, 0.4, 5, 50, E, E, ma, mb, mc, md, me, -, -, mf, +flag\n".to_string()),
        ("pedvariations.dat", "peda, 2\nfeet, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, 3\nhead, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, 1\nend\n".to_string()),
        ("pedprops.dat", "peda_p, 1\nhead, 0, 0, 0, 0, 0, 0, 0\nend\n".to_string()),
        ("weaponinfo.xml", "<weaponinfo version=\"1\"><weapon type=\"W\"><data slot=\"S\" firetype=\"F\" damagetype=\"D\" group=\"G\" targetrange=\"1.0\" weaponrange=\"2.0\" clipsize=\"3\"/><damage base=\"1\"/><flags><flag>A</flag></flags></weapon></weaponinfo>".to_string()),
        ("thrownweaponinfo.xml", "<thrownweaponinfo><object name=\"O\"><offset x=\"1\" y=\"2\" z=\"3\"/><rotoffset x=\"0\" y=\"90\" z=\"0\"/></object></thrownweaponinfo>".to_string()),
        ("timecyc.dat", random_timecyc(rng).0),
        ("timecyclemodifiers.dat", format!("MODA {}\n", row(rng, 69))),
        ("water.dat", format!("{} 0 1\n", row(rng, 28))),
        ("popcycle.dat", format!("// POPCYCLE_ZONE_A\n{}", (0..24).map(|_| row(rng, 48) + "\n").collect::<String>())),
        ("object.dat", "; c\nboxa, 10.0, 10.0, 0.9, 0.03, 50.0, 0.0, 2.5, 20, 2, 1, 0, 2, 0, 0.0, 0.0, fxa\n".to_string()),
        ("materials.dat", "2.00\n# c\nMATA MATA DEFAULT 1.0 0.1 1800.0 1.00 -0.10 0 0.0 0.0 6.0 0.7 0 0 0 MATA\n".to_string()),
        ("procedural.dat", "1.00\n# c\n".to_string()),
        ("mtl_convert.txt", "DEFAULT default - 0 7 0\nTAGA taga ## EXTA 0 7 0\n".to_string()),
        ("hud.dat", "[HD]\nHUD_A 0.5,0.45 0.6,1.0 HUD_COLOUR_A 255\n".to_string()),
        ("hudcolor.dat", "[HD]\nHUD_COLOUR_A 1 2 3\n".to_string()),
        ("visualsettings.dat", "rain.a -0.98\nrain.b 16\n".to_string()),
        ("ped.dat", "# Acquaintance\nGA\nHate GB GC\nRespect GD\n".to_string()),
        ("effects/explosionfx.dat", format!("1.0\n# c\nEXPLOSIONFX_TABLE_START\nEXPA {} fxa fxb 1.0 0 0 0.0 0.0 0.6 EXPA 0 1.0 0.3 0.0 250.0 24.0 0.3 2\n", row(rng, 11))),
        ("songlist.csv", "Artist,Song,Station,\n0,SongA,StationA,RADIO_A\n".to_string()),
        ("vehoff.csv", "VEHICLE cara FIELD F POS 0.1 -2.0 -0.2 ANGLES 0.9 0.2 2.7 FOV 55.0\n".to_string()),
        ("shorelines.dat", "1\n2 0\n-1.5 2.5\n-3.5 4.5\n".to_string()),
        ("numplate.dat", "; c\n1 2 3\n4 5 6\n".to_string()),
        ("stockshake.txt", format!("{}\n", row(rng, 10))),
        ("traincamnodes.txt", format!("cama {}\n", row(rng, 4))),
        ("stream.ini", "# c\nKEY_A=2\n".to_string()),
        ("images.txt", "# c\nIMGLIST common:/data/a.txt\nIDE common:/data/b.ide\n".to_string()),
        ("frontend_menus.xml", random_xml(rng, 4).0),
        ("version.txt", "[VERSION]\n1.0\n".to_string()),
        ("fonts.dat", "[RESOLUTION]\n1,2\n[MAP]\n1 2 3\n".to_string()),
        ("unknown.dat", "# c\n\nGROUP: A\nITEM: a 1 0 100 0\n".to_string()),
    ]
}

#[test]
fn fuzz_every_routed_parser() {
    let mut rng = Rng::for_test("gamedata fuzz seeds");
    let all = seeds(&mut rng);
    // The seeds themselves must reach their intended parser.
    let bad: Vec<String> = all
        .iter()
        .filter_map(|(name, text)| {
            parse_file(name, text.as_bytes())
                .err()
                .map(|e| format!("{name}: {e}"))
        })
        .collect();
    assert!(bad.is_empty(), "seeds that do not parse: {bad:#?}");
    for (name, text) in &all {
        fuzz(
            &format!("gamedata {name}"),
            &[text.clone().into_bytes()],
            300,
            |b| {
                if let Ok(p) = parse_file(name, b) {
                    let _ = (p.parser(), p.counts());
                }
            },
        );
    }
}

#[test]
fn regression_deeply_nested_xml_is_an_error() {
    // Elements are parsed recursively; twenty thousand nested elements used
    // to overflow the stack and abort the process.
    let depth = 20_000;
    let text = format!("{}{}", "<a>".repeat(depth), "</a>".repeat(depth));
    assert!(parse_file("leaderboards_data.xml", text.as_bytes()).is_err());
}
