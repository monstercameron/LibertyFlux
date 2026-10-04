// original: 0x00d74ca0 overlay_compute_layout
//
// Recomputes the overlay's float layout table from two global base values,
// small constant adjustments, and per-pass scale selectors. Four scripted
// selector answers pick between paired global scale words; two scripted
// float answers feed an accumulator chain; then a fixed 17-step loop emits
// coordinate pairs while accumulating further scripted floats.
//
// All single-precision adds and multiplies run through fadd/fmul below,
// which pin the original's operand order: when both operands are NaN, SSE
// forwards the destination's payload, and the backend otherwise emits the
// operation with either operand as the destination. Boxing both operands
// and keeping them live past the operation holds the order (r-b141's
// idiom); plain black_box on one operand and the _mm_add_ss intrinsic
// idiom both still commuted (verified in DLL dumps). Any rebuild must
// re-run the multi-NaN trials in the contract.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

// Callee ids (see contract).
const C_SETUP: u32 = 1; // 0xd72d70 thiscall/1: per-pass setup (answer unused)
const C_SEL: u32 = 2; // 0x425480 cdecl/0: scale selector, low byte decides
const C_F1: u32 = 3; // 0x91bc50 cdecl/0: float answer on x87 ST0
const C_F2: u32 = 4; // 0x91c620 cdecl/0: float answer on x87 ST0

// Globals (file VAs).
const G_BASE0: u32 = 0x0118EFFC;
const G_BASE1: u32 = 0x0118EFF4;
const G_ADJ_A: u32 = 0x00FE87E8;
const G_ADJ_B: u32 = 0x00FE86EC;
const G_ADJ_C: u32 = 0x00FE870C;
const G_MUL16: u32 = 0x00FE8B28;
const G_SEL0A: u32 = 0x0105C884;
const G_SEL0B: u32 = 0x0105C888;
const G_SEL1A: u32 = 0x0105C880;
const G_SEL1B: u32 = 0x0105C87C;

#[inline(always)]
unsafe fn gf(va: u32) -> f32 {
    unsafe { f32::from_bits(global::<u32>(va).read()) }
}

#[inline(always)]
unsafe fn gi(va: u32) -> i32 {
    unsafe { global::<u32>(va).read() as i32 }
}

#[inline(always)]
unsafe fn rf(addr: u32) -> f32 {
    unsafe { f32::from_bits((addr as *const u32).read_unaligned()) }
}

#[inline(always)]
unsafe fn wf(addr: u32, v: f32) {
    unsafe { (addr as *mut u32).write_unaligned(v.to_bits()) }
}

/// Single-precision multiply/add with pinned operand order: when both
/// operands are NaN, SSE forwards the destination's payload, and LLVM
/// otherwise emits the add/mul with either operand as the destination
/// (identical for finite values, wrong NaN winner for mixed NaNs).
/// Boxing both operands and keeping them live past the operation stops
/// the backend from reusing a dead operand's register as the
/// destination (r-b141's idiom; verified in the DLL dump). Subtraction
/// is never commuted and stays plain.
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
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

export!(thiscall, rw_d74ca0(this_ptr: u32) -> u32 {
    unsafe {
        // Base value and per-pass setup.
        let base = fadd(gf(G_BASE0), gf(G_BASE1));
        callee_thiscall!(C_SETUP, u32, this_ptr, 1);

        let a0 = fsub(base, gf(G_ADJ_A));
        let raw = rf(this_ptr.wrapping_add(0x0C));
        let t1 = fadd(raw, gf(G_ADJ_B));
        let a2 = fadd(a0, gf(G_ADJ_B));

        // Four scale selects: low byte of the answer picks the scale word.
        // Selects 1 and 3 use the 0x884/0x888 pair, 2 and 4 the 0x880/0x87C pair.
        let s: u32 = callee_cdecl!(C_SEL, u32,);
        let sel = if s & 0xFF != 0 { gi(G_SEL0B) } else { gi(G_SEL0A) };
        wf(this_ptr.wrapping_add(0xEC), fmul(sel as f32, a0));

        let s: u32 = callee_cdecl!(C_SEL, u32,);
        let sel = if s & 0xFF != 0 { gi(G_SEL1B) } else { gi(G_SEL1A) };
        wf(this_ptr.wrapping_add(0xF0), fmul(sel as f32, raw));

        let s: u32 = callee_cdecl!(C_SEL, u32,);
        let sel = if s & 0xFF != 0 { gi(G_SEL0B) } else { gi(G_SEL0A) };
        wf(this_ptr.wrapping_add(0xF4), fmul(sel as f32, fadd(a0, gf(G_ADJ_A))));

        let s: u32 = callee_cdecl!(C_SEL, u32,);
        let esi_sel = if s & 0xFF != 0 { gi(G_SEL1B) } else { gi(G_SEL1A) };

        // Accumulator chain over the two float answers.
        let v3: f32 = callee_cdecl!(C_F1, f32,);
        let acc = fadd(v3, fadd(raw, gf(G_ADJ_C)));
        let v4: f32 = callee_cdecl!(C_F2, f32,);
        let w = fadd(fmul(v4, gf(G_MUL16)), acc);
        wf(this_ptr.wrapping_add(0xF8), fmul(w, esi_sel as f32));

        // Fixed 17-step emission loop; the accumulator starts at t1 and
        // each step adds the fresh answer to the running value.
        let mut acc2 = t1;
        let mut last = v4;
        let mut p = this_ptr.wrapping_add(0x160);
        let mut n = 0x11u32;
        while n != 0 {
            wf(p.wrapping_sub(4), a2);
            wf(p, acc2);
            let v: f32 = callee_cdecl!(C_F2, f32,);
            last = v;
            acc2 = fadd(v, acc2);
            p = p.wrapping_add(8);
            n -= 1;
        }
        last.to_bits()
    }
});
