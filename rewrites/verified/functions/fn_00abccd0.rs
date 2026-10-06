// original: 0x00abccd0 input_probe_update
//! Input probe update: filters a probe sample through entry threshold gates,
//! optionally normalises two direction vectors and derives a swept direction
//! from them, folds per-mode clamp state with a plane test against game
//! structs, then runs a flag-driven helper chain (probe predicate, slot
//! lookup, heavy solver, index scan) and finishes by appending a bounded
//! non-negative residual through the neighbour routine and recording the
//! current value in the subsystem index table. Returns the recorded value on
//! the main path; the threshold and helper exits return early.
//!
//! Proof notes: every float operation keeps the original's exact operand
//! order (pinned with black_box) so results match bit for bit. Ordered
//! float comparisons are written as strict greater/less only, matching the
//! original's branch sense including unordered inputs. The three callees in
//! the encrypted code region are intercepted by call site like every other
//! callee, so nothing about them needs the encrypted bytes. The original
//! rewrites its incoming flags word on the stack, which a Rust rewrite
//! cannot observe back, so the stack comparison is off and the flags value
//! is observed through the helper calls it feeds instead.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Game struct pointer feeding the plane test and the distance block.
const STRUCT_PTR: u32 = 0x118D800;
/// Solver selector flags word.
const SOLVER_FLAGS: u32 = 0x118D834;
/// Probe gate byte; zero skips the flag-sign exit.
const GATE_BYTE: u32 = 0x129576C;
/// Probe gate flags word; bit 0x400 arms the flag-sign exit.
const GATE_FLAGS: u32 = 0x129577C;
/// Index into the subsystem table, read from its global slot.
const TABLE_INDEX: u32 = 0x150E244;
/// Current subsystem value; written to the table and returned.
const TABLE_VALUE: u32 = 0x154CBBC;
/// Subsystem index table base.
const TABLE_BASE: u32 = 0x154DFD4;
/// Secondary struct pointer for the slot lookup.
const SLOT_STRUCT: u32 = 0x1601088;
/// Object pointer for the slot lookup call.
const SLOT_OBJECT: u32 = 0x1601098;
/// Slot value table base, indexed by the struct word.
const SLOT_TABLE: u32 = 0x16010AC;
/// Entry threshold constant address.
const C_MILLI: u32 = 0x00FE86B4;
/// Clamp span constant address (ninety).
const C_SPAN: u32 = 0x00FE8BA8;
/// Unity constant address (high copy).
const C_ONE_HI: u32 = 0x0103EFE8;
/// Unity constant address (low copy).
const C_ONE: u32 = 0x00FE88E8;
/// Absolute-value mask address (low word).
const C_ABSMASK: u32 = 0x00FE8F80;
/// Normalisation tolerance address (one ten-thousandth).
const C_EPS: u32 = 0x00FE868C;
/// Row offset of the plane coefficients in the game struct.
const PLANE_ROW: u32 = 0x330;
/// Constant term offset of the plane coefficients.
const PLANE_CONST: u32 = 0x33C;
/// Offset of the reference point in the game struct.
const REF_POINT: u32 = 0x70;
/// Offset of the slot index word in the secondary struct.
const SLOT_INDEX_OFF: u32 = 0x8F8;
/// Flag bits forcing the combined flag shape.
const FLAG_COMBINED_LO: u32 = 0x82;
const FLAG_COMBINED_HI: u32 = 0x300;
const FLAG_COMBINED_SET: u32 = 0x60;
/// Flag bit set when the selector word is present, cleared otherwise.
const FLAG_PRESENT: u32 = 0x20;
/// Flag bit set when the selector word is absent, cleared otherwise.
const FLAG_ABSENT: u32 = 0x40;
/// Flag bits skipping the probe predicate call.
const FLAG_SKIP_PRED: u32 = 0x280;
/// Flag bits arming the slot lookup block.
const FLAG_SLOT_ARM: u32 = 0x140;
/// Solver selector bits choosing the heavy solver path.
const SOLVER_BIT_A: u32 = 0x2000;
const SOLVER_BIT_B: u32 = 0x4000;
/// Sign bit tested by the flag-sign exit.
const FLAG_SIGN_BIT: u32 = 0x80;
/// Gate flag bit arming the flag-sign exit.
const GATE_ARM_BIT: u32 = 0x400;
/// Bits cleared from the flags word on the light solver path.
const LIGHT_CLEAR: u32 = 0x6;
/// Callee ids in the proof contract.
const ID_NORMALISE: u32 = 0;
const ID_PLANE: u32 = 1;
const ID_PREDICATE: u32 = 2;
const ID_SLOT: u32 = 3;
const ID_SOLVER: u32 = 4;
const ID_SCAN: u32 = 5;
const ID_APPEND: u32 = 6;

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

#[inline(always)]
fn fgt(a: f32, b: f32) -> bool {
    black_box(a) > black_box(b)
}

#[inline(always)]
fn flt(a: f32, b: f32) -> bool {
    black_box(a) < black_box(b)
}

#[inline(always)]
fn fabsf(x: f32, mask: u32) -> f32 {
    f32::from_bits(x.to_bits() & mask)
}

#[inline(always)]
fn fneg(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

#[inline(always)]
fn fconst(va: u32) -> f32 {
    unsafe { (relocated(va) as *const f32).read_unaligned() }
}

#[inline(always)]
fn rd_f(base: u32, off: u32) -> f32 {
    unsafe { (base.wrapping_add(off) as *const f32).read_unaligned() }
}

#[inline(always)]
fn rd_u(base: u32, off: u32) -> u32 {
    unsafe { (base.wrapping_add(off) as *const u32).read_unaligned() }
}

#[inline(always)]
fn rd_gu(va: u32) -> u32 {
    unsafe { global::<u32>(va).read() }
}

/// Shared body: `TABLE_ADD` shifts the subsystem table write for the mutant.
#[inline(always)]
unsafe fn body(
    a0w: u32,
    mode: u32,
    a2w: u32,
    avec: u32,
    bvec: u32,
    cptr: u32,
    a6w: u32,
    a7w: u32,
    a8w: u32,
    a9w: u32,
    a10w: u32,
    a11w: u32,
    a12w: u32,
    a13w: u32,
    a14w: u32,
    a15w: u32,
    a16w: u32,
    a17w: u32,
    table_add: u32,
) -> u32 {
    unsafe {
        let milli = fconst(C_MILLI);
        let a7f = f32::from_bits(a7w);
        let a10f = f32::from_bits(a10w);
        let a11f = f32::from_bits(a11w);
        let a12f = f32::from_bits(a12w);
        let a13f = f32::from_bits(a13w);
        // Entry threshold gates. Both exits return the entry register value,
        // which the contract avoids by constraining the thresholds.
        if !flt(milli, a10f) {
            return 0;
        }
        if fgt(milli, a7f) && fgt(milli, a13f) {
            return 0;
        }
        // Direction vectors staged from the two object pointers.
        let mut na0 = rd_f(avec, 0);
        let mut na1 = rd_f(avec, 4);
        let mut na2 = rd_f(avec, 8);
        let mut nb0 = rd_f(bvec, 0);
        let mut nb1 = rd_f(bvec, 4);
        let mut nb2 = rd_f(bvec, 8);
        // Per-mode clamp state.
        let (m1c, m18);
        if mode == 2 {
            let span = fsub(fconst(C_SPAN), fconst(C_ONE_HI));
            let mut c12 = a12f;
            if fgt(0.0, c12) {
                c12 = 0.0;
            } else if fgt(c12, span) {
                c12 = span;
            }
            let c12m1 = fsub(c12, fconst(C_ONE_HI));
            let mut c11 = a11f;
            if fgt(0.0, c11) {
                c11 = 0.0;
            } else if fgt(c11, c12m1) {
                c11 = c12m1;
            }
            m1c = c12;
            m18 = c11;
        } else if mode == 0 {
            m1c = 0.0;
            m18 = 0.0;
        } else {
            m1c = a12f;
            m18 = a11f;
        }
        let absmask = rd_u(relocated(C_ABSMASK), 0);
        let one = fconst(C_ONE);
        let eps = fconst(C_EPS);
        if a0w == 0 && mode != 3 {
            // Normalise the first vector unless already unit length.
            let len_a = fadd(fadd(fmul(na0, na0), fmul(na1, na1)), fmul(na2, na2));
            if fgt(fabsf(fsub(len_a, one), absmask), eps) {
                let mut v = [na0.to_bits(), na1.to_bits(), na2.to_bits()];
                callee_thiscall!(ID_NORMALISE, u32, v.as_mut_ptr() as u32);
                na0 = f32::from_bits(v[0]);
                na1 = f32::from_bits(v[1]);
                na2 = f32::from_bits(v[2]);
            }
            // Sweep a perpendicular direction unless already orthogonal.
            let dot = fadd(fadd(fmul(nb1, na1), fmul(na0, nb0)), fmul(nb2, na2));
            if fgt(fabsf(dot, absmask), milli) {
                let x1 = na2;
                let mut x2 = fmul(nb1, x1);
                let mut x0 = fmul(nb2, na1);
                let mut x4 = fmul(nb2, na0);
                x2 = fsub(x2, x0);
                x0 = fmul(x1, nb0);
                let mut x1b = fmul(na1, nb0);
                let mut x3 = fmul(nb1, na0);
                x4 = fsub(x4, x0);
                x1b = fsub(x1b, x3);
                x3 = fmul(x2, na2);
                x2 = fmul(x2, na1);
                x0 = fmul(x4, na2);
                let mut x5 = fmul(x1b, na1);
                x1b = fmul(x1b, na0);
                x4 = fmul(x4, na0);
                x5 = fsub(x5, x0);
                x3 = fsub(x3, x1b);
                x4 = fsub(x4, x2);
                nb0 = x5;
                nb1 = x3;
                nb2 = x4;
            }
            // Normalise the swept vector unless already unit length.
            let len_b = fadd(fadd(fmul(nb1, nb1), fmul(nb0, nb0)), fmul(nb2, nb2));
            if fgt(fabsf(fsub(len_b, one), absmask), eps) {
                let mut v = [nb0.to_bits(), nb1.to_bits(), nb2.to_bits()];
                callee_thiscall!(ID_NORMALISE, u32, v.as_mut_ptr() as u32);
                nb0 = f32::from_bits(v[0]);
                nb1 = f32::from_bits(v[1]);
                nb2 = f32::from_bits(v[2]);
            }
        }
        // Flag shaping from the low bits and the selector word.
        let mut flags = a2w;
        if a2w as u8 & FLAG_COMBINED_LO as u8 != 0 || a2w & FLAG_COMBINED_HI != 0 {
            flags |= FLAG_COMBINED_SET;
        } else if a15w == 0xFFFF_FFFF {
            flags = (flags & !FLAG_PRESENT) | FLAG_ABSENT;
        } else {
            flags = (flags & !FLAG_ABSENT) | FLAG_PRESENT;
        }
        let sptr = rd_gu(STRUCT_PTR).wrapping_add(0x10);
        let mut maxv = a10f;
        if mode == 3 {
            let m = if fgt(m18, m1c) { m18 } else { m1c };
            maxv = m;
            if fgt(a10f, maxv) {
                maxv = a10f;
            }
        }
        // Plane test of the probe point against the game struct.
        let c0 = rd_f(cptr, 0);
        let c1 = rd_f(cptr, 4);
        let c2 = rd_f(cptr, 8);
        let px = fsub(
            fneg(fadd(
                fadd(
                    fmul(c0, rd_f(sptr, PLANE_ROW)),
                    fmul(c1, rd_f(sptr, PLANE_ROW + 4)),
                ),
                fmul(c2, rd_f(sptr, PLANE_ROW + 8)),
            )),
            rd_f(sptr, PLANE_CONST),
        );
        // Gate-byte and gate-flag exits; the taken exit returns the struct base.
        if global::<u8>(GATE_BYTE).read() != 0
            && rd_gu(GATE_FLAGS) & GATE_ARM_BIT != 0
            && flags & FLAG_SIGN_BIT == 0
        {
            return sptr;
        }
        // Plane-distance exit; also returns the struct base.
        if !flt(px, fneg(maxv)) {
            return sptr;
        }
        let ans1 = callee_thiscall!(
            ID_PLANE,
            u32,
            sptr,
            c0.to_bits(),
            c1.to_bits(),
            c2.to_bits(),
            maxv.to_bits(),
            0,
        );
        if ans1 == 0 {
            return 0;
        }
        let esi2 = flags;
        if esi2 & FLAG_SKIP_PRED == 0 {
            let ans = callee_cdecl!(ID_PREDICATE, u32, a15w, a16w);
            if ans & 0xFF == 0 {
                // The exit returns the stub's full answer word verbatim.
                return ans;
            }
            if a16w == 0xFFFF_FFFF && esi2 & FLAG_SLOT_ARM != 0 {
                let s2 = rd_gu(SLOT_STRUCT);
                let idx = rd_u(s2.wrapping_add(SLOT_INDEX_OFF), 0);
                let tval = rd_u(relocated(SLOT_TABLE), idx.wrapping_mul(4));
                let obj = rd_gu(SLOT_OBJECT);
                let ans3 = callee_thiscall!(ID_SLOT, u32, obj, tval, cptr, maxv.to_bits(), 0, 0);
                if ans3 & 0xFF == 1 {
                    // The exit returns the stub's full answer word verbatim.
                    return ans3;
                }
            }
        }
        let g = rd_gu(SOLVER_FLAGS);
        if g & SOLVER_BIT_A != 0 && g & SOLVER_BIT_B != 0 {
            let mut fp1 = [nb0.to_bits(), nb1.to_bits(), nb2.to_bits()];
            let mut fp2 = [na0.to_bits(), na1.to_bits(), na2.to_bits()];
            let mut obj = [0u32; 32];
            callee_thiscall!(
                ID_SOLVER,
                u32,
                obj.as_mut_ptr() as u32,
                mode,
                esi2,
                fp2.as_mut_ptr() as u32,
                fp1.as_mut_ptr() as u32,
                cptr,
                a6w,
                a7w,
                a8w,
                a9w,
                a10w,
                m18.to_bits(),
                m1c.to_bits(),
                a13w,
                a14w,
                a15w,
                a16w,
                a17w,
            );
            let w_out = obj[0x60 / 4];
            let w_flag = obj[0x44 / 4];
            if w_out != 0 && (w_flag == 0 || w_flag == 2) {
                let mut slot = [0u32; 1];
                let _ = callee_cdecl!(ID_SCAN, u32, slot.as_mut_ptr() as u32);
            }
        } else {
            let esi3 = esi2 & !LIGHT_CLEAR;
            let mut fp1 = [nb0.to_bits(), nb1.to_bits(), nb2.to_bits()];
            let mut fp2 = [na0.to_bits(), na1.to_bits(), na2.to_bits()];
            let mut obj = [0u32; 32];
            callee_thiscall!(
                ID_SOLVER,
                u32,
                obj.as_mut_ptr() as u32,
                mode,
                esi3,
                fp2.as_mut_ptr() as u32,
                fp1.as_mut_ptr() as u32,
                cptr,
                a6w,
                a7w,
                a8w,
                a9w,
                a10w,
                m18.to_bits(),
                m1c.to_bits(),
                a13w,
                a14w,
                a15w,
                a16w,
                0,
            );
        }
        // Tail distance block: bounded non-negative residual past the limit.
        let dc1 = fsub(c1, rd_f(sptr, REF_POINT + 4));
        let dc0 = fsub(c0, rd_f(sptr, REF_POINT));
        let dc2 = fsub(c2, rd_f(sptr, REF_POINT + 8));
        let dist2 = fadd(fadd(fmul(dc1, dc1), fmul(dc0, dc0)), fmul(dc2, dc2));
        let dist = black_box(black_box(dist2).sqrt());
        let dd = fsub(dist, maxv);
        let farg = if fgt(dd, 0.0) { dd } else { 0.0 };
        let mut slot = [0u32; 1];
        callee_cdecl!(ID_APPEND, u32, slot.as_mut_ptr() as u32, farg.to_bits());
        let tidx = rd_gu(TABLE_INDEX);
        let tvalw = rd_gu(TABLE_VALUE);
        (relocated(TABLE_BASE).wrapping_add(tidx.wrapping_add(table_add).wrapping_mul(4))
            as *mut u32)
            .write(tvalw);
        tvalw
    }
}

export!(
    cdecl,
    rw_00abccd0(
        a0w: u32,
        mode: u32,
        a2w: u32,
        avec: u32,
        bvec: u32,
        cptr: u32,
        a6w: u32,
        a7w: u32,
        a8w: u32,
        a9w: u32,
        a10w: u32,
        a11w: u32,
        a12w: u32,
        a13w: u32,
        a14w: u32,
        a15w: u32,
        a16w: u32,
        a17w: u32,
    ) -> u32 {
        unsafe {
            body(
                a0w, mode, a2w, avec, bvec, cptr, a6w, a7w, a8w, a9w, a10w, a11w, a12w, a13w,
                a14w, a15w, a16w, a17w, 0,
            )
        }
    }
);

export!(
    cdecl,
    rw_00abccd0_m1(
        a0w: u32,
        mode: u32,
        a2w: u32,
        avec: u32,
        bvec: u32,
        cptr: u32,
        a6w: u32,
        a7w: u32,
        a8w: u32,
        a9w: u32,
        a10w: u32,
        a11w: u32,
        a12w: u32,
        a13w: u32,
        a14w: u32,
        a15w: u32,
        a16w: u32,
        a17w: u32,
    ) -> u32 {
        unsafe {
            body(
                a0w, mode, a2w, avec, bvec, cptr, a6w, a7w, a8w, a9w, a10w, a11w, a12w, a13w,
                a14w, a15w, a16w, a17w, 1,
            )
        }
    }
);
