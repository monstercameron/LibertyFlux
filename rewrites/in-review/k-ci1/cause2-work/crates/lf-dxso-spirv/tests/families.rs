//! One or more tests per instruction family. Every program is hand-built
//! with the assembler in `common`, translated, run through the structural
//! validator, and checked for the SPIR-V it should contain.

mod common;

use common::*;
use lf_dxso_spirv::spirv::{builtin, decoration, glsl, mode, op as sop};
use lf_dxso_spirv::{BuiltIn, Error, Options, TextureDim};

/// Vertex program: inputs v0 (position), output o0 (position); `body`
/// writes r0..; finishes with `mov o0, r0`.
fn vs_with(body: impl FnOnce(&mut Asm)) -> Vec<u32> {
    let mut a = Asm::vs_basic();
    a.mov(rd(0), v(0));
    a.mov(rd(1), c(1));
    a.mov(rd(2), c(2));
    body(&mut a);
    a.mov(od(0), r(0));
    a.end()
}

/// Pixel program: input v0 (texcoord0); `body`; finishes with `mov oC0, r0`.
fn ps_with(body: impl FnOnce(&mut Asm)) -> Vec<u32> {
    let mut a = Asm::ps_basic();
    a.mov(rd(0), v(0));
    a.mov(rd(1), c(1));
    a.mov(rd(2), c(2));
    body(&mut a);
    a.mov(oc(0), r(0));
    a.end()
}

#[test]
fn minimal_vertex_and_pixel_shaders() {
    let (m, rep) = check(&vs_with(|_| {}));
    assert_eq!(m.stage, lf_dxso_spirv::Stage::Vertex);
    assert_eq!(rep.functions, 1);
    assert_eq!(m.words[0], 0x0723_0203);
    assert_eq!(m.words[1], 0x0001_0000);
    let (m, _) = check(&ps_with(|_| {}));
    assert_eq!(m.stage, lf_dxso_spirv::Stage::Pixel);
    assert!(execution_modes(&m).contains(&mode::ORIGIN_UPPER_LEFT));
}

#[test]
fn empty_programs_translate() {
    check(&Asm::vs().end());
    check(&Asm::ps().end());
    // nop is accepted and produces nothing.
    check(&Asm::ps().ins(op::NOP, 0, None, &[]).end());
}

#[test]
fn add_sub_mul_mad_mov() {
    let (m, _) = check(&vs_with(|a| {
        a.op2(op::ADD, rd(0), r(0), r(1))
            .op2(op::SUB, rd(0), r(0), r(2))
            .op2(op::MUL, rd(0), r(0), r(1))
            .op3(op::MAD, rd(0), r(0), r(1), r(2));
    }));
    assert_eq!(count(&m, sop::F_ADD), 2 + 1); // add, mad, and the position fixup
    assert_eq!(count(&m, sop::F_SUB), 1);
    assert_eq!(count(&m, sop::F_MUL), 2 + 2); // two here, two in the position fixup
}

#[test]
fn scalar_instructions() {
    let (m, _) = check(&vs_with(|a| {
        a.op1(op::RCP, rd(3), r(1).sw("x"))
            .op1(op::RSQ, rd(3), r(1).sw("y"))
            .op1(op::EXP, rd(3), r(1).sw("z"))
            .op1(op::EXPP, rd(3), r(1).sw("z"))
            .op1(op::LOG, rd(3), r(1).sw("w"))
            .op1(op::LOGP, rd(3), r(1).sw("w"))
            .op2(op::POW, rd(3), r(1).sw("x"), r(2).sw("y"));
    }));
    assert_eq!(count(&m, sop::F_DIV), 2);
    assert_eq!(count_ext(&m, glsl::SQRT), 1);
    assert_eq!(count_ext(&m, glsl::EXP2), 3);
    assert_eq!(count_ext(&m, glsl::LOG2), 3);
    // log(0) = -inf is explicit: one ordered-equal test per log2.
    assert_eq!(count(&m, sop::F_ORD_EQUAL), 3);
}

#[test]
fn dot_products() {
    let (m, _) = check(&ps_with(|a| {
        a.op2(op::DP3, rd(0), r(1), r(2))
            .op2(op::DP4, rd(0), r(1), r(2))
            .op3(op::DP2ADD, rd(0), r(1), r(2), c(3).sw("x"));
    }));
    assert_eq!(count(&m, sop::DOT), 3);
}

#[test]
fn compare_and_misc_vector_ops() {
    let (m, _) = check(&ps_with(|a| {
        a.op2(op::MIN, rd(0), r(1), r(2))
            .op2(op::MAX, rd(0), r(1), r(2))
            .op2(op::SLT, rd(0), r(1), r(2))
            .op2(op::SGE, rd(0), r(1), r(2))
            .op1(op::ABS, rd(0), r(1))
            .op1(op::FRC, rd(0), r(1))
            .op3(op::SGN, rd(0), r(1), r(2), r(3));
    }));
    assert_eq!(count_ext(&m, glsl::FMIN), 1);
    assert_eq!(count_ext(&m, glsl::FMAX), 1);
    assert_eq!(count_ext(&m, glsl::FABS), 1);
    assert_eq!(count_ext(&m, glsl::FRACT), 1);
    assert_eq!(count_ext(&m, glsl::FSIGN), 1);
    assert_eq!(count(&m, sop::F_ORD_LESS_THAN), 1);
    assert_eq!(count(&m, sop::F_ORD_GREATER_THAN_EQUAL), 1);
    assert_eq!(count(&m, sop::SELECT), 2);
}

#[test]
fn sgn_with_one_source_is_model_3_form() {
    check(&ps_with(|a| {
        a.op1(op::SGN, rd(0), r(1));
    }));
}

#[test]
fn crs_nrm_lrp_cmp_cnd() {
    let (m, _) = check(&ps_with(|a| {
        a.op2(op::CRS, rd(0).m("xyz"), r(1), r(2))
            .op1(op::NRM, rd(0), r(1))
            .op3(op::LRP, rd(0), r(1), r(2), r(0))
            .op3(op::CMP, rd(0), r(1), r(2), r(0))
            .op3(op::CND, rd(0), r(1), r(2), r(0));
    }));
    assert_eq!(count_ext(&m, glsl::CROSS), 1);
    assert_eq!(count(&m, sop::F_ORD_GREATER_THAN_EQUAL), 1);
    assert_eq!(count(&m, sop::F_ORD_GREATER_THAN), 1);
    assert_eq!(count(&m, sop::DOT), 1);
}

#[test]
fn sincos_writes_only_x_and_y() {
    let (m, _) = check(&vs_with(|a| {
        a.op1(op::SINCOS, rd(0).m("xy"), r(1).sw("x"))
            .op1(op::SINCOS, rd(0).m("x"), r(1).sw("y"))
            .op1(op::SINCOS, rd(0).m("y"), r(1).sw("z"))
            // A mask with z/w: those components stay unwritten.
            .op1(op::SINCOS, rd(3).m("xyzw"), r(1).sw("w"));
        // Model 2 form with two extra operands is accepted too.
        a.op3(op::SINCOS, rd(0).m("xy"), r(1).sw("x"), c(5), c(6));
    }));
    assert_eq!(count_ext(&m, glsl::SIN), 5);
    assert_eq!(count_ext(&m, glsl::COS), 5);
    // Every sincos is a masked write (load, shuffle, store).
    assert!(count(&m, sop::VECTOR_SHUFFLE) >= 5);
}

#[test]
fn matrix_instructions_use_one_dot_per_row() {
    for (opcode, rows, mask_text) in [
        (op::M4X4, 4, "xyzw"),
        (op::M4X3, 3, "xyz"),
        (op::M3X4, 4, "xyzw"),
        (op::M3X3, 3, "xyz"),
        (op::M3X2, 2, "xy"),
    ] {
        let (m, _) = check(&vs_with(|a| {
            a.op2(opcode, rd(0).m(mask_text), r(0), c(10));
        }));
        assert_eq!(count(&m, sop::DOT), rows, "opcode {opcode}");
        // Rows are consecutive constants c10.. read from the buffer.
        let want: Vec<u16> = (10..).take(rows).collect();
        let got: Vec<u16> = m
            .constants
            .float_read
            .iter()
            .copied()
            .filter(|&x| x >= 10)
            .collect();
        assert_eq!(got, want, "opcode {opcode}");
    }
}

#[test]
fn dst_and_lit() {
    let (m, _) = check(&vs_with(|a| {
        a.op2(op::DST, rd(0), r(1), r(2)).op1(op::LIT, rd(0), r(1));
    }));
    // lit: clamp of the exponent and exp2/log2 power.
    assert_eq!(count_ext(&m, glsl::FMIN), 1);
    assert_eq!(count_ext(&m, glsl::FMAX), 1);
    assert_eq!(count_ext(&m, glsl::EXP2), 1);
    assert_eq!(count(&m, sop::LOGICAL_AND), 1);
}

#[test]
fn source_modifiers() {
    let (m, _) = check(&ps_with(|a| {
        a.mov(rd(0), r(1).md(sm::NEG))
            .mov(rd(0), r(1).md(sm::ABS))
            .mov(rd(0), r(1).md(sm::ABSNEG))
            .mov(rd(0), r(1).md(sm::BIAS))
            .mov(rd(0), r(1).md(sm::BIASNEG))
            .mov(rd(0), r(1).md(sm::SIGN))
            .mov(rd(0), r(1).md(sm::SIGNNEG))
            .mov(rd(0), r(1).md(sm::COMP))
            .mov(rd(0), r(1).md(sm::X2))
            .mov(rd(0), r(1).md(sm::X2NEG));
    }));
    assert_eq!(count(&m, sop::F_NEGATE), 5);
    assert_eq!(count_ext(&m, glsl::FABS), 2);
}

#[test]
fn dz_dw_and_not_on_floats_are_rejected() {
    let e = fail(&ps_with(|a| {
        a.mov(rd(0), r(1).md(sm::DZ));
    }));
    assert!(e.is_unsupported(), "{e}");
    let e = fail(&ps_with(|a| {
        a.mov(rd(0), r(1).md(sm::DW));
    }));
    assert!(e.is_unsupported(), "{e}");
    let e = fail(&ps_with(|a| {
        a.mov(rd(0), r(1).md(sm::NOT));
    }));
    assert!(matches!(e, Error::Invalid { .. }), "{e}");
}

#[test]
fn saturate_maps_nan_to_zero_with_two_selects() {
    let (m, _) = check(&ps_with(|a| {
        a.mov(rd(0).sat(), r(1));
    }));
    assert_eq!(count(&m, sop::SELECT), 2);
    assert_eq!(count(&m, sop::F_ORD_LESS_THAN), 1);
    assert_eq!(count(&m, sop::F_ORD_GREATER_THAN), 1);
}

#[test]
fn partial_precision_is_accepted_and_shift_rejected() {
    check(&ps_with(|a| {
        a.mov(rd(0).pp(), r(1));
    }));
    let e = fail(&ps_with(|a| {
        a.mov(rd(0).shift(1), r(1));
    }));
    assert!(e.is_unsupported(), "{e}");
}

#[test]
fn swizzles_and_write_masks() {
    let (m, _) = check(&ps_with(|a| {
        a.mov(rd(0), r(1).sw("wzyx"));
    }));
    assert_eq!(count(&m, sop::VECTOR_SHUFFLE), 1);
    let (m, _) = check(&ps_with(|a| {
        a.mov(rd(0).m("xz"), r(1));
    }));
    let shuffles: Vec<Vec<u32>> = lf_dxso_spirv::validate::instructions(&m.words)
        .unwrap()
        .iter()
        .filter(|i| i.opcode == sop::VECTOR_SHUFFLE)
        .map(|i| i.operands[4..].to_vec())
        .collect();
    assert_eq!(shuffles, vec![vec![0, 5, 2, 7]]);
    // An empty write mask writes nothing.
    check(&ps_with(|a| {
        a.mov(rd(0).raw_mask(0), r(1));
    }));
}

#[test]
fn def_overrides_constant_reads() {
    let (m, _) = check(&vs_with(|a| {
        a.def(5, [1.0, 2.0, 3.0, 4.0]);
        a.op2(op::ADD, rd(0), c(5), c(6));
    }));
    assert_eq!(m.constants.float_defined, vec![5]);
    assert!(m.constants.float_read.contains(&6));
    assert!(!m.constants.float_read.contains(&5));
    assert!(!m.constants.float_relative);
    assert_eq!(m.float_constant_binding, Some(0));
}

#[test]
fn def_keeps_negative_zero_and_nan_bits() {
    let mut a = Asm::ps();
    a.words.extend([
        0x51 | (5 << 24),
        regbits(reg::CONST, 0) | (0xF << 16),
        0x8000_0000,
        0x7FC0_1234,
        0x3F80_0000,
        0,
    ]);
    a.mov(oc(0), c(0));
    let (m, _) = check(&a.end());
    let consts: Vec<u32> = lf_dxso_spirv::validate::instructions(&m.words)
        .unwrap()
        .iter()
        .filter(|i| i.opcode == sop::CONSTANT)
        .map(|i| i.operands[2])
        .collect();
    assert!(consts.contains(&0x8000_0000));
    assert!(consts.contains(&0x7FC0_1234));
}

#[test]
fn relative_constant_addressing_with_a0() {
    let (m, _) = check(&vs_with(|a| {
        a.def(12, [9.0, 9.0, 9.0, 9.0]);
        a.op1(op::MOVA, d(reg::ADDR, 0).m("x"), r(1).sw("x"));
        a.mov(rd(0), c(10).rel(A0X));
    }));
    assert!(m.constants.float_relative);
    assert_eq!(count_ext(&m, glsl::SCLAMP), 1);
    assert_eq!(count(&m, sop::CONVERT_F_TO_S), 1);
    assert_eq!(count_ext(&m, glsl::FLOOR), 1);
    // The def'd register is selected by index equality.
    assert_eq!(count(&m, sop::I_EQUAL), 1);
}

#[test]
fn relative_addressing_rules() {
    // a0 in a pixel shader is invalid.
    let e = fail(&ps_with(|a| {
        a.mov(rd(0), c(10).rel(A0X));
    }));
    assert!(matches!(e, Error::Invalid { .. }), "{e}");
    // Relative addressing on temps is invalid.
    let e = fail(&vs_with(|a| {
        a.mov(rd(0), r(1).rel(A0X));
    }));
    assert!(matches!(e, Error::Invalid { .. }), "{e}");
}

#[test]
fn bool_and_int_constants_and_if_else() {
    let (m, _) = check(&ps_with(|a| {
        a.flow(op::IF, 0, &[b(3)]);
        a.mov(rd(0), r(1));
        a.flow(op::ELSE, 0, &[]);
        a.mov(rd(0), r(2));
        a.flow(op::ENDIF, 0, &[]);
        a.flow(op::IF, 0, &[b(4).not()]);
        a.mov(rd(0), r(1));
        a.flow(op::ENDIF, 0, &[]);
    }));
    assert_eq!(count(&m, sop::SELECTION_MERGE), 2);
    assert_eq!(count(&m, sop::BRANCH_CONDITIONAL), 2);
    assert_eq!(count(&m, sop::SHIFT_RIGHT_LOGICAL), 2);
    assert_eq!(count(&m, sop::LOGICAL_NOT), 1);
    assert_eq!(m.constants.bool_read, vec![3, 4]);
    assert_eq!(m.int_bool_constant_binding, Some(3));
}

#[test]
fn defb_overrides_bool_reads() {
    let (m, _) = check(&ps_with(|a| {
        a.defb(2, true);
        a.flow(op::IF, 0, &[b(2)]);
        a.mov(rd(0), r(1));
        a.flow(op::ENDIF, 0, &[]);
    }));
    assert!(m.constants.bool_read.is_empty());
    assert_eq!(m.constants.bool_defined, vec![2]);
    assert_eq!(m.int_bool_constant_binding, None);
    assert!(has(&m, sop::CONSTANT_TRUE));
}

#[test]
fn ifc_comparisons() {
    for (code, opcode) in [
        (cmp::GT, sop::F_ORD_GREATER_THAN),
        (cmp::EQ, sop::F_ORD_EQUAL),
        (cmp::GE, sop::F_ORD_GREATER_THAN_EQUAL),
        (cmp::LT, sop::F_ORD_LESS_THAN),
        (cmp::NE, sop::F_UNORD_NOT_EQUAL),
        (cmp::LE, sop::F_ORD_LESS_THAN_EQUAL),
    ] {
        let (m, _) = check(&ps_with(|a| {
            a.flow(op::IFC, code, &[r(1).sw("x"), r(2).sw("y")]);
            a.mov(rd(0), r(1));
            a.flow(op::ENDIF, 0, &[]);
        }));
        assert_eq!(count(&m, opcode), 1, "comparison {code}");
    }
    for bad in [0, 7] {
        let e = fail(&ps_with(|a| {
            a.flow(op::IFC, bad, &[r(1), r(2)]);
            a.flow(op::ENDIF, 0, &[]);
        }));
        assert!(matches!(e, Error::Invalid { .. }), "{e}");
    }
}

#[test]
fn unbalanced_flow_control_is_invalid() {
    for body in [
        vec![(op::ELSE, vec![])],
        vec![(op::ENDIF, vec![])],
        vec![(op::ENDLOOP, vec![])],
        vec![(op::ENDREP, vec![])],
        vec![(op::BREAK, vec![])],
        vec![(op::IF, vec![b(0)])],
        vec![(op::IF, vec![b(0)]), (op::ELSE, vec![]), (op::ELSE, vec![])],
        vec![(op::REP, vec![i(0)]), (op::ENDLOOP, vec![])],
        vec![(op::LOOP, vec![al(), i(0)]), (op::ENDREP, vec![])],
    ] {
        let e = fail(&ps_with(|a| {
            for (o, srcs) in &body {
                a.flow(*o, 0, srcs);
            }
        }));
        assert!(matches!(e, Error::Invalid { .. }), "{body:?}: {e}");
    }
}

#[test]
fn rep_loop_and_breaks() {
    let (m, rep) = check(&vs_with(|a| {
        a.defi(0, [4, 0, 1, 0]);
        a.flow(op::REP, 0, &[i(0)]);
        a.op2(op::ADD, rd(0), r(0), r(1));
        a.flow(op::BREAKC, cmp::GT, &[r(0).sw("x"), c(3).sw("x")]);
        a.flow(op::ENDREP, 0, &[]);
        a.flow(op::LOOP, 0, &[al(), i(1)]);
        a.op2(op::ADD, rd(0), r(0), c(20).rel(AL));
        a.flow(op::IF, 0, &[b(0)]);
        a.flow(op::BREAK, 0, &[]);
        a.flow(op::ENDIF, 0, &[]);
        a.flow(op::ENDLOOP, 0, &[]);
    }));
    assert_eq!(count(&m, sop::LOOP_MERGE), 2);
    assert_eq!(m.constants.int_defined, vec![0]);
    assert_eq!(m.constants.int_read, vec![1]);
    assert!(m.constants.float_relative);
    assert!(rep.blocks > 8);
}

#[test]
fn breakp_and_nested_loops() {
    let (m, _) = check(&ps_with(|a| {
        a.defi(0, [3, 0, 1, 0]);
        a.ins(op::SETP, cmp::LT, Some(d(reg::PRED, 0)), &[r(1), r(2)]);
        a.flow(op::LOOP, 0, &[al(), i(0)]);
        a.flow(op::REP, 0, &[i(0)]);
        a.flow(op::BREAKP, 0, &[p0().sw("y")]);
        a.op2(op::ADD, rd(0), r(0), v(0));
        a.flow(op::ENDREP, 0, &[]);
        a.flow(op::BREAKP, 0, &[p0().sw("x").not()]);
        a.flow(op::ENDLOOP, 0, &[]);
    }));
    assert_eq!(count(&m, sop::LOOP_MERGE), 2);
    // aL is saved and restored around the loop.
    assert!(named(&m, "aL_saved").is_some());
}

#[test]
fn subroutines_call_callnz_label_ret() {
    let mut a = Asm::vs_basic();
    a.mov(rd(0), v(0));
    a.flow(op::CALL, 0, &[l(1)]);
    a.flow(op::CALLNZ, 0, &[l(2), b(0)]);
    a.ins(op::SETP, cmp::GT, Some(d(reg::PRED, 0)), &[r(0), c(0)]);
    a.flow(op::CALLNZ, 0, &[l(2), p0().sw("x").not()]);
    a.mov(od(0), r(0));
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(1)]);
    a.op2(op::ADD, rd(0), r(0), c(1));
    a.flow(op::CALL, 0, &[l(2)]);
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(2)]);
    a.op2(op::MUL, rd(0), r(0), c(2));
    a.flow(op::RET, 0, &[]);
    let (m, rep) = check(&a.end());
    assert_eq!(rep.functions, 3);
    assert_eq!(count(&m, sop::FUNCTION_CALL), 4);
}

#[test]
fn output_written_in_a_subroutine_reaches_the_interface() {
    let mut a = Asm::ps_basic();
    a.flow(op::CALL, 0, &[l(0)]);
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(0)]);
    a.mov(oc(1), v(0));
    a.flow(op::RET, 0, &[]);
    let (m, _) = check(&a.end());
    assert!(m.outputs.iter().any(|o| o.location == Some(1)));
}

#[test]
fn call_errors() {
    // Undefined label.
    let mut a = Asm::ps_basic();
    a.flow(op::CALL, 0, &[l(7)]);
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
    // Recursion.
    let mut a = Asm::ps_basic();
    a.flow(op::CALL, 0, &[l(0)]);
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(0)]);
    a.flow(op::CALL, 0, &[l(1)]);
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(1)]);
    a.flow(op::CALL, 0, &[l(0)]);
    a.flow(op::RET, 0, &[]);
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
    // Label defined twice.
    let mut a = Asm::ps_basic();
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(0)]);
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(0)]);
    a.flow(op::RET, 0, &[]);
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
    // Instruction after the last ret.
    let mut a = Asm::ps_basic();
    a.flow(op::RET, 0, &[]);
    a.mov(rd(0), v(0));
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
}

#[test]
fn early_return_inside_flow_control() {
    let mut a = Asm::vs_basic();
    a.mov(od(0), v(0));
    a.flow(op::IF, 0, &[b(0)]);
    a.flow(op::RET, 0, &[]);
    a.flow(op::ENDIF, 0, &[]);
    a.mov(od(0), c(0));
    let (m, _) = check(&a.end());
    assert_eq!(count(&m, sop::RETURN), 2);
}

#[test]
fn predication_setp_and_predicated_writes() {
    let (m, _) = check(&ps_with(|a| {
        a.ins(
            op::SETP,
            cmp::GE,
            Some(d(reg::PRED, 0).m("xy")),
            &[r(1), r(2)],
        );
        a.pred(op::MOV, p0().sw("x"), rd(0), &[r(1)]);
        a.pred(op::ADD, p0().not(), rd(0).m("xz"), &[r(1), r(2)]);
    }));
    // One select per predicated write; the bvec4 predicate is negated once.
    assert_eq!(count(&m, sop::SELECT), 2);
    assert_eq!(count(&m, sop::LOGICAL_NOT), 1);
    assert_eq!(count(&m, sop::F_ORD_GREATER_THAN_EQUAL), 1);
}

#[test]
fn predicated_flow_control_is_unsupported() {
    let e = fail(&ps_with(|a| {
        a.ins_full(op::IF, 0, None, Some(p0()), &[b(0)]);
        a.flow(op::ENDIF, 0, &[]);
    }));
    assert!(e.is_unsupported(), "{e}");
}

#[test]
fn texture_sampling_variants() {
    let (m, _) = check(&{
        let mut a = Asm::ps_basic();
        a.dcl_sampler(2, 0).dcl_sampler(3, 1).dcl_sampler(4, 2);
        a.op2(op::TEX, rd(0), v(0), sampler(0));
        a.op2(op::TEX, rd(1), v(0), sampler(1));
        a.op2(op::TEX, rd(2), v(0), sampler(2));
        a.ins(op::TEX, 1, Some(rd(3)), &[v(0), sampler(0)]); // texldp
        a.ins(op::TEX, 2, Some(rd(3)), &[v(0), sampler(0)]); // texldb
        a.op2(op::TEXLDL, rd(3), v(0), sampler(1));
        a.ins(op::TEXLDD, 0, Some(rd(3)), &[v(0), sampler(2), c(0), c(1)]);
        a.op2(op::TEX, rd(3), v(0), sampler(0).sw("wzyx"));
        a.mov(oc(0), r(0));
        a.end()
    });
    assert_eq!(count(&m, sop::IMAGE_SAMPLE_IMPLICIT_LOD), 6);
    assert_eq!(count(&m, sop::IMAGE_SAMPLE_EXPLICIT_LOD), 2);
    assert_eq!(count(&m, sop::F_DIV), 1);
    let dims: Vec<(u16, TextureDim, u32)> = m
        .samplers
        .iter()
        .map(|s| (s.register, s.dim, s.binding))
        .collect();
    assert_eq!(
        dims,
        vec![
            (0, TextureDim::D2, 0),
            (1, TextureDim::Cube, 1),
            (2, TextureDim::D3, 2)
        ]
    );
    assert!(m.samplers.iter().all(|s| s.set == 1 && !s.depth_compare));
    assert_eq!(decoration_of(&m, "s1", decoration::BINDING), Some(vec![1]));
    assert_eq!(
        decoration_of(&m, "s1", decoration::DESCRIPTOR_SET),
        Some(vec![1])
    );
}

#[test]
fn vertex_texture_fetch() {
    let mut a = Asm::vs_basic();
    a.dcl_sampler(2, 1);
    a.op2(op::TEXLDL, rd(0), v(0), sampler(1));
    a.mov(od(0), r(0));
    let (m, _) = check(&a.end());
    assert_eq!(m.samplers[0].binding, 17);
    assert_eq!(count(&m, sop::IMAGE_SAMPLE_EXPLICIT_LOD), 1);
    // Implicit level of detail is not available in vertex shaders.
    let mut a = Asm::vs_basic();
    a.dcl_sampler(2, 0);
    a.op2(op::TEX, rd(0), v(0), sampler(0));
    assert!(fail(&a.end()).is_unsupported());
}

#[test]
fn texture_errors() {
    // Sampler without dcl.
    let e = fail(&ps_with(|a| {
        a.op2(op::TEX, rd(0), v(0), sampler(5));
    }));
    assert!(matches!(e, Error::Invalid { .. }), "{e}");
    // Unknown texture type.
    let mut a = Asm::ps_basic();
    a.dcl_sampler(0, 0);
    assert!(fail(&a.end()).is_unsupported());
    // Project and bias together.
    let mut a = Asm::ps_basic();
    a.dcl_sampler(2, 0);
    a.ins(op::TEX, 3, Some(rd(0)), &[v(0), sampler(0)]);
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
}

#[test]
fn depth_compare_samplers_use_dref() {
    let mut a = Asm::ps_basic();
    a.dcl_sampler(2, 3);
    a.op2(op::TEX, rd(0), v(0), sampler(3));
    a.ins(op::TEX, 1, Some(rd(1)), &[v(0), sampler(3)]);
    a.op2(op::TEXLDL, rd(1), v(0), sampler(3));
    a.mov(oc(0), r(0));
    let opts = Options {
        depth_compare_samplers: 1 << 3,
        ..Options::default()
    };
    let (m, _) = check_with(&a.end(), opts);
    assert_eq!(count(&m, sop::IMAGE_SAMPLE_DREF_IMPLICIT_LOD), 2);
    assert_eq!(count(&m, sop::IMAGE_SAMPLE_DREF_EXPLICIT_LOD), 1);
    assert!(m.samplers[0].depth_compare);
    // Cube depth compare is not supported.
    let mut a = Asm::ps_basic();
    a.dcl_sampler(3, 3);
    let e = lf_dxso_spirv::translate_with(&a.end(), &opts).unwrap_err();
    assert!(e.is_unsupported());
}

#[test]
fn texkill_and_derivatives() {
    let (m, _) = check(&ps_with(|a| {
        a.ins(op::TEXKILL, 0, Some(rd(1).m("xyz")), &[]);
        a.op1(op::DSX, rd(0), r(1));
        a.op1(op::DSY, rd(0), r(1));
        a.pred(op::TEXKILL, p0().sw("x"), rd(2), &[]);
    }));
    assert_eq!(count(&m, sop::KILL), 2);
    assert_eq!(count(&m, sop::DPDX), 1);
    assert_eq!(count(&m, sop::DPDY), 1);
    // texkill .xyz checks three components.
    assert!(count(&m, sop::LOGICAL_OR) >= 2);
    for opcode in [op::DSX, op::DSY] {
        let e = fail(&vs_with(|a| {
            a.op1(opcode, rd(0), r(1));
        }));
        assert!(matches!(e, Error::Invalid { .. }), "{e}");
    }
    let e = fail(&vs_with(|a| {
        a.ins(op::TEXKILL, 0, Some(rd(1)), &[]);
    }));
    assert!(matches!(e, Error::Invalid { .. }), "{e}");
}

#[test]
fn pixel_outputs_colors_and_depth() {
    let mut a = Asm::ps_basic();
    for n in 0..4 {
        a.mov(oc(n), v(0));
    }
    a.mov(d(reg::DEPTHOUT, 0), v(0).sw("z"));
    let (m, _) = check(&a.end());
    for n in 0..4u32 {
        assert_eq!(
            decoration_of(&m, &format!("oC{n}"), decoration::LOCATION),
            Some(vec![n])
        );
    }
    assert_eq!(
        decoration_of(&m, "oDepth", decoration::BUILT_IN),
        Some(vec![builtin::FRAG_DEPTH])
    );
    assert!(execution_modes(&m).contains(&mode::DEPTH_REPLACING));
    assert!(m.depth_replacing);
    assert!(
        m.outputs
            .iter()
            .any(|o| o.builtin == Some(BuiltIn::FragDepth))
    );
}

#[test]
fn vpos_and_vface() {
    let mut a = Asm::ps();
    a.dcl_misc(0).dcl_misc(1);
    a.op2(op::ADD, rd(0), s(reg::MISC, 0), s(reg::MISC, 1));
    a.mov(oc(0), r(0));
    let (m, _) = check(&a.end());
    assert_eq!(
        decoration_of(&m, "vPos", decoration::BUILT_IN),
        Some(vec![builtin::FRAG_COORD])
    );
    assert_eq!(
        decoration_of(&m, "vFace", decoration::BUILT_IN),
        Some(vec![builtin::FRONT_FACING])
    );
    // vPos.xy - 0.5 and vFace ? 1 : -1.
    assert_eq!(count(&m, sop::F_SUB), 1);
    assert_eq!(count(&m, sop::SELECT), 1);
    // Reading vPos without dcl is invalid.
    let mut a = Asm::ps();
    a.mov(oc(0), s(reg::MISC, 0));
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
}

#[test]
fn vertex_outputs_position_psize_fog_and_varyings() {
    let mut a = Asm::vs();
    a.dcl(usage::POSITION, 0, vd(0))
        .dcl(usage::TEXCOORD, 0, vd(1))
        .dcl(usage::POSITION, 0, od(0))
        .dcl(usage::PSIZE, 0, od(1).m("x"))
        .dcl(usage::FOG, 0, od(2).m("x"))
        .dcl(usage::TEXCOORD, 3, od(3))
        .dcl(usage::COLOR, 1, od(4));
    a.mov(od(0), v(0))
        .mov(od(1).m("x"), c(0))
        .mov(od(2).m("x"), c(0).sw("y"))
        .mov(od(3), v(1))
        .mov(od(4), c(1));
    let (m, _) = check(&a.end());
    assert_eq!(m.push_constant_size, 16);
    let outs: Vec<(Option<u32>, Option<BuiltIn>)> =
        m.outputs.iter().map(|o| (o.location, o.builtin)).collect();
    assert_eq!(
        outs,
        vec![
            (None, Some(BuiltIn::Position)),
            (None, Some(BuiltIn::PointSize)),
            (Some(12), None),
            (Some(3), None),
            (Some(11), None),
        ]
    );
    let ins: Vec<Option<u32>> = m.inputs.iter().map(|i| i.location).collect();
    assert_eq!(ins, vec![Some(0), Some(6)]);
    assert_eq!(
        decoration_of(&m, "o0_position0", decoration::BUILT_IN),
        Some(vec![builtin::POSITION])
    );
    assert_eq!(
        decoration_of(&m, "o1_psize0", decoration::BUILT_IN),
        Some(vec![builtin::POINT_SIZE])
    );
}

#[test]
fn position_fixup_can_be_turned_off() {
    let opts = Options {
        position_fixup: false,
        ..Options::default()
    };
    let (m, _) = check_with(&vs_with(|_| {}), opts);
    assert_eq!(m.push_constant_size, 0);
    assert!(named(&m, "push").is_none());
}

#[test]
fn packed_output_and_input_registers() {
    // Two semantics in one output register.
    let mut a = Asm::vs_basic();
    a.dcl(usage::TEXCOORD, 0, od(1).m("xy"))
        .dcl(usage::TEXCOORD, 1, od(1).m("zw"));
    a.mov(od(0), v(0)).mov(od(1), c(0));
    let (m, _) = check(&a.end());
    assert_eq!(m.outputs.len(), 3);
    // ...and the pixel side reading both from one register.
    let mut a = Asm::ps();
    a.dcl(usage::TEXCOORD, 0, vd(0).m("xy"))
        .dcl(usage::TEXCOORD, 1, vd(0).m("zw"));
    a.mov(oc(0), v(0));
    let (m, _) = check(&a.end());
    assert_eq!(m.inputs.len(), 2);
    let shuffles: Vec<Vec<u32>> = lf_dxso_spirv::validate::instructions(&m.words)
        .unwrap()
        .iter()
        .filter(|i| i.opcode == sop::VECTOR_SHUFFLE)
        .map(|i| i.operands[4..].to_vec())
        .collect();
    assert_eq!(shuffles, vec![vec![0, 1, 6, 7]]);
}

#[test]
fn relative_output_and_input_addressing() {
    let mut a = Asm::vs_basic();
    a.dcl(usage::TEXCOORD, 0, od(1))
        .dcl(usage::TEXCOORD, 1, od(2));
    a.defi(0, [2, 1, 1, 0]);
    a.mov(od(0), v(0));
    a.flow(op::LOOP, 0, &[al(), i(0)]);
    a.mov(od(0).rel(AL), c(0));
    a.flow(op::ENDLOOP, 0, &[]);
    let (m, _) = check(&a.end());
    assert!(named(&m, "o").is_some());

    let mut a = Asm::ps();
    a.dcl(usage::TEXCOORD, 0, vd(0))
        .dcl(usage::TEXCOORD, 1, vd(1));
    a.defi(0, [2, 0, 1, 0]);
    a.mov(rd(0), c(0));
    a.flow(op::LOOP, 0, &[al(), i(0)]);
    a.op2(op::ADD, rd(0), r(0), v(0).rel(AL));
    a.flow(op::ENDLOOP, 0, &[]);
    a.mov(oc(0), r(0));
    let (m, _) = check(&a.end());
    assert!(named(&m, "v").is_some());
}

#[test]
fn centroid_inputs_are_decorated() {
    let mut a = Asm::ps();
    a.dcl(usage::TEXCOORD, 2, vd(0).centroid());
    a.mov(oc(0), v(0));
    let (m, _) = check(&a.end());
    assert_eq!(
        decoration_of(&m, "v0_texcoord2", decoration::CENTROID),
        Some(vec![])
    );
    assert_eq!(
        decoration_of(&m, "v0_texcoord2", decoration::LOCATION),
        Some(vec![2])
    );
    assert!(m.inputs[0].centroid);
}

#[test]
fn varying_locations_agree_between_stages() {
    let mut vs = Asm::vs_basic();
    vs.dcl(usage::TEXCOORD, 5, od(3))
        .dcl(usage::COLOR, 0, od(7))
        .dcl(usage::NORMAL, 0, od(2));
    vs.mov(od(0), v(0))
        .mov(od(3), c(0))
        .mov(od(7), c(1))
        .mov(od(2), c(2));
    let (vm, _) = check(&vs.end());
    let mut ps = Asm::ps();
    ps.dcl(usage::NORMAL, 0, vd(9))
        .dcl(usage::TEXCOORD, 5, vd(0))
        .dcl(usage::COLOR, 0, vd(4));
    ps.op2(op::ADD, rd(0), v(0), v(4))
        .op2(op::ADD, rd(0), r(0), v(9));
    ps.mov(oc(0), r(0));
    let (pm, _) = check(&ps.end());
    for usage_pair in [(5u8, 5u8), (10, 0), (3, 0)] {
        let vl = vm
            .outputs
            .iter()
            .find(|o| o.usage == Some(usage_pair))
            .unwrap();
        let pl = pm
            .inputs
            .iter()
            .find(|o| o.usage == Some(usage_pair))
            .unwrap();
        assert_eq!(vl.location, pl.location, "{usage_pair:?}");
    }
}

#[test]
fn missing_declarations_are_invalid() {
    // Undeclared input.
    let mut a = Asm::ps();
    a.mov(oc(0), v(3));
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
    // Undeclared output.
    let mut a = Asm::vs();
    a.mov(od(3), c(0));
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
    // Out-of-range temp and constant.
    let mut a = Asm::ps();
    a.mov(rd(32), c(0));
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
    let mut a = Asm::ps();
    a.mov(rd(0), c(224));
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
    // Duplicate usage.
    let mut a = Asm::ps();
    a.dcl(usage::TEXCOORD, 0, vd(0))
        .dcl(usage::TEXCOORD, 0, vd(1));
    assert!(matches!(fail(&a.end()), Error::Invalid { .. }));
}

#[test]
fn unsupported_constructs_are_reported() {
    // Shader model 2.
    let e = fail(&[0xFFFF_0200, END]);
    assert_eq!(e, Error::UnsupportedVersion { major: 2, minor: 0 });
    assert!(e.is_unsupported());
    // vs_3_sw.
    let e = fail(&[0xFFFE_03FF, END]);
    assert!(e.is_unsupported());
    // Pixel shader 1.x instruction.
    let e = fail(&Asm::ps().ins(op::TEXBEM, 0, Some(rd(0)), &[r(1)]).end());
    assert!(e.is_unsupported(), "{e}");
    // Co-issue bit.
    let mut a = Asm::ps();
    a.raw(op::MOV | (2 << 24) | (1 << 30))
        .raw(regbits(reg::TEMP, 0) | (0xF << 16))
        .raw(regbits(reg::TEMP, 1) | (0xE4 << 16));
    assert!(fail(&a.end()).is_unsupported());
    // RASTOUT register file (vertex shaders before 3.0).
    let e = fail(&Asm::vs().mov(d(reg::RASTOUT, 0), c(0)).end());
    assert!(e.is_unsupported(), "{e}");
    // Vertex input without a location.
    let e = fail(&Asm::vs().dcl(usage::TEXCOORD, 8, vd(0)).end());
    assert!(e.is_unsupported(), "{e}");
    let e = fail(&Asm::vs().dcl(usage::PSIZE, 0, vd(0)).end());
    assert!(e.is_unsupported(), "{e}");
    // Pixel input position0.
    let e = fail(&Asm::ps().dcl(usage::POSITION, 0, vd(0)).end());
    assert!(e.is_unsupported(), "{e}");
    // Reading the loop counter as a float.
    let e = fail(&ps_with(|a| {
        a.mov(rd(0), al());
    }));
    assert!(e.is_unsupported(), "{e}");
}

#[test]
fn ctab_comment_is_skipped() {
    let mut a = Asm::ps_basic();
    a.comment(&[u32::from_le_bytes(*b"CTAB"), 1, 2, 3]);
    a.mov(oc(0), v(0));
    a.comment(&[]);
    let words = a.end();
    let sh = lf_dxso_spirv::decode(&words).unwrap();
    assert!(sh.has_ctab);
    assert_eq!(sh.instructions.len(), 2);
    check(&words);
}

#[test]
fn byte_input_matches_word_input() {
    let words = vs_with(|a| {
        a.op2(op::DP4, rd(0), r(0), c(4));
    });
    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let a = lf_dxso_spirv::translate(&words).unwrap();
    let b = lf_dxso_spirv::translate_bytes(&bytes).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.to_bytes().len(), a.words.len() * 4);
    assert!(lf_dxso_spirv::translate_bytes(&bytes[..bytes.len() - 1]).is_err());
}

#[test]
fn translation_is_deterministic() {
    let words = ps_with(|a| {
        a.op2(op::DP3, rd(0), r(1), r(2)).mov(rd(0).sat(), r(0));
    });
    let a = lf_dxso_spirv::translate(&words).unwrap();
    let b = lf_dxso_spirv::translate(&words).unwrap();
    assert_eq!(a.words, b.words);
}

#[test]
fn location_tables_are_consistent() {
    use lf_dxso_spirv::binding::{VARYING_LOCATIONS, VERTEX_INPUT_LOCATIONS};
    for table in [VERTEX_INPUT_LOCATIONS, VARYING_LOCATIONS] {
        let mut keys: Vec<(u8, u8)> = table.iter().map(|&(u, i, _)| (u, i)).collect();
        let mut locs: Vec<u32> = table.iter().map(|&(_, _, l)| l).collect();
        let n = table.len();
        keys.sort_unstable();
        keys.dedup();
        locs.sort_unstable();
        locs.dedup();
        assert_eq!(keys.len(), n, "duplicate semantic");
        assert_eq!(locs.len(), n, "duplicate location");
        // Locations are dense from zero.
        assert_eq!(locs, (0..len_u32(n)).collect::<Vec<_>>());
    }
    assert_eq!(VERTEX_INPUT_LOCATIONS.len(), 16);
    // The common varyings fit Vulkan's guaranteed 16 locations.
    for (u, i) in [
        (5u8, 0u8),
        (5, 9),
        (10, 0),
        (10, 1),
        (11, 0),
        (3, 0),
        (6, 0),
        (7, 0),
    ] {
        assert!(lf_dxso_spirv::binding::varying_location(u, i).unwrap() < 16);
    }
}
