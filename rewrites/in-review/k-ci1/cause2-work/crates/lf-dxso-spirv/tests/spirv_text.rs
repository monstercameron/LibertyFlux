//! Text dump tests: every module translated from the hand-built corpus
//! dumps with every word accounted for, every opcode the crate emits has a
//! name, and broken input is reported rather than panicked on. Nothing
//! here comes from the game.

mod common;

use common::*;
use lf_dxso_spirv::spirv::{glsl, op as sop};
use lf_dxso_spirv::spirv_text::{
    TextOptions, disassemble, disassemble_with, glsl_name, opcode_name, reflection_summary,
};
use lf_dxso_spirv::validate::instructions;
use lf_dxso_spirv::{Options, translate_with};
use std::collections::BTreeSet;

/// Every opcode constant in `lf_dxso_spirv::spirv::op`.
const ALL_OPCODES: [u16; 74] = [
    sop::NAME,
    sop::MEMBER_NAME,
    sop::EXT_INST_IMPORT,
    sop::EXT_INST,
    sop::MEMORY_MODEL,
    sop::ENTRY_POINT,
    sop::EXECUTION_MODE,
    sop::CAPABILITY,
    sop::TYPE_VOID,
    sop::TYPE_BOOL,
    sop::TYPE_INT,
    sop::TYPE_FLOAT,
    sop::TYPE_VECTOR,
    sop::TYPE_IMAGE,
    sop::TYPE_SAMPLED_IMAGE,
    sop::TYPE_ARRAY,
    sop::TYPE_STRUCT,
    sop::TYPE_POINTER,
    sop::TYPE_FUNCTION,
    sop::CONSTANT_TRUE,
    sop::CONSTANT_FALSE,
    sop::CONSTANT,
    sop::CONSTANT_COMPOSITE,
    sop::FUNCTION,
    sop::FUNCTION_END,
    sop::FUNCTION_CALL,
    sop::VARIABLE,
    sop::LOAD,
    sop::STORE,
    sop::ACCESS_CHAIN,
    sop::DECORATE,
    sop::MEMBER_DECORATE,
    sop::VECTOR_SHUFFLE,
    sop::COMPOSITE_CONSTRUCT,
    sop::COMPOSITE_EXTRACT,
    sop::IMAGE_SAMPLE_IMPLICIT_LOD,
    sop::IMAGE_SAMPLE_EXPLICIT_LOD,
    sop::IMAGE_SAMPLE_DREF_IMPLICIT_LOD,
    sop::IMAGE_SAMPLE_DREF_EXPLICIT_LOD,
    sop::CONVERT_F_TO_S,
    sop::F_NEGATE,
    sop::I_ADD,
    sop::F_ADD,
    sop::F_SUB,
    sop::F_MUL,
    sop::F_DIV,
    sop::DOT,
    sop::ANY,
    sop::LOGICAL_OR,
    sop::LOGICAL_AND,
    sop::LOGICAL_NOT,
    sop::SELECT,
    sop::I_EQUAL,
    sop::I_NOT_EQUAL,
    sop::S_GREATER_THAN_EQUAL,
    sop::S_LESS_THAN,
    sop::F_ORD_EQUAL,
    sop::F_UNORD_NOT_EQUAL,
    sop::F_ORD_LESS_THAN,
    sop::F_ORD_GREATER_THAN,
    sop::F_ORD_LESS_THAN_EQUAL,
    sop::F_ORD_GREATER_THAN_EQUAL,
    sop::SHIFT_RIGHT_LOGICAL,
    sop::BITWISE_AND,
    sop::DPDX,
    sop::DPDY,
    sop::LOOP_MERGE,
    sop::SELECTION_MERGE,
    sop::LABEL,
    sop::BRANCH,
    sop::BRANCH_CONDITIONAL,
    sop::KILL,
    sop::RETURN,
    sop::UNREACHABLE,
];

/// Every constant in `lf_dxso_spirv::spirv::glsl`.
const ALL_GLSL: [u32; 13] = [
    glsl::FABS,
    glsl::FSIGN,
    glsl::FLOOR,
    glsl::FRACT,
    glsl::SIN,
    glsl::COS,
    glsl::EXP2,
    glsl::LOG2,
    glsl::SQRT,
    glsl::FMIN,
    glsl::FMAX,
    glsl::SCLAMP,
    glsl::CROSS,
];

/// The corpus program called `name`, translated.
fn corpus_module(name: &str) -> lf_dxso_spirv::Module {
    let (_, words, opts) = corpus().into_iter().find(|(n, _, _)| *n == name).unwrap();
    check_with(&words, opts).0
}

/// The corpus plus one program for the two emittable opcodes the corpus
/// does not reach: an `le` comparison and depth-compare `texldl`.
fn programs() -> Vec<(&'static str, Vec<u32>, Options)> {
    let mut all = corpus();
    let mut a = Asm::ps_basic();
    a.dcl_sampler(2, 0);
    a.op2(op::TEXLDL, rd(0), v(0), sampler(0));
    a.flow(op::IFC, cmp::LE, &[r(0).sw("x"), c(0).sw("x")]);
    a.mov(rd(0), c(1));
    a.flow(op::ENDIF, 0, &[]);
    a.mov(oc(0), r(0));
    all.push((
        "ps_le_shadow_lod",
        a.end(),
        Options {
            depth_compare_samplers: 1,
            ..Options::default()
        },
    ));
    all
}

#[test]
fn every_test_module_dumps_with_every_word_accounted_for() {
    let mut seen = BTreeSet::new();
    for (name, words, opts) in programs() {
        let (m, report) = check_with(&words, opts);
        for options in [
            TextOptions::default(),
            TextOptions {
                names: false,
                offsets: true,
            },
        ] {
            let t = disassemble_with(&m.words, &options);
            assert!(
                t.is_clean(),
                "{name}: unknown {:?}, mismatched {:?}, error {:?}",
                t.unknown,
                t.mismatched,
                t.error
            );
            assert_eq!(t.instructions, report.instructions, "{name}");
            assert!(!t.text.contains("Op<"), "{name}");
            // Five header lines, then one line per instruction.
            assert_eq!(t.text.lines().count(), 5 + report.instructions, "{name}");
        }
        for inst in instructions(&m.words).unwrap() {
            seen.insert(inst.opcode);
        }
    }
    // Every opcode the translator emits was dumped; `OpAny` and
    // `OpUnreachable` are named for the validator but never emitted.
    let unseen: Vec<u16> = ALL_OPCODES
        .iter()
        .copied()
        .filter(|o| !seen.contains(o))
        .collect();
    assert_eq!(unseen, [sop::ANY, sop::UNREACHABLE]);
}

#[test]
fn every_emitted_opcode_and_extended_instruction_has_a_unique_name() {
    let names: BTreeSet<&str> = ALL_OPCODES
        .iter()
        .map(|&o| opcode_name(o).unwrap_or_else(|| panic!("opcode {o} has no name")))
        .collect();
    assert_eq!(names.len(), ALL_OPCODES.len());
    assert!(names.iter().all(|n| n.starts_with("Op")));
    assert_eq!(opcode_name(sop::F_ADD), Some("OpFAdd"));
    assert_eq!(opcode_name(sop::DPDX), Some("OpDPdx"));
    assert_eq!(opcode_name(1), None);
    let glsl_names: BTreeSet<&str> = ALL_GLSL.iter().map(|&g| glsl_name(g).unwrap()).collect();
    assert_eq!(glsl_names.len(), ALL_GLSL.len());
    assert_eq!(glsl_name(glsl::SCLAMP), Some("SClamp"));
    assert_eq!(glsl_name(1), None);
}

#[test]
fn ids_take_unique_names_only() {
    let m = corpus_module("ps_flow_texture");
    let text = disassemble(&m.words).text;
    assert!(text.contains("%main = OpFunction"), "{text}");
    assert!(text.contains("OpDecorate %s0 DescriptorSet 1"), "{text}");
    // Three loops each make a `loop_counter`: the shared name is not used.
    assert!(text.contains("\"loop_counter\""));
    assert!(!text.contains("%loop_counter"), "{text}");
    let raw = disassemble_with(
        &m.words,
        &TextOptions {
            names: false,
            offsets: false,
        },
    )
    .text;
    assert!(!raw.contains("%main"));
    assert!(raw.contains("\"main\""));
}

#[test]
fn offsets_are_word_offsets() {
    let m = corpus_module("ps_textured");
    let t = disassemble_with(
        &m.words,
        &TextOptions {
            names: true,
            offsets: true,
        },
    );
    let insts = instructions(&m.words).unwrap();
    let lines: Vec<&str> = t.text.lines().skip(5).collect();
    assert_eq!(lines.len(), insts.len());
    for (line, inst) in lines.iter().zip(&insts) {
        let offset: usize = line.split_whitespace().next().unwrap().parse().unwrap();
        assert_eq!(offset, inst.offset, "{line}");
    }
    assert!(lines[0].starts_with("     5 "), "{}", lines[0]);
}

#[test]
fn enumerants_decorations_and_extended_instructions_by_name() {
    let ps = disassemble(&corpus_module("ps_textured").words).text;
    for needle in [
        "OpCapability Shader",
        "OpMemoryModel Logical GLSL450",
        "OpEntryPoint Fragment %main \"main\"",
        "OpExecutionMode %main OriginUpperLeft",
        "OpDecorate %vFace BuiltIn FrontFacing",
        "OpDecorate %v0_texcoord0 Location 0",
        "OpDecorate %s1 Binding 1",
        "OpTypeImage %3 2D 0 0 0 1 Unknown",
        "OpTypeImage %3 Cube 0 0 0 1 Unknown",
        "OpTypePointer UniformConstant",
        "OpImageSampleImplicitLod",
        "OpKill",
        " None",
        "= OpExtInstImport \"GLSL.std.450\"",
    ] {
        assert!(ps.contains(needle), "{needle}\n{ps}");
    }
    let flow = disassemble(&corpus_module("ps_flow_texture").words).text;
    for needle in [
        "OpExecutionMode %main DepthReplacing",
        "OpDecorate %oDepth BuiltIn FragDepth",
        "OpDecorate %vPos BuiltIn FragCoord",
        "OpDecorate %v1_color0 Centroid",
        "OpTypeImage %3 3D 0 0 0 1 Unknown",
        " Bias %",
        "OpImageSampleExplicitLod",
        " Lod %",
        " Grad %",
        "OpLoopMerge",
        "OpFunctionCall %2 %l3",
        "OpDPdx",
        "OpDPdy",
        "OpTypePointer Uniform",
        "OpMemberDecorate %IntBoolConstants 1 Offset 256",
        "OpDecorate %ib Binding 3",
    ] {
        assert!(flow.contains(needle), "{needle}\n{flow}");
    }
    let vs = disassemble(&corpus_module("vs_arithmetic").words).text;
    for name in ALL_GLSL.iter().map(|&g| glsl_name(g).unwrap()) {
        assert!(vs.contains(&format!(" {name} %")), "{name}");
    }
    assert!(vs.contains("OpTypePointer PushConstant"));
    assert!(vs.contains("ArrayStride 16"));
    let shadow = disassemble(&corpus_module("ps_shadow").words).text;
    assert!(
        shadow.contains("OpTypeImage %3 2D 1 0 0 1 Unknown"),
        "{shadow}"
    );
    assert!(shadow.contains("OpImageSampleDrefImplicitLod"));
    let outputs = disassemble(&corpus_module("vs_outputs").words).text;
    assert!(outputs.contains("BuiltIn PointSize"));
    assert!(outputs.contains("OpEntryPoint Vertex"));
}

#[test]
fn constants_print_by_their_type() {
    let mut a = Asm::ps_basic();
    a.raw(op::DEF | (5 << 24))
        .raw(regbits(reg::CONST, 0) | (0xF << 16))
        .raw((-0.0f32).to_bits())
        .raw(0x7FC0_0001)
        .raw(f32::INFINITY.to_bits())
        .raw(0.25f32.to_bits());
    a.defi(0, [-1, 2, 1, 0]);
    a.mov(rd(0), c(0));
    a.flow(op::REP, 0, &[i(0)]);
    a.op2(op::ADD, rd(0), r(0), c(1));
    a.flow(op::ENDREP, 0, &[]);
    a.mov(oc(0), r(0));
    let (m, _) = check(&a.end());
    let text = disassemble(&m.words).text;
    for needle in [
        "OpConstant %3 -0.0",
        "OpConstant %3 nan(0x7fc00001)",
        "OpConstant %3 inf",
        "OpConstant %3 0.25",
        // Signed integers print signed; the unsigned array length does not.
        "OpConstant %7 -1",
        "OpConstant %9 224",
    ] {
        assert!(text.contains(needle), "{needle}\n{text}");
    }
}

#[test]
fn unknown_opcodes_and_operand_mismatches_are_reported() {
    let m = corpus_module("ps_textured");
    let insts = instructions(&m.words).unwrap();
    // Turn the last OpReturn into an opcode the crate never emits.
    let ret = insts
        .iter()
        .rev()
        .find(|i| i.opcode == sop::RETURN)
        .unwrap();
    let mut words = m.words.clone();
    let unknown: u16 = 9999;
    words[ret.offset] = (1 << 16) | u32::from(unknown);
    let t = disassemble(&words);
    assert_eq!(t.unknown, vec![(ret.offset, 9999)]);
    assert!(t.text.contains("Op<9999>"));
    assert!(!t.is_clean());
    // Hand-made modules: an extra operand word, then a missing one.
    let header = [0x0723_0203, 0x0001_0000, 0, 10, 0];
    let mut w = header.to_vec();
    w.extend([(3 << 16) | u32::from(sop::CAPABILITY), 1, 77]);
    w.extend([(2 << 16) | u32::from(sop::TYPE_INT), 5]);
    w.extend([(3 << 16) | u32::from(sop::IMAGE_SAMPLE_IMPLICIT_LOD), 1, 2]);
    // An OpConstant without its value word.
    w.extend([(3 << 16) | u32::from(sop::CONSTANT), 1, 2]);
    let t = disassemble(&w);
    assert_eq!(t.mismatched, vec![5, 8, 10, 13]);
    assert!(
        t.text.contains("OpCapability Shader ; extra 0x0000004d"),
        "{}",
        t.text
    );
    assert!(t.text.contains("<missing operand>"));
    assert!(t.error.is_none());
    // A Grad image operand needs two ids.
    let mut w = header.to_vec();
    w.extend([
        (7 << 16) | u32::from(sop::IMAGE_SAMPLE_EXPLICIT_LOD),
        1,
        2,
        3,
        4,
        0x4,
        6,
    ]);
    assert_eq!(disassemble(&w).mismatched, vec![5]);
}

#[test]
fn header_and_word_count_errors_stop_the_walk() {
    let t = disassemble(&[0x0723_0203, 0x0001_0000]);
    assert_eq!(t.error.as_ref().map(|e| e.offset), Some(0));
    assert_eq!(t.instructions, 0);
    let t = disassemble(&[0xDEAD_BEEF, 0, 0, 0, 0]);
    assert!(t.error.unwrap().message.contains("magic"));
    let header = [0x0723_0203, 0x0001_0000, 0, 10, 0];
    let mut w = header.to_vec();
    w.extend([(2 << 16) | u32::from(sop::CAPABILITY), 1, 0]);
    let t = disassemble(&w);
    assert_eq!(t.instructions, 1);
    assert_eq!(t.error.as_ref().map(|e| e.offset), Some(7));
    assert!(
        t.text
            .contains("; error at word 7: instruction word count is zero")
    );
    let mut w = header.to_vec();
    w.push((4 << 16) | u32::from(sop::CAPABILITY));
    let t = disassemble(&w);
    assert!(t.error.unwrap().message.contains("past the end"));
    // Random words never panic.
    let mut x = 0x1234_5678u32;
    for _ in 0..200 {
        let mut w = header.to_vec();
        for _ in 0..40 {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            w.push(((x % 7) << 16) | ((x >> 20) % 260));
        }
        let _ = disassemble(&w);
    }
}

#[test]
fn reflection_summaries() {
    let vs = corpus_module("vs_skinned_lit");
    let expected = format!(
        "\
stage: vertex; {} SPIR-V words
inputs:
  v0 position0 -> location 0
  v1 normal0 -> location 3
  v2 texcoord0 -> location 6
  v3 blendindices0 -> location 2
outputs:
  o0 position0 -> built-in Position
  o1 texcoord0 -> location 0, mask .xy
  o2 texcoord1 -> location 1
  o3 color0 -> location 10
  o4 fog0 -> location 12, mask .x
samplers: none
constants:
  float read: c0-c6, c8-c9; relative addressing: yes
  float defined by def: c95
  integer read: none; defined by defi: i0
  boolean read: none; defined by defb: none
buffers: float constants set 0 binding 0; integer/boolean constants none; push constants 16 bytes
depth replacing: no
",
        vs.words.len()
    );
    assert_eq!(reflection_summary(&vs), expected);
    let ps = reflection_summary(&corpus_module("ps_flow_texture"));
    for needle in [
        "stage: pixel (fragment)",
        "  v1 color0 -> location 10, centroid",
        "  vPos -> built-in FragCoord",
        "  oDepth -> built-in FragDepth, mask .x",
        "  s2: volume, set 1 binding 2",
        "integer/boolean constants set 0 binding 3",
        "  boolean read: b1-b2; defined by defb: b0",
        "depth replacing: yes",
    ] {
        assert!(ps.contains(needle), "{needle}\n{ps}");
    }
    let shadow = translate_with(
        &corpus()
            .into_iter()
            .find(|(n, _, _)| *n == "ps_shadow")
            .unwrap()
            .1,
        &Options {
            depth_compare_samplers: 1,
            ..Options::default()
        },
    )
    .unwrap();
    assert!(reflection_summary(&shadow).contains("  s0: 2D, set 1 binding 0, depth compare"));
    let outputs = reflection_summary(&corpus_module("vs_outputs"));
    assert!(
        outputs.contains("  o2 psize0 -> built-in PointSize, mask .x"),
        "{outputs}"
    );
}
