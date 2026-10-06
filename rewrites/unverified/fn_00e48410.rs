// original: 0x00e48410 uibenchmark_gauge_update
//! Benchmark gauge update: picks a scale from a status table, solves a
//! float target through a helper, blends it into the gauge accumulator,
//! then clamps and settles the gauge level.
//!
//! Control flow notes: the scale-select gate leaves through a
//! compare/load-flags sequence that sets the middle return byte to 0x42 on
//! the exit path; the settle comparisons are ordered float compares where an
//! unordered (NaN) outcome skips the clamp; the post call sits inside the
//! gated region and is skipped with it.
//!
//! The solve call reads its first, second and fifth words: 1.0,
//! 127.0 and a path-dependent float (table-arm absolute conversion,
//! 127.0 otherwise); all three are snapshotted.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const D_INIT: u32 = 1; // 0xdb3e10 thiscall/0
const D_SETUP: u32 = 2; // 0xe46920 thiscall/0
const D_LOOKUP: u32 = 3; // 0x8f69d0 cdecl/1: status record, bytes decide
const D_QUERY: u32 = 4; // 0x8b9020 cdecl/2
const D_SOLVE: u32 = 5; // 0xb1b5e0 thiscall/5: five out-words + f32 on ST0
const D_GATE: u32 = 6; // 0xe47bb0 thiscall/0: low byte decides
const D_POST: u32 = 7; // 0xe47c10 thiscall/0
const D_FIN: u32 = 8; // 0xe47be0 thiscall/0

const F_ACCUM: u32 = 0x1e8d8;
const F_LEVEL: u32 = 0x1e8e4;
const F_LIMIT: u32 = 0x1e8e8;
const F_ACTIVE: u32 = 0x1e8ec;
const F_DIR: u32 = 0x1e8ed;
const F_MODE: u32 = 0x1e8ee;
const F_TARGET: u32 = 0x1e8f8;

const G_ROWSEL: u32 = 0x010330F8;
const G_TABLE: u32 = 0x0118D470;
const G_HALF: u32 = 0x00FE8830;
const G_Q124: u32 = 0x00FE8920;

#[inline(always)]
unsafe fn rb4(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn rw4(a: u32) -> u32 {
    unsafe { (a as *const u32).read() }
}

#[inline(always)]
unsafe fn gf4(va: u32) -> f32 {
    unsafe { f32::from_bits(global::<u32>(va).read()) }
}

#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    // Pinned operand order (r-b141/r-b186 idiom): boxing both operands and
    // keeping them live stops the backend from commuting the add, which
    // would pick the wrong NaN payload when both operands are NaN.
    let r = core::hint::black_box(a) + core::hint::black_box(b);
    core::hint::black_box(a);
    core::hint::black_box(b);
    r
}

#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    a - b
}

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    let r = core::hint::black_box(a) * core::hint::black_box(b);
    core::hint::black_box(a);
    core::hint::black_box(b);
    r
}

export!(thiscall, rw_48410(obj: u32) -> u32 {
    unsafe {
        callee_thiscall!(D_INIT, u32, obj);
        let mut eax: u32 = callee_thiscall!(D_SETUP, u32, obj);
        if rb4(obj + F_ACTIVE) == 0 {
            return eax;
        }
        let idx = global::<u32>(G_ROWSEL).read() as i32;
        let row = if idx == -1 {
            0
        } else {
            (idx as u32).wrapping_mul(0xbc).wrapping_add(relocated(G_TABLE))
        };
        let p: u32 = callee_cdecl!(D_LOOKUP, u32, 0);
        eax = p;
        let cl = rb4(obj + F_MODE);
        // Scale select. The table-value arm is taken only when the mode is
        // clear, the record's first status byte is set and a row is present;
        // its converted value is never stored, it only gates the exit below
        // (zero converts to +0.0, which compares equal and exits). Both
        // exits below leave through the flag-loading gate, which sets the
        // middle return byte to 0x42 for the ordered-equal outcome.
        let s18: f32;
        let mut o4: u32;
        if cl == 0 && rb4(p.wrapping_add(0x328c)) != 0 && row != 0 {
            let rv = rw4(row.wrapping_add(0x10)) as i32;
            if rv == 0 {
                return (eax & 0xFFFF00FF) | 0x4200;
            }
            s18 = gf4(G_HALF);
            o4 = (rv as f32).abs().to_bits();
        } else if cl != 0 {
            s18 = gf4(G_HALF);
            o4 = 0x42fe0000;
        } else if rb4(p.wrapping_add(0x328d)) != 0 {
            s18 = gf4(G_Q124);
            o4 = global::<u32>(0x00FE8BC4).read();
        } else {
            return (eax & 0xFFFF00FF) | 0x4200;
        }
        let mut probe = 0u32;
        callee_cdecl!(D_QUERY, u32, &mut probe as *mut u32 as u32, 0x2f);
        let mut o0 = 0x3f800000u32;
        let mut o1 = 0x42fe0000u32;
        let mut w_a2 = 0u32;
        let mut w_a3 = 0u32;
        let st0: f32 = callee_thiscall!(
            D_SOLVE, f32, obj,
            &mut o0 as *mut u32 as u32,
            &mut o1 as *mut u32 as u32,
            &mut w_a2 as *mut u32 as u32,
            &mut w_a3 as *mut u32 as u32,
            &mut o4 as *mut u32 as u32
        );
        let mut x1 = st0;
        let x2 = f32::from_bits(w_a2);
        let mut x0 = f32::from_bits(w_a3);
        if x2 > x1 {
            x1 = x2;
        }
        if x0 > x1 {
            x1 = x0;
        }
        x0 = fmul(fsub(fadd(x0, x2), x1), gf4(G_HALF));
        let s = x0;
        let g: u32 = callee_thiscall!(D_GATE, u32, obj);
        eax = g;
        if (g & 0xFF) != 0 {
            let dir = rb4(obj + F_DIR);
            if dir == 0 {
                if (rw4(obj + F_LEVEL) as i32) < (rw4(obj + F_LIMIT) as i32) {
                    let t = fadd(
                        fmul(f32::from_bits(rw4(obj + F_ACCUM)) / s, s18),
                        f32::from_bits(rw4(obj + F_TARGET)),
                    );
                    ((obj + F_TARGET) as *mut u32).write(t.to_bits());
                }
            } else if (rw4(obj + F_LEVEL) as i32) > 0 {
                let t = fmul(f32::from_bits(rw4(obj + F_ACCUM)) / s, s18);
                let t2 = f32::from_bits(rw4(obj + F_TARGET)) - t;
                ((obj + F_TARGET) as *mut u32).write(t2.to_bits());
            }
        }
        if (g & 0xFF) != 0 {
            // The post call sits inside the gated region: skipped with it.
            eax = callee_thiscall!(D_POST, u32, obj);
        }
        let al = rb4(obj + F_DIR);
        eax = (eax & 0xFFFFFF00) | (al as u32);
        if al == 0 {
            if f32::from_bits(rw4(obj + F_TARGET)) >= f32::from_bits(rw4(obj + F_ACCUM)) {
                let lim = rw4(obj + F_LIMIT);
                eax = lim;
                let lv = rw4(obj + F_LEVEL).wrapping_add(1);
                ((obj + F_LEVEL) as *mut u32).write(lv);
                if (lv as i32) > (lim as i32) {
                    ((obj + F_LEVEL) as *mut u32).write(lim);
                }
                ((obj + F_TARGET) as *mut u32).write(0);
            }
        } else {
            let nd = f32::from_bits(rw4(obj + F_ACCUM) ^ 0x80000000);
            if nd >= f32::from_bits(rw4(obj + F_TARGET)) {
                let lv = rw4(obj + F_LEVEL).wrapping_sub(1);
                let lv = if (lv as i32) < 0 { 0 } else { lv };
                eax = lv;
                ((obj + F_LEVEL) as *mut u32).write(lv);
                ((obj + F_TARGET) as *mut u32).write(0);
            }
        }
        if rb4(obj + F_MODE) != 0 {
            eax = callee_thiscall!(D_FIN, u32, obj);
            let a2 = rb4(obj + F_DIR);
            eax = (eax & 0xFFFFFF00) | (a2 as u32);
            if a2 != 0 {
                if rw4(obj + F_LEVEL) == 0 {
                    ((obj + F_MODE) as *mut u8).write(0);
                }
            } else {
                let lv = rw4(obj + F_LEVEL);
                eax = lv;
                if lv == rw4(obj + F_LIMIT) {
                    ((obj + F_MODE) as *mut u8).write(0);
                }
            }
        }
        eax
    }
});
