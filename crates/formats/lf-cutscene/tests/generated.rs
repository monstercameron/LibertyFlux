//! Property and fuzz tests on generated `.cut` files. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); all names are
//! invented. No game files are needed.
//!
//! - Property: random files (one or more cutscenes, each with frame
//!   headers, flags, subtitles, draw distances, player starts and one or
//!   more sections with models, animation, audio, camera and duration, with
//!   LF or CRLF line ends and zero slack after the text) parse back to the
//!   same values with no warnings; catalog rows and their JSON agree with
//!   the parsed file.
//! - Fuzz: mutated files never panic in parsing or cataloguing.

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

use lf_cutscene::catalog::{CatalogRow, ParsedCut, catalog_json, catalog_rows};
use lf_cutscene::cut::CutsceneFile;
use support::{Rng, fuzz};

#[derive(Debug, Clone)]
struct SectionSpec {
    models: Vec<(i32, String, String)>,
    anim: String,
    audio: String,
    camera: String,
    duration: f32,
}

#[derive(Debug, Clone)]
struct GroupSpec {
    frames: Vec<i32>,
    flags: Vec<String>,
    texts: Vec<(i32, i32, String)>,
    draw: Vec<(i32, i32, f32)>,
    start: [f32; 3],
    sections: Vec<SectionSpec>,
}

/// A number with an exact short decimal form.
fn num(rng: &mut Rng) -> f32 {
    rng.below(100_000) as f32 / 8.0 - 5000.0
}

fn random_groups(rng: &mut Rng) -> Vec<GroupSpec> {
    (0..rng.range(1, 3))
        .map(|_| {
            let sections: Vec<SectionSpec> = (0..rng.range(1, 3))
                .map(|s| SectionSpec {
                    models: (0..rng.range(1, 4))
                        .map(|m| {
                            (
                                m as i32,
                                format!("model_{}", rng.ident(1, 8)),
                                format!("anim_{s}_{m}"),
                            )
                        })
                        .collect(),
                    anim: format!("wad_{}", rng.ident(1, 8).to_lowercase()),
                    audio: format!("AUD_{}", rng.ident(1, 6).to_uppercase()),
                    camera: format!("cam_{s}"),
                    duration: rng.below(100_000) as f32 / 4.0,
                })
                .collect();
            let mut frame = 0;
            let frames = (0..=sections.len())
                .map(|_| {
                    frame += rng.range(1, 500) as i32;
                    frame
                })
                .collect();
            GroupSpec {
                frames,
                flags: (0..rng.below(3)).map(|i| format!("FLAG_{i}")).collect(),
                texts: (0..rng.below(4))
                    .map(|i| {
                        (
                            rng.below(10_000) as i32,
                            rng.range(1, 5000) as i32,
                            format!("KEY_{i}"),
                        )
                    })
                    .collect(),
                draw: (0..rng.below(3))
                    .map(|i| (rng.below(10_000) as i32, i as i32, num(rng)))
                    .collect(),
                start: [num(rng), num(rng), num(rng)],
                sections,
            }
        })
        .collect()
}

fn write_cut(groups: &[GroupSpec], crlf: bool, slack: usize) -> Vec<u8> {
    let nl = if crlf { "\r\n" } else { "\n" };
    let mut s = String::new();
    let tag = |s: &mut String, name: &str, rows: &[String]| {
        s.push_str(&format!("[{name}]{nl}"));
        for r in rows {
            s.push_str(r);
            s.push_str(nl);
        }
        s.push_str(&format!("[/{name}]{nl}{nl}"));
    };
    for g in groups {
        let frames: Vec<String> = g.frames.iter().map(ToString::to_string).collect();
        tag(&mut s, "CUTSCENE_HEADER", &[frames.join("\t")]);
        tag(&mut s, "FLAGS", &g.flags);
        let texts: Vec<String> = g
            .texts
            .iter()
            .map(|(a, b, k)| format!("{a}\t{b}\t{k}"))
            .collect();
        tag(&mut s, "TEXT", &texts);
        let draw: Vec<String> = g
            .draw
            .iter()
            .map(|(a, b, d)| format!("{a} {b} {d}"))
            .collect();
        tag(&mut s, "DRAW_DISTANCE", &draw);
        tag(
            &mut s,
            "PLAYER_START",
            &[format!("{} {} {}", g.start[0], g.start[1], g.start[2])],
        );
        for sec in &g.sections {
            s.push_str(&format!("[SECTION_START]{nl}"));
            let models: Vec<String> = sec
                .models
                .iter()
                .map(|(i, m, a)| format!("{i} {m} {a}"))
                .collect();
            tag(&mut s, "MODELS", &models);
            tag(&mut s, "DURATION", &[format!("{}", sec.duration)]);
            tag(&mut s, "AUDIO", std::slice::from_ref(&sec.audio));
            tag(&mut s, "ANIM", std::slice::from_ref(&sec.anim));
            tag(&mut s, "CAMERA", std::slice::from_ref(&sec.camera));
            s.push_str(&format!("[SECTION_END]{nl}"));
        }
    }
    let mut bytes = s.into_bytes();
    bytes.resize(bytes.len() + slack, 0);
    bytes
}

#[test]
fn cut_round_trip() {
    let mut rng = Rng::for_test("cutscene round trip");
    for case in 0..80 {
        let groups = random_groups(&mut rng);
        let slack = if case % 3 == 0 { rng.below(2048) } else { 0 };
        let bytes = write_cut(&groups, case % 2 == 0, slack);
        let file = CutsceneFile::parse(&bytes).expect("generated cut parses");
        assert!(file.warnings.is_empty(), "{:?}", file.warnings);
        assert!(file.orphans.is_empty(), "{:?}", file.orphans);
        assert_eq!(file.trailing_slack_len, slack);
        assert_eq!(file.groups.len(), groups.len());
        for (g, want) in file.groups.iter().zip(&groups) {
            assert_eq!(g.header_frames, want.frames);
            assert_eq!(g.flags, want.flags);
            let texts: Vec<(i32, i32, String)> = g
                .texts
                .iter()
                .map(|t| (t.start_ms, t.len_ms, t.key.clone()))
                .collect();
            assert_eq!(texts, want.texts);
            let draw: Vec<(i32, i32, f32)> = g
                .draw_distance
                .iter()
                .map(|d| (d.time_ms, d.second, d.distance))
                .collect();
            assert_eq!(draw, want.draw);
            assert_eq!(g.player_starts, vec![want.start]);
            assert!(g.bad_rows.is_empty() && g.unknown.is_empty());
            assert_eq!(g.sections.len(), want.sections.len());
            for (s, w) in g.sections.iter().zip(&want.sections) {
                let models: Vec<(i32, String, String)> = s
                    .models
                    .iter()
                    .map(|m| (m.id, m.model.clone(), m.anim.clone()))
                    .collect();
                assert_eq!(models, w.models);
                assert_eq!(s.anims, vec![w.anim.clone()]);
                assert_eq!(s.audios, vec![w.audio.clone()]);
                assert_eq!(s.cameras, vec![w.camera.clone()]);
                assert_eq!(s.durations_ms, vec![w.duration]);
                assert!(s.bad_rows.is_empty() && s.unknown.is_empty());
            }
            let total: f32 = want.sections.iter().map(|s| s.duration).sum();
            assert!((g.duration_ms() - total).abs() <= total * 1e-6);
        }
        // Catalog rows mirror the parsed file.
        let parsed = vec![ParsedCut {
            archive: "cuts.img".into(),
            episode: "base".into(),
            name: "gen.cut".into(),
            size: bytes.len() as u64,
            file,
        }];
        let rows: Vec<CatalogRow> = catalog_rows(&parsed);
        assert_eq!(rows.len(), groups.len());
        for (r, g) in rows.iter().zip(&groups) {
            assert_eq!(r.sections, g.sections.len());
            assert_eq!(r.subtitles, g.texts.len());
        }
        let json = catalog_json(&rows);
        assert_eq!(json.matches("\"file\"").count(), rows.len());
    }
}

#[test]
fn fuzz_cut_files() {
    let mut rng = Rng::for_test("cutscene fuzz");
    let seeds: Vec<Vec<u8>> = (0..4)
        .map(|i| write_cut(&random_groups(&mut rng), i % 2 == 0, 16))
        .collect();
    fuzz("cut", &seeds, 3000, |b| {
        if let Ok(file) = CutsceneFile::parse(b) {
            assert!(file.trailing_slack_len <= b.len());
            for g in &file.groups {
                let _ = (g.duration_ms(), g.model_count());
            }
            let parsed = vec![ParsedCut {
                archive: String::new(),
                episode: String::new(),
                name: "f.cut".into(),
                size: b.len() as u64,
                file,
            }];
            let _ = catalog_json(&catalog_rows(&parsed));
        }
    });
}

#[test]
fn regression_crlf_slack_excludes_the_carriage_return() {
    // The CR of the final CRLF used to be counted as one byte of slack.
    let bytes = b"[CUTSCENE_HEADER]\r\n1 2\r\n[/CUTSCENE_HEADER]\r\n\0\0\0";
    let file = CutsceneFile::parse(bytes).unwrap();
    assert_eq!(file.trailing_slack_len, 3);
}
