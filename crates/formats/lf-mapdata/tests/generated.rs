//! Property and fuzz tests on generated map files. Every byte is generated
//! here (see `crates/formats/tests/support.rs`); names and numbers are
//! random. No game files are needed.
//!
//! - Property: random binary `.wpl` files (every record kind, in the
//!   on-disk section order) parse back to equal records; random `.ide`
//!   texts (`objs`, `txdp`, `2dfx`), `.ipl` texts (`cull`, `vnod`, `link`)
//!   and load lists parse back to the rows written, with no errors.
//! - Fuzz: mutated `.wpl` files and mutated texts never panic, and a
//!   successful `.wpl` parse accounts for every input byte.

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

use lf_mapdata::ide::{IdeFile, ObjRecord};
use lf_mapdata::ipl::IplFile;
use lf_mapdata::loadlist::LoadList;
use lf_mapdata::wpl::{self, Blok, Car, Grge, Inst, Lodm, Mlop, Slow, Tcyc, WplFile};
use support::{Buf, Rng, fuzz};

fn f3(rng: &mut Rng) -> [f32; 3] {
    [
        rng.f32_in(-3000.0, 3000.0),
        rng.f32_in(-3000.0, 3000.0),
        rng.f32_in(-50.0, 300.0),
    ]
}

fn fixed(b: &mut Buf, s: &str, n: usize) {
    let mut raw = vec![0u8; n];
    raw[..s.len()].copy_from_slice(s.as_bytes());
    b.bytes(&raw);
}

fn floats(b: &mut Buf, v: &[f32]) {
    for x in v {
        b.f32(*x);
    }
}

/// A random `.wpl` file and the records it must parse to.
#[allow(clippy::too_many_lines)]
fn random_wpl(rng: &mut Rng) -> (Vec<u8>, WplFile) {
    let mut w = WplFile::default();
    for _ in 0..rng.below(6) {
        let p = f3(rng);
        let r = [
            rng.f32_in(-1.0, 1.0),
            rng.f32_in(-1.0, 1.0),
            rng.f32_in(-1.0, 1.0),
            rng.f32_in(-1.0, 1.0),
        ];
        w.inst.push(Inst {
            pos: p,
            rot: r,
            model_hash: rng.next_u32(),
            flags: rng.next_u32(),
            lod_index: rng.next_u32() as i32,
            unknown: rng.next_u32(),
            unknown_float: rng.f32_in(0.0, 1.0),
        });
    }
    for _ in 0..rng.below(3) {
        w.grge.push(Grge {
            corner_a: f3(rng),
            front: [rng.f32_in(-1.0, 1.0), rng.f32_in(-1.0, 1.0)],
            corner_b: f3(rng),
            door_type: rng.below(4) as u32,
            garage_type: rng.below(4) as u32,
            name: rng.ident(1, 7),
        });
    }
    for _ in 0..rng.below(3) {
        w.cars.push(Car {
            pos: f3(rng),
            unknown_a: rng.f32_in(0.0, 1.0),
            rot: [rng.f32_in(-1.0, 1.0), rng.f32_in(-1.0, 1.0)],
            model_hash: rng.next_u32(),
            color_a: rng.below(100) as i32,
            color_b: rng.below(100) as i32,
            color_c: rng.below(100) as i32,
            color_spec: rng.below(100) as i32,
            flags: rng.next_u32(),
            alarm: rng.below(100) as i32,
            unknown_b: rng.next_u32() as i32,
        });
    }
    for _ in 0..rng.below(3) {
        w.tcyc.push(Tcyc {
            corner_a: f3(rng),
            corner_b: f3(rng),
            unknown: [
                rng.next_u32(),
                rng.next_u32(),
                rng.next_u32(),
                rng.next_u32(),
            ],
            hash: rng.next_u32(),
        });
    }
    for _ in 0..rng.below(3) {
        w.mlop.push(Mlop {
            model: rng.ident(1, 23),
            flags: rng.next_u32(),
            interior_index: rng.below(10) as u32,
            unknown: rng.next_u32(),
            pos: f3(rng),
            rot: [0.0, 0.0, 0.0, 1.0],
        });
    }
    for _ in 0..rng.below(3) {
        let mut fl = [0.0; 8];
        for x in &mut fl {
            *x = rng.f32_in(-10.0, 10.0);
        }
        w.blok.push(Blok {
            unknown_a: rng.next_u32(),
            text: format!("{} {}", rng.ident(1, 20), rng.ident(1, 20)),
            unknown_b: rng.next_u32(),
            floats: fl,
        });
    }
    for _ in 0..rng.below(3) {
        let mut hashes = [0u32; 10];
        let mut names: [String; 10] = Default::default();
        let count = rng.range(1, 10);
        for i in 0..count {
            hashes[i] = rng.next_u32();
            names[i] = rng.ident(1, 31);
        }
        w.lodm.push(Lodm {
            corner_a: f3(rng),
            corner_b: f3(rng),
            count: count as u32,
            hashes,
            names,
        });
    }
    for _ in 0..rng.below(3) {
        w.slow.push(Slow {
            corner_a: f3(rng),
            corner_b: f3(rng),
        });
    }
    let mut counts = [0u32; 16];
    counts[0] = w.inst.len() as u32;
    counts[2] = w.grge.len() as u32;
    counts[3] = w.cars.len() as u32;
    counts[4] = w.tcyc.len() as u32;
    counts[8] = w.mlop.len() as u32;
    counts[15] = w.blok.len() as u32;
    counts[9] = w.lodm.len() as u32;
    counts[10] = w.slow.len() as u32;
    w.counts = counts;
    let mut b = Buf::new();
    b.u32(wpl::VERSION);
    for c in counts {
        b.u32(c);
    }
    for r in &w.inst {
        floats(&mut b, &r.pos);
        floats(&mut b, &r.rot);
        b.u32(r.model_hash)
            .u32(r.flags)
            .u32(r.lod_index as u32)
            .u32(r.unknown)
            .f32(r.unknown_float);
    }
    for r in &w.grge {
        floats(&mut b, &r.corner_a);
        floats(&mut b, &r.front);
        floats(&mut b, &r.corner_b);
        b.u32(r.door_type).u32(r.garage_type);
        fixed(&mut b, &r.name, 8);
    }
    for r in &w.cars {
        floats(&mut b, &r.pos);
        b.f32(r.unknown_a);
        floats(&mut b, &r.rot);
        b.u32(r.model_hash);
        for v in [r.color_a, r.color_b, r.color_c, r.color_spec] {
            b.u32(v as u32);
        }
        b.u32(r.flags).u32(r.alarm as u32).u32(r.unknown_b as u32);
    }
    for r in &w.tcyc {
        floats(&mut b, &r.corner_a);
        floats(&mut b, &r.corner_b);
        for v in r.unknown {
            b.u32(v);
        }
        b.u32(r.hash);
    }
    for r in &w.mlop {
        fixed(&mut b, &r.model, 24);
        b.u32(r.flags).u32(r.interior_index).u32(r.unknown);
        floats(&mut b, &r.pos);
        floats(&mut b, &r.rot);
    }
    for r in &w.blok {
        b.u32(r.unknown_a);
        fixed(&mut b, &r.text, 92);
        b.u32(r.unknown_b);
        floats(&mut b, &r.floats);
    }
    for r in &w.lodm {
        floats(&mut b, &r.corner_a);
        floats(&mut b, &r.corner_b);
        b.u32(r.count);
        for h in r.hashes {
            b.u32(h);
        }
        for n in &r.names {
            fixed(&mut b, n, 32);
        }
    }
    for r in &w.slow {
        floats(&mut b, &r.corner_a);
        floats(&mut b, &r.corner_b);
    }
    (b.0, w)
}

#[test]
fn wpl_round_trip() {
    let mut rng = Rng::for_test("mapdata wpl");
    for _ in 0..100 {
        let (bytes, want) = random_wpl(&mut rng);
        let got = WplFile::parse(&bytes).expect("generated wpl parses");
        assert_eq!(got.counts, want.counts);
        assert_eq!(got.inst, want.inst);
        assert_eq!(got.grge, want.grge);
        assert_eq!(got.cars, want.cars);
        assert_eq!(got.tcyc, want.tcyc);
        assert_eq!(got.mlop, want.mlop);
        assert_eq!(got.blok, want.blok);
        assert_eq!(got.lodm, want.lodm);
        assert_eq!(got.slow, want.slow);
        assert!(got.trailing_bytes.is_empty());
        assert_eq!(got.len(), want.len());
    }
}

#[test]
fn ide_ipl_loadlist_round_trip() {
    let mut rng = Rng::for_test("mapdata text");
    for _ in 0..80 {
        // IDE: full objs rows, txdp pairs, 2dfx rows.
        let objs: Vec<(String, String, u32)> = (0..rng.below(6))
            .map(|i| {
                (
                    format!("{}{i}", rng.ident(1, 10)),
                    rng.ident(1, 10),
                    rng.range(10, 500) as u32,
                )
            })
            .collect();
        let txdp: Vec<(String, String)> = (0..rng.below(4))
            .map(|i| (format!("c{i}"), format!("p{i}")))
            .collect();
        let mut text = String::from("# generated\nobjs\n");
        for (m, t, d) in &objs {
            text.push_str(&format!(
                "{m}, {t}, {d}, 0, 0, -1, -1, -1, 1, 1, 1, 0, 0, 0, 2, null\n"
            ));
        }
        text.push_str("end\ntxdp\n");
        for (c, p) in &txdp {
            text.push_str(&format!("{c}, {p}\n"));
        }
        text.push_str("end\n2dfx\nlamp, 1, 2, 3, 0, 9, 8, 7\nend\n");
        let f = IdeFile::parse(text.as_bytes()).expect("generated ide parses");
        assert!(f.errors.is_empty(), "{:?}", f.errors);
        assert_eq!(f.objs.len(), objs.len());
        for (o, (m, t, d)) in f.objs.iter().zip(&objs) {
            let ObjRecord::Full(o) = o else {
                panic!("full row parsed short")
            };
            assert_eq!(
                (&o.model, &o.texture_dict, o.draw_distance),
                (m, t, *d as f32)
            );
        }
        assert_eq!(
            f.txdp
                .iter()
                .map(|t| (t.child.clone(), t.parent.clone()))
                .collect::<Vec<_>>(),
            txdp
        );
        assert_eq!(f.fx.len(), 1);
        assert_eq!(f.len(), objs.len() + txdp.len() + 1);

        // IPL: cull, vnod and link rows.
        let (nc, nv, nl) = (rng.below(4), rng.below(4), rng.below(4));
        let mut text = String::from("inst\nend\ncull\n");
        for _ in 0..nc {
            text.push_str("1, 2, 3, 0, 4, 3, 5, 0, 6, 8, 0\n");
        }
        text.push_str("end\nvnod\n");
        for i in 0..nv {
            text.push_str(&format!("{i}, 2, 3, 0, 0, 0, 1, 0, 1, 5, 0, 0, 255\n"));
        }
        text.push_str("end\nlink\n");
        for i in 0..nl {
            text.push_str(&format!("{i}, 0, 0, 2, 0, 0\n"));
        }
        text.push_str("end\n");
        let f = IplFile::parse(text.as_bytes()).expect("generated ipl parses");
        assert!(f.errors.is_empty(), "{:?}", f.errors);
        assert_eq!((f.cull.len(), f.vnod.len(), f.link.len()), (nc, nv, nl));

        // Load list.
        let dirs: Vec<(String, String)> = (0..rng.below(6))
            .map(|i| {
                (
                    ["IMGLIST", "IDE", "IPL", "COLFILE"][i % 4].to_string(),
                    format!("common:/data/{}.txt", rng.ident(1, 9)),
                )
            })
            .collect();
        let text: String = dirs.iter().map(|(k, a)| format!("{k} {a}\n")).collect();
        let l = LoadList::parse(text.as_bytes()).expect("generated load list parses");
        let got: Vec<(String, Vec<String>)> = l
            .directives
            .iter()
            .map(|d| (d.keyword.clone(), d.args.clone()))
            .collect();
        let want: Vec<(String, Vec<String>)> =
            dirs.into_iter().map(|(k, a)| (k, vec![a])).collect();
        assert_eq!(got, want);
    }
}

#[test]
fn fuzz_wpl() {
    let mut rng = Rng::for_test("mapdata wpl fuzz");
    let seeds: Vec<Vec<u8>> = (0..4).map(|_| random_wpl(&mut rng).0).collect();
    fuzz("wpl", &seeds, 3000, |b| {
        if let Ok(f) = WplFile::parse(b) {
            let sizes = [
                (f.inst.len(), wpl::INST_SIZE),
                (f.grge.len(), wpl::GRGE_SIZE),
                (f.cars.len(), wpl::CARS_SIZE),
                (f.tcyc.len(), wpl::TCYC_SIZE),
                (f.mlop.len(), wpl::MLOP_SIZE),
                (f.blok.len(), wpl::BLOK_SIZE),
                (f.lodm.len(), wpl::LODM_SIZE),
                (f.slow.len(), wpl::SLOW_SIZE),
            ];
            let used: usize = sizes.iter().map(|(n, s)| n * s).sum::<usize>() + wpl::HEADER_SIZE;
            assert_eq!(used + f.trailing_bytes.len(), b.len());
            let _ = f.trailing_is_zero_padding();
        }
    });
}

#[test]
fn fuzz_text_formats() {
    let seeds: Vec<Vec<u8>> = vec![
        b"# c\nobjs\nmyprop, mytxd, 100, 0, 0, -1, -1, -1, 1, 1, 1, 0, 0, 0, 2, null\ntiny, gen, 50, 0\nend\ntobj\ntprop, ttxd, 30, 8, 0, -1, -1, -1, 1, 1, 1, 0, 0, 0, 2, null, 7\nend\ncars\nmycar, mycar, car, MYCAR, MYCAR, VEH@STD, NULL, 100, 999, 0.3, 0.3, 0, 5, 1.0, 0 noboot\nend\nmlo\nRoom, 1, 2, 3, 4, 5, 6, 7\nmloroomstart\nchair, 1, 2, 3, 0, 0, 0, 1, 0, 384,\nroomend\nmloend\nend\n2dfx\nlamp, 1, 2, 3, 0, 9, 8, 7\nend\ntree\nend\n".to_vec(),
        b"inst\nend\nblok\nArea, me, 2007:00:00:00:00:00, 128, 0, x\nend\ncull\n1, 2, 3, 0, 4, 3, 5, 0, 6, 8, 0\nend\noccl\n1, 2, 3, 0.1, 0.2, 0.3, 0, 0, 0, 0\nend\nvnod\n1, 2, 3, 0, 0, 0, 1, 0, 1, 5, 0, 0, 255\nend\nlink\n1, 0, 0, 2, 0, 0\nend\n2dfx\nworld, 1, 2, 3, 13, 0, 0, 0, -1, fx, 0\nend\n".to_vec(),
        b"# c\nIMGLIST common:/data/a.txt\nIDE common:/data/b.ide\nIPL platform:/data/maps/c.wpl\n".to_vec(),
    ];
    fuzz("mapdata text", &seeds, 4000, |b| {
        let _ = IdeFile::parse(b).map(|f| f.len());
        let _ = IplFile::parse(b).map(|f| f.len());
        let _ = LoadList::parse(b);
        let _ = lf_mapdata::loadlist::ImagesList::parse(b);
    });
}
