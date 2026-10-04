//! Property and fuzz tests on generated shader packs. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); the bytecode
//! blobs are synthetic token streams, not compiled shaders. No game files
//! are needed.
//!
//! - Property: random packs (vertex and pixel programs with bound names and
//!   valid token streams, shared and material parameters with all three
//!   annotation kinds and default dwords, techniques with passes and render
//!   states) parse back to the same fields with no trailing bytes, and
//!   every blob validates with the instruction count it was built with.
//!   Random `.dcl` and `.sps` texts parse back to the values written.
//! - Fuzz: mutated packs, blobs and texts never panic.

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

use lf_shaderpack::{AnnotationValue, ShaderPack, bytecode, dcl, sps};
use support::{Buf, Rng, fuzz};

/// Annotation value kinds on disk.
const ANN_INT: u8 = 0;
const ANN_FLOAT: u8 = 1;
const ANN_TEXT: u8 = 2;

fn sized(b: &mut Buf, s: &str) {
    b.u8((s.len() + 1) as u8).bytes(s.as_bytes()).u8(0);
}

/// A synthetic token stream: version, `n` instructions with up to three
/// parameter dwords, an optional comment block, then the end token.
fn blob(rng: &mut Rng, pixel: bool, n: usize) -> Vec<u8> {
    let mut b = Buf::new();
    b.u32(if pixel { 0xFFFF_0300 } else { 0xFFFE_0300 });
    if rng.chance(1, 2) {
        b.u32(0x0002_FFFE).bytes(b"CTAB").u32(rng.next_u32());
    }
    for _ in 0..n {
        let params = rng.below(4) as u32;
        b.u32((params << 24) | (rng.below(0x60) as u32 + 1));
        for _ in 0..params {
            b.u32(rng.next_u32() & 0x7FFF_FFFF);
        }
    }
    b.u32(bytecode::END);
    b.0
}

#[derive(Debug, Clone)]
struct ProgSpec {
    vars: Vec<(u8, u8, u16, String)>,
    instrs: usize,
    blob: Vec<u8>,
}

#[derive(Debug, Clone)]
struct ParamSpec {
    type_code: u8,
    slot: u8,
    name: String,
    semantic: String,
    annotations: Vec<(String, AnnotationValue)>,
    defaults: Vec<u32>,
}

#[derive(Debug, Clone)]
struct PackSpec {
    vs: Vec<ProgSpec>,
    ps: Vec<ProgSpec>,
    padding: [u8; 5],
    shared: Vec<ParamSpec>,
    material: Vec<ParamSpec>,
    /// Techniques: name, passes of (vs, ps (None = no pixel program), states).
    techniques: Vec<(String, Vec<(usize, Option<usize>, Vec<(u32, u32)>)>)>,
}

fn random_prog(rng: &mut Rng, pixel: bool) -> ProgSpec {
    let instrs = rng.below(10);
    ProgSpec {
        vars: (0..rng.below(4))
            .map(|_| {
                (
                    rng.below(16) as u8,
                    rng.below(4) as u8,
                    rng.below(256) as u16,
                    rng.ident(1, 20),
                )
            })
            .collect(),
        instrs,
        blob: blob(rng, pixel, instrs),
    }
}

fn random_param(rng: &mut Rng) -> ParamSpec {
    ParamSpec {
        type_code: rng.below(10) as u8,
        slot: rng.below(3) as u8,
        name: rng.ident(1, 20),
        semantic: rng.ident(1, 20),
        annotations: (0..rng.below(3))
            .map(|_| {
                let v = match rng.below(3) {
                    0 => AnnotationValue::Int(rng.next_u32()),
                    1 => AnnotationValue::Float(rng.f32_in(-10.0, 10.0)),
                    _ => AnnotationValue::Text(rng.ident(1, 30)),
                };
                (rng.ident(1, 12), v)
            })
            .collect(),
        defaults: (0..rng.below(9)).map(|_| rng.next_u32()).collect(),
    }
}

fn random_pack(rng: &mut Rng) -> PackSpec {
    let vs: Vec<ProgSpec> = (0..rng.range(1, 3))
        .map(|_| random_prog(rng, false))
        .collect();
    let ps: Vec<ProgSpec> = (0..rng.below(3)).map(|_| random_prog(rng, true)).collect();
    let techniques = (0..rng.below(3))
        .map(|_| {
            let passes = (0..rng.range(1, 3))
                .map(|_| {
                    let ps_index = if ps.is_empty() || rng.chance(1, 4) {
                        None
                    } else {
                        Some(rng.below(ps.len()))
                    };
                    let states = (0..rng.below(4))
                        .map(|_| (rng.below(200) as u32, rng.next_u32()))
                        .collect();
                    (rng.below(vs.len()), ps_index, states)
                })
                .collect();
            (rng.ident(1, 16), passes)
        })
        .collect();
    PackSpec {
        vs,
        ps,
        padding: [0; 5],
        shared: (0..rng.below(4)).map(|_| random_param(rng)).collect(),
        material: (0..rng.below(4)).map(|_| random_param(rng)).collect(),
        techniques,
    }
}

fn write_prog(b: &mut Buf, p: &ProgSpec) {
    b.u8(p.vars.len() as u8);
    for (t, i, r, n) in &p.vars {
        b.u8(*t).u8(*i).u16(*r);
        sized(b, n);
    }
    b.u16(p.blob.len() as u16)
        .u16(p.blob.len() as u16)
        .bytes(&p.blob);
}

fn write_param(b: &mut Buf, p: &ParamSpec) {
    b.u8(p.type_code).u8(p.slot);
    sized(b, &p.name);
    sized(b, &p.semantic);
    b.u8(p.annotations.len() as u8);
    for (n, v) in &p.annotations {
        sized(b, n);
        match v {
            AnnotationValue::Int(x) => {
                b.u8(ANN_INT).u32(*x);
            }
            AnnotationValue::Float(x) => {
                b.u8(ANN_FLOAT).f32(*x);
            }
            AnnotationValue::Text(s) => {
                b.u8(ANN_TEXT);
                sized(b, s);
            }
        }
    }
    b.u8(p.defaults.len() as u8);
    for d in &p.defaults {
        b.u32(*d);
    }
}

fn write_pack(p: &PackSpec) -> Vec<u8> {
    let mut b = Buf::new();
    b.u32(lf_shaderpack::MAGIC).u8(p.vs.len() as u8);
    for prog in &p.vs {
        write_prog(&mut b, prog);
    }
    // The stored pixel count is one more than the programs that follow.
    b.u8(if p.ps.is_empty() {
        0
    } else {
        p.ps.len() as u8 + 1
    })
    .bytes(&p.padding);
    for prog in &p.ps {
        write_prog(&mut b, prog);
    }
    for list in [&p.shared, &p.material] {
        b.u8(list.len() as u8);
        for param in list {
            write_param(&mut b, param);
        }
    }
    b.u8(p.techniques.len() as u8);
    for (name, passes) in &p.techniques {
        sized(&mut b, name);
        b.u8(passes.len() as u8);
        for (vs, ps, states) in passes {
            b.u8(*vs as u8)
                .u8(ps.map_or(0, |i| i as u8 + 1))
                .u8(states.len() as u8);
            for (id, v) in states {
                b.u32(*id).u32(*v);
            }
        }
    }
    b.0
}

fn check_prog(got: &lf_shaderpack::Program<'_>, want: &ProgSpec) {
    let vars: Vec<(u8, u8, u16, String)> = got
        .vars
        .iter()
        .map(|v| (v.type_code, v.index, v.register, v.name.clone()))
        .collect();
    assert_eq!(vars, want.vars);
    assert_eq!(got.bytecode, &want.blob[..]);
    assert_eq!(usize::from(got.declared_size), want.blob.len());
    let info = got.validate().expect("generated blob validates");
    assert_eq!(info.instr_count, want.instrs);
    assert_eq!(info.end_offset, want.blob.len());
}

fn check_param(got: &lf_shaderpack::Param, want: &ParamSpec) {
    assert_eq!((got.type_code, got.slot), (want.type_code, want.slot));
    assert_eq!((&got.name, &got.semantic), (&want.name, &want.semantic));
    let anns: Vec<(String, AnnotationValue)> = got
        .annotations
        .iter()
        .map(|a| (a.name.clone(), a.value.clone()))
        .collect();
    assert_eq!(anns, want.annotations);
    assert_eq!(got.defaults, want.defaults);
    let _ = (
        got.kind(),
        got.is_array(),
        got.defaults_f32(),
        got.sampler_states(),
    );
}

#[test]
fn pack_round_trip() {
    let mut rng = Rng::for_test("shaderpack round trip");
    for _ in 0..100 {
        let spec = random_pack(&mut rng);
        let bytes = write_pack(&spec);
        let pack = ShaderPack::parse(&bytes).expect("generated pack parses");
        assert!(pack.trailing_bytes.is_empty());
        assert_eq!(pack.vs_programs.len(), spec.vs.len());
        assert_eq!(pack.ps_programs.len(), spec.ps.len());
        for (g, w) in pack.vs_programs.iter().zip(&spec.vs) {
            check_prog(g, w);
        }
        for (g, w) in pack.ps_programs.iter().zip(&spec.ps) {
            check_prog(g, w);
        }
        assert_eq!(pack.shared_params.len(), spec.shared.len());
        for (g, w) in pack.shared_params.iter().zip(&spec.shared) {
            check_param(g, w);
        }
        assert_eq!(pack.material_params.len(), spec.material.len());
        for (g, w) in pack.material_params.iter().zip(&spec.material) {
            check_param(g, w);
        }
        assert_eq!(pack.techniques.len(), spec.techniques.len());
        for (t, (name, passes)) in pack.techniques.iter().zip(&spec.techniques) {
            assert_eq!(&t.name, name);
            let got: Vec<(usize, Option<usize>, Vec<(u32, u32)>)> = t
                .passes
                .iter()
                .map(|p| {
                    (
                        p.vs_index,
                        p.ps_index,
                        p.states.iter().map(|s| (s.id, s.value)).collect(),
                    )
                })
                .collect();
            assert_eq!(&got, passes);
        }
        assert_eq!(pack.programs().count(), spec.vs.len() + spec.ps.len());
        assert_eq!(
            pack.all_params().count(),
            spec.shared.len() + spec.material.len()
        );
    }
}

#[test]
fn dcl_and_sps_round_trip() {
    let mut rng = Rng::for_test("shaderpack texts");
    for _ in 0..100 {
        let mask = rng.below(0x10000) as u32;
        let channels: Vec<String> = (0..rng.below(6)).map(|_| rng.ident(1, 12)).collect();
        // The first comment cites a header path, which is never a channel list.
        let text = format!("{mask} ; see core/channels.h\n; {}\n", channels.join(" "));
        let decl = dcl::parse_dcl(&text).expect("generated dcl parses");
        assert_eq!(decl.mask, mask);
        assert_eq!(decl.channels, channels);

        let shader = rng.ident(1, 20);
        let overrides: Vec<(String, String, String)> = (0..rng.below(5))
            .map(|_| {
                let vals: Vec<String> = (0..rng.range(1, 4))
                    .map(|_| rng.below(100).to_string())
                    .collect();
                (
                    rng.ident(1, 16),
                    (*rng.pick(&["int", "float"])).to_string(),
                    vals.join(" "),
                )
            })
            .collect();
        let mut text = format!("shader {shader}\n");
        for (n, t, v) in &overrides {
            text.push_str(&format!("{n} {{\n\t{t} {v}\n}}\n"));
        }
        let preset = sps::parse_preset(&text).expect("generated preset parses");
        assert_eq!(preset.shader, shader);
        let got: Vec<(String, String, String)> = preset
            .overrides
            .iter()
            .map(|o| (o.name.clone(), o.type_name.clone(), o.values.clone()))
            .collect();
        assert_eq!(got, overrides);
    }
}

#[test]
fn fuzz_packs() {
    let mut rng = Rng::for_test("shaderpack fuzz");
    let seeds: Vec<Vec<u8>> = (0..4).map(|_| write_pack(&random_pack(&mut rng))).collect();
    fuzz("shaderpack", &seeds, 3000, |b| {
        if let Ok(pack) = ShaderPack::parse(b) {
            for (_, prog) in pack.programs() {
                let _ = (prog.validate(), prog.version_token());
            }
            for p in pack.all_params() {
                let _ = (p.kind(), p.defaults_f32(), p.sampler_states());
            }
            let _ = pack.param("x");
        }
    });
}

#[test]
fn fuzz_blobs_and_texts() {
    let mut rng = Rng::for_test("shaderpack blob fuzz");
    let blobs: Vec<Vec<u8>> = (0..3).map(|i| blob(&mut rng, i % 2 == 0, 6)).collect();
    fuzz("bytecode", &blobs, 3000, |b| {
        if let Ok(info) = bytecode::validate(b) {
            assert!(info.end_offset <= b.len());
        }
        let _ = bytecode::find_fourcc(b, *b"CTAB");
    });
    let texts = vec![
        b"89 ; see header\n; position diffuse texcoord0 normal\n".to_vec(),
        b"shader s\nA {\n\tint 1\n}\nB {\n\tfloat 1 0.5 0.25\n}\n".to_vec(),
    ];
    fuzz("dcl and sps", &texts, 3000, |b| {
        let text = String::from_utf8_lossy(b);
        let _ = dcl::parse_dcl(&text);
        let _ = sps::parse_preset(&text);
    });
}
