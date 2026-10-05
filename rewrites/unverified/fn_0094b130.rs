// original: 0x0094b130 global_matrix_pass
//! Global matrix pass: two-word gate, optional setup block, float section,
//! tail dispatch (cdecl/2 -> u32).
//!
//! Behaviour. The function takes two stack words. It first issues two fixed
//! setup calls and two parameterised calls carrying one global word, a
//! pointer to freshly loaded global floats, and two zero words; all four
//! answers are ignored. It then reads its first argument and compares it
//! with a global word: on mismatch it runs the cookie check and returns the
//! argument unchanged. Otherwise it compares the low bit of a global mode
//! byte with the low byte of its second argument: on mismatch it runs the
//! cookie check and returns the first argument with its low byte replaced by
//! that mode bit.
//!
//! On the main path, when bit 0x20 of the mode byte is set and a second
//! global flag byte is nonzero, a setup block runs: constant-pair calls, a
//! fixed eight-word block of unit and zero words handed to a five-argument
//! call as four strided pairs plus a shifted global byte, a zero-argument
//! call, a second five-argument call carrying four constant image addresses
//! plus the same shifted byte, more constant-pair calls, and a closing
//! zero-argument call. All answers are ignored and the mode byte is
//! re-read afterwards (same trial value).
//!
//! The float section branches on whether a global float is an ordered zero:
//! either sign of zero takes a shuffle path that rearranges the four loaded
//! global floats into eight result words, while any other value (including
//! not-a-number) takes an arithmetic path of about forty single-precision
//! additions, multiplications and subtractions in strict order, fed by two
//! helper calls that each take the gating float in the vector register and
//! answer in the vector register. Every arithmetic step is pinned with an
//! opaque compiler barrier so the emitted operand order and the absence of
//! fused multiply-add contraction match the original bit for bit.
//!
//! The tail tests bit 0x40 of the mode byte: when set it queries a fixed
//! global object (low byte of the answer kept), otherwise it forwards one
//! global word to a one-argument call. It then passes the eight result words
//! as four strided pairs plus one constant image address to a five-argument
//! call, and, only when the 0x40 bit is set and the kept answer byte is
//! nonzero, finishes with a second query to the same global object. Exit
//! value is that last query's answer when it runs, otherwise the
//! five-argument call's answer; the cookie check runs on every path and
//! preserves all registers.
//!
//! Proof notes. The contract observes every outgoing call: constant arguments
//! compare directly, frame-pointer arguments are skipped with call-time
//! snapshots of the pointed-to words (two words per result pair, four for
//! the widest float span), and the vector-register helper inputs compare
//! through the call log with the bits carried on the stack on this side.
//! Input cycles cover both early exits, setup-block on/off, shuffle and
//! arithmetic float paths (the zero gate sees both signed zeros, finite
//! values, infinity and not-a-number), both tail branches, and the final
//! query taken and skipped. The arithmetic inputs keep not-a-number values
//! out of two-operand positions only if a both-NaN payload mismatch appears;
//! single NaNs are order-free and stay in the edge pools.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// File VAs (image base 0x400000) of the globals this function reads.
const G_FORWARD: u32 = 0x0103_7660;
const G_FLT_A: u32 = 0x0103_7664;
const G_FLT_B: u32 = 0x0103_7668;
const G_FLT_C: u32 = 0x0103_766C;
const G_FLT_D: u32 = 0x0103_7670;
const G_GATE_FLT: u32 = 0x0103_7674;
const G_ARG_MATCH: u32 = 0x0103_7678;
const G_PARAM: u32 = 0x0103_767C;
const G_SHIFT_BYTE: u32 = 0x0103_7683;
const G_MODE_BYTE: u32 = 0x0103_769A;
const G_FLAG_BYTE: u32 = 0x011E_61CB;
const G_FACTOR: u32 = 0x00FE_8830;

/// Fixed addresses the original passes as plain values (never dereferenced
/// here; the intercepted callees only log them).
const OBJ_FIXED: u32 = 0x0128_E94C;
const TAIL_CONST: u32 = 0x0103_7680;
const SETUP_CONSTS: [u32; 4] = [0x011E_E28C, 0x011E_E294, 0x011E_E29C, 0x011E_E2A4];

/// Mode-byte bits: argument-bit match, setup block enable, tail branch.
const MODE_ARG_BIT: u8 = 0x01;
const MODE_SETUP: u8 = 0x20;
const MODE_TAIL: u8 = 0x40;

/// Single-precision steps pinned against operand reorder and contraction.
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    black_box(black_box(a) + black_box(b))
}
#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    black_box(black_box(a) - black_box(b))
}
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    black_box(black_box(a) * black_box(b))
}

/// The cookie check preserves every register; its answer is meaningless and
/// only the call itself is observed, so the result is dropped on purpose.
#[inline(always)]
fn cookie_check() {
    let _ = callee_cdecl!(15, u32,);
}

fn body<const MUT: bool>(a0: u32, a1: u32) -> u32 {
    let g_forward = unsafe { global::<u32>(G_FORWARD).read() };
    let fa = f32::from_bits(unsafe { global::<u32>(G_FLT_A).read() });
    let fb = f32::from_bits(unsafe { global::<u32>(G_FLT_B).read() });
    let fc = f32::from_bits(unsafe { global::<u32>(G_FLT_C).read() });
    let fd = f32::from_bits(unsafe { global::<u32>(G_FLT_D).read() });
    let gate_bits = unsafe { global::<u32>(G_GATE_FLT).read() };
    let gate = f32::from_bits(gate_bits);
    let g_match = unsafe { global::<u32>(G_ARG_MATCH).read() };
    let g_param = unsafe { global::<u32>(G_PARAM).read() };
    let shifted = (unsafe { global::<u8>(G_SHIFT_BYTE).read() } as u32) << 24;
    let mode = unsafe { global::<u8>(G_MODE_BYTE).read() };
    let flag = unsafe { global::<u8>(G_FLAG_BYTE).read() };
    let factor = f32::from_bits(unsafe { global::<u32>(G_FACTOR).read() });

    let _ = callee_cdecl!(1, u32,);
    let pair_hi = [fa.to_bits(), fb.to_bits()];
    let quad = [fc.to_bits(), fd.to_bits(), fa.to_bits(), fb.to_bits()];
    let _ = callee_cdecl!(2, u32, g_param, pair_hi.as_ptr() as u32, 0, 0);
    let _ = callee_cdecl!(3, u32, g_param, quad.as_ptr() as u32, 0, 0);
    let _ = callee_cdecl!(1, u32,);
    if MUT {
        let _ = callee_cdecl!(4, u32, 6, 1);
    } else {
        let _ = callee_cdecl!(4, u32, 6, 0);
    }

    if a0 != g_match {
        cookie_check();
        return a0;
    }
    let mode_bit = mode & MODE_ARG_BIT;
    if mode_bit != (a1 & 0xFF) as u8 {
        cookie_check();
        return (a0 & 0xFFFF_FF00) | mode_bit as u32;
    }

    let setup_on = (mode & MODE_SETUP) != 0 && flag != 0;
    if setup_on {
        let _ = callee_cdecl!(4, u32, 2, 5);
        let _ = callee_cdecl!(4, u32, 0xF, 8);
        let one = 1.0f32.to_bits();
        let block = [one, one, one, 0, 0, one, 0, 0];
        let shift_word = [shifted];
        let _ = callee_cdecl!(5, u32, 0);
        let _ = callee_cdecl!(6, u32,);
        let _ = callee_cdecl!(
            7, u32,
            &block[0] as *const u32 as u32,
            &block[2] as *const u32 as u32,
            &block[4] as *const u32 as u32,
            &block[6] as *const u32 as u32,
            shift_word.as_ptr() as u32
        );
        let _ = callee_cdecl!(4, u32, 2, 8);
        let _ = callee_cdecl!(4, u32, 0xF, 8);
        let _ = callee_cdecl!(
            8, u32,
            SETUP_CONSTS[0], SETUP_CONSTS[1], SETUP_CONSTS[2], SETUP_CONSTS[3],
            shift_word.as_ptr() as u32
        );
        let _ = callee_cdecl!(4, u32, 2, 6);
        let _ = callee_cdecl!(4, u32, 0xF, 0xF);
        let _ = callee_cdecl!(10, u32,);
    }

    let res: [u32; 8];
    if gate != 0.0 {
        // Arithmetic path: strict original order, every step pinned.
        let acc_hi = fmul(fadd(fc, fa), factor);
        let acc_lo = fmul(fadd(fb, fd), factor);
        let tail_hi = fsub(acc_hi, fa);
        let tail_lo = fsub(acc_lo, fb);
        let ans1 = f32::from_bits(callee_cdecl!(11, u32, gate_bits));
        let ans2 = f32::from_bits(callee_cdecl!(12, u32, gate_bits));
        let prod0 = fmul(ans1, tail_hi);
        let _stored_prod0 = prod0;
        let prod1 = fmul(ans1, tail_lo);
        let prod2 = fmul(ans2, tail_lo);
        let prod3 = fmul(ans2, tail_hi);
        let mut run2 = fsub(acc_hi, prod1);
        let mut run1 = fsub(acc_lo, prod2);
        let mut run3 = fadd(prod1, acc_hi);
        let mut run4 = fadd(prod2, acc_lo);
        let e1 = fsub(run2, prod3);
        run2 = fadd(run2, prod3);
        let tmp0 = fadd(run1, prod0);
        let fold0 = run2;
        run2 = prod0;
        run1 = fsub(run1, run2);
        let e2 = tmp0;
        let tmp1 = fsub(run3, prod3);
        let e3 = fold0;
        let fold1 = run1;
        let fold0b = tmp1;
        let tmp2 = fadd(run4, run2);
        let e4 = fold1;
        let e5 = fold0b;
        run3 = fadd(run3, prod3);
        run4 = fsub(run4, run2);
        let fold1b = tmp2;
        let e6 = fold1b;
        let e7 = run3;
        let e8 = run4;
        res = [
            e1.to_bits(), e2.to_bits(), e3.to_bits(), e4.to_bits(),
            e5.to_bits(), e6.to_bits(), e7.to_bits(), e8.to_bits(),
        ];
    } else {
        // Shuffle path: the four loaded floats rearranged into eight words.
        res = [
            fa.to_bits(), fd.to_bits(), fa.to_bits(), fb.to_bits(),
            fc.to_bits(), fd.to_bits(), fc.to_bits(), fb.to_bits(),
        ];
    }

    let mut kept: u8 = 0;
    let tail_query = (mode & MODE_TAIL) != 0;
    if tail_query {
        kept = (callee_thiscall!(13, u32, OBJ_FIXED) & 0xFF) as u8;
    } else {
        let _ = callee_cdecl!(5, u32, g_forward);
    }
    let last = callee_cdecl!(
        9, u32,
        &res[0] as *const u32 as u32,
        &res[2] as *const u32 as u32,
        &res[4] as *const u32 as u32,
        &res[6] as *const u32 as u32,
        TAIL_CONST
    );
    if tail_query && kept != 0 {
        let fin = callee_thiscall!(14, u32, OBJ_FIXED);
        cookie_check();
        return fin;
    }
    cookie_check();
    last
}

export!(cdecl, rw_0094b130(a0: u32, a1: u32) -> u32 {
    body::<false>(a0, a1)
});

export!(cdecl, mut_0094b130(a0: u32, a1: u32) -> u32 {
    body::<true>(a0, a1)
});
