// original: 0x00a98c40 attach_and_dispatch_state
//! Attaches a state object to a parameter block, resolves it through a
//! dispatch table, and runs the tail configuration.
//!
//! `this` points to the state being attached, `a0` to the parameter block
//! and `a1` is a slot index. The function links the two, evaluates a
//! floating-point probe through a helper (whose out-structure yields both a
//! status flag and the working object pointer), sets a bit in the block's
//! occupancy table, resolves the working object through two virtual
//! dispatches, then either scans the counted children of the resolved region
//! or takes the direct tag dispatch, runs the flagged-region configuration
//! or the fallback scan depending on the region flag, and finishes with
//! constant stamps plus a masked mode value, which it also returns.
//!
//! Proof: one contract covers every stage (entry/probe/occupancy, direct-tag
//! dispatch, counted-child scan, fallback scan, flagged region with indexed
//! scan and single-entry fallback, tail). Every branch direction and every
//! callee fired on the passing run; six fault-parity buckets cover the wild
//! inputs. The only narrowing is the skipped out-pointer argument of the
//! probe helper (its target is compared through the heap diff instead).

#![allow(unsafe_code)]
#![allow(clippy::pedantic)]

use lf_checker_rt::export;

const SVC_ADDR: u32 = 0x01305D30;
const REGION_FLAG: u32 = 0x04000000;
const MODE_MASK: u32 = 0x3C0;
const STAMP_A: u32 = 0x3D99999A;
const STAMP_B: u32 = 0x3CA3D70A;
const SHARED_OBJ: u32 = 0x018B8968;
const DIRECT_TAG: u8 = 0x0C;
const CHILD_MATCH: u16 = 0x1E;
const CHILD_MATCH_ALT: u16 = 0x1F;
const CHILD_EMPTY: u32 = 0xFFFF_FFFF;

/// `child_init` is the scan slot's initial value (the all-ones sentinel in
/// the proven export; the honest mutant this proof ran against clears it).
export!(thiscall, rw_a98c40f(this_: u32, a0: u32, a1: u32) -> u32 {
    unsafe { run_full(this_, a0, a1, CHILD_EMPTY) }
});

unsafe fn run_full(this_: u32, a0: u32, a1: u32, child_init: u32) -> u32 {
    unsafe {
        ((this_.wrapping_add(0x68)) as *mut u32).write_unaligned(a0);
        lf_checker_rt::callee_thiscall!(1, u32, a0, this_.wrapping_add(0x68));
        let back = (this_.wrapping_add(0x68) as *const u32).read_unaligned();
        let sx = (back.wrapping_add(0x2e) as *const i16).read_unaligned() as i32;
        (this_.wrapping_add(0x64) as *mut u32).write_unaligned(sx as u32);
        let vp = (a0.wrapping_add(0x20) as *const u32).read_unaligned();
        let (vx, vy, vz) = if vp != 0 {
            let b = vp.wrapping_add(0x30);
            ((b as *const f32).read_unaligned(),
             ((b.wrapping_add(4)) as *const f32).read_unaligned(),
             ((b.wrapping_add(8)) as *const f32).read_unaligned())
        } else {
            (((a0.wrapping_add(0x10)) as *const f32).read_unaligned(),
             ((a0.wrapping_add(0x14)) as *const f32).read_unaligned(),
             ((a0.wrapping_add(0x18)) as *const f32).read_unaligned())
        };
        let mut out = [0u32; 6];
        let fr = lf_checker_rt::callee_cdecl!(2, f32, vx.to_bits(), vy.to_bits(), vz.to_bits(),
            out.as_mut_ptr() as u32, 0, 4);
        (this_.wrapping_add(0x6c) as *mut f32).write_unaligned(fr);
        let edi = (out[3] >> 8) | (out[4] << 24);
        if out[0] & 0xFF == 0 {
            let f = f32::from_bits((out[4] >> 8) | (out[5] << 24)) - 10.0;
            (this_.wrapping_add(0x6c) as *mut f32).write_unaligned(f);
        }
        let wi = (a1 as i32).wrapping_shr(5);
        let bit = 1u32 << (a1 & 0x1F);
        let ba = (a0 as i32).wrapping_add(wi.wrapping_mul(4)).wrapping_add(0x1ac) as u32;
        let old = (ba as *const u32).read_unaligned();
        (ba as *mut u32).write_unaligned(old | bit);
        (edi.wrapping_add(0x70) as *mut u32).write_unaligned(0);
        (edi.wrapping_add(8) as *mut u32).write_unaligned(a1);
        lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(SVC_ADDR), a0);
        let v1 = (edi.wrapping_add(0x68) as *const u32).read_unaligned();
        let vt = (v1 as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vt.wrapping_add(0xa0)) as *const u32).read_unaligned()));
        let r1 = f1(v1);
        let e2 = if r1 != 0 {
            r1
        } else {
            (v1.wrapping_add(0x38) as *const u32).read_unaligned()
        };
        let e3 = (e2.wrapping_add(4) as *const u32).read_unaligned();
        let cy = (e3.wrapping_add(0xc) as *const u32).read_unaligned();
        if ((cy.wrapping_add(4)) as *const u8).read_unaligned() == DIRECT_TAG {
            let t1 = (cy.wrapping_add(0x80) as *const u32).read_unaligned();
            let row = (t1.wrapping_add(a1.wrapping_mul(4)) as *const u32).read_unaligned();
            let u = (row as *const u32).read_unaligned();
            let f2: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(((u.wrapping_add(0x5c)) as *const u32).read_unaligned());
            let r2 = f2(row, 0);
            (edi.wrapping_add(0x60) as *mut u32).write_unaligned(r2 & 0xFF);
        } else {
            counted_child_scan(edi, cy, child_init);
        }
        (edi.wrapping_add(0x87) as *mut u8).write_unaligned(0);
        if ((a0.wrapping_add(0x24)) as *const u32).read_unaligned() & REGION_FLAG != 0 {
            flagged_region(edi, a0);
        } else {
            fallback_scan(edi, a0);
        }
        (edi.wrapping_add(0x78) as *mut u32).write_unaligned(0);
        (edi.wrapping_add(0x84) as *mut u16).write_unaligned(0);
        (edi.wrapping_add(0x86) as *mut u8).write_unaligned(0);
        (edi.wrapping_add(0x8c) as *mut u32).write_unaligned(STAMP_A);
        (edi.wrapping_add(0x88) as *mut u32).write_unaligned(STAMP_B);
        let d8 = lf_checker_rt::callee_thiscall!(6, u32, a0);
        if (d8 as u8) == 0 {
            (edi.wrapping_add(0x85) as *mut u8).write_unaligned(1);
        } else if ((edi.wrapping_add(0x87)) as *const u8).read_unaligned() != 0 {
            (edi.wrapping_add(0x85) as *mut u8).write_unaligned(1);
        }
        let m = ((a0.wrapping_add(0x28)) as *const u32).read_unaligned() & MODE_MASK;
        (edi.wrapping_add(0x90) as *mut u32).write_unaligned(0);
        if m == 0x80 {
            (edi.wrapping_add(0x85) as *mut u8).write_unaligned(1);
        }
        m
    }
}

/// Scans the counted children of the resolved region, keeping the first
/// child the shared matcher accepts. Each child is offered to the shared
/// matcher twice (the retry reuses the same matcher: the slot that held the
/// region handle is overwritten with the shared handle before the retry
/// call), accepted on either tag, and the bound is re-read after every child.
#[inline(never)]
unsafe fn counted_child_scan(edi: u32, cy: u32, child_init: u32) {
    unsafe {
        (edi.wrapping_add(0x60) as *mut u32).write_unaligned(child_init);
        let cyv = (cy as *const u32).read_unaligned();
        let fb: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((cyv.wrapping_add(0x54)) as *const u32).read_unaligned());
        if (fb(cy) as i32) <= 0 {
            return;
        }
        let shared = lf_checker_rt::global::<u32>(SHARED_OBJ).read_unaligned();
        let mut i: i32 = 0;
        loop {
            let v = (cy as *const u32).read_unaligned();
            let fr: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(((v.wrapping_add(0x5c)) as *const u32).read_unaligned());
            let row = fr(cy, i as u32);
            let vg = (shared as *const u32).read_unaligned();
            let fq: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(((vg.wrapping_add(0x14)) as *const u32).read_unaligned());
            let q = fq(shared, row);
            let mut accept = ((q.wrapping_add(0x20)) as *const u16).read_unaligned() == CHILD_MATCH;
            if !accept {
                let q2 = fq(shared, row);
                accept = ((q2.wrapping_add(0x20)) as *const u16).read_unaligned() == CHILD_MATCH_ALT;
            }
            if accept && (edi.wrapping_add(0x60) as *const u32).read_unaligned() == CHILD_EMPTY {
                (edi.wrapping_add(0x60) as *mut u32).write_unaligned(row);
            }
            let v3 = (cy as *const u32).read_unaligned();
            let fb2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(((v3.wrapping_add(0x54)) as *const u32).read_unaligned());
            let bound = fb2(cy) as i32;
            i = i.wrapping_add(1);
            if !(i < bound) {
                break;
            }
        }
    }
}

/// Flagged-region configuration: resolves the slot entry through the
/// block's own dispatch, gates on the helper result and a positive level,
/// then either walks the indexed scan or falls back to the single entry.
#[inline(never)]
unsafe fn flagged_region(edi: u32, a0: u32) {
    unsafe {
        (edi.wrapping_add(0x87) as *mut u8).write_unaligned(1);
        let va0 = (a0 as *const u32).read_unaligned();
        let fc: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((va0.wrapping_add(0xa0)) as *const u32).read_unaligned());
        let c = fc(a0);
        if c == 0 {
            return flagged_fallback(edi, a0, 0);
        }
        let r = lf_checker_rt::callee_thiscall!(14, u32, c);
        if r == 0 {
            return flagged_fallback(edi, a0, 0);
        }
        let edx = (r.wrapping_add(0xb4) as *const u32).read_unaligned();
        let level = (edx.wrapping_add(0x70) as *const f32).read_unaligned();
        if !(level > 0.0) {
            return flagged_fallback(edi, a0, r);
        }
        let t = (r.wrapping_add(0xd4) as *const u32).read_unaligned();
        let idx = (edi.wrapping_add(8) as *const u32).read_unaligned();
        let e = (t.wrapping_add(idx.wrapping_mul(4)) as *const u32).read_unaligned();
        let sv = ((e.wrapping_add(0xe)) as *const i16).read_unaligned() as i32;
        if edx == 0 {
            return;
        }
        let p1 = (edx.wrapping_add(0x40) as *const u32).read_unaligned();
        let p2 = (p1 as *const u32).read_unaligned();
        let p = (p2 as *const u32).read_unaligned();
        let count = ((p.wrapping_add(0x1a)) as *const u16).read_unaligned() as i32;
        let mut i: i32 = 0;
        while i < count {
            let s = ((p.wrapping_add(0x10)) as *const u32).read_unaligned();
            let w = (((s.wrapping_add((i as u32).wrapping_mul(2))) as *const u16)
                .read_unaligned()) as u32;
            let g1 = ((edx.wrapping_add(8)) as *const u32).read_unaligned();
            let g2 = ((g1.wrapping_add(8)) as *const u32).read_unaligned();
            let warg = ((g2.wrapping_add(w.wrapping_mul(4))) as *const u32).read_unaligned();
            let g = lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(SVC_ADDR), warg);
            if (g as u8) != 0 {
                lf_checker_rt::callee_thiscall!(5, u32, edi, a0, edx, p, i as u32, sv as u32);
                lf_checker_rt::callee_cdecl!(15, u32, edi.wrapping_add(0x20), a0, sv as u32, edi);
            }
            i = i.wrapping_add(1);
        }
    }
}

/// Single-entry fallback of the flagged region, also taken when the helper
/// reports nothing or the level is not positive.
#[inline(never)]
unsafe fn flagged_fallback(edi: u32, a0: u32, r: u32) {
    unsafe {
        let t = (r.wrapping_add(0xd4) as *const u32).read_unaligned();
        let idx = (edi.wrapping_add(8) as *const u32).read_unaligned();
        let e = (t.wrapping_add(idx.wrapping_mul(4)) as *const u32).read_unaligned();
        let cc = ((e.wrapping_add(0x90)) as *const u32).read_unaligned();
        if cc == 0 {
            return;
        }
        let p = ((cc.wrapping_add(0x40)) as *const u32).read_unaligned();
        if p == 0 {
            return;
        }
        let p1 = (p as *const u32).read_unaligned();
        let s = (p1 as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(5, u32, edi, a0, cc, s, 0, 0xFFFF_FFFFu32);
        let b = ((s.wrapping_add(0x17)) as *const u8).read_unaligned() as u32;
        lf_checker_rt::callee_cdecl!(15, u32, edi.wrapping_add(0x20), a0, b, edi);
    }
}

/// Fallback scan over the block's secondary list.
#[inline(never)]
unsafe fn fallback_scan(edi: u32, a0: u32) {
    unsafe {
        let r2obj = (a0.wrapping_add(0x34) as *const u32).read_unaligned();
        if r2obj != 0 {
            let r2b = (r2obj as *const u32).read_unaligned();
            if r2b != 0 {
                let r2c = ((r2b.wrapping_add(0x40)) as *const u32).read_unaligned();
                let r2d = (r2c as *const u32).read_unaligned();
                let r2e = (r2d as *const u32).read_unaligned();
                let count = ((r2e.wrapping_add(0x1a)) as *const u16).read_unaligned() as i32;
                let mut i: i32 = 0;
                while i < count {
                    let r2f = ((r2e.wrapping_add(0x10)) as *const u32).read_unaligned();
                    let sx2 = (((r2f.wrapping_add((i as u32).wrapping_mul(2))) as *const u16)
                        .read_unaligned()) as u32;
                    let r2g = ((r2b.wrapping_add(8)) as *const u32).read_unaligned();
                    let r2h = ((r2g.wrapping_add(8)) as *const u32).read_unaligned();
                    let w = ((r2h.wrapping_add(sx2.wrapping_mul(4))) as *const u32).read_unaligned();
                    let g = lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(SVC_ADDR), w);
                    if (g as u8) != 0 {
                        lf_checker_rt::callee_thiscall!(5, u32, edi, a0, r2b, r2e, i as u32, 0xFFFFFFFFu32);
                    }
                    i += 1;
                }
            }
        }
    }
}
