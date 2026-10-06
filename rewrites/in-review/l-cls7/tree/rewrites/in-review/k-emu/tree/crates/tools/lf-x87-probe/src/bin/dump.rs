//! `lf-x87-probe-dump`: run every probe and print one JSON document with
//! each probe's name, input bits and measured result bits to stdout.
//!
//! No assertions here: the coordinator runs the same binary on a real x86
//! runner and compares dumps. Builds and prints a stub on other targets.

use lf_x87_probe::emulation_details;
#[cfg(target_arch = "x86")]
use lf_x87_probe::{hex16, hex32, hex64, hex_f80};

#[cfg(target_arch = "x86")]
use lf_x87_probe::{MXCSR_DEFAULT, MXCSR_FTZ, MXCSR_RC_SHIFT, PC_24, PC_53, PC_64};
#[cfg(target_arch = "x86")]
use lf_x87_probe::{RC_CHOP, RC_DOWN, RC_NEAR, RC_UP};
#[cfg(target_arch = "x86")]
use lf_x87_probe::{sse, x87, x87cmp, x87trans};

#[cfg(target_arch = "x86")]
struct Rec {
    group: u8,
    name: String,
    input: String,
    output: String,
    extra: String,
}

#[cfg(target_arch = "x86")]
fn push(recs: &mut Vec<Rec>, group: u8, name: String, input: String, output: String, extra: String) {
    recs.push(Rec {
        group,
        name,
        input,
        output,
        extra,
    });
}

#[cfg(target_arch = "x86")]
fn run_x86() -> Vec<Rec> {
    let mut r: Vec<Rec> = Vec::new();

    // Group 1, f32 (memory roundtrip and float-stack return).
    let f32s: &[(&str, u32)] = &[
        ("snan_min", 0x7F80_0001),
        ("snan_payload", 0x7F80_0123),
        ("snan_neg", 0xFF80_0001),
        ("qnan_zero", 0x7FC0_0000),
        ("qnan_payload", 0x7FC1_2345),
        ("qnan_neg", 0xFFC0_0000),
        ("denorm_min", 0x0000_0001),
        ("denorm_max", 0x007F_FFFF),
        ("denorm_neg", 0x8000_0001),
        ("inf_pos", 0x7F80_0000),
        ("inf_neg", 0xFF80_0000),
        ("zero_neg", 0x8000_0000),
        ("zero_pos", 0x0000_0000),
        ("one", 0x3F80_0000),
    ];
    for (tag, v) in f32s {
        let m = x87::g1_f32_mem_roundtrip(*v);
        push(
            &mut r,
            1,
            format!("g1.f32.mem.{tag}"),
            hex32(*v),
            hex32(m.bits),
            format!("sw={}", hex16(m.status)),
        );
        let f = x87::g1_f32_return_roundtrip(*v);
        push(
            &mut r,
            1,
            format!("g1.f32.ret.{tag}"),
            hex32(*v),
            hex32(f.bits),
            format!("sw={}", hex16(f.status)),
        );
    }

    // Group 1, f64.
    let f64s: &[(&str, u64)] = &[
        ("snan_min", 0x7FF0_0000_0000_0001),
        ("snan_payload", 0x7FF0_0000_0000_1234),
        ("snan_neg", 0xFFF0_0000_0000_0001),
        ("qnan_zero", 0x7FF8_0000_0000_0000),
        ("qnan_payload", 0x7FF8_0000_0000_1234),
        ("qnan_neg", 0xFFC0_0000_0000_0000),
        ("denorm_min", 0x0000_0000_0000_0001),
        ("denorm_max", 0x000F_FFFF_FFFF_FFFF),
        ("denorm_neg", 0x8000_0000_0000_0001),
        ("inf_pos", 0x7FF0_0000_0000_0000),
        ("inf_neg", 0xFFF0_0000_0000_0000),
        ("zero_neg", 0x8000_0000_0000_0000),
        ("zero_pos", 0x0000_0000_0000_0000),
        ("one", 0x3FF0_0000_0000_0000),
    ];
    for (tag, v) in f64s {
        let m = x87::g1_f64_mem_roundtrip(*v);
        push(
            &mut r,
            1,
            format!("g1.f64.mem.{tag}"),
            hex64(*v),
            hex64(m.bits),
            format!("sw={}", hex16(m.status)),
        );
        let f = x87::g1_f64_return_roundtrip(*v);
        push(
            &mut r,
            1,
            format!("g1.f64.ret.{tag}"),
            hex64(*v),
            hex64(f.bits),
            format!("sw={}", hex16(f.status)),
        );
    }

    // Group 1, 80-bit.
    let f80s: &[(&str, u64, u16)] = &[
        ("snan", 0x8000_0000_0000_0001, 0x7FFF),
        ("qnan", 0xC000_0000_0000_0000, 0x7FFF),
        ("indefinite", 0xC000_0000_0000_0000, 0xFFFF),
        ("denormal", 0x0000_0000_0000_0001, 0x0000),
        ("inf_pos", 0x8000_0000_0000_0000, 0x7FFF),
        ("inf_neg", 0x8000_0000_0000_0000, 0xFFFF),
        ("zero_neg", 0x0000_0000_0000_0000, 0x8000),
        ("one", 0x8000_0000_0000_0000, 0x3FFF),
    ];
    for (tag, m, e) in f80s {
        let o = x87::g1_f80_mem_roundtrip(*m, *e);
        push(
            &mut r,
            1,
            format!("g1.f80.mem.{tag}"),
            hex_f80(*m, *e),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }

    // Group 2: precision control.
    let pcs: &[(&str, u16)] = &[("pc24", PC_24), ("pc53", PC_53), ("pc64", PC_64)];
    let one: u64 = 0x3FF0_0000_0000_0000;
    let p2m25: u64 = 0x3E60_0000_0000_0000; // 2^-25
    let p2m54: u64 = 0x3C90_0000_0000_0000; // 2^-54
    let m2m25: u64 = 0xBE60_0000_0000_0000; // -2^-25
    let p1p2m12: u64 = 0x3FF0_0100_0000_0000; // 1+2^-12
    let p1p2m26: u64 = 0x3FF0_0000_4000_0000; // 1+2^-26
    let p1p2m27: u64 = 0x3FF0_0000_2000_0000; // 1+2^-27
    let three: u64 = 0x4008_0000_0000_0000;
    let two: u64 = 0x4000_0000_0000_0000;
    for (pc_tag, pc) in pcs {
        for (op, a, b) in [
            ("add_1p2m25", one, p2m25),
            ("add_1p2m54", one, p2m54),
            ("sub_1m2m25", one, p2m25),
            ("sub_1m2m54", one, p2m54),
        ] {
            let o = if op.starts_with("add") {
                x87::g2_fadd(*pc, a, b)
            } else {
                x87::g2_fsub(*pc, a, b)
            };
            let _ = m2m25;
            push(
                &mut r,
                2,
                format!("g2.{op}.{pc_tag}"),
                format!("a={} b={}", hex64(a), hex64(b)),
                hex_f80(o.mant, o.exp),
                format!("sw={}", hex16(o.status)),
            );
        }
        for (op, a, b) in [
            ("mul_sq12", p1p2m12, p1p2m12),
            ("mul_26x27", p1p2m26, p1p2m27),
            ("div_1d3", one, three),
        ] {
            let o = if op.starts_with("mul") {
                x87::g2_fmul(*pc, a, b)
            } else {
                x87::g2_fdiv(*pc, a, b)
            };
            push(
                &mut r,
                2,
                format!("g2.{op}.{pc_tag}"),
                format!("a={} b={}", hex64(a), hex64(b)),
                hex_f80(o.mant, o.exp),
                format!("sw={}", hex16(o.status)),
            );
        }
        let o = x87::g2_fsqrt(*pc, two);
        push(
            &mut r,
            2,
            format!("g2.sqrt2.{pc_tag}"),
            format!("a={}", hex64(two)),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }

    // Group 6 exact square roots (via the group 2 probe at full precision).
    for (tag, a) in [
        ("sqrt_4", 0x4010_0000_0000_0000u64),
        ("sqrt_0", 0x0000_0000_0000_0000u64),
        ("sqrt_inf", 0x7FF0_0000_0000_0000u64),
    ] {
        let o = x87::g2_fsqrt(PC_64, a);
        push(
            &mut r,
            6,
            format!("g6.{tag}"),
            hex64(a),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }

    // Group 3, x87 integer stores.
    let rcs: &[(&str, u16)] = &[
        ("near", RC_NEAR),
        ("down", RC_DOWN),
        ("up", RC_UP),
        ("chop", RC_CHOP),
    ];
    let f2_5: u64 = 0x4004_0000_0000_0000;
    let f2_7: u64 = 0x4005_9999_9999_999A;
    let fn2_5: u64 = 0xC004_0000_0000_0000;
    let fn2_7: u64 = 0xC005_9999_9999_999A;
    for (rc_tag, rc) in rcs {
        for (tag, v) in [("p2_5", f2_5), ("n2_5", fn2_5), ("p2_7", f2_7), ("n2_7", fn2_7)] {
            let o = x87cmp::g3_fistp_i32(*rc, v);
            push(
                &mut r,
                3,
                format!("g3.fistp32.{tag}.{rc_tag}"),
                hex64(v),
                hex32(o.val),
                format!("sw={}", hex16(o.status)),
            );
        }
        for (tag, v) in [("p2_5", f2_5), ("n2_5", fn2_5)] {
            let o16 = x87cmp::g3_fistp_i16(*rc, v);
            push(
                &mut r,
                3,
                format!("g3.fistp16.{tag}.{rc_tag}"),
                hex64(v),
                hex16(o16.val),
                format!("sw={}", hex16(o16.status)),
            );
            let o64 = x87cmp::g3_fistp_i64(*rc, v);
            push(
                &mut r,
                3,
                format!("g3.fistp64.{tag}.{rc_tag}"),
                hex64(v),
                hex64(o64.val),
                format!("sw={}", hex16(o64.status)),
            );
        }
    }
    // Out-of-range and NaN integer stores (round-nearest).
    let big1e20: u64 = 0x4415_AF1D_78B5_8C40; // 1e20
    let big1e10: u64 = 0x4202_A05F_2000_0000; // 1e10
    let qnan64: u64 = 0x7FF8_0000_0000_0000;
    let inf64: u64 = 0x7FF0_0000_0000_0000;
    for (tag, v) in [("oor_1e20", big1e20), ("nan", qnan64), ("inf", inf64)] {
        let o = x87cmp::g3_fistp_i32(RC_NEAR, v);
        push(
            &mut r,
            3,
            format!("g3.fistp32.{tag}.near"),
            hex64(v),
            hex32(o.val),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (tag, v) in [("oor_1e10", big1e10), ("nan", qnan64)] {
        let o = x87cmp::g3_fistp_i16(RC_NEAR, v);
        push(
            &mut r,
            3,
            format!("g3.fistp16.{tag}.near"),
            hex64(v),
            hex16(o.val),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (tag, v) in [("oor_1e20", big1e20), ("nan", qnan64)] {
        let o = x87cmp::g3_fistp_i64(RC_NEAR, v);
        push(
            &mut r,
            3,
            format!("g3.fistp64.{tag}.near"),
            hex64(v),
            hex64(o.val),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (tag, v) in [("p2_5", f2_5), ("n2_5", fn2_5)] {
        let o = x87cmp::g3_fist_i32(RC_NEAR, v);
        push(
            &mut r,
            3,
            format!("g3.fist32.{tag}.near"),
            hex64(v),
            hex32(o.val),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (tag, v) in [
        ("p2_5", f2_5),
        ("n2_5", fn2_5),
        ("oor_1e20", big1e20),
        ("nan", qnan64),
    ] {
        match x87cmp::g3_fisttp_i32(v) {
            Some(o) => push(
                &mut r,
                3,
                format!("g3.fisttp32.{tag}"),
                hex64(v),
                hex32(o.val),
                format!("sw={}", hex16(o.status)),
            ),
            None => push(
                &mut r,
                3,
                format!("g3.fisttp32.{tag}"),
                hex64(v),
                "unsupported-no-sse3".to_string(),
                String::new(),
            ),
        }
        match x87cmp::g3_fisttp_i64(v) {
            Some(o) => push(
                &mut r,
                3,
                format!("g3.fisttp64.{tag}"),
                hex64(v),
                hex64(o.val),
                format!("sw={}", hex16(o.status)),
            ),
            None => push(
                &mut r,
                3,
                format!("g3.fisttp64.{tag}"),
                hex64(v),
                "unsupported-no-sse3".to_string(),
                String::new(),
            ),
        }
    }

    // Group 3, SSE conversions.
    let f2_5s: u32 = 0x4020_0000;
    let fn2_5s: u32 = 0xC020_0000;
    let big1e20s: u32 = 0x60AD_78EC; // 1e20 as f32
    let qnan32: u32 = 0x7FC0_0000;
    for (rc_tag, rc) in rcs {
        let mx = MXCSR_DEFAULT | ((u32::from(*rc) >> 10) << MXCSR_RC_SHIFT);
        for (tag, v) in [("p2_5", f2_5s), ("n2_5", fn2_5s)] {
            let (val, mxcsr) = sse::g3_cvtss2si(mx, v);
            push(
                &mut r,
                3,
                format!("g3.cvtss2si.{tag}.{rc_tag}"),
                hex32(v),
                format!("{val:#010x}"),
                format!("mxcsr={}", hex32(mxcsr)),
            );
        }
        for (tag, v) in [("p2_5", f2_5), ("n2_5", fn2_5)] {
            let (val, mxcsr) = sse::g3_cvtsd2si(mx, v);
            push(
                &mut r,
                3,
                format!("g3.cvtsd2si.{tag}.{rc_tag}"),
                hex64(v),
                format!("{val:#010x}"),
                format!("mxcsr={}", hex32(mxcsr)),
            );
        }
    }
    for (tag, v) in [
        ("p2_5", f2_5s),
        ("n2_5", fn2_5s),
        ("oor_1e20", big1e20s),
        ("nan", qnan32),
    ] {
        let (val, mxcsr) = sse::g3_cvttss2si(v);
        push(
            &mut r,
            3,
            format!("g3.cvttss2si.{tag}"),
            hex32(v),
            format!("{val:#010x}"),
            format!("mxcsr={}", hex32(mxcsr)),
        );
    }
    for (tag, v) in [
        ("p2_5", f2_5),
        ("n2_5", fn2_5),
        ("oor_1e20", big1e20),
        ("nan", qnan64),
    ] {
        let (val, mxcsr) = sse::g3_cvttsd2si(v);
        push(
            &mut r,
            3,
            format!("g3.cvttsd2si.{tag}"),
            hex64(v),
            format!("{val:#010x}"),
            format!("mxcsr={}", hex32(mxcsr)),
        );
    }

    // Group 4: comparisons.
    let f1: u64 = 0x3FF0_0000_0000_0000;
    let f2: u64 = 0x4000_0000_0000_0000;
    let snan64: u64 = 0x7FF0_0000_0000_0001;
    let pzero: u64 = 0x0000_0000_0000_0000;
    let nzero: u64 = 0x8000_0000_0000_0000;
    let pinf: u64 = 0x7FF0_0000_0000_0000;
    for (tag, a, b) in [
        ("gt", f2, f1),
        ("lt", f1, f2),
        ("eq", f1, f1),
        ("un_qnan_first", qnan64, f1),
        ("un_qnan_second", f1, qnan64),
        ("un_snan", snan64, f1),
        ("zeros", pzero, nzero),
        ("infs", pinf, pinf),
    ] {
        let o = x87cmp::g4_fucompp(a, b);
        push(
            &mut r,
            4,
            format!("g4.fucompp.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex16(o.status),
            "cond-in-status".to_string(),
        );
    }
    for (tag, a, b) in [("eq", f1, f1), ("un_qnan", qnan64, f1)] {
        let o = x87cmp::g4_fcomp(a, b);
        push(
            &mut r,
            4,
            format!("g4.fcomp.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex16(o.status),
            "cond-in-status".to_string(),
        );
        let o = x87cmp::g4_fucomp(a, b);
        push(
            &mut r,
            4,
            format!("g4.fucomp.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex16(o.status),
            "cond-in-status".to_string(),
        );
    }
    for (tag, a, b) in [
        ("gt", f2, f1),
        ("lt", f1, f2),
        ("eq", f1, f1),
        ("un_qnan", qnan64, f1),
        ("un_snan", snan64, f1),
    ] {
        let o = x87cmp::g4_fcomip(a, b);
        push(
            &mut r,
            4,
            format!("g4.fcomip.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex32(o.eflags),
            format!("sw={}", hex16(o.status)),
        );
        let o = x87cmp::g4_fucomip(a, b);
        push(
            &mut r,
            4,
            format!("g4.fucomip.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex32(o.eflags),
            format!("sw={}", hex16(o.status)),
        );
    }

    // Group 5: masked-exception flags.
    for (tag, o) in [
        ("div_0_0", x87cmp::g5_fdiv_0_0()),
        ("div_1_0", x87cmp::g5_fdiv_1_0()),
        ("div_1_3", x87cmp::g5_fdiv_1_3()),
        ("sqrt_neg1", x87cmp::g5_fsqrt_neg1()),
        ("add_overflow", x87cmp::g5_fadd_overflow()),
        ("mul_underflow", x87cmp::g5_fmul_underflow()),
        ("fld_denormal", x87cmp::g5_fld_denormal_single()),
    ] {
        push(
            &mut r,
            5,
            format!("g5.{tag}"),
            "fixed".to_string(),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }

    // Group 6: transcendentals and remainders.
    let f5_5: u64 = 0x4016_0000_0000_0000;
    let f7: u64 = 0x401C_0000_0000_0000;
    let f1e20: u64 = 0x4415_AF1D_78B5_8C40;
    let f0: u64 = 0x0000_0000_0000_0000;
    let p2e63: u64 = 0x43E0_0000_0000_0000; // 2^63
    let p2e64: u64 = 0x43F0_0000_0000_0000; // 2^64
    let f1_5: u64 = 0x3FF8_0000_0000_0000;
    let fn1: u64 = 0xBFF0_0000_0000_0000;
    for (tag, a, b) in [("rem_5p5_2", f5_5, f2), ("rem_1e20_2", f1e20, f2)] {
        let o = x87trans::g6_fprem(a, b);
        push(
            &mut r,
            6,
            format!("g6.fprem.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (tag, a, b) in [("rem1_5p5_2", f5_5, f2), ("rem1_7_2", f7, f2)] {
        let o = x87trans::g6_fprem1(a, b);
        push(
            &mut r,
            6,
            format!("g6.fprem1.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (tag, a) in [
        ("sin_0", f0),
        ("sin_1", f1),
        ("sin_2e63", p2e63),
        ("sin_2e64", p2e64),
        ("cos_0", f0),
        ("cos_2e64", p2e64),
    ] {
        let o = if tag.starts_with("sin") {
            x87trans::g6_fsin(a)
        } else {
            x87trans::g6_fcos(a)
        };
        push(
            &mut r,
            6,
            format!("g6.{tag}"),
            hex64(a),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (tag, a) in [("sincos_0", f0), ("sincos_1", f1)] {
        let (s0, s1) = x87trans::g6_fsincos(a);
        push(
            &mut r,
            6,
            format!("g6.{tag}"),
            hex64(a),
            format!(
                "st0={} st1={}",
                hex_f80(s0.mant, s0.exp),
                hex_f80(s1.mant, s1.exp)
            ),
            format!("sw={}", hex16(s0.status)),
        );
        let _ = s1;
    }
    for (tag, y, x) in [
        ("patan_0_1", f0, f1),
        ("patan_1_1", f1, f1),
        ("patan_1_0", f1, f0),
    ] {
        let o = x87trans::g6_fpatan(y, x);
        push(
            &mut r,
            6,
            format!("g6.{tag}"),
            format!("y={} x={}", hex64(y), hex64(x)),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (tag, a, n) in [("scale_1p5_2", f1_5, f2), ("scale_1_m1", f1, fn1)] {
        let o = x87trans::g6_fscale(a, n);
        push(
            &mut r,
            6,
            format!("g6.{tag}"),
            format!("a={} n={}", hex64(a), hex64(n)),
            hex_f80(o.mant, o.exp),
            format!("sw={}", hex16(o.status)),
        );
    }
    for (rc_tag, rc) in rcs {
        for (tag, v) in [("p2_5", f2_5), ("n2_5", fn2_5)] {
            let o = x87trans::g6_frndint(*rc, v);
            push(
                &mut r,
                6,
                format!("g6.frndint.{tag}.{rc_tag}"),
                hex64(v),
                hex_f80(o.mant, o.exp),
                format!("sw={}", hex16(o.status)),
            );
        }
    }

    // Group 7: SSE scalars.
    let qnan_a: u32 = 0x7FC0_1111;
    let qnan_b: u32 = 0x7FC0_2222;
    let snan_a: u32 = 0x7F80_1111;
    let one32: u32 = 0x3F80_0000;
    let pzero32: u32 = 0x0000_0000;
    let nzero32: u32 = 0x8000_0000;
    let den32: u32 = 0x0000_0001;
    let neg1_32: u32 = 0xBF80_0000;
    for (tag, a, b) in [
        ("nan_first", qnan_a, one32),
        ("nan_second", one32, qnan_b),
        ("nan_both", qnan_a, qnan_b),
        ("snan_first", snan_a, one32),
    ] {
        for (op, o) in [
            ("addss", sse::g7_addss(a, b)),
            ("subss", sse::g7_subss(a, b)),
            ("mulss", sse::g7_mulss(a, b)),
            ("divss", sse::g7_divss(a, b)),
        ] {
            push(
                &mut r,
                7,
                format!("g7.{op}.{tag}"),
                format!("a={} b={}", hex32(a), hex32(b)),
                hex32(o.bits),
                format!("mxcsr={}", hex32(o.mxcsr)),
            );
        }
    }
    for (tag, a) in [("qnan", qnan_a), ("neg1", neg1_32)] {
        let o = sse::g7_sqrtss(a);
        push(
            &mut r,
            7,
            format!("g7.sqrtss.{tag}"),
            hex32(a),
            hex32(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
    }
    for (tag, a, b) in [
        ("nan_first", qnan_a, one32),
        ("nan_second", one32, qnan_b),
        ("m0_p0", nzero32, pzero32),
        ("p0_m0", pzero32, nzero32),
    ] {
        let o = sse::g7_minss(a, b);
        push(
            &mut r,
            7,
            format!("g7.minss.{tag}"),
            format!("a={} b={}", hex32(a), hex32(b)),
            hex32(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
        let o = sse::g7_maxss(a, b);
        push(
            &mut r,
            7,
            format!("g7.maxss.{tag}"),
            format!("a={} b={}", hex32(a), hex32(b)),
            hex32(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
    }
    let qnan_da: u64 = 0x7FF8_0000_0001_1111;
    let qnan_db: u64 = 0x7FF8_0000_0002_2222;
    let one64: u64 = 0x3FF0_0000_0000_0000;
    for (tag, a, b) in [
        ("nan_first", qnan_da, one64),
        ("nan_second", one64, qnan_db),
        ("m0_p0", nzero, pzero),
        ("p0_m0", pzero, nzero),
    ] {
        let o = sse::g7_minsd(a, b);
        push(
            &mut r,
            7,
            format!("g7.minsd.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex64(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
        let o = sse::g7_maxsd(a, b);
        push(
            &mut r,
            7,
            format!("g7.maxsd.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex64(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
    }
    for (tag, a, b) in [("nan_both", qnan_da, qnan_db)] {
        let o = sse::g7_addsd(a, b);
        push(
            &mut r,
            7,
            format!("g7.addsd.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex64(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
        let o = sse::g7_mulsd(a, b);
        push(
            &mut r,
            7,
            format!("g7.mulsd.{tag}"),
            format!("a={} b={}", hex64(a), hex64(b)),
            hex64(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
    }
    let o = sse::g7_sqrtsd(qnan_da);
    push(
        &mut r,
        7,
        "g7.sqrtsd.qnan".to_string(),
        hex64(qnan_da),
        hex64(o.bits),
        format!("mxcsr={}", hex32(o.mxcsr)),
    );
    for (tag, a) in [
        ("qnan_pay", 0x7FC1_2345u32),
        ("snan", 0x7F80_0001u32),
        ("inf", 0x7F80_0000u32),
        ("denorm", den32),
    ] {
        let o = sse::g7_cvtss2sd(a);
        push(
            &mut r,
            7,
            format!("g7.cvtss2sd.{tag}"),
            hex32(a),
            hex64(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
    }
    let f1e100: u64 = 0x54B2_49AD_2594_C37D; // 1e100, overflows f32
    let third: u64 = 0x3FD5_5555_5555_5555; // 1/3 as f64
    for (tag, a) in [
        ("qnan_pay", qnan_da),
        ("third", third),
        ("overflow_1e100", f1e100),
        ("denorm_min", 0x0000_0000_0000_0001u64),
    ] {
        let o = sse::g7_cvtsd2ss(a);
        push(
            &mut r,
            7,
            format!("g7.cvtsd2ss.{tag}"),
            hex64(a),
            hex32(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
    }
    for (tag, a) in [("one", 1i32), ("round_up", 16_777_217i32)] {
        let o = sse::g7_cvtsi2ss(a);
        push(
            &mut r,
            7,
            format!("g7.cvtsi2ss.{tag}"),
            format!("{a}"),
            hex32(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
        let o = sse::g7_cvtsi2sd(a);
        push(
            &mut r,
            7,
            format!("g7.cvtsi2sd.{tag}"),
            format!("{a}"),
            hex64(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
    }
    for (tag, mx) in [("ftz_off", MXCSR_DEFAULT), ("ftz_on", MXCSR_DEFAULT | MXCSR_FTZ)] {
        let o = sse::g7_addss_mxcsr(mx, den32, den32);
        push(
            &mut r,
            7,
            format!("g7.addss_denorm.{tag}"),
            format!("a={} b={}", hex32(den32), hex32(den32)),
            hex32(o.bits),
            format!("mxcsr={}", hex32(o.mxcsr)),
        );
    }

    r
}

#[cfg(target_arch = "x86")]
fn print_x86() {
    let d = emulation_details();
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!(
        "\"tool\": \"lf-x87-probe\", \"version\": \"0.1.0\", \"arch\": \"{}\", \"os\": \"{}\", \"emulated\": {}, ",
        d.arch,
        d.os,
        d.emulated
    ));
    out.push_str(&format!(
        "\"ambient_cw\": \"{}\", \"ambient_mxcsr\": \"{}\",\n\"probes\": [\n",
        hex16(x87cmp::ambient_cw()),
        hex32(sse::ambient_mxcsr())
    ));
    let recs = run_x86();
    for (i, p) in recs.iter().enumerate() {
        out.push_str(&format!(
            "  {{\"group\": {}, \"name\": \"{}\", \"input\": \"{}\", \"output\": \"{}\", \"extra\": \"{}\"}}{}\n",
            p.group,
            p.name,
            p.input,
            p.output,
            p.extra,
            if i + 1 == recs.len() { "" } else { "," }
        ));
    }
    out.push_str("]\n}\n");
    print!("{out}");
}

#[cfg(not(target_arch = "x86"))]
fn print_stub() {
    let d = emulation_details();
    println!(
        "{{\"tool\": \"lf-x87-probe\", \"version\": \"0.1.0\", \"arch\": \"{}\", \"os\": \"{}\", \"emulated\": {}, \"note\": \"non-x86 stub, no probes\", \"probes\": []}}",
        d.arch, d.os, d.emulated
    );
}

fn main() {
    #[cfg(target_arch = "x86")]
    print_x86();
    #[cfg(not(target_arch = "x86"))]
    print_stub();
}
