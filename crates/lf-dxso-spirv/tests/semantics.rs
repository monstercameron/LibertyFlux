//! Semantic tests: translated programs are executed by the interpreter in
//! `common::interp` and their outputs compared with values worked out by
//! hand from the Direct3D 9 instruction definitions. A mutant test at the
//! end shows these checks see a wrong comparison.

// Results are compared bit-exactly where Direct3D 9 defines them exactly.
#![allow(clippy::float_cmp)]

mod common;

use common::interp::Machine;
use common::*;
use lf_dxso_spirv::Options;

const INF: f32 = f32::INFINITY;
const NAN: f32 = f32::NAN;

/// A source modifier and the function it applies.
type Case = (u32, fn(f32) -> f32);

fn same(a: f32, b: f32) -> bool {
    (a.is_nan() && b.is_nan()) || a == b || (a - b).abs() <= 1e-5 * b.abs().max(1.0)
}

#[track_caller]
fn assert_vec(got: [f32; 4], want: [f32; 4]) {
    assert!(
        got.iter().zip(&want).all(|(&g, &w)| same(g, w)),
        "got {got:?}, want {want:?}"
    );
}

/// Run a pixel program built as `dcl_texcoord0 v0; <body>; mov oC0, r0`.
fn ps(body: impl FnOnce(&mut Asm), v0: [f32; 4], consts: &[(usize, [f32; 4])]) -> [f32; 4] {
    ps_full(body, v0, consts, &[], 0).0
}

fn ps_full(
    body: impl FnOnce(&mut Asm),
    v0: [f32; 4],
    consts: &[(usize, [f32; 4])],
    ints: &[(usize, [i32; 4])],
    bools: u32,
) -> ([f32; 4], bool) {
    let mut a = Asm::ps_basic();
    body(&mut a);
    a.mov(oc(0), r(0));
    let (m, _) = check(&a.end());
    let mut mach = Machine::new(&m.words);
    mach.set_vec4("v0_texcoord0", v0);
    mach.set_c(consts);
    mach.set_ib(ints, bools);
    mach.run();
    let killed = mach.killed;
    let out = if killed {
        [0.0; 4]
    } else {
        mach.get_vec4("oC0")
    };
    (out, killed)
}

/// Run a vertex program `dcl_position v0; dcl_position o0; dcl_texcoord0 o1; <body>`
/// and return (o0 position after fixup, o1).
fn vs(
    body: impl FnOnce(&mut Asm),
    v0: [f32; 4],
    consts: &[(usize, [f32; 4])],
    ints: &[(usize, [i32; 4])],
) -> ([f32; 4], [f32; 4]) {
    let mut a = Asm::vs_basic();
    a.dcl(usage::TEXCOORD, 0, od(1));
    body(&mut a);
    let (m, _) = check(&a.end());
    let mut mach = Machine::new(&m.words);
    mach.set_vec4("v0_position0", v0);
    mach.set_c(consts);
    mach.set_ib(ints, 0);
    mach.set_push([1.0, 1.0, 0.0, 0.0]);
    mach.run();
    (mach.get_vec4("o0_position0"), mach.get_vec4("o1_texcoord0"))
}

#[test]
fn slt_sge_with_nan() {
    let c1 = (1, [2.0; 4]);
    let x = [1.0, 2.0, 3.0, NAN];
    assert_vec(
        ps(
            |a| {
                a.op2(op::SLT, rd(0), v(0), c(1));
            },
            x,
            &[c1],
        ),
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_vec(
        ps(
            |a| {
                a.op2(op::SGE, rd(0), v(0), c(1));
            },
            x,
            &[c1],
        ),
        [0.0, 1.0, 1.0, 0.0],
    );
}

#[test]
fn cmp_is_ge_zero_and_cnd_is_gt_half() {
    let consts = [(3, [10.0; 4]), (4, [20.0; 4])];
    let cmp = |x: [f32; 4]| {
        ps(
            |a| {
                a.op3(op::CMP, rd(0), v(0), c(3), c(4));
            },
            x,
            &consts,
        )
    };
    assert_vec(cmp([-1.0, 0.0, 1.0, -0.0]), [20.0, 10.0, 10.0, 10.0]);
    assert_vec(cmp([NAN, -INF, INF, 0.5]), [20.0, 20.0, 10.0, 10.0]);
    let cnd = |x: [f32; 4]| {
        ps(
            |a| {
                a.op3(op::CND, rd(0), v(0), c(3), c(4));
            },
            x,
            &consts,
        )
    };
    assert_vec(cnd([0.5, 0.6, -1.0, NAN]), [20.0, 10.0, 20.0, 20.0]);
}

#[test]
fn min_max_abs_frc_sgn() {
    let x = [-1.5, 2.25, 0.0, -0.0];
    let c1 = (1, [0.0, 1.0, -1.0, 0.5]);
    assert_vec(
        ps(
            |a| {
                a.op2(op::MIN, rd(0), v(0), c(1));
            },
            x,
            &[c1],
        ),
        [-1.5, 1.0, -1.0, -0.0],
    );
    assert_vec(
        ps(
            |a| {
                a.op2(op::MAX, rd(0), v(0), c(1));
            },
            x,
            &[c1],
        ),
        [0.0, 2.25, 0.0, 0.5],
    );
    assert_vec(
        ps(
            |a| {
                a.op1(op::ABS, rd(0), v(0));
            },
            x,
            &[],
        ),
        [1.5, 2.25, 0.0, 0.0],
    );
    assert_vec(
        ps(
            |a| {
                a.op1(op::FRC, rd(0), v(0));
            },
            x,
            &[],
        ),
        [0.5, 0.25, 0.0, 0.0],
    );
    assert_vec(
        ps(
            |a| {
                a.op1(op::SGN, rd(0), v(0));
            },
            x,
            &[],
        ),
        [-1.0, 1.0, 0.0, 0.0],
    );
}

#[test]
fn rcp_rsq_special_values() {
    let got = ps(
        |a| {
            a.op1(op::RCP, rd(0).m("x"), v(0).sw("x"))
                .op1(op::RCP, rd(0).m("y"), v(0).sw("y"))
                .op1(op::RSQ, rd(0).m("z"), v(0).sw("z"))
                .op1(op::RSQ, rd(0).m("w"), v(0).sw("x"));
        },
        [0.0, -0.0, -4.0, 0.0],
        &[],
    );
    assert_vec(got, [INF, -INF, 0.5, INF]);
    // Scalar instructions replicate the result.
    assert_vec(
        ps(
            |a| {
                a.op1(op::RCP, rd(0), v(0).sw("y"));
            },
            [0.0, 4.0, 0.0, 0.0],
            &[],
        ),
        [0.25; 4],
    );
}

#[test]
fn exp_log_pow_special_values() {
    let got = ps(
        |a| {
            a.op1(op::EXP, rd(0).m("x"), v(0).sw("x"))
                .op1(op::LOG, rd(0).m("y"), v(0).sw("y"))
                .op1(op::LOG, rd(0).m("z"), v(0).sw("z"))
                .op2(op::POW, rd(0).m("w"), v(0).sw("y"), c(1).sw("x"));
        },
        [3.0, -8.0, 0.0, 0.0],
        &[(1, [2.0, 0.0, 0.0, 0.0])],
    );
    // exp2(3), log2(|-8|), log2(0) = -inf, pow(|-8|, 2).
    assert_vec(got, [8.0, 3.0, -INF, 64.0]);
    let got = ps(
        |a| {
            a.op1(op::EXPP, rd(0).m("x"), v(0).sw("x"))
                .op1(op::LOGP, rd(0).m("y"), v(0).sw("y"))
                .op2(op::POW, rd(0).m("z"), v(0).sw("z"), v(0).sw("x"))
                .op2(op::POW, rd(0).m("w"), v(0).sw("z"), v(0).sw("z"));
        },
        [1.0, 4.0, 0.0, 0.0],
        &[],
    );
    // pow(0, 1) = 0; pow(0, 0) = NaN with the exp2/log2 form.
    assert_vec(got, [2.0, 2.0, 0.0, NAN]);
}

#[test]
fn sincos_writes_cos_to_x_and_sin_to_y() {
    let got = ps(
        |a| {
            a.mov(rd(0), c(3));
            a.op1(op::SINCOS, rd(0).m("xy"), v(0).sw("x"));
        },
        [0.0; 4],
        &[(3, [7.0; 4])],
    );
    assert_vec(got, [1.0, 0.0, 7.0, 7.0]);
    let got = ps(
        |a| {
            a.mov(rd(0), c(3));
            a.op1(op::SINCOS, rd(0).m("y"), v(0).sw("w"));
        },
        [0.0, 0.0, 0.0, std::f32::consts::FRAC_PI_2],
        &[(3, [7.0; 4])],
    );
    assert_vec(got, [7.0, 1.0, 7.0, 7.0]);
}

#[test]
fn lit_cases() {
    let lit = |x: [f32; 4]| {
        ps(
            |a| {
                a.op1(op::LIT, rd(0), v(0));
            },
            x,
            &[],
        )
    };
    assert_vec(lit([2.0, 4.0, 0.0, 0.5]), [1.0, 2.0, 2.0, 1.0]);
    assert_vec(lit([-1.0, 4.0, 0.0, 0.5]), [1.0, 0.0, 0.0, 1.0]);
    assert_vec(lit([2.0, -1.0, 0.0, 3.0]), [1.0, 2.0, 0.0, 1.0]);
    // Exponent clamped to 127.9961: 2^200 would overflow, 2^127.9961 does not.
    let got = lit([1.0, 2.0, 0.0, 200.0]);
    assert!(got[2].is_finite() && got[2] > 1e38, "{got:?}");
}

#[test]
fn dst_and_dot_products() {
    let consts = [(1, [9.0, 5.0, 9.0, 7.0])];
    assert_vec(
        ps(
            |a| {
                a.op2(op::DST, rd(0), v(0), c(1));
            },
            [9.0, 2.0, 3.0, 9.0],
            &consts,
        ),
        [1.0, 10.0, 3.0, 7.0],
    );
    let consts = [(1, [5.0, 6.0, 7.0, 8.0]), (2, [5.0, 0.0, 0.0, 0.0])];
    let x = [1.0, 2.0, 3.0, 4.0];
    assert_vec(
        ps(
            |a| {
                a.op2(op::DP3, rd(0), v(0), c(1));
            },
            x,
            &consts,
        ),
        [38.0; 4],
    );
    assert_vec(
        ps(
            |a| {
                a.op2(op::DP4, rd(0), v(0), c(1));
            },
            x,
            &consts,
        ),
        [70.0; 4],
    );
    assert_vec(
        ps(
            |a| {
                a.op3(op::DP2ADD, rd(0), v(0), c(1), c(2).sw("x"));
            },
            x,
            &consts,
        ),
        [22.0; 4],
    );
}

#[test]
fn crs_nrm_lrp() {
    let got = ps(
        |a| {
            a.mov(rd(0), c(3));
            a.op2(op::CRS, rd(0).m("xyz"), v(0), c(1));
        },
        [1.0, 0.0, 0.0, 5.0],
        &[(1, [0.0, 1.0, 0.0, 5.0]), (3, [9.0; 4])],
    );
    assert_vec(got, [0.0, 0.0, 1.0, 9.0]);
    assert_vec(
        ps(
            |a| {
                a.op1(op::NRM, rd(0), v(0));
            },
            [3.0, 0.0, 4.0, 10.0],
            &[],
        ),
        [0.6, 0.0, 0.8, 2.0],
    );
    assert_vec(
        ps(
            |a| {
                a.op3(op::LRP, rd(0), v(0), c(1), c(2));
            },
            [0.25; 4],
            &[(1, [8.0; 4]), (2, [4.0; 4])],
        ),
        [5.0; 4],
    );
}

#[test]
fn matrix_rows_are_consecutive_registers() {
    let m = [
        (10, [1.0, 0.0, 0.0, 0.0]),
        (11, [0.0, 2.0, 0.0, 0.0]),
        (12, [0.0, 0.0, 3.0, 0.0]),
        (13, [1.0, 1.0, 1.0, 1.0]),
        (3, [7.0; 4]),
    ];
    let x = [1.0, 2.0, 3.0, 4.0];
    assert_vec(
        ps(
            |a| {
                a.op2(op::M4X4, rd(0), v(0), c(10));
            },
            x,
            &m,
        ),
        [1.0, 4.0, 9.0, 10.0],
    );
    let run = |opcode: u32, mask_text: &str| {
        ps(
            |a| {
                a.mov(rd(0), c(3));
                a.op2(opcode, rd(0).m(mask_text), v(0), c(10));
            },
            x,
            &m,
        )
    };
    assert_vec(run(op::M4X3, "xyz"), [1.0, 4.0, 9.0, 7.0]);
    assert_vec(run(op::M3X4, "xyzw"), [1.0, 4.0, 9.0, 6.0]);
    assert_vec(run(op::M3X3, "xyz"), [1.0, 4.0, 9.0, 7.0]);
    assert_vec(run(op::M3X2, "xy"), [1.0, 4.0, 7.0, 7.0]);
}

#[test]
fn source_modifiers_values() {
    let x = [0.25, -2.0, 3.0, 0.5];
    let cases: [Case; 10] = [
        (sm::NEG, |t| -t),
        (sm::ABS, f32::abs),
        (sm::ABSNEG, |t| -t.abs()),
        (sm::BIAS, |t| t - 0.5),
        (sm::BIASNEG, |t| -(t - 0.5)),
        (sm::SIGN, |t| 2.0 * t - 1.0),
        (sm::SIGNNEG, |t| -(2.0 * t - 1.0)),
        (sm::COMP, |t| 1.0 - t),
        (sm::X2, |t| 2.0 * t),
        (sm::X2NEG, |t| -2.0 * t),
    ];
    for (m, g) in cases {
        let got = ps(
            |a| {
                a.mov(rd(0), v(0).md(m));
            },
            x,
            &[],
        );
        assert_vec(got, x.map(g));
    }
}

#[test]
fn saturate_swizzle_and_mask() {
    assert_vec(
        ps(
            |a| {
                a.mov(rd(0).sat(), v(0));
            },
            [NAN, 2.0, -1.0, 0.25],
            &[],
        ),
        [0.0, 1.0, 0.0, 0.25],
    );
    let got = ps(
        |a| {
            a.mov(rd(0), c(3));
            a.mov(rd(0).m("xz"), v(0).sw("wzyx"));
        },
        [1.0, 2.0, 3.0, 4.0],
        &[(3, [9.0; 4])],
    );
    assert_vec(got, [4.0, 9.0, 2.0, 9.0]);
}

#[test]
fn relative_constants_rounding_defs_and_range() {
    let consts: Vec<(usize, [f32; 4])> = (0..256u16)
        .map(|n| (usize::from(n), [f32::from(n); 4]))
        .collect();
    let run = |x: f32| {
        vs(
            |a| {
                a.def(12, [-5.0; 4]);
                a.op1(op::MOVA, d(reg::ADDR, 0).m("x"), v(0).sw("x"));
                a.mov(od(1), c(10).rel(A0X));
                a.mov(od(0), v(0));
            },
            [x, 0.0, 0.0, 1.0],
            &consts,
            &[],
        )
        .1[0]
    };
    assert!(same(run(0.49), 10.0)); // round(0.49) = 0
    assert!(same(run(0.5), 11.0)); // floor(0.5 + 0.5) = 1
    assert!(same(run(1.5), -5.0)); // a0 = 2: c12 is def'd, the def wins
    assert!(same(run(-1.5), 9.0)); // floor(-1.0) = -1
    assert!(same(run(-20.0), 0.0)); // out of range below: zero
    assert!(same(run(300.0), 0.0)); // out of range above: zero
    assert!(same(run(245.0), 255.0)); // last register
}

#[test]
fn position_fixup_push_constant() {
    let mut a = Asm::vs_basic();
    a.mov(od(0), v(0));
    let (m, _) = check(&a.end());
    let mut mach = Machine::new(&m.words);
    mach.set_vec4("v0_position0", [1.0, 2.0, 3.0, 4.0]);
    mach.set_push([1.0, -1.0, 0.01, 0.02]);
    mach.run();
    assert_vec(mach.get_vec4("o0_position0"), [1.04, -1.92, 3.0, 4.0]);
    // With the fixup off the position is copied unchanged.
    let opts = Options {
        position_fixup: false,
        ..Options::default()
    };
    let (m, _) = check_with(&a.end(), opts);
    let mut mach = Machine::new(&m.words);
    mach.set_vec4("v0_position0", [1.0, 2.0, 3.0, 4.0]);
    mach.run();
    assert_vec(mach.get_vec4("o0_position0"), [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn loop_al_relative_sum_and_nested_restore() {
    let consts = [
        (20, [0.0; 4]),
        (21, [1.0; 4]),
        (22, [1000.0; 4]),
        (23, [10.0; 4]),
        (24, [1000.0; 4]),
        (25, [100.0; 4]),
        (30, [1.0; 4]),
        (31, [2.0; 4]),
    ];
    // loop aL = 1, 3, 5.
    let (got, _) = ps_full(
        |a| {
            a.flow(op::LOOP, 0, &[al(), i(0)]);
            a.op2(op::ADD, rd(0), r(0), c(20).rel(AL));
            a.flow(op::ENDLOOP, 0, &[]);
        },
        [0.0; 4],
        &consts,
        &[(0, [3, 1, 2, 0])],
        0,
    );
    assert_vec(got, [111.0; 4]);
    // Outer aL = 0, 1; after each inner loop aL is the outer value again.
    let (got, _) = ps_full(
        |a| {
            a.defi(1, [2, 0, 1, 0]);
            a.flow(op::LOOP, 0, &[al(), i(1)]);
            a.flow(op::LOOP, 0, &[al(), i(0)]);
            a.op2(op::ADD, rd(0), r(0), c(20).rel(AL));
            a.flow(op::ENDLOOP, 0, &[]);
            a.op2(op::ADD, rd(0), r(0), c(30).rel(AL));
            a.flow(op::ENDLOOP, 0, &[]);
        },
        [0.0; 4],
        &consts,
        &[(0, [3, 1, 2, 0])],
        0,
    );
    assert_vec(got, [111.0 + 1.0 + 111.0 + 2.0; 4]);
    // A zero count runs no iterations.
    let (got, _) = ps_full(
        |a| {
            a.flow(op::LOOP, 0, &[al(), i(0)]);
            a.op2(op::ADD, rd(0), r(0), c(21));
            a.flow(op::ENDLOOP, 0, &[]);
        },
        [0.0; 4],
        &consts,
        &[(0, [0, 1, 2, 0])],
        0,
    );
    assert_vec(got, [0.0; 4]);
}

#[test]
fn rep_with_breakc_break_and_breakp() {
    let consts = [(1, [1.0; 4]), (2, [4.0; 4])];
    let (got, _) = ps_full(
        |a| {
            a.defi(0, [10, 0, 0, 0]);
            a.flow(op::REP, 0, &[i(0)]);
            a.op2(op::ADD, rd(0), r(0), c(1));
            a.flow(op::BREAKC, cmp::GE, &[r(0).sw("x"), c(2).sw("x")]);
            a.flow(op::ENDREP, 0, &[]);
        },
        [0.0; 4],
        &consts,
        &[],
        0,
    );
    assert_vec(got, [4.0; 4]);
    let (got, _) = ps_full(
        |a| {
            a.defi(0, [10, 0, 0, 0]);
            a.flow(op::REP, 0, &[i(0)]);
            a.op2(op::ADD, rd(0), r(0), c(1));
            a.ins(op::SETP, cmp::EQ, Some(d(reg::PRED, 0)), &[r(0), c(2)]);
            a.flow(op::BREAKP, 0, &[p0().sw("y")]);
            a.flow(op::ENDREP, 0, &[]);
        },
        [0.0; 4],
        &consts,
        &[],
        0,
    );
    assert_vec(got, [4.0; 4]);
    let (got, _) = ps_full(
        |a| {
            a.defi(0, [10, 0, 0, 0]);
            a.flow(op::REP, 0, &[i(0)]);
            a.op2(op::ADD, rd(0), r(0), c(1));
            a.flow(op::IFC, cmp::EQ, &[r(0).sw("x"), c(2).sw("x")]);
            a.flow(op::BREAK, 0, &[]);
            a.flow(op::ENDIF, 0, &[]);
            a.flow(op::ENDREP, 0, &[]);
        },
        [0.0; 4],
        &consts,
        &[],
        0,
    );
    assert_vec(got, [4.0; 4]);
}

#[test]
fn if_on_bool_constants_from_the_buffer_and_defb() {
    let consts = [(1, [1.0; 4]), (2, [2.0; 4])];
    let prog = |a: &mut Asm| {
        a.flow(op::IF, 0, &[b(3)]);
        a.mov(rd(0), c(1));
        a.flow(op::ELSE, 0, &[]);
        a.mov(rd(0), c(2));
        a.flow(op::ENDIF, 0, &[]);
        a.flow(op::IF, 0, &[b(5).not()]);
        a.op2(op::ADD, rd(0), r(0), c(2));
        a.flow(op::ENDIF, 0, &[]);
    };
    assert_vec(ps_full(prog, [0.0; 4], &consts, &[], 1 << 3).0, [3.0; 4]);
    assert_vec(ps_full(prog, [0.0; 4], &consts, &[], 0).0, [4.0; 4]);
    assert_vec(
        ps_full(prog, [0.0; 4], &consts, &[], (1 << 3) | (1 << 5)).0,
        [1.0; 4],
    );
    // defb overrides the buffer.
    let prog = |a: &mut Asm| {
        a.defb(3, false);
        prog(a);
    };
    assert_vec(ps_full(prog, [0.0; 4], &consts, &[], 1 << 3).0, [4.0; 4]);
}

#[test]
fn ifc_comparisons_values() {
    for (code, want) in [
        (cmp::GT, [false, false, true, false]),
        (cmp::EQ, [false, true, false, false]),
        (cmp::GE, [false, true, true, false]),
        (cmp::LT, [true, false, false, false]),
        (cmp::NE, [true, false, true, true]),
        (cmp::LE, [true, true, false, false]),
    ] {
        for (k, x) in [1.0, 2.0, 3.0, NAN].into_iter().enumerate() {
            let got = ps(
                |a| {
                    a.flow(op::IFC, code, &[v(0).sw("x"), c(1).sw("x")]);
                    a.mov(rd(0), c(3));
                    a.flow(op::ENDIF, 0, &[]);
                },
                [x, 0.0, 0.0, 0.0],
                &[(1, [2.0; 4]), (3, [1.0; 4])],
            );
            assert_eq!(got[0] == 1.0, want[k], "comparison {code} with {x}");
        }
    }
}

#[test]
fn predicated_writes() {
    let got = ps(
        |a| {
            a.ins(op::SETP, cmp::LT, Some(d(reg::PRED, 0)), &[v(0), c(1)]);
            a.mov(rd(0), c(2));
            a.pred(op::MOV, p0(), rd(0), &[c(3)]);
            a.pred(op::MOV, p0().sw("y").not(), rd(0).m("w"), &[c(4)]);
        },
        [1.0, 5.0, 1.0, 5.0],
        &[
            (1, [3.0; 4]),
            (2, [9.0; 4]),
            (3, [1.0, 2.0, 3.0, 4.0]),
            (4, [7.0; 4]),
        ],
    );
    assert_vec(got, [1.0, 9.0, 3.0, 7.0]);
    // setp with a write mask keeps the other predicate components.
    let got = ps(
        |a| {
            a.ins(op::SETP, cmp::GT, Some(d(reg::PRED, 0)), &[v(0), c(1)]);
            a.ins(
                op::SETP,
                cmp::LT,
                Some(d(reg::PRED, 0).m("x")),
                &[v(0), c(1)],
            );
            a.mov(rd(0), c(2));
            a.pred(op::MOV, p0(), rd(0), &[c(3)]);
        },
        [1.0, 5.0, 1.0, 5.0],
        &[(1, [3.0; 4]), (2, [9.0; 4]), (3, [1.0, 2.0, 3.0, 4.0])],
    );
    assert_vec(got, [1.0, 2.0, 9.0, 4.0]);
}

#[test]
fn subroutines_and_conditional_calls() {
    let mut a = Asm::ps_basic();
    a.mov(rd(0), c(1));
    a.flow(op::CALL, 0, &[l(0)]);
    a.flow(op::CALLNZ, 0, &[l(1), b(0)]);
    a.flow(op::CALLNZ, 0, &[l(1), b(0).not()]);
    a.mov(oc(0), r(0));
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(0)]);
    a.op2(op::MUL, rd(0), r(0), c(2));
    a.flow(op::RET, 0, &[]);
    a.flow(op::LABEL, 0, &[l(1)]);
    a.op2(op::ADD, rd(0), r(0), c(3));
    a.flow(op::RET, 0, &[]);
    let (m, _) = check(&a.end());
    let mut mach = Machine::new(&m.words);
    mach.set_c(&[(1, [1.0; 4]), (2, [3.0; 4]), (3, [100.0; 4])]);
    mach.set_ib(&[], 0);
    mach.run();
    assert_vec(mach.get_vec4("oC0"), [103.0; 4]);
}

#[test]
fn texkill_kills_on_any_masked_negative_component() {
    let kill = |mask_text: &str, x: [f32; 4]| {
        ps_full(
            |a| {
                a.ins(op::TEXKILL, 0, Some(vd(0).m(mask_text)), &[]);
            },
            x,
            &[],
            &[],
            0,
        )
        .1
    };
    assert!(kill("xyzw", [1.0, 1.0, -1.0, 1.0]));
    assert!(!kill("xyzw", [1.0, 1.0, 1.0, 0.0]));
    assert!(!kill("xy", [1.0, 1.0, -1.0, 1.0]));
    assert!(kill("xy", [1.0, -0.5, 1.0, 1.0]));
}

#[test]
fn vpos_and_vface_values() {
    let mut a = Asm::ps();
    a.dcl_misc(0).dcl_misc(1);
    a.op2(op::ADD, rd(0), s(reg::MISC, 0), s(reg::MISC, 1));
    a.mov(oc(0), r(0));
    let (m, _) = check(&a.end());
    for (front, face) in [(true, 1.0), (false, -1.0)] {
        let mut mach = Machine::new(&m.words);
        mach.set_vec4("vPos", [10.5, 20.5, 0.25, 1.0]);
        mach.set_bool("vFace", front);
        mach.run();
        assert_vec(
            mach.get_vec4("oC0"),
            [10.0 + face, 20.0 + face, 0.25 + face, 1.0 + face],
        );
    }
}

#[test]
fn texture_coordinates_and_operands_reach_the_sampler() {
    let run = |body: &dyn Fn(&mut Asm)| {
        ps(
            |a| {
                a.dcl_sampler(2, 0).dcl_sampler(3, 1).dcl_sampler(4, 2);
                body(a);
            },
            [0.5, 0.25, 0.75, 2.0],
            &[(1, [0.1; 4]), (2, [0.2; 4])],
        )
    };
    assert_vec(
        run(&|a| {
            a.op2(op::TEX, rd(0), v(0), sampler(0));
        }),
        [0.5, 0.25, 0.0, 0.0],
    );
    // texldp divides by w.
    assert_vec(
        run(&|a| {
            a.ins(op::TEX, 1, Some(rd(0)), &[v(0), sampler(0)]);
        }),
        [0.25, 0.125, 0.0, 0.0],
    );
    // texldb: bias = w.
    assert_vec(
        run(&|a| {
            a.ins(op::TEX, 2, Some(rd(0)), &[v(0), sampler(0)]);
        }),
        [0.5, 0.25, 2.0, 0.0],
    );
    // texldl on a cube: xyz coordinate, lod = w, binding 1.
    assert_vec(
        run(&|a| {
            a.op2(op::TEXLDL, rd(0), v(0), sampler(1));
        }),
        [0.5, 0.25, 2.75, 1.0],
    );
    // texldd on a volume: gradients from c1, c2.
    assert_vec(
        run(&|a| {
            a.ins(op::TEXLDD, 0, Some(rd(0)), &[v(0), sampler(2), c(1), c(2)]);
        }),
        [0.5, 0.25, 1.05, 2.0],
    );
    // Sampler swizzle reorders the result.
    assert_vec(
        run(&|a| {
            a.op2(op::TEX, rd(0), v(0), sampler(0).sw("wzyx"));
        }),
        [0.0, 0.0, 0.25, 0.5],
    );
}

#[test]
fn depth_compare_result_is_replicated() {
    let mut a = Asm::ps_basic();
    a.dcl_sampler(2, 0);
    a.op2(op::TEX, rd(0), v(0), sampler(0));
    a.mov(oc(0), r(0));
    let opts = Options {
        depth_compare_samplers: 1,
        ..Options::default()
    };
    let (m, _) = check_with(&a.end(), opts);
    for (z, want) in [(0.25, 1.0), (0.75, 0.0)] {
        let mut mach = Machine::new(&m.words);
        mach.set_vec4("v0_texcoord0", [0.5, 0.5, z, 1.0]);
        mach.run();
        assert_vec(mach.get_vec4("oC0"), [want; 4]);
    }
}

#[test]
fn packed_and_relative_outputs() {
    let mut a = Asm::vs_basic();
    a.dcl(usage::TEXCOORD, 0, od(1).m("xy"))
        .dcl(usage::TEXCOORD, 1, od(1).m("zw"))
        .dcl(usage::TEXCOORD, 2, od(2));
    a.defi(0, [2, 1, 1, 0]);
    a.mov(od(0), v(0));
    a.flow(op::LOOP, 0, &[al(), i(0)]);
    a.mov(od(0).rel(AL), c(10).rel(AL));
    a.flow(op::ENDLOOP, 0, &[]);
    let (m, _) = check(&a.end());
    let mut mach = Machine::new(&m.words);
    mach.set_vec4("v0_position0", [1.0, 2.0, 3.0, 4.0]);
    mach.set_c(&[(11, [1.0, 2.0, 3.0, 4.0]), (12, [5.0, 6.0, 7.0, 8.0])]);
    mach.set_push([1.0, 1.0, 0.0, 0.0]);
    mach.run();
    // o1 holds both packed semantics, components in place.
    assert_vec(mach.get_vec4("o1_texcoord0"), [1.0, 2.0, 3.0, 4.0]);
    assert_vec(mach.get_vec4("o1_texcoord1"), [1.0, 2.0, 3.0, 4.0]);
    assert_vec(mach.get_vec4("o2_texcoord2"), [5.0, 6.0, 7.0, 8.0]);
    assert_vec(mach.get_vec4("o0_position0"), [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn relative_inputs_and_packed_inputs() {
    let mut a = Asm::ps();
    a.dcl(usage::TEXCOORD, 0, vd(0))
        .dcl(usage::TEXCOORD, 1, vd(1));
    a.defi(0, [2, 0, 1, 0]);
    a.flow(op::LOOP, 0, &[al(), i(0)]);
    a.op2(op::ADD, rd(0), r(0), v(0).rel(AL));
    a.flow(op::ENDLOOP, 0, &[]);
    a.mov(oc(0), r(0));
    let (m, _) = check(&a.end());
    let mut mach = Machine::new(&m.words);
    mach.set_vec4("v0_texcoord0", [1.0, 2.0, 3.0, 4.0]);
    mach.set_vec4("v1_texcoord1", [10.0, 20.0, 30.0, 40.0]);
    mach.run();
    assert_vec(mach.get_vec4("oC0"), [11.0, 22.0, 33.0, 44.0]);

    let mut a = Asm::ps();
    a.dcl(usage::TEXCOORD, 0, vd(0).m("xy"))
        .dcl(usage::TEXCOORD, 1, vd(0).m("zw"));
    a.mov(oc(0), v(0));
    let (m, _) = check(&a.end());
    let mut mach = Machine::new(&m.words);
    mach.set_vec4("v0_texcoord0", [1.0, 2.0, 3.0, 4.0]);
    mach.set_vec4("v0_texcoord1", [10.0, 20.0, 30.0, 40.0]);
    mach.run();
    assert_vec(mach.get_vec4("oC0"), [1.0, 2.0, 30.0, 40.0]);
}

#[test]
fn depth_output_and_multiple_colors() {
    let mut a = Asm::ps_basic();
    a.mov(oc(0), v(0)).mov(oc(2), v(0).sw("wzyx"));
    a.mov(d(reg::DEPTHOUT, 0), v(0).sw("z"));
    let (m, _) = check(&a.end());
    let mut mach = Machine::new(&m.words);
    mach.set_vec4("v0_texcoord0", [1.0, 2.0, 3.0, 4.0]);
    mach.run();
    assert!(same(mach.get_f32("oDepth"), 3.0));
    assert_vec(mach.get_vec4("oC2"), [4.0, 3.0, 2.0, 1.0]);
}

/// The semantic checks must see a wrong translation: swap the comparison
/// emitted for `slt` and the expected result no longer comes out.
#[test]
fn mutant_comparison_is_detected() {
    use lf_dxso_spirv::spirv::op as sop;
    let mut a = Asm::ps_basic();
    a.op2(op::SLT, rd(0), v(0), c(1));
    a.mov(oc(0), r(0));
    let (m, _) = check(&a.end());
    let run = |words: &[u32]| {
        let mut mach = Machine::new(words);
        mach.set_vec4("v0_texcoord0", [1.0, 2.0, 3.0, NAN]);
        mach.set_c(&[(1, [2.0; 4])]);
        mach.run();
        mach.get_vec4("oC0")
    };
    assert_vec(run(&m.words), [1.0, 0.0, 0.0, 0.0]);
    let mut mutant = m.words.clone();
    let insts = lf_dxso_spirv::validate::instructions(&m.words).unwrap();
    let at = insts
        .iter()
        .find(|i| i.opcode == sop::F_ORD_LESS_THAN)
        .unwrap()
        .offset;
    mutant[at] = (mutant[at] & 0xFFFF_0000) | u32::from(sop::F_ORD_GREATER_THAN_EQUAL);
    lf_dxso_spirv::validate::validate(&mutant).unwrap();
    let got = run(&mutant);
    assert!(
        !got.iter()
            .zip(&[1.0, 0.0, 0.0, 0.0])
            .all(|(&g, &w)| same(g, w)),
        "mutant went unnoticed: {got:?}"
    );
}
