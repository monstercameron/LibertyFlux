//! Property and fuzz tests on generated text, font and front-end files.
//! Every byte is generated here (see `crates/formats/tests/support.rs`); no
//! game files are needed.
//!
//! - Property: random GXT databases (8- and 16-bit units, MAIN plus named
//!   tables, labels hashed with [`label_hash`]) parse back to the same
//!   tables, hashes and glyph codes, and lookups by label find every
//!   string. Random `fonts.dat`, `hud.dat`, `hudColor.dat`, frontend
//!   layouts, both `radiohud.dat` variants and `frontend_menus.xml` texts
//!   parse back to the values written.
//! - Fuzz: mutated GXT files and mutated texts never panic.

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

use lf_text::{
    FontFile, FrontendLayout, GxtFile, HudColours, HudFile, MenuFile, RadioHudFile,
    data_file_lines, decode_western_lossy, label_hash,
};
use support::{Buf, Rng, fuzz};

/// One generated GXT table: name and (label, glyph codes) pairs.
type Table = (String, Vec<(String, Vec<u16>)>);

fn random_gxt(rng: &mut Rng, bits: u16) -> Vec<Table> {
    let mut tables = vec![("MAIN".to_string(), Vec::new())];
    for i in 0..rng.below(4) {
        tables.push((
            format!("T{}{i}", rng.ident(1, 5).to_uppercase()),
            Vec::new(),
        ));
    }
    for (_, entries) in &mut tables {
        for i in 0..rng.below(8) {
            let len = rng.below(20);
            let max = if bits == 8 { 0xFF } else { 0xFFFF };
            let text: Vec<u16> = (0..len)
                .map(|_| 1 + (rng.next_u32() % max) as u16)
                .collect();
            entries.push((format!("{}_{i}", rng.ident(1, 7).to_uppercase()), text));
        }
    }
    tables
}

fn write_gxt(tables: &[Table], bits: u16) -> Vec<u8> {
    let mut b = Buf::new();
    b.u16(4)
        .u16(bits)
        .bytes(b"TABL")
        .u32((tables.len() * 12) as u32);
    let dir = b.len();
    b.put(dir + tables.len() * 12 - 1, &[0]);
    for (ti, (name, entries)) in tables.iter().enumerate() {
        let mut nm = [0u8; 8];
        nm[..name.len()].copy_from_slice(name.as_bytes());
        b.put(dir + ti * 12, &nm);
        b.put_u32(dir + ti * 12 + 8, b.len() as u32);
        if ti > 0 {
            b.bytes(&nm);
        }
        let mut strings = Buf::new();
        let mut keys = Buf::new();
        for (label, text) in entries {
            keys.u32(strings.len() as u32).u32(label_hash(label));
            for &u in text {
                if bits == 8 {
                    strings.u8(u as u8);
                } else {
                    strings.u16(u);
                }
            }
            if bits == 8 {
                strings.u8(0);
            } else {
                strings.u16(0);
            }
        }
        b.bytes(b"TKEY").u32(keys.len() as u32).bytes(&keys.0);
        b.bytes(b"TDAT").u32(strings.len() as u32).bytes(&strings.0);
    }
    b.0
}

#[test]
fn gxt_round_trip() {
    let mut rng = Rng::for_test("text gxt");
    for case in 0..100 {
        let bits = if case % 4 == 3 { 8 } else { 16 };
        let tables = random_gxt(&mut rng, bits);
        let file = GxtFile::parse(&write_gxt(&tables, bits)).expect("generated GXT parses");
        assert_eq!(file.version(), 4);
        assert_eq!(file.bits_per_char(), bits);
        assert_eq!(file.tables().len(), tables.len());
        let mut count = 0;
        for (t, (name, entries)) in file.tables().iter().zip(&tables) {
            assert_eq!(&t.name, name);
            assert_eq!(t.entries.len(), entries.len());
            for (e, (label, text)) in t.entries.iter().zip(entries) {
                assert_eq!(e.hash, label_hash(label));
                assert_eq!(&e.text, text);
                assert_eq!(file.lookup(name, label), Some(text.as_slice()));
                assert_eq!(
                    file.lookup(name, &label.to_lowercase()),
                    Some(text.as_slice())
                );
                let _ = decode_western_lossy(&e.text);
                count += 1;
            }
        }
        assert_eq!(file.entry_count(), count);
        assert_eq!(file.iter_entries().count(), count);
    }
}

/// A font description, written and expected back.
fn random_fonts(rng: &mut Rng) -> (String, Vec<(u32, Vec<u16>, Vec<i16>)>, (u32, u32)) {
    let res = (rng.range(100, 2000) as u32, rng.range(100, 2000) as u32);
    let mut s = format!(
        "# generated\n[RESOLUTION]\n{},{}\n[BUTTONS]\n",
        res.0, res.1
    );
    for i in 0..rng.below(5) {
        s.push_str(&format!("{} # FO_SLOT_{i}\n", rng.below(64)));
    }
    s.push_str(&format!("[RADAR_BLIP]\n{} # SIZE\n", rng.below(64)));
    let mut fonts = Vec::new();
    for id in 0..rng.range(1, 3) as u32 {
        let n = rng.range(1, 30);
        let map: Vec<u16> = (0..n).map(|_| rng.below(0x3000) as u16).collect();
        let prop: Vec<i16> = (0..rng.range(1, n))
            .map(|_| rng.below(60) as i16 - 5)
            .collect();
        let join = |v: &[String]| v.join(" ");
        s.push_str(&format!(
            "[FONT_ID]\n{id}\n[MAP]\n{}\n[/MAP]\n[MAINFONT]\n0 {n}\n[SUBFONT_1]\n0 0\n[SUBFONT_2]\n0 0\n[COMMON_FONT]\n0 0\n[PROP]\n{}\n[/PROP]\n[UNPROP]\n26\n[SPACE_BETWEEN_CHARS]\n0 -2 1\n[WHITESPACE]\n8\n",
            join(&map.iter().map(ToString::to_string).collect::<Vec<_>>()),
            join(&prop.iter().map(ToString::to_string).collect::<Vec<_>>()),
        ));
        fonts.push((id, map, prop));
    }
    (s, fonts, res)
}

#[test]
fn fonts_round_trip() {
    let mut rng = Rng::for_test("text fonts");
    for _ in 0..60 {
        let (text, fonts, res) = random_fonts(&mut rng);
        let file = FontFile::parse(text.as_bytes()).expect("generated fonts.dat parses");
        assert_eq!(file.resolution, res);
        assert_eq!(file.fonts.len(), fonts.len());
        for (f, (id, map, prop)) in file.fonts.iter().zip(&fonts) {
            assert_eq!(f.id, *id);
            assert_eq!(&f.map, map);
            assert_eq!(&f.prop, prop);
            assert_eq!(f.main, (0, map.len() as u32));
            assert_eq!((f.unprop, f.spacing, f.whitespace), (26, (0, -2, 1), 8));
            assert!(file.font(*id).is_some());
        }
    }
}

/// A number with an exact shortest decimal form.
fn num(rng: &mut Rng) -> f32 {
    rng.below(20_000) as f32 / 1000.0 - 5.0
}

#[test]
fn hud_files_round_trip() {
    let mut rng = Rng::for_test("text hud");
    for _ in 0..60 {
        let mut hud = String::new();
        let mut colours = String::new();
        let mut layout = String::new();
        let mut want_items = Vec::new();
        let mut want_colours = Vec::new();
        let mut want_layout = Vec::new();
        for section in ["HD", "CRT"].iter().take(rng.range(1, 2)) {
            hud.push_str(&format!("[{section}]\n"));
            colours.push_str(&format!("[{section}]\n"));
            layout.push_str(&format!("# c\n[{section}]\n"));
            for i in 0..rng.below(6) {
                let name = format!("HUD_{}{i}", rng.ident(1, 8).to_uppercase());
                let (p, s) = (
                    (num(&mut rng), num(&mut rng)),
                    (num(&mut rng), num(&mut rng)),
                );
                let tail = rng
                    .chance(1, 2)
                    .then(|| (format!("HUD_COLOUR_{i}"), rng.below(256) as u32));
                match &tail {
                    Some((c, a)) => {
                        hud.push_str(&format!("{name} {},{} {},{} {c} {a}\n", p.0, p.1, s.0, s.1));
                    }
                    None => hud.push_str(&format!("{name} {},{} {},{}\n", p.0, p.1, s.0, s.1)),
                }
                want_items.push((name, p, s, tail));
                let rgb = (
                    rng.below(256) as u8,
                    rng.below(256) as u8,
                    rng.below(256) as u8,
                );
                colours.push_str(&format!("HUD_COLOUR_{i} {} {} {}\n", rgb.0, rgb.1, rgb.2));
                want_colours.push(rgb);
                let nums: Vec<f32> = (0..rng.range(1, 4)).map(|_| num(&mut rng)).collect();
                let text: Vec<String> = nums.iter().map(ToString::to_string).collect();
                layout.push_str(&format!("KEY_{i} {}\n", text.join(" ")));
                want_layout.push(nums);
            }
        }
        let h = HudFile::parse(hud.as_bytes()).expect("generated hud.dat parses");
        let items: Vec<_> = h.sections.iter().flat_map(|s| &s.items).collect();
        assert_eq!(items.len(), want_items.len());
        for (it, (name, p, s, tail)) in items.iter().zip(&want_items) {
            assert_eq!(&it.name, name);
            assert_eq!((it.position, it.size), (*p, *s));
            assert_eq!(it.colour.as_ref(), tail.as_ref().map(|t| &t.0));
            assert_eq!(it.alpha, tail.as_ref().map(|t| t.1));
        }
        let c = HudColours::parse(colours.as_bytes()).expect("generated hudColor.dat parses");
        let got: Vec<_> = c
            .sections
            .iter()
            .flat_map(|s| &s.colours)
            .map(|c| (c.rgb.r, c.rgb.g, c.rgb.b))
            .collect();
        assert_eq!(got, want_colours);
        let l = FrontendLayout::parse(layout.as_bytes()).expect("generated layout parses");
        let got: Vec<_> = l
            .sections
            .iter()
            .flat_map(|s| &s.values)
            .map(|v| v.numbers.clone())
            .collect();
        assert_eq!(got, want_layout);
        assert!(data_file_lines(layout.as_bytes()).is_ok());
    }
}

#[test]
fn radiohud_round_trip() {
    let mut rng = Rng::for_test("text radiohud");
    for case in 0..60 {
        let n = rng.range(1, 6);
        if case % 2 == 0 {
            let mut s = String::from("# c\n+\n");
            let containers = rng.range(1, 3);
            for i in 0..containers {
                s.push_str(&format!("platform:/textures/c{i} {}\n", i % 2));
            }
            s.push_str("+\n");
            let mut want = Vec::new();
            for i in 0..n {
                let w = rng.below(200) as u32;
                let (a, b) = (num(&mut rng), num(&mut rng));
                s.push_str(&format!("RADIO_{i} NAME_{i} bw{i} col{i} {w} {a} {b}\n"));
                want.push((w, a, b));
            }
            let RadioHudFile::Full(f) = RadioHudFile::parse(s.as_bytes()).expect("parses") else {
                panic!("marker variant detected as simple");
            };
            assert_eq!(f.containers.len(), containers);
            let got: Vec<_> = f
                .stations
                .iter()
                .map(|st| (st.visible_width, st.y_hd, st.y_sd))
                .collect();
            assert_eq!(got, want);
        } else {
            let mut s = String::new();
            for i in 0..n {
                s.push_str(&format!("st{i} bw{i} col{i} cont{i}\n"));
            }
            let RadioHudFile::Simple(rows) = RadioHudFile::parse(s.as_bytes()).expect("parses")
            else {
                panic!("simple variant detected as marker");
            };
            assert_eq!(rows.len(), n);
            assert_eq!(rows[n - 1].container, format!("cont{}", n - 1));
        }
    }
}

fn random_menus(rng: &mut Rng) -> (String, Vec<Vec<Vec<String>>>) {
    let mut s = String::from("<FrontendMenu version=\"1\">\n<!-- generated -->\n");
    let mut want = Vec::new();
    for si in 0..rng.range(1, 3) {
        s.push_str(&format!("<sSection{si}>\n"));
        let mut menus = Vec::new();
        for mi in 0..rng.below(4) {
            s.push_str(&format!("<menu enum=\"MENU_{si}_{mi}\">\n"));
            let labels: Vec<String> = (0..rng.below(4))
                .map(|oi| format!("L_{si}_{mi}_{oi}"))
                .collect();
            for l in &labels {
                s.push_str(&format!(
                    "<options text=\"{l}\" action=\"ACTION_NONE\" value=\"0\"/>\n"
                ));
            }
            s.push_str("</menu>\n");
            menus.push(labels);
        }
        s.push_str(&format!("</sSection{si}>\n"));
        want.push(menus);
    }
    s.push_str("</FrontendMenu>\n");
    (s, want)
}

#[test]
fn menus_round_trip() {
    let mut rng = Rng::for_test("text menus");
    for _ in 0..60 {
        let (text, want) = random_menus(&mut rng);
        let file = MenuFile::parse(text.as_bytes()).expect("generated menus parse");
        assert_eq!(file.version, "1");
        assert_eq!(file.sections.len(), want.len());
        for (sec, wm) in file.sections.iter().zip(&want) {
            assert_eq!(sec.menus.len(), wm.len());
            for (m, labels) in sec.menus.iter().zip(wm) {
                let got: Vec<&str> = m.options.iter().filter_map(|o| o.text.as_deref()).collect();
                assert_eq!(got, labels.iter().map(String::as_str).collect::<Vec<_>>());
            }
        }
        let all: usize = want.iter().flatten().map(Vec::len).sum();
        assert_eq!(file.iter_labels().count(), all);
    }
}

#[test]
fn fuzz_gxt() {
    let mut rng = Rng::for_test("text gxt fuzz");
    let seeds: Vec<Vec<u8>> = (0..4)
        .map(|i| {
            let bits = if i == 3 { 8 } else { 16 };
            write_gxt(&random_gxt(&mut rng, bits), bits)
        })
        .collect();
    fuzz("gxt", &seeds, 3000, |b| {
        if let Ok(f) = GxtFile::parse(b) {
            for (_, _, text) in f.iter_entries() {
                let _ = decode_western_lossy(text);
            }
            let _ = f.lookup("MAIN", "X");
        }
    });
}

#[test]
fn fuzz_text_files() {
    let mut rng = Rng::for_test("text files fuzz");
    let seeds: Vec<Vec<u8>> = vec![
        random_fonts(&mut rng).0.into_bytes(),
        b"[HD]\nHUD_RADAR 0.064,0.745 0.161,0.211 HUD_COLOUR_WHITE 150\nHUD_ICON 0.0,0.0 0.046,0.060\n".to_vec(),
        b"[HD]\nHUD_COLOUR_RED 153 69 69\n[CRT]\nHUD_COLOUR_BLUE 1 2 3\n".to_vec(),
        b"# c\n[HD]\nMID_background_opacity 160.0 0.0\nFAD_main_fade_time 200\n".to_vec(),
        b"# c\n+\nplatform:/textures/a 1\n+\nRADIO_0 BEAT_95 beat_bw beat_col 74 -0.004 -0.028\n".to_vec(),
        random_menus(&mut rng).0.into_bytes(),
    ];
    fuzz("text files", &seeds, 4000, |b| {
        let _ = FontFile::parse(b).map(|f| f.font(0).is_some());
        let _ = HudFile::parse(b);
        let _ = HudColours::parse(b);
        let _ = FrontendLayout::parse(b);
        let _ = RadioHudFile::parse(b);
        let _ = MenuFile::parse(b).map(|m| m.iter_labels().count());
        let _ = data_file_lines(b);
    });
}

#[test]
fn regression_gxt_truncated_before_key_block_is_an_error() {
    // A directory entry pointing past the end of the file: building the
    // "bad tag" error used to slice out of range.
    let mut b = Buf::new();
    b.u16(4)
        .u16(16)
        .bytes(b"TABL")
        .u32(12)
        .bytes(b"MAIN\0\0\0\0")
        .u32(56);
    assert_eq!(b.len(), 24);
    assert!(GxtFile::parse(&b.0).is_err());
}
