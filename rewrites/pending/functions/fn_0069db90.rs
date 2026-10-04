// original: 0x0069DB90 state_refresh
//! State refresh for the files-memory controller.
//!
//! With a nonzero flag byte the six state words are cleared. With a zero
//! flag the attached object (if any) is probed through its vtable: a live
//! probe clears two words, otherwise the state is refreshed from the source
//! words (quotient, saved-word rotation, flag mask, and, when enabled, a
//! float rescale with clamping).
//!
//! This file holds the verified rewrite plus the small helpers it uses.
//! It is written against `lf-checker-rt`; see the lane crate for the build.

use lf_checker_rt::{export, global, relocated};

/// Truncating float-to-int conversion with x86 `cvttss2si` semantics:
/// round toward zero; NaN or out-of-range yields `i32::MIN` (it never
/// saturates, unlike a Rust `as` cast).
#[inline(always)]
fn cvtt_ss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0x80000000u32 as i32
    } else {
        x as i32
    }
}

/// The quotient step the original computes with a magic multiply
/// (`0x88888889 * m`, high half adjusted and shifted): exact emulation.
#[inline(always)]
fn magic_quotient(m: i32) -> i32 {
    let prod = (0x88888889u32 as i32 as i64).wrapping_mul(m as i64);
    let mut edx = (prod >> 32) as i32;
    edx = edx.wrapping_add(m);
    edx >>= 6;
    let adjust = ((edx as u32) >> 31) as i32;
    adjust.wrapping_add(edx)
}

const ST_A: u32 = 0x018B7A68;
const ST_B: u32 = 0x018B7A6C;
const ST_Q: u32 = 0x018B7A70;
const ST_SAVED: u32 = 0x018B7A74;
const ST_C: u32 = 0x018B7A7C;
const ST_W: u32 = 0x018B7A80;
const ST_D: u32 = 0x018B7A84;
const ST_H: u32 = 0x018B7A88;
const ST_H2: u32 = 0x018B7A8C;
const FL_FAST: u32 = 0x018B7A5E;
const FL_MASK: u32 = 0x018B7A5F;
const SRC_A: u32 = 0x01BB3928;
const SRC_B: u32 = 0x01BB392C;
const SRC_M: u32 = 0x01BB3930;
const SRC_FLAGS: u32 = 0x01BB3934;
const SRC_OBJ: u32 = 0x01BB393C;
const LIM_B: u32 = 0x017ACCDC;
const LIM_SCALE: u32 = 0x017ACCE8;
const LIM_A: u32 = 0x017ACCEC;
const LIM_W: u32 = 0x017ACCF0;
const K_SMALL: u32 = 0x00FE86A8;
const K_HALF: u32 = 0x00FE8830;

/// State refresh for the files-memory controller (see module docs).
export!(thiscall, rw_69db90(flag: u32) -> u32 {
    unsafe {
        if (flag & 0xFF) != 0 {
            *global::<u32>(ST_A) = 0;
            *global::<u32>(ST_B) = 0;
            *global::<u32>(ST_Q) = 0;
            *global::<u32>(ST_C) = 0;
            *global::<u32>(ST_D) = 0;
            *global::<u32>(ST_H) = 0;
            // The original leaves EAX untouched on this path, so incoming
            // EAX passes through; the contract pins incoming EAX to 0.
            return 0;
        }
        let obj = *global::<u32>(SRC_OBJ);
        if obj == 0 {
            *global::<u32>(ST_Q) = *global::<u32>(ST_C);
            *global::<u32>(ST_D) = *global::<u32>(ST_H);
            let saved = *global::<u32>(ST_SAVED);
            *global::<u32>(ST_C) = 0;
            *global::<u32>(ST_H) = saved;
            return saved;
        }
        // Probe the object through its vtable exactly like the original;
        // both sides land on the same planted recorder stubs.
        type Probe = extern "stdcall" fn(u32, u32, u32) -> u32;
        type Close = extern "stdcall" fn(u32) -> u32;
        let probe = |this: u32| -> u32 {
            let vt = *(this as *const u32);
            let f: Probe =
                core::mem::transmute(*((vt.wrapping_add(0x24)) as *const u32));
            f(this, 0x14, relocated(SRC_A))
        };
        if probe(obj) != 0 {
            let obj2 = *global::<u32>(SRC_OBJ);
            let vt2 = *(obj2 as *const u32);
            let g: Close =
                core::mem::transmute(*((vt2.wrapping_add(0x1C)) as *const u32));
            g(obj2);
            let obj3 = *global::<u32>(SRC_OBJ);
            let second = probe(obj3);
            if second != 0 {
                *global::<u32>(ST_H) = 0;
                *global::<u32>(ST_D) = 0;
                return second;
            }
        }
        let m = *global::<i32>(SRC_M);
        let q = magic_quotient(m);
        let esi = *global::<u32>(SRC_A);
        let edi = *global::<u32>(SRC_B);
        let fast = *global::<u8>(FL_FAST);
        let mask_path = *global::<u8>(FL_MASK) == 0;
        *global::<u32>(ST_Q) = q as u32;
        *global::<u32>(ST_A) = esi;
        *global::<u32>(ST_B) = edi;
        *global::<u32>(ST_D) = *global::<u32>(ST_H);
        if !mask_path && fast == 0 {
            let saved = *global::<u32>(ST_SAVED);
            *global::<u32>(ST_H) = saved;
            return saved;
        }
        let flags = global::<u8>(SRC_FLAGS);
        let mut mask: u32 = 0;
        if *flags.add(0) != 0 {
            mask |= 0x01;
        }
        if *flags.add(1) != 0 {
            mask |= 0x02;
        }
        if *flags.add(2) != 0 {
            mask |= 0x04;
        }
        if *flags.add(3) != 0 {
            mask |= 0x08;
        }
        if *flags.add(4) != 0 {
            mask |= 0x10;
        }
        if *flags.add(5) != 0 {
            mask |= 0x12;
        }
        if *flags.add(6) != 0 {
            mask |= 0x14;
        }
        let last = if *flags.add(7) != 0 { 0x18 } else { 0 };
        mask |= last;
        *global::<u32>(ST_H) = mask;
        if fast == 0 {
            return last;
        }
        let w0 = *global::<i32>(ST_W);
        let lim_a = *global::<i32>(LIM_A);
        let lim_b = *global::<i32>(LIM_B);
        let scale = *global::<f32>(LIM_SCALE);
        let k_small = *global::<f32>(K_SMALL);
        let k_half = *global::<f32>(K_HALF);
        let w_coef = *global::<f32>(LIM_W);
        let mut x1 = w0 as f32;
        x1 *= scale;
        let lim_a_f = lim_a as f32;
        let lim_b_f = lim_b as f32;
        x1 *= lim_a_f;
        let mut t = (esi as i32) as f32;
        t *= k_small;
        t *= lim_b_f;
        x1 += t;
        let mut u = (*global::<i32>(ST_H2)) as f32;
        x1 += k_half;
        u *= w_coef;
        let mut e = cvtt_ss2si(x1);
        let mut v = (edi as i32) as f32;
        v *= k_small;
        if lim_a < e {
            e = lim_a;
        }
        if e < 0 {
            e = 0;
        }
        *global::<i32>(ST_W) = e;
        v += u;
        v *= lim_b_f;
        v += k_half;
        let mut e2 = cvtt_ss2si(v);
        if lim_b < e2 {
            e2 = lim_b;
        }
        if e2 < 0 {
            e2 = 0;
        }
        *global::<i32>(ST_H2) = e2;
        e2 as u32
    }
});
