//! Census tests on hand-built programs: static counts, the translator's
//! verdict per program, and the report. Nothing here comes from the game.

mod common;

use common::*;
use lf_dxso_spirv::coverage::{Census, Tally};
use lf_dxso_spirv::{Options, RegType, Stage, decode};

fn tally(uses: usize, programs: usize) -> Tally {
    Tally { uses, programs }
}

/// `dcl_texcoord0 v0; dcl_2d s0; texld r0, v0, s0; texld r1, v0_abs, s0;
/// mul r0, r0, r1; mov_sat oC0, -r0`
fn small_pixel_program() -> Vec<u32> {
    let mut a = Asm::ps_basic();
    a.dcl_sampler(2, 0);
    a.op2(op::TEX, rd(0), v(0), sampler(0));
    a.op2(op::TEX, rd(1), v(0).md(sm::ABS), sampler(0));
    a.op2(op::MUL, rd(0), r(0), r(1));
    a.mov(oc(0).sat(), r(0).neg());
    a.end()
}

#[test]
fn the_corpus_translates_completely() {
    let mut census = Census::default();
    for (_, words, _) in corpus() {
        census.add_words(&words);
    }
    assert_eq!(census.programs, 6);
    assert_eq!(census.translated, 6, "{}", census.report());
    assert_eq!(census.vertex.programs, 3);
    assert_eq!(census.pixel.programs, 3);
    assert_eq!(census.unsupported_programs(), 0);
    assert_eq!(census.invalid_programs(), 0);
    assert_eq!(census.decode_failures(), 0);
    assert_eq!(census.validator_failures(), 0);
    let instructions: usize = corpus()
        .iter()
        .map(|(_, w, _)| decode(w).unwrap().instructions.len())
        .sum();
    assert_eq!(
        census.vertex.instructions + census.pixel.instructions,
        instructions
    );
    assert!(census.pixel.opcodes.contains_key("texldd"));
    assert!(!census.vertex.opcodes.contains_key("texkill"));
}

#[test]
fn static_counts_on_a_small_program() {
    let words = small_pixel_program();
    let mut census = Census::default();
    census.add_words(&words);
    census.add_bytes(
        &words
            .iter()
            .flat_map(|w| w.to_le_bytes())
            .collect::<Vec<u8>>(),
    );
    assert_eq!((census.programs, census.translated), (2, 2));
    let ps = census.stage(Stage::Pixel);
    assert_eq!(ps.programs, 2);
    assert_eq!(ps.instructions, 12);
    assert_eq!(ps.opcodes["dcl"], tally(4, 2));
    assert_eq!(ps.opcodes["texld"], tally(4, 2));
    assert_eq!(ps.opcodes["mul"], tally(2, 2));
    assert_eq!(ps.opcodes["mov"], tally(2, 2));
    // v0: dcl + two reads; s0: dcl + two reads; r#: 3 writes + 3 reads.
    assert_eq!(ps.register_files[&RegType::Input], tally(6, 2));
    assert_eq!(ps.register_files[&RegType::Sampler], tally(6, 2));
    assert_eq!(ps.register_files[&RegType::Temp], tally(12, 2));
    assert_eq!(ps.register_files[&RegType::ColorOut], tally(2, 2));
    assert_eq!(ps.source_modifiers["abs"], tally(2, 2));
    assert_eq!(ps.source_modifiers["neg"], tally(2, 2));
    assert_eq!(ps.source_modifiers.len(), 2);
    assert_eq!(ps.result_modifiers["sat"], tally(2, 2));
    assert_eq!(ps.declarations["input texcoord0"], tally(2, 2));
    assert_eq!(ps.declarations["sampler 2d"], tally(2, 2));
    assert!(ps.relative.is_empty());
    assert_eq!(ps.predicated, Tally::default());
    assert_eq!(census.vertex.programs, 0);
}

#[test]
fn relative_addressing_and_predication() {
    let mut census = Census::default();
    census.add_words(&programs::skinned_lit_vertex_shader());
    let vs = &census.vertex;
    // c100[a0.x], c20[aL], c30[aL].
    assert_eq!(vs.relative[&RegType::Const], tally(3, 1));
    // `mova a0.x` and the address of c100[a0.x].
    assert_eq!(vs.register_files[&RegType::Addr], tally(2, 1));
    // `loop aL, i0` and two relative addresses.
    assert_eq!(vs.register_files[&RegType::Loop], tally(3, 1));
    assert_eq!(vs.predicated, tally(1, 1));
    assert_eq!(vs.declarations["output fog0"], tally(1, 1));
    assert_eq!(vs.declarations["input blendindices0"], tally(1, 1));
    assert_eq!(vs.opcodes["defi"], tally(1, 1));
    assert_eq!(vs.opcodes["setp"], tally(1, 1));
}

#[test]
fn rejected_programs_are_counted_with_their_reason() {
    let mut census = Census::default();
    // Translator: a vertex input usage with no location.
    let mut a = Asm::vs_basic();
    a.dcl(usage::TEXCOORD, 8, vd(1));
    a.mov(od(0), v(1));
    census.add_words(&a.end());
    // Translator: predicated flow control.
    let mut a = Asm::ps_basic();
    a.ins(op::SETP, cmp::GT, Some(d(reg::PRED, 0)), &[v(0), c(0)]);
    a.ins_full(op::IF, 0, None, Some(p0().sw("x")), &[b(0)]);
    a.flow(op::ENDIF, 0, &[]);
    census.add_words(&a.end());
    // Decoder: a pixel shader 1.x instruction, and another shader model.
    census.add_words(&Asm::ps().ins(op::TEXBEM, 0, Some(rd(0)), &[r(1)]).end());
    census.add_words(&[0xFFFE_0200, END]);
    // Translator: invalid (an input read without a dcl).
    census.add_words(&Asm::ps().mov(oc(0), v(0)).end());
    // Decoder errors.
    census.add_words(&[PS30]);
    census.add_words(&Asm::ps().raw(75).end());
    census.add_bytes(&[1, 2, 3]);
    census.add_bytes(&[]);
    // And one that works.
    census.add_words(&small_pixel_program());

    assert_eq!(census.programs, 10);
    assert_eq!(census.translated, 1);
    assert_eq!(census.unsupported_programs(), 4);
    assert_eq!(
        census.unsupported["input usage texcoord8 has no location in the binding convention"],
        1
    );
    assert_eq!(census.unsupported["predicated flow-control instruction"], 1);
    assert_eq!(
        census.unsupported["instruction texbem (pixel shader 1.x only, not valid in shader model 3)"],
        1
    );
    assert_eq!(census.unsupported["shader model 2.0 (3.0 only)"], 1);
    let blocked: Vec<(&str, usize)> = census
        .blocked_at
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();
    assert_eq!(
        blocked,
        [
            ("dcl_texcoord8", 1),
            ("if", 1),
            ("texbem", 1),
            ("version token", 1)
        ]
    );
    assert_eq!(census.invalid["input v0 read without a dcl"], 1);
    assert_eq!(census.decode_failures(), 4);
    assert_eq!(census.decode_errors["no end token"], 1);
    assert_eq!(census.decode_errors["unknown opcode 75"], 1);
    assert_eq!(
        census.decode_errors["input length is not a whole number of tokens"],
        2
    );
    // Rejected programs that decoded still count statically.
    assert_eq!(census.vertex.programs, 1);
    assert_eq!(census.pixel.programs, 3);
    assert_eq!(census.vertex.declarations["input texcoord8"], tally(1, 1));
}

#[test]
fn options_change_the_verdict() {
    let mut a = Asm::ps_basic();
    a.dcl_sampler(3, 1);
    a.op2(op::TEX, rd(0), v(0), sampler(1));
    a.mov(oc(0), r(0));
    let words = a.end();
    let mut plain = Census::default();
    plain.add_words(&words);
    assert_eq!(plain.translated, 1);
    let mut depth = Census::new(Options {
        depth_compare_samplers: 1 << 1,
        ..Options::default()
    });
    depth.add_words(&words);
    assert_eq!(depth.translated, 0);
    assert_eq!(
        depth.unsupported["depth-compare sampler that is not two-dimensional"],
        1
    );
    assert_eq!(depth.blocked_at["dcl_cube"], 1);
}

#[test]
fn the_report_shows_the_fraction_and_the_reasons() {
    let empty = Census::default().report();
    assert!(
        empty.contains("translated and validated: 0 of 0 (n/a)"),
        "{empty}"
    );
    let mut census = Census::default();
    for (_, words, _) in corpus().into_iter().take(3) {
        census.add_words(&words);
    }
    census.add_words(&[0xFFFF_0200, END]);
    let report = census.report();
    for needle in [
        "census of 4 programs: 2 vertex and 1 pixel decoded, 1 not decoded",
        "translated and validated: 3 of 4 (75.0%)",
        "unsupported: 1 (25.0%)",
        "\nunsupported, by reason (programs):\n         1  shader model 2.0 (3.0 only)\n",
        "\nunsupported, by the instruction where translation stopped (programs):\n         1  version token\n",
        "\nvertex programs: 2 decoded,",
        "\npixel programs: 1 decoded,",
        "  opcodes (instructions, programs):",
        "  register files (operands, programs):",
        "  relative addressing (operands, programs):",
        "  declarations (instructions, programs):",
    ] {
        assert!(report.contains(needle), "{needle}\n{report}");
    }
    // Tables are sorted by frequency: 9 + 2 declarations lead the opcodes.
    let vertex = report.split("\nvertex programs:").nth(1).unwrap();
    let rows: Vec<Vec<&str>> = vertex
        .lines()
        .skip_while(|l| !l.starts_with("  opcodes"))
        .skip(1)
        .take_while(|l| l.starts_with("    "))
        .map(|l| l.split_whitespace().collect())
        .collect();
    assert_eq!(rows[0], ["dcl", "11", "2"]);
    let uses: Vec<usize> = rows.iter().map(|r| r[1].parse().unwrap()).collect();
    assert!(uses.windows(2).all(|w| w[0] >= w[1]), "{uses:?}");
    // 2 of 3 rounds down to 66.6%.
    let mut census = Census::default();
    census.add_words(&small_pixel_program());
    census.add_words(&small_pixel_program());
    census.add_words(&[PS30]);
    assert!(census.report().contains("(66.6%)"));
}
