// original: 0x00e47c40 uibenchmark_layout_init
//! Benchmark layout init: unless a readiness hook vetoes, gathers two scale
//! selectors, derives a ratio-gated factor, solves two helper queries and
//! publishes the resulting geometry row, then runs two setup hooks and four
//! single-argument table calls.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const E_READY: u32 = 1; // vtable+0x140 thiscall/0: nonzero vetoes (early exit)
const E_PREP: u32 = 2; // 0xe46a00 thiscall/0
const E_QUERY: u32 = 3; // 0x8b9020 cdecl/2: second answer is dereferenced
const E_SEL1: u32 = 4; // 0x425480 cdecl/0: low byte picks first scale
const E_SEL2: u32 = 5; // 0x425480 cdecl/0: low byte picks second scale
const E_AUX: u32 = 6; // 0x8b8ee0 cdecl/1
const E_SOLVE: u32 = 7; // 0x8fc260 cdecl/4
const E_FIN1: u32 = 8; // 0xe47be0 thiscall/0
const E_FIN2: u32 = 9; // 0xe47c10 thiscall/0
// ids 10-13: vtable+0x28/+0x24/+0x18/+0x13c thiscall/1 via object.

const L_BASE: u32 = 0x1e2d0;
const L_F0: u32 = 0x1e8d8;
const L_F1: u32 = 0x1e8dc;
const L_F2: u32 = 0x1e8e0;
const L_I0: u32 = 0x1e8e4;
const L_COUNT: u32 = 0x1e8e8;
const L_W: u32 = 0x1e8ec;
const L_B: u32 = 0x1e8ee;
const L_PTR: u32 = 0x1e8f0;
const L_ONE: u32 = 0x1e8fc;
const L_EXTRA: u32 = 0x1e8f8;

const S_A0: u32 = 0x0105C884;
const S_A1: u32 = 0x0105C888;
const S_B0: u32 = 0x0105C880;
const S_B1: u32 = 0x0105C87C;
const S_MUL: u32 = 0x00FE8B1C;

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
fn fmul(a: f32, b: f32) -> f32 {
    let r = core::hint::black_box(a) * core::hint::black_box(b);
    core::hint::black_box(a);
    core::hint::black_box(b);
    r
}

export!(thiscall, rw_47c40(obj: u32) -> u32 {
    unsafe {
        let vt = rw4(obj);
        let ready: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rw4(vt.wrapping_add(0x140)) as usize);
        let a: u32 = ready(obj);
        if (a & 0xFF) != 0 {
            return a;
        }
        callee_thiscall!(E_PREP, u32, obj);
        let mut q0 = 0u32;
        callee_cdecl!(E_QUERY, u32, &mut q0 as *mut u32 as u32, 0x30);
        let s1: u32 = callee_cdecl!(E_SEL1, u32,);
        let va = if (s1 & 0xFF) != 0 {
            global::<u32>(S_A1).read() as i32
        } else {
            global::<u32>(S_A0).read() as i32
        };
        let s2: u32 = callee_cdecl!(E_SEL2, u32,);
        let vb = if (s2 & 0xFF) != 0 {
            global::<u32>(S_B1).read() as i32
        } else {
            global::<u32>(S_B0).read() as i32
        };
        let ratio = (va as f32) / (vb as f32);
        // The factor seed and the later addend are uninitialized stack reads
        // (a dead local slot and a caller-stack slot past the frame); the
        // contract defines them as zero on both sides.
        let mut x0 = 0.0f32;
        if 1.0f32 > ratio {
            x0 = fmul(x0, ratio);
        }
        ((obj + L_F0) as *mut u32).write(x0.to_bits());
        let mut t1 = 0u32;
        callee_cdecl!(E_AUX, u32, &mut t1 as *mut u32 as u32);
        let mut t2 = 0u32;
        callee_cdecl!(E_SOLVE, u32, 2, &mut t2 as *mut u32 as u32, 0, 0);
        let acc = fmul(f32::from_bits(rw4(obj + L_F0)), gf4(S_MUL));
        let x1 = 0.0f32;
        let e = rw4(obj + L_BASE).wrapping_sub(0xE);
        let acc = fadd(acc, x1);
        ((obj + L_COUNT) as *mut u32).write(e);
        ((obj + L_F1) as *mut u32).write(x1.to_bits());
        ((obj + L_F2) as *mut u32).write(acc.to_bits());
        ((obj + L_I0) as *mut u32).write(0);
        ((obj + L_W) as *mut u16).write(0);
        ((obj + L_B) as *mut u8).write(0);
        ((obj + L_EXTRA) as *mut u32).write(0);
        let mut q1 = 0u32;
        let p2: u32 = callee_cdecl!(E_QUERY, u32, &mut q1 as *mut u32 as u32, 0x32);
        ((obj + L_PTR) as *mut u32).write(rw4(p2));
        callee_thiscall!(E_FIN1, u32, obj);
        ((obj + L_ONE) as *mut u32).write(0x3F800000);
        callee_thiscall!(E_FIN2, u32, obj);
        let vt = rw4(obj);
        let f1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rw4(vt.wrapping_add(0x28)) as usize);
        f1(obj, 1);
        let f2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rw4(vt.wrapping_add(0x24)) as usize);
        f2(obj, 1);
        let f3: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rw4(vt.wrapping_add(0x18)) as usize);
        f3(obj, 1);
        let f4: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rw4(vt.wrapping_add(0x13c)) as usize);
        f4(obj, 1)
    }
});
