// original: 0x00ad30e0 audio_spatial_params_update
//! Spatial-audio parameter update: programs a bank of parameter slots
//! through two setter helpers and finishes with two computed blocks.
//!
//! Specification. The routine takes ten stack words `(mode, p1, f2, p3, p4,
//! f5, f6, p7, sel, obj)` and returns the last setter answer. `mode`
//! selects one of four programs (2, 3, 0, anything else); every program
//! issues the same four head calls and the same four tail calls and differs
//! only in its middle six or seven calls:
//!
//! - Head (all modes): setter A with `(slot(0x1E4), p1)` and `(slot(0x1EC),
//!   p4)`; setter B with `(slot(0x1F0), 1.0/(f2*f2))` and `(slot(0x1F4),
//!   f2)`.
//! - Mode 2: setter A with `(slot(0x1E8), p3)`; convert `f5` and `f6`
//!   through helper C; clamp the difference
//!   `d = f5-f6` up to a floor constant when `d` is not above it; when `0.0`
//!   is above `f6` program `(4.0, C(f5))`, else `(2.0, 0.0)`; then three
//!   more B calls whose values depend on the taken side.
//! - Mode 3: B with `(slot(0x1E0), 3.0)`, A with `(slot(0x1E8), p3)`, then
//!   B calls with `0.0, 0.0, f5, f6`.
//! - Mode 0: B with `(slot(0x1E0), 0.0)`, A with `(slot(0x1E8))` given a
//!   pointer to three zero words, then B calls with `0.0, 0.0, -2.0, 1.0`.
//! - Any other mode: B with `(slot(0x1E0), mode as float)`, A with
//!   `(slot(0x1E8), p3)`, then the same trailing constants as mode 0.
//! - Tail (all modes): A with `(slot(0x20C), p7)`; setter D with
//!   `(slot(0x210), sel)` where a zero `sel` selects a fallback word; A
//!   with `(slot(0x21C))` given `[0, 0, 1.0/obj[2], 1.0/obj[3]]`; A with
//!   `(slot(0x220))` given four words derived from a shared state object.
//!
//! Every setter receives the same object word in ECX. All floating-point
//! steps are single-precision SSE evaluated in program order; integer
//! conversion of `mode` uses wrapping conversion. The routine writes no
//! globals and no heap; the two trailing blocks live in frame scratch.

use lf_checker_rt::{callee_cdecl, callee_thiscall, global};

const ECX_OBJ: u32 = 0x0154_E190;
const SLOT_1E0: u32 = 0x0154_E1E0;
const SLOT_1E4: u32 = 0x0154_E1E4;
const SLOT_1E8: u32 = 0x0154_E1E8;
const SLOT_1EC: u32 = 0x0154_E1EC;
const SLOT_1F0: u32 = 0x0154_E1F0;
const SLOT_1F4: u32 = 0x0154_E1F4;
const SLOT_1FC: u32 = 0x0154_E1FC;
const SLOT_200: u32 = 0x0154_E200;
const SLOT_204: u32 = 0x0154_E204;
const SLOT_208: u32 = 0x0154_E208;
const SLOT_20C: u32 = 0x0154_E20C;
const SLOT_210: u32 = 0x0154_E210;
const SLOT_21C: u32 = 0x0154_E21C;
const SLOT_220: u32 = 0x0154_E220;
const FALLBACK_SEL: u32 = 0x017E_D954;
const SHARED_OBJ: u32 = 0x017F_583C;
const CONST_ONE: u32 = 0x00FE_88E8;
const CONST_FLOOR: u32 = 0x00FE_870C;
const CONST_NEGMASK: u32 = 0x00FE_8FA0;

const ID_A_VAL: u32 = 1;
const ID_A_ZERO: u32 = 2;
const ID_A_OBJ2: u32 = 3;
const ID_A_OBJ3: u32 = 4;
const ID_B: u32 = 5;
const ID_C5: u32 = 6;
const ID_D: u32 = 7;
const ID_C6: u32 = 8;

#[inline(always)]
fn g32(file_va: u32) -> u32 {
    unsafe { global::<u32>(file_va).read() }
}

#[inline(always)]
fn gf(file_va: u32) -> f32 {
    f32::from_bits(g32(file_va))
}

#[inline(always)]
fn bb(x: f32) -> f32 {
    core::hint::black_box(x)
}

#[inline(always)]
fn set_a(id: u32, ecx: u32, slot_va: u32, value: u32) -> u32 {
    callee_thiscall!(id, u32, ecx, g32(slot_va), value)
}

#[inline(always)]
fn set_b(ecx: u32, slot_va: u32, bits: u32) -> u32 {
    callee_thiscall!(ID_B, u32, ecx, g32(slot_va), bits)
}

#[inline(always)]
fn convert5(v: u32) -> u32 {
    callee_cdecl!(ID_C5, u32, v)
}

#[inline(always)]
fn convert6(v: u32) -> u32 {
    callee_cdecl!(ID_C6, u32, v)
}

#[allow(clippy::too_many_arguments)]
fn body(
    mode: u32,
    p1: u32,
    f2b: u32,
    p3: u32,
    p4: u32,
    f5b: u32,
    f6b: u32,
    p7: u32,
    sel: u32,
    obj: u32,
) -> u32 {
    let ecx = g32(ECX_OBJ);
    let one = gf(CONST_ONE);
    set_a(ID_A_VAL, ecx, SLOT_1E4, p1);
    set_a(ID_A_VAL, ecx, SLOT_1EC, p4);
    let f2 = bb(f32::from_bits(f2b));
    let sq = bb(bb(f2) * bb(f2));
    let inv = bb(bb(one) / bb(sq));
    set_b(ecx, SLOT_1F0, inv.to_bits());
    set_b(ecx, SLOT_1F4, f2b);

    let mode_i = mode as i32;
    if mode_i == 2 {
        set_a(ID_A_VAL, ecx, SLOT_1E8, p3);
        let c5 = convert5(f5b);
        let c6 = convert6(f6b);
        let f5 = f32::from_bits(f5b);
        let f6 = f32::from_bits(f6b);
        let d = bb(bb(f5) - bb(f6));
        let floor = gf(CONST_FLOOR);
        let dd = if bb(d) > bb(floor) { d } else { floor };
        if 0.0f32 > bb(f6) {
            set_b(ecx, SLOT_1E0, 4.0f32.to_bits());
            set_b(ecx, SLOT_1FC, c5);
            set_b(ecx, SLOT_200, c6);
            set_b(ecx, SLOT_204, (-2.0f32).to_bits());
            set_b(ecx, SLOT_208, 1.0f32.to_bits());
        } else {
            set_b(ecx, SLOT_1E0, 2.0f32.to_bits());
            set_b(ecx, SLOT_1FC, 0);
            set_b(ecx, SLOT_200, c6);
            set_b(ecx, SLOT_204, f6b);
            let r = bb(bb(one) / bb(dd));
            set_b(ecx, SLOT_208, r.to_bits());
        }
    } else if mode_i == 3 {
        set_b(ecx, SLOT_1E0, 3.0f32.to_bits());
        set_a(ID_A_VAL, ecx, SLOT_1E8, p3);
        set_b(ecx, SLOT_1FC, 0);
        set_b(ecx, SLOT_200, 0);
        set_b(ecx, SLOT_204, f5b);
        set_b(ecx, SLOT_208, f6b);
    } else if mode_i == 0 {
        set_b(ecx, SLOT_1E0, 0);
        let zeros = [0u32; 3];
        set_a(ID_A_ZERO, ecx, SLOT_1E8, zeros.as_ptr() as u32);
        set_b(ecx, SLOT_1FC, 0);
        set_b(ecx, SLOT_200, 0);
        set_b(ecx, SLOT_204, (-2.0f32).to_bits());
        set_b(ecx, SLOT_208, 1.0f32.to_bits());
    } else {
        set_b(ecx, SLOT_1E0, (mode_i as f32).to_bits());
        set_a(ID_A_ZERO, ecx, SLOT_1E8, p3);
        set_b(ecx, SLOT_1FC, 0);
        set_b(ecx, SLOT_200, 0);
        set_b(ecx, SLOT_204, (-2.0f32).to_bits());
        set_b(ecx, SLOT_208, 1.0f32.to_bits());
    }

    set_a(ID_A_VAL, ecx, SLOT_20C, p7);
    let sel_v = if sel == 0 { g32(FALLBACK_SEL) } else { sel };
    callee_thiscall!(ID_D, u32, ecx, g32(SLOT_210), sel_v);

    let w2 = unsafe { ((obj + 8) as *const u32).read() };
    let w3 = unsafe { ((obj + 12) as *const u32).read() };
    let r2 = bb(bb(one) / bb(f32::from_bits(w2)));
    let r3 = bb(bb(one) / bb(f32::from_bits(w3)));
    let blk2 = [0u32, 0u32, r2.to_bits(), r3.to_bits()];
    set_a(ID_A_OBJ2, ecx, SLOT_21C, blk2.as_ptr() as u32);

    let base = g32(SHARED_OBJ);
    let m1 = f32::from_bits(unsafe { ((base + 0x2C0) as *const u32).read() });
    let m3 = f32::from_bits(unsafe { ((base + 0x2C4) as *const u32).read() });
    let m2 = f32::from_bits(unsafe { ((base + 0x2C8) as *const u32).read() });
    let m0 = f32::from_bits(unsafe { ((base + 0x2CC) as *const u32).read() });
    let sub = bb(bb(m3) - bb(m1));
    let ratio = bb(bb(m3) / bb(sub));
    let prod = bb(bb(ratio) * bb(m1));
    let mask = g32(CONST_NEGMASK);
    let neg = f32::from_bits(prod.to_bits() ^ mask);
    let div = bb(bb(one) / bb(neg));
    let scaled = bb(bb(div) * bb(ratio));
    let blk3 = [m0.to_bits(), m2.to_bits(), div.to_bits(), scaled.to_bits()];
    set_a(ID_A_OBJ3, ecx, SLOT_220, blk3.as_ptr() as u32)
}

lf_checker_rt::export!(cdecl, fn_00ad30e0(
    mode: u32, p1: u32, f2b: u32, p3: u32, p4: u32,
    f5b: u32, f6b: u32, p7: u32, sel: u32, obj: u32,
) -> u32 {
    body(mode, p1, f2b, p3, p4, f5b, f6b, p7, sel, obj)
});

