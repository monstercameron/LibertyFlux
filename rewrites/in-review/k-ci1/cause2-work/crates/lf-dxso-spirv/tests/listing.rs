//! Listing tests: every element of the listing syntax, on hand-built
//! programs written with the test assembler. Nothing here comes from the
//! game.

mod common;

use common::*;
use lf_dxso_spirv::decode;
use lf_dxso_spirv::listing::{
    ListingOptions, instruction_text, listing, listing_with, mnemonic, register_file_name,
    register_name,
};
use lf_dxso_spirv::{RegType, Register, Stage};

/// Every register file, in `RegType::raw` order.
const ALL_FILES: [RegType; 20] = [
    RegType::Temp,
    RegType::Input,
    RegType::Const,
    RegType::Addr,
    RegType::RastOut,
    RegType::AttrOut,
    RegType::Output,
    RegType::ConstInt,
    RegType::ColorOut,
    RegType::DepthOut,
    RegType::Sampler,
    RegType::Const2,
    RegType::Const3,
    RegType::Const4,
    RegType::ConstBool,
    RegType::Loop,
    RegType::TempFloat16,
    RegType::MiscType,
    RegType::Label,
    RegType::Predicate,
];

/// The text of instruction `index` of a program.
fn text_of(words: &[u32], index: usize) -> String {
    let sh = decode(words).unwrap();
    instruction_text(&sh.instructions[index], sh.stage)
}

/// The text of the only instruction of a pixel program.
fn ps_line(build: impl FnOnce(&mut Asm)) -> String {
    let mut a = Asm::ps();
    build(&mut a);
    text_of(&a.end(), 0)
}

#[test]
fn skinned_vertex_shader_listing() {
    let sh = decode(&programs::skinned_lit_vertex_shader()).unwrap();
    let expected = "\
vs_3_0
; 31 instructions in 114 dwords; CTAB comment absent
    1  dcl_position0 v0
    4  dcl_normal0 v1
    7  dcl_texcoord0 v2
   10  dcl_blendindices0 v3
   13  dcl_position0 o0
   16  dcl_texcoord0 o1.xy
   19  dcl_texcoord1 o2
   22  dcl_color0 o3
   25  dcl_fog0 o4.x
   28  defi i0, 4, 0, 1, 0
   34  def c95, 3.0, 0.0, 1.0, 0.5
   40  mul r0, v3, c95.x
   44  mova a0.x, r0.x
   47  m4x3 r1.xyz, v0, c100[a0.x]
   52  mov r1.w, c95.z
   55  m4x4 o0, r1, c0
   59  m3x3 r2.xyz, v1, c4
   63  nrm r3.xyz, r2
   66  mov r4, c95.y
   69  loop aL, i0
   72    dp3 r5.x, r3, c20[aL]
   77    max r5.x, r5.x, c95.y
   81    mad r4.xyz, r5.x, c30[aL], r4
   87  endloop
   88  setp_gt p0.x, r4.x, c95.z
   92  (p0.x) mov r4.xyz, c95.z
   96  mov o3, r4
   99  mov o1.xy, v2
  102  dp4 r6.x, r1, c8
  106  mul o4.x, r6.x, c9.x
  110  mov o2, r3
";
    assert_eq!(listing(&sh), expected);
}

#[test]
fn options_turn_offsets_and_indentation_off() {
    let sh = decode(&programs::skinned_lit_vertex_shader()).unwrap();
    let plain = listing_with(
        &sh,
        &ListingOptions {
            offsets: false,
            indent: false,
        },
    );
    let lines: Vec<&str> = plain.lines().collect();
    assert_eq!(lines.len(), sh.instructions.len() + 2);
    for (line, ins) in lines[2..].iter().zip(&sh.instructions) {
        assert_eq!(*line, instruction_text(ins, sh.stage));
    }
    assert!(lines.contains(&"dp3 r5.x, r3, c20[aL]"));
}

#[test]
fn every_corpus_program_lists_one_line_per_instruction() {
    for (name, words, _) in corpus() {
        let sh = decode(&words).unwrap();
        let text = listing(&sh);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), sh.instructions.len() + 2, "{name}");
        assert!(lines[0] == "vs_3_0" || lines[0] == "ps_3_0", "{name}");
        for (line, ins) in lines[2..].iter().zip(&sh.instructions) {
            let offset: usize = line.split_whitespace().next().unwrap().parse().unwrap();
            assert_eq!(offset, ins.offset, "{name}: {line}");
        }
    }
}

#[test]
fn flow_control_is_indented_and_never_underflows() {
    let (_, words, _) = corpus()
        .into_iter()
        .find(|(n, _, _)| *n == "ps_flow_texture")
        .unwrap();
    let text = listing(&decode(&words).unwrap());
    // if > rep > loop: three levels of two spaces after the offset column.
    assert!(
        text.contains("\n   84        add r2, r2, v0[aL]\n"),
        "{text}"
    );
    assert!(text.contains("\n   94  else\n"), "{text}");
    assert!(text.contains("\n  125  label l3\n"), "{text}");
    // Unbalanced input still lists, at depth zero.
    let w = Asm::ps()
        .flow(op::ENDIF, 0, &[])
        .flow(op::ENDLOOP, 0, &[])
        .end();
    let plain = listing_with(
        &decode(&w).unwrap(),
        &ListingOptions {
            offsets: false,
            indent: true,
        },
    );
    assert!(plain.ends_with("\nendif\nendloop\n"), "{plain}");
}

#[test]
fn source_modifiers_and_swizzles() {
    let cases = [
        (sm::NEG, "-r1"),
        (sm::BIAS, "r1_bias"),
        (sm::BIASNEG, "-r1_bias"),
        (sm::SIGN, "r1_bx2"),
        (sm::SIGNNEG, "-r1_bx2"),
        (sm::COMP, "1 - r1"),
        (sm::X2, "r1_x2"),
        (sm::X2NEG, "-r1_x2"),
        (sm::DZ, "r1_dz"),
        (sm::DW, "r1_dw"),
        (sm::ABS, "r1_abs"),
        (sm::ABSNEG, "-r1_abs"),
        (sm::NOT, "!r1"),
    ];
    for (m, want) in cases {
        assert_eq!(
            ps_line(|a| {
                a.mov(rd(0), r(1).md(m));
            }),
            format!("mov r0, {want}")
        );
    }
    // The swizzle follows the modifier suffix.
    assert_eq!(
        ps_line(|a| {
            a.mov(rd(0), r(1).md(sm::ABSNEG).sw("x"));
        }),
        "mov r0, -r1_abs.x"
    );
    for (sw, want) in [
        ("xyzw", ""),
        ("x", ".x"),
        ("w", ".w"),
        ("wzyx", ".wzyx"),
        ("xyz", ".xyzz"),
        ("yx", ".yxxx"),
    ] {
        assert_eq!(
            ps_line(|a| {
                a.mov(rd(0), r(1).sw(sw));
            }),
            format!("mov r0, r1{want}"),
            "{sw}"
        );
    }
}

#[test]
fn result_modifiers_masks_and_shift() {
    assert_eq!(
        ps_line(|a| {
            a.mov(rd(7).m("xw").sat().pp().centroid(), r(1));
        }),
        "mov_sat_pp_centroid r7.xw, r1"
    );
    for (shift, want) in [
        (1, "_x2"),
        (2, "_x4"),
        (3, "_x8"),
        (0xF, "_d2"),
        (0xE, "_d4"),
        (0xD, "_d8"),
        (4, "_shift4"),
        (8, "_shift-8"),
    ] {
        assert_eq!(
            ps_line(|a| {
                a.mov(rd(0).shift(shift), r(1));
            }),
            format!("mov{want} r0, r1"),
            "{shift}"
        );
    }
    assert_eq!(
        ps_line(|a| {
            a.mov(rd(0).raw_mask(0), r(1));
        }),
        "mov r0.none, r1"
    );
    assert_eq!(
        ps_line(|a| {
            a.mov(rd(0).m("yz"), r(1));
        }),
        "mov r0.yz, r1"
    );
}

#[test]
fn relative_addresses() {
    let mut a = Asm::vs();
    a.mov(rd(0), c(30).rel(A0X))
        .mov(rd(0), c(30).rel(Rel(reg::ADDR, 2)))
        .mov(rd(0), v(0).rel(AL))
        .mov(od(1).rel(AL), r(0));
    let w = a.end();
    assert_eq!(text_of(&w, 0), "mov r0, c30[a0.x]");
    assert_eq!(text_of(&w, 1), "mov r0, c30[a0.z]");
    assert_eq!(text_of(&w, 2), "mov r0, v0[aL]");
    assert_eq!(text_of(&w, 3), "mov o1[aL], r0");
}

#[test]
fn comparisons_texture_flags_and_control_bits() {
    for (code, suffix) in [
        (cmp::GT, "_gt"),
        (cmp::EQ, "_eq"),
        (cmp::GE, "_ge"),
        (cmp::LT, "_lt"),
        (cmp::NE, "_ne"),
        (cmp::LE, "_le"),
        (0, "_cmp0"),
        (7, "_cmp7"),
    ] {
        let mut a = Asm::ps();
        a.flow(op::IFC, code, &[r(0).sw("x"), c(0).sw("y")])
            .flow(op::BREAKC, code, &[r(0).sw("x"), c(0).sw("y")])
            .ins(op::SETP, code, Some(d(reg::PRED, 0).m("x")), &[r(0), c(0)]);
        let w = a.end();
        assert_eq!(text_of(&w, 0), format!("if{suffix} r0.x, c0.y"));
        assert_eq!(text_of(&w, 1), format!("break{suffix} r0.x, c0.y"));
        assert_eq!(text_of(&w, 2), format!("setp{suffix} p0.x, r0, c0"));
    }
    for (control, want) in [(0, "texld"), (1, "texldp"), (2, "texldb"), (3, "texldpb")] {
        assert_eq!(
            ps_line(|a| {
                a.ins(op::TEX, control, Some(rd(0)), &[v(0), sampler(1)]);
            }),
            format!("{want} r0, v0, s1")
        );
    }
    // Bits the mnemonic does not show are kept as a comment.
    assert_eq!(
        ps_line(|a| {
            a.ins(op::MOV, 4, Some(rd(0)), &[r(1)]);
        }),
        "mov r0, r1 ; control 0x04"
    );
    assert_eq!(
        ps_line(|a| {
            a.ins(op::IFC, 0x11, None, &[r(0), r(1)]);
        }),
        "if_gt r0, r1 ; control 0x11"
    );
    assert_eq!(
        ps_line(|a| {
            a.ins(op::TEX, 0x05, Some(rd(0)), &[v(0), sampler(0)]);
        }),
        "texldp r0, v0, s0 ; control 0x05"
    );
}

#[test]
fn declarations() {
    let mut a = Asm::vs();
    a.dcl(usage::TEXCOORD, 8, vd(1))
        .dcl(14, 0, vd(2))
        .dcl(usage::PSIZE, 0, od(2).m("x"))
        .dcl(usage::BLENDWEIGHT, 3, vd(3).m("xy"));
    let w = a.end();
    assert_eq!(text_of(&w, 0), "dcl_texcoord8 v1");
    assert_eq!(text_of(&w, 1), "dcl_usage14_0 v2");
    assert_eq!(text_of(&w, 2), "dcl_psize0 o2.x");
    assert_eq!(text_of(&w, 3), "dcl_blendweight3 v3.xy");
    let mut a = Asm::ps();
    a.dcl(usage::COLOR, 1, vd(1).centroid())
        .dcl_sampler(2, 0)
        .dcl_sampler(3, 1)
        .dcl_sampler(4, 2)
        .dcl_sampler(5, 3)
        .dcl_misc(0)
        .dcl_misc(1);
    let w = a.end();
    let lines: Vec<String> = (0..7).map(|i| text_of(&w, i)).collect();
    assert_eq!(
        lines,
        [
            "dcl_color1_centroid v1",
            "dcl_2d s0",
            "dcl_cube s1",
            "dcl_volume s2",
            "dcl_textype5 s3",
            "dcl vPos",
            "dcl vFace",
        ]
    );
    let sh = decode(&w).unwrap();
    assert_eq!(mnemonic(&sh.instructions[0]), "dcl_color1");
    assert_eq!(mnemonic(&sh.instructions[1]), "dcl_2d");
}

#[test]
fn definitions_keep_every_bit_visible() {
    let mut a = Asm::ps();
    a.def(0, [-0.0, f32::INFINITY, f32::NEG_INFINITY, 1.0e-7]);
    // A NaN with a payload, written as raw words.
    a.raw(op::DEF | (5 << 24))
        .raw(regbits(reg::CONST, 1) | (0xF << 16))
        .raw(0x7FC0_0001)
        .raw(0xFFC0_0000)
        .raw(0x3F80_0000)
        .raw(0x0000_0001);
    a.defi(2, [-1, 255, 0, 7]).defb(3, false).defb(4, true);
    let w = a.end();
    assert_eq!(text_of(&w, 0), "def c0, -0.0, inf, -inf, 1e-7");
    assert_eq!(
        text_of(&w, 1),
        "def c1, nan(0x7fc00001), nan(0xffc00000), 1.0, 1e-45"
    );
    assert_eq!(text_of(&w, 2), "defi i2, -1, 255, 0, 7");
    assert_eq!(text_of(&w, 3), "defb b3, false");
    assert_eq!(text_of(&w, 4), "defb b4, true");
}

#[test]
fn predicates_subroutines_and_texkill() {
    let mut a = Asm::ps();
    a.pred(op::MOV, p0().sw("x"), rd(0).m("xyz"), &[c(1).sw("z")])
        .pred(op::ADD, p0().sw("z").not(), rd(1), &[r(0), r(2)])
        .ins(op::TEXKILL, 0, Some(rd(1).m("xyz")), &[])
        .flow(op::CALLNZ, 0, &[l(3), b(1).not()])
        .flow(op::CALLNZ, 0, &[l(4), p0().sw("y")])
        .flow(op::RET, 0, &[])
        .flow(op::LABEL, 0, &[l(3)])
        .flow(op::CALL, 0, &[l(4)])
        .flow(op::LOOP, 0, &[al(), i(2)])
        .flow(op::REP, 0, &[i(1)])
        .flow(op::BREAKP, 0, &[p0().sw("w").not()]);
    let w = a.end();
    let lines: Vec<String> = (0..11).map(|i| text_of(&w, i)).collect();
    assert_eq!(
        lines,
        [
            "(p0.x) mov r0.xyz, c1.z",
            "(!p0.z) add r1, r0, r2",
            "texkill r1.xyz",
            "callnz l3, !b1",
            "callnz l4, p0.y",
            "ret",
            "label l3",
            "call l4",
            "loop aL, i2",
            "rep i1",
            "breakp !p0.w",
        ]
    );
}

#[test]
fn register_names_in_both_stages() {
    let name = |kind, num, stage| register_name(Register { kind, num }, stage);
    let vs = Stage::Vertex;
    let ps = Stage::Pixel;
    assert_eq!(name(RegType::Temp, 5, vs), "r5");
    assert_eq!(name(RegType::Input, 5, ps), "v5");
    assert_eq!(name(RegType::Const, 5, vs), "c5");
    assert_eq!(name(RegType::Addr, 0, vs), "a0");
    assert_eq!(name(RegType::Addr, 5, ps), "t5");
    assert_eq!(name(RegType::RastOut, 0, vs), "oPos");
    assert_eq!(name(RegType::RastOut, 1, vs), "oFog");
    assert_eq!(name(RegType::RastOut, 2, vs), "oPts");
    assert_eq!(name(RegType::RastOut, 3, vs), "oRast3");
    assert_eq!(name(RegType::AttrOut, 1, vs), "oD1");
    assert_eq!(name(RegType::Output, 5, vs), "o5");
    assert_eq!(name(RegType::ConstInt, 5, vs), "i5");
    assert_eq!(name(RegType::ColorOut, 3, ps), "oC3");
    assert_eq!(name(RegType::DepthOut, 0, ps), "oDepth");
    assert_eq!(name(RegType::DepthOut, 1, ps), "oDepth1");
    assert_eq!(name(RegType::Sampler, 15, ps), "s15");
    assert_eq!(name(RegType::Const2, 5, vs), "c2053");
    assert_eq!(name(RegType::Const3, 5, vs), "c4101");
    assert_eq!(name(RegType::Const4, 5, vs), "c6149");
    assert_eq!(name(RegType::ConstBool, 5, ps), "b5");
    assert_eq!(name(RegType::Loop, 0, ps), "aL");
    assert_eq!(name(RegType::Loop, 1, ps), "aL1");
    assert_eq!(name(RegType::TempFloat16, 5, ps), "half5");
    assert_eq!(name(RegType::MiscType, 0, ps), "vPos");
    assert_eq!(name(RegType::MiscType, 1, ps), "vFace");
    assert_eq!(name(RegType::MiscType, 2, ps), "vMisc2");
    assert_eq!(name(RegType::Label, 5, vs), "l5");
    assert_eq!(name(RegType::Predicate, 0, vs), "p0");
    // What the decoder reads from each register type field is what is named.
    for kind in ALL_FILES {
        let w = Asm::ps().mov(rd(0), s(kind.raw(), 5)).end();
        assert_eq!(
            text_of(&w, 0),
            format!("mov r0, {}", name(kind, 5, Stage::Pixel)),
            "{kind:?}"
        );
    }
}

#[test]
fn register_file_names_are_unique_per_stage() {
    for stage in [Stage::Vertex, Stage::Pixel] {
        let mut names: Vec<&str> = ALL_FILES
            .iter()
            .map(|&k| register_file_name(k, stage))
            .collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), ALL_FILES.len(), "{stage:?}");
    }
    assert_eq!(register_file_name(RegType::Addr, Stage::Vertex), "a0");
    assert_eq!(register_file_name(RegType::Addr, Stage::Pixel), "t#");
}

#[test]
fn header_reports_size_and_constant_table() {
    let mut a = Asm::ps();
    a.comment(&[u32::from_le_bytes(*b"CTAB"), 1, 2]);
    let text = listing(&decode(&a.end()).unwrap());
    assert_eq!(
        text,
        "ps_3_0\n; 0 instructions in 6 dwords; CTAB comment present\n"
    );
}
