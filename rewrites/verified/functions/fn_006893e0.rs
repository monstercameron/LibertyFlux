// original: 0x006893e0 weighted_vec4_blend
//! Blend up to four float 4-vectors under scripted weights, normalize, report.
//!
//! Calling convention is thiscall-like: the object arrives in ECX, one live
//! stack word (an argument object) plus three dead words are popped by the
//! single the callee pops 0x10 bytes, the incoming XMM2 low float is a clamped dividend, and
//! a match flag returns in AL.
//!
//! Behaviour. The entry compares a tag word from the `this` sub-object with a
//! word of the argument object. On a match the function walks table 1 (loop 1);
//! on a mismatch it calls a lookup helper whose out-object selects loop 2 (an
//! indexed walk) or loop 3 (a key-merge walk). Before the loops it clamps the
//! XMM2 entry to `[0, N]`, divides by a fabricated word M, takes the IEEE
//! remainder of the widened dividend modulo the widened M via an inlined
//! bit-exact software fmod (the original calls a helper taking both doubles
//! on the x87 stack, which no checker transport observes; the helper runs
//! natively on the original side and argument errors surface transitively
//! through every remainder-dependent output), rounds, truncates with an
//! emulated round-toward-zero conversion plus a `{0.0, 2^32}` correction, and
//! keeps the fraction in `[0, 1]` as the per-call float argument. Each loop
//! iteration dispatches on tag bytes to one of three arms: a fetch group
//! (slot `0x30` setters then slot `0x34` getters returning f32 in ST0),
//! a three-call slot `0x24` group (or single slot `0x20` call), or a short
//! group ending in a slot `0x1c` / slot `0x24` / slot `0x28` call. Fetch arms
//! accumulate `x^2+y^2+z^2+w^2`, skip the square root when the sum compares
//! equal to `+0.0` (the inverted-test idiom: unordered/NaN still calls it),
//! and normalize by the reciprocal root. Every vtable site is thiscall/3 with
//! `this` = ECX: stack words are `(esi, float-bits, pointer)` where the last
//! word is a frame out-slot for the fetch groups (skipped, snapshotted zeros)
//! or a heap vector word for the dispatch groups (compared). The tail compares
//! a join counter with the entry bound for AL, releases one reference, and
//! fires two data-table calls when the lookup out-object is present.
//!
//! Omitted with no observable effect: `prefetcht0` hints and their dedicated
//! prefetch-only reads (a hint never faults and writes nothing); the dead
//! `cvttss2si` (its EAX is overwritten before any read and the worker does not
//! compare MXCSR sticky flags); stores overwritten before any read on every
//! path. The loop-lookahead reads are kept: they can fault and so are
//! observable.
//!
//! Two details the rewrite must get right. First, the pushed ESI at every loop
//! call site is the truncated-long low word: the entry overwrites ESI with the
//! `fistp` result before any loop runs (the lookup call, made earlier, is the
//! only site that passes the object). Second, the square-root helper is cdecl
//! and the caller cleans up only after the normalize loads, so those loads
//! read one word below their lexical slots; what looks like reads of
//! unwritten frame slots is really reads of the just-stored fetch results,
//! and every normalize is an elementwise scale of those results.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, xmm_word};

// ---------------------------------------------------------------------------
// Bit-exact software fmod (finite dividends except -0.0, small-int divisors).
// Proven against the native x87 helper: 818/818 plus 818/818 sign-negated.
// ---------------------------------------------------------------------------

const MAN_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
const SIGN_MASK: u64 = 0x8000_0000_0000_0000;
const EXP_MAX: i32 = 0x7FF;
const BIAS: i32 = 1023;

/// Exact `fmod(x, y)` on raw bits for finite `x` and finite nonzero `y`.
/// Integer bit manipulation only: no rounding-mode or operand-order dependence.
/// Non-finite inputs and a zero divisor return a quiet NaN (guarded totality;
/// the proof contract keeps them out: positive finite entry, nonzero word).
pub fn soft_fmod(xb: u64, yb: u64) -> u64 {
    let sx = xb & SIGN_MASK;
    let mut ex = ((xb >> 52) & 0x7FF) as i32;
    let mut mx = xb & MAN_MASK;
    let mut ey = ((yb >> 52) & 0x7FF) as i32;
    let mut my = yb & MAN_MASK;
    if ex == EXP_MAX || ey == EXP_MAX {
        return sx | 0x7FF8_0000_0000_0000;
    }
    if ex == 0 {
        if mx == 0 {
            return xb;
        }
        let s = mx.leading_zeros() - 11;
        mx <<= s;
        ex = -1022 - s as i32;
    } else {
        mx |= 1 << 52;
        ex -= BIAS;
    }
    if ey == 0 {
        if my == 0 {
            return sx | 0x7FF8_0000_0000_0000;
        }
        let s = my.leading_zeros() - 11;
        my <<= s;
        ey = -1022 - s as i32;
    } else {
        my |= 1 << 52;
        ey -= BIAS;
    }
    if ex < ey {
        return xb;
    }
    let uy = my as u128;
    let mut t = (mx as u128) % uy;
    let mut i: i32 = 0;
    let n = ex - ey;
    while i < n {
        t = (t << 1) % uy;
        if t == 0 {
            return sx;
        }
        i += 1;
    }
    if t == 0 {
        return sx;
    }
    let p = 127 - t.leading_zeros() as i32;
    let m53 = (t << (52 - p)) as u64;
    let e_biased = ey - 52 + p + BIAS;
    if e_biased >= 1 {
        sx | ((e_biased as u64) << 52) | (m53 & MAN_MASK)
    } else {
        let shift = 1 - e_biased;
        sx | (m53 >> shift)
    }
}

// ---------------------------------------------------------------------------
// Raw memory and callee helpers.
// ---------------------------------------------------------------------------

#[inline(always)]
unsafe fn rd8(p: u32) -> u8 {
    unsafe { core::ptr::read_unaligned(p as *const u8) }
}
#[inline(always)]
unsafe fn rd16(p: u32) -> u16 {
    unsafe { core::ptr::read_unaligned(p as *const u16) }
}
#[inline(always)]
unsafe fn rd32(p: u32) -> u32 {
    unsafe { core::ptr::read_unaligned(p as *const u32) }
}
#[inline(always)]
unsafe fn wr32(p: u32, v: u32) {
    unsafe { core::ptr::write_unaligned(p as *mut u32, v) }
}
#[inline(always)]
unsafe fn wr8(p: u32, v: u8) {
    unsafe { core::ptr::write_unaligned(p as *mut u8, v) }
}
#[inline(always)]
unsafe fn rdf(p: u32) -> f32 {
    f32::from_bits(unsafe { rd32(p) })
}
#[inline(always)]
unsafe fn wrf(p: u32, v: f32) {
    unsafe { wr32(p, v.to_bits()) }
}

/// One vtable call returning nothing observed: thiscall/3, float in word 1.
#[inline(always)]
unsafe fn slot_u32(this: u32, slot: u32, a0: u32, a1f: f32, a2: u32) -> u32 {
    let vtab = unsafe { rd32(this) };
    let addr = unsafe { rd32(vtab + slot) };
    let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(addr as usize) };
    f(this, a0, a1f.to_bits(), a2)
}

/// One vtable call returning f32 in ST0: thiscall/3, float in word 1.
#[inline(always)]
unsafe fn slot_f32(this: u32, slot: u32, a0: u32, a1f: f32, a2: u32) -> f32 {
    let vtab = unsafe { rd32(this) };
    let addr = unsafe { rd32(vtab + slot) };
    let f: extern "thiscall" fn(u32, u32, u32, u32) -> f32 =
        unsafe { core::mem::transmute(addr as usize) };
    f(this, a0, a1f.to_bits(), a2)
}

/// Replicate `cvttss2si` bit for bit, including the indefinite value.
#[inline(always)]
fn cvttss2si_bits(x: f32) -> u32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0x80000000
    } else {
        (x as i32) as u32
    }
}

/// Replicate `fistp qword` under a chop control word.
#[inline(always)]
fn fistp_qword_bits(x: f32) -> u64 {
    if x.is_nan() || x >= 9223372036854775808.0 || x < -9223372036854775808.0 {
        0x8000000000000000
    } else {
        (x as i64) as u64
    }
}

const TWO32: f64 = 4294967296.0;
const MAGIC_ROUND: f32 = 8388608.0; // 2^23, the add/sub rounding constant
const ONE: f32 = 1.0;

unsafe fn sqrt_stub(sum: f32) -> f32 {
    callee_cdecl!(2, f32, sum.to_bits())
}

/// The inverted zero test around each square-root call: the root is skipped
/// only when the sum compares equal to +0.0; NaN still calls it.
unsafe fn recip_sqrt(sum: f32) -> f32 {
    if sum == 0.0 {
        0.0
    } else {
        let r = unsafe { sqrt_stub(sum) };
        black_box(ONE) / r
    }
}

export!(thiscall, rw_006893e0(this: u32, arg: u32, _s1: u32, _s2: u32, _s3: u32) -> u8 {
    unsafe { body(this, arg) }
});

unsafe fn body(this: u32, arg: u32) -> u8 {
    unsafe {
        let esi = rd32(this);
        let edx_tag = rd32(esi + 8);
        let mut entry_xmm2 = f32::from_bits(xmm_word(2, 0));
        let tag_match: u8 = if edx_tag != 0 && edx_tag == rd32(arg + 0x10) {
            1
        } else {
            0
        };
        // Optional lookup: two out-words, then the table is [word1 + 0xc].
        let mut out = [0u32; 2];
        let mut table: u32 = 0;
        if tag_match == 0
            && rd32(esi + 4) != 0
            && edx_tag != 0
            && rd32(arg + 0x10) != 0
        {
            let outptr = (&mut out as *mut u32) as u32;
            let this4 = rd32(esi + 4);
            callee_thiscall!(1, u32, this4, outptr, esi, arg, this4);
            entry_xmm2 = f32::from_bits(xmm_word(2, 0));
            let edi = out[1];
            table = if edi != 0 { rd32(edi + 0xc) } else { 0 };
        }
        let table_nonnull = table != 0;
        // Clamp the entry to [0, N].
        let n = rd16(arg + 8) as u32;
        let nf = (n.wrapping_sub(1) as i32) as f32;
        if entry_xmm2 < 0.0 {
            // Negative: zeroed, and the upper clamp is skipped.
            entry_xmm2 = 0.0;
        } else if entry_xmm2 > nf {
            entry_xmm2 = nf;
        }
        // Divide by M; divisor index is min(truncate, count - 1).
        let m = rd16(arg + 0xa) as u32;
        let mf = (m as i32) as f32;
        let quot = black_box(entry_xmm2) / mf;
        let si = rd16(arg + 0x18).wrapping_sub(1);
        let cx = cvttss2si_bits(quot) as u16;
        let dx = if cx > si { si } else { cx } as u32;
        let dividend = entry_xmm2 as f64;
        let divisor = (m as i32) as f64;
        let etab = rd32(arg + 0x14);
        let row_f = rd32(etab + dx.wrapping_mul(4));
        let (xb, yb) = (dividend.to_bits(), divisor.to_bits());
        let rem = f64::from_bits(soft_fmod(xb, yb));
        let mut remf = rem as f32;
        // Clamp the remainder to [0, M + 1].
        let bf = (m.wrapping_add(1) as i32) as f32;
        if remf < 0.0 {
            remf = 0.0;
        }
        if remf > bf {
            remf = bf;
        }
        let x3 = remf;
        // Round to nearest via the 2^23 add/sub idiom with sign handling.
        let sign_only = f32::from_bits(x3.to_bits() & 0x80000000);
        let absx3 = x3.abs();
        let mask: u32 = if absx3 < MAGIC_ROUND {
            0xFFFFFFFF
        } else {
            0
        };
        let magic = f32::from_bits(MAGIC_ROUND.to_bits() & mask | sign_only.to_bits());
        let mut x5 = black_box(x3) + magic;
        x5 = black_box(x5) - magic;
        let w = rd16(row_f + 0xc) as u32;
        let bound = (w.wrapping_sub(2) as i32) as f32;
        let diff = black_box(x5) - x3;
        // cmpnless is "not less-or-equal": !(diff <= sign).
        let corr = if diff <= sign_only { 0.0 } else { ONE };
        x5 = black_box(x5) - corr;
        if x5 < 0.0 {
            x5 = 0.0;
        }
        if x5 > bound {
            x5 = bound;
        }
        // Truncate toward zero (emulated chop conversion) and keep the fraction.
        let chopped = fistp_qword_bits(x5);
        let esi_l = chopped as u32;
        let mut xd = (esi_l as i32) as f64;
        xd = black_box(xd) + if esi_l >> 31 == 0 { 0.0 } else { TWO32 };
        let mut frac = black_box(x3) - (xd as f32);
        if frac < 0.0 {
            frac = 0.0;
        }
        if frac > 1.0 {
            frac = 1.0;
        }
        let edx_t = rd32(this);
        let bound1 = rd16(edx_t + 0x10) as u32;
        let f4 = rd16(row_f + 4) as u32;
        // (Prefetch sweep over [row_f] omitted: a hint writes nothing.)
        if tag_match == 0 {
            path_b(
                esi_l, edx_t, row_f, table, table_nonnull, frac,
                bound1, f4, out,
            )
        } else {
            path_a(esi_l, edx_t, row_f, frac, bound1, out)
        }
    }
}

/// Pinned sum of four squares: ((a^2 + b^2) + c^2) + d^2.
#[inline(always)]
fn sum4(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let t0 = black_box(a) * black_box(a);
    let t1 = black_box(b) * black_box(b);
    let s0 = black_box(t0) + black_box(t1);
    let t2 = black_box(c) * black_box(c);
    let s1 = black_box(s0) + black_box(t2);
    let t3 = black_box(d) * black_box(d);
    black_box(s1) + black_box(t3)
}

/// Pinned sum of three squares: (a^2 + b^2) + c^2.
#[inline(always)]
fn sum3(a: f32, b: f32, c: f32) -> f32 {
    let t0 = black_box(a) * black_box(a);
    let t1 = black_box(b) * black_box(b);
    let s0 = black_box(t0) + black_box(t1);
    let t2 = black_box(c) * black_box(c);
    black_box(s0) + black_box(t2)
}

/// Tail shared by all paths: flag, release, two data-table calls.
unsafe fn tail(ebx: u32, bound1: u32, out: [u32; 2]) -> u8 {
    unsafe {
        let bl = (ebx == bound1) as u8;
        let edi = out[1];
        if edi != 0 {
            let esi_o = out[0];
            let tag = rd32(esi_o + 0x10);
            if tag != 0 {
                let slot = global::<u32>(0xe73188) as u32;
                let addr = rd32(slot);
                let f: extern "stdcall" fn(u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(tag, 0xFFFFFFFF);
            }
            wr32(edi + 8, rd32(edi + 8).wrapping_sub(1));
            if rd32(esi_o + 0x10) != 0 {
                let slot = global::<u32>(0xe731b0) as u32;
                let addr = rd32(slot);
                let f: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(rd32(esi_o + 0x10));
            }
        }
        bl
    }
}

// ---------------------------------------------------------------------------
// Loop 1 (table walk on the tag-match path).
// ---------------------------------------------------------------------------

/// Loop-1 fetch arm: four setters, four getters, normalize into the vector.
unsafe fn arm1_loop1(
    row: u32, vec: u32, frac: f32, esi_l: u32, zero: u32,
) {
    unsafe {
        let j4 = rd32(row + 4);
        let j8 = rd32(row + 8);
        let jc = rd32(row + 0xc);
        let j10 = rd32(row + 0x10);
        slot_u32(j4, 0x30, esi_l, frac, zero);
        slot_u32(j8, 0x30, esi_l, frac, zero);
        slot_u32(jc, 0x30, esi_l, frac, zero);
        slot_u32(j10, 0x30, esi_l, frac, zero);
        let r0 = slot_f32(j4, 0x34, esi_l, frac, zero);
        wrf(vec + 0x10, r0);
        let r1 = slot_f32(j8, 0x34, esi_l, frac, zero);
        wrf(vec + 0x14, r1);
        let r2 = slot_f32(jc, 0x34, esi_l, frac, zero);
        wrf(vec + 0x18, r2);
        let r3 = slot_f32(j10, 0x34, esi_l, frac, zero);
        let sum = sum4(r0, r1, r2, r3);
        let inv = recip_sqrt(sum);
        wrf(vec + 0x10, black_box(r0) * inv);
        wrf(vec + 0x14, black_box(r1) * inv);
        wrf(vec + 0x18, black_box(r2) * inv);
        wrf(vec + 0x1c, black_box(r3) * inv);
    }
}

/// Loop-1 dispatch arm: slot 0x20 for a zero sub-tag, else three 0x24 calls.
unsafe fn arm2_loop1(
    row: u32, vec: u32, j4: u32, tag2: u8, frac: f32, esi_l: u32,
) {
    unsafe {
        if tag2 == 0 {
            slot_u32(j4, 0x20, esi_l, frac, vec + 0x10);
        } else {
            slot_u32(j4, 0x24, esi_l, frac, vec + 0x10);
            let j8 = rd32(row + 8);
            slot_u32(j8, 0x24, esi_l, frac, vec + 0x14);
            let jc = rd32(row + 0xc);
            slot_u32(jc, 0x24, esi_l, frac, vec + 0x18);
            let x3 = rdf(vec + 0x10);
            let x4 = rdf(vec + 0x14);
            let x5 = rdf(vec + 0x18);
            let s = sum3(x3, x4, x5);
            if !(s > ONE) {
                wrf(vec + 0x1c, (black_box(ONE) - s).sqrt());
            } else {
                let r = s.sqrt();
                wr32(vec + 0x1c, 0);
                let k = black_box(ONE) / r;
                wrf(vec + 0x10, black_box(x3) * k);
                wrf(vec + 0x14, black_box(x4) * k);
                wrf(vec + 0x18, black_box(x5) * k);
            }
        }
    }
}

/// Loop-1 third arm: short group, slot 0x1c, or slot 0x24/0x28 by tag.
unsafe fn arm3_loop1(
    row: u32, vec: u32, j4: u32, tag1: u8, frac: f32, esi_l: u32, zero: u32,
) {
    unsafe {
        if tag1 == 0 {
            let tag2 = rd8(row + 1) & 0xf0;
            if tag2 == 0x10 {
                let j8 = rd32(row + 8);
                let jc = rd32(row + 0xc);
                slot_u32(j4, 0x30, esi_l, frac, zero);
                slot_u32(j8, 0x30, esi_l, frac, zero);
                slot_u32(jc, 0x30, esi_l, frac, zero);
                wrf(vec + 0x10, slot_f32(j4, 0x34, esi_l, frac, zero));
                wrf(vec + 0x14, slot_f32(j8, 0x34, esi_l, frac, zero));
                wrf(vec + 0x18, slot_f32(jc, 0x34, esi_l, frac, zero));
            } else {
                slot_u32(j4, 0x1c, esi_l, frac, zero);
            }
        } else if tag1 == 2 {
            slot_u32(j4, 0x24, esi_l, frac, vec + 0x10);
        } else {
            slot_u32(j4, 0x28, esi_l, frac, vec + 0x10);
        }
    }
}

unsafe fn path_a(
    esi_l: u32, edx_t: u32, row_f: u32, frac: f32, bound1: u32, out: [u32; 2],
) -> u8 {
    unsafe {
        // Both early exits test the same bound; the second is unreachable.
        if (bound1 as i32) <= 0 {
            return tail(0, bound1, out);
        }
        let zero_word = 0u32;
        let zero = (&zero_word as *const u32) as u32;
        let mut tcur = rd32(row_f);
        let mut ocur = rd32(edx_t + 0xc);
        let mut cnt = bound1;
        loop {
            let row = rd32(tcur);
            let vec = rd32(ocur);
            let j4 = rd32(row + 4);
            tcur += 4;
            black_box(rd32(tcur)); // lookahead read (fault fidelity)
            let tag1 = rd8(vec + 4) & 0xf;
            if tag1 != 1 {
                arm3_loop1(row, vec, j4, tag1, frac, esi_l, zero);
            } else {
                let tag2 = rd8(row + 1) & 0xf0;
                if tag2 != 0x10 {
                    arm2_loop1(row, vec, j4, tag2, frac, esi_l);
                } else {
                    arm1_loop1(row, vec, frac, esi_l, zero);
                }
            }
            ocur += 4;
            wr8(vec + 4, rd8(vec + 4) & 0xef);
            cnt -= 1;
            if cnt == 0 {
                break;
            }
        }
        tail(bound1, bound1, out)
    }
}

// ---------------------------------------------------------------------------
// Loop 2 (indexed walk when the lookup hit).
// ---------------------------------------------------------------------------

/// Loop-2 fetch arm: same calls as loop 1; the last lane multiplies
/// the reciprocal by the fetch result instead of the other way round.
unsafe fn arm1_loop2(
    row: u32, vec: u32, frac: f32, esi_l: u32, zero: u32,
) {
    unsafe {
        let j4 = rd32(row + 4);
        let j8 = rd32(row + 8);
        let jc = rd32(row + 0xc);
        let j10 = rd32(row + 0x10);
        slot_u32(j4, 0x30, esi_l, frac, zero);
        slot_u32(j8, 0x30, esi_l, frac, zero);
        slot_u32(jc, 0x30, esi_l, frac, zero);
        slot_u32(j10, 0x30, esi_l, frac, zero);
        let r0 = slot_f32(j4, 0x34, esi_l, frac, zero);
        wrf(vec + 0x10, r0);
        let r1 = slot_f32(j8, 0x34, esi_l, frac, zero);
        wrf(vec + 0x14, r1);
        let r2 = slot_f32(jc, 0x34, esi_l, frac, zero);
        wrf(vec + 0x18, r2);
        let r3 = slot_f32(j10, 0x34, esi_l, frac, zero);
        let sum = sum4(r0, r1, r2, r3);
        let inv = recip_sqrt(sum);
        wrf(vec + 0x10, black_box(r0) * inv);
        wrf(vec + 0x14, black_box(r1) * inv);
        wrf(vec + 0x18, black_box(r2) * inv);
        wrf(vec + 0x1c, black_box(inv) * r3);
    }
}

unsafe fn path_b1(
    esi_l: u32,
    edx_t: u32,
    row_f: u32,
    table: u32,
    frac: f32,
    bound1: u32,
    out: [u32; 2],
) -> u8 {
    unsafe {
        let m = rd32(edx_t + 0xc);
        let g = rd32(row_f);
        let mut lcur = table + 4;
        let cnt0 = rd32(table);
        if (cnt0 as i32) <= 0 {
            return tail(cnt0, bound1, out);
        }
        let zero_word = 0u32;
        let zero = (&zero_word as *const u32) as u32;
        let mut cnt = cnt0;
        loop {
            let packed = rd32(lcur);
            let hi = packed >> 16;
            lcur += 4;
            let vec = rd32(m + hi.wrapping_mul(4));
            let lo = packed & 0xffff;
            let row = rd32(g + lo.wrapping_mul(4));
            let look = rd16(lcur) as u32;
            black_box(rd32(g + look.wrapping_mul(4))); // lookahead (faults)
            let j4 = rd32(row + 4);
            let tag1 = rd8(vec + 4) & 0xf;
            if tag1 != 1 {
                arm3_loop1(row, vec, j4, tag1, frac, esi_l, zero);
            } else {
                let tag2 = rd8(row + 1) & 0xf0;
                if tag2 != 0x10 {
                    arm2_loop1(row, vec, j4, tag2, frac, esi_l);
                } else {
                    arm1_loop2(row, vec, frac, esi_l, zero);
                }
            }
            wr8(vec + 4, rd8(vec + 4) & 0xef);
            cnt -= 1;
            if cnt == 0 {
                break;
            }
        }
        tail(cnt0, bound1, out)
    }
}

// ---------------------------------------------------------------------------
// Loop 3 (key-merge walk when the lookup missed).
// ---------------------------------------------------------------------------

/// Loop-3 fetch arm: getters land in the node vector; every float
/// argument is the entry fraction, as in the other loops.
unsafe fn arm1_loop3(
    rowh: u32, node: u32, frac: f32, esi_l: u32, zero: u32,
) {
    unsafe {
        let nvec = node.wrapping_add(0x10);
        let j4 = rd32(rowh + 4);
        let j8 = rd32(rowh + 8);
        let jc = rd32(rowh + 0xc);
        let j10 = rd32(rowh + 0x10);
        slot_u32(j4, 0x30, esi_l, frac, zero);
        slot_u32(j8, 0x30, esi_l, frac, zero);
        slot_u32(jc, 0x30, esi_l, frac, zero);
        slot_u32(j10, 0x30, esi_l, frac, zero);
        let r0 = slot_f32(j4, 0x34, esi_l, frac, zero);
        wrf(nvec, r0);
        let r1 = slot_f32(j8, 0x34, esi_l, frac, zero);
        wrf(nvec + 4, r1);
        let r2 = slot_f32(jc, 0x34, esi_l, frac, zero);
        wrf(nvec + 8, r2);
        let r3 = slot_f32(j10, 0x34, esi_l, frac, zero);
        let sum = sum4(r0, r1, r2, r3);
        let inv = recip_sqrt(sum);
        wrf(nvec, black_box(r0) * inv);
        wrf(nvec + 4, black_box(r1) * inv);
        wrf(nvec + 8, black_box(r2) * inv);
        wrf(nvec + 0xc, black_box(r3) * inv);
    }
}

/// Loop-3 dispatch arm: same shape, heap out-words in the node vector.
unsafe fn arm2_loop3(
    rowh: u32, node: u32, j4: u32, tag2: u8, frac: f32, esi_l: u32,
) {
    unsafe {
        let nvec = node.wrapping_add(0x10);
        if tag2 == 0 {
            slot_u32(j4, 0x20, esi_l, frac, nvec);
        } else {
            slot_u32(j4, 0x24, esi_l, frac, nvec);
            let j8 = rd32(rowh + 8);
            slot_u32(j8, 0x24, esi_l, frac, nvec + 4);
            let jc = rd32(rowh + 0xc);
            slot_u32(jc, 0x24, esi_l, frac, nvec + 8);
            let x3 = rdf(nvec);
            let x4 = rdf(nvec + 4);
            let x5 = rdf(nvec + 8);
            let s = sum3(x3, x4, x5);
            if !(s > ONE) {
                wrf(nvec + 0xc, (black_box(ONE) - s).sqrt());
            } else {
                let r = s.sqrt();
                wr32(nvec + 0xc, 0);
                let k = black_box(ONE) / r;
                wrf(nvec, black_box(x3) * k);
                wrf(nvec + 4, black_box(x4) * k);
                wrf(nvec + 8, black_box(x5) * k);
            }
        }
    }
}

/// Loop-3 third arm: short group into the node vector, or slot 0x24/0x28.
unsafe fn arm3_loop3(
    rowh: u32, node: u32, j4: u32, tag1: u8, frac: f32, esi_l: u32, zero: u32,
) {
    unsafe {
        let nvec = node.wrapping_add(0x10);
        if tag1 == 0 {
            let tag2 = rd8(rowh + 1) & 0xf0;
            if tag2 == 0x10 {
                let j8 = rd32(rowh + 8);
                let jc = rd32(rowh + 0xc);
                slot_u32(j4, 0x30, esi_l, frac, zero);
                slot_u32(j8, 0x30, esi_l, frac, zero);
                slot_u32(jc, 0x30, esi_l, frac, zero);
                wrf(nvec, slot_f32(j4, 0x34, esi_l, frac, zero));
                wrf(nvec + 4, slot_f32(j8, 0x34, esi_l, frac, zero));
                wrf(nvec + 8, slot_f32(jc, 0x34, esi_l, frac, zero));
            } else {
                slot_u32(j4, 0x1c, esi_l, frac, zero);
            }
        } else if tag1 == 2 {
            slot_u32(j4, 0x24, esi_l, frac, nvec);
        } else {
            slot_u32(j4, 0x28, esi_l, frac, nvec);
        }
    }
}

unsafe fn path_b2(
    esi_l: u32,
    edx_t: u32,
    row_f: u32,
    frac: f32,
    bound1: u32,
    f4: u32,
    out: [u32; 2],
) -> u8 {
    unsafe {
        if (bound1 as i32) <= 0 {
            return tail(0, bound1, out);
        }
        if (f4 as i32) <= 0 {
            return tail(0, bound1, out);
        }
        let mut ebx_g = rd32(row_f);
        let m = rd32(edx_t + 0xc);
        let mut mcur = m;
        let mut cnt = bound1;
        if (cnt as i32) <= 0 {
            return tail(0, bound1, out);
        }
        let mut walkcount = 0u32;
        let mut joincount = 0u32;
        let mut edi_stale = 0u32;
        let zero_word = 0u32;
        let zero = (&zero_word as *const u32) as u32;
        loop {
            let node = rd32(mcur);
            if (walkcount as i32) >= (f4 as i32) {
                // Walk-cap path: skips the counter block (unreached in proof:
                // the walk runs at most two steps against a cap of 64).
                mcur += 4;
                edi_stale = edi_stale.wrapping_sub(1);
                cnt = edi_stale;
                if cnt == 0 {
                    break;
                }
                continue;
            }
            let mut edi_h = rd32(ebx_g);
            let mut frame24 = ebx_g.wrapping_add(4);
            let key_n =
                ((rd8(node + 5) as u32) << 16) | (rd16(node + 6) as u32);
            black_box(rd32(frame24)); // prefetch deref (fault fidelity)
            let mut key_h =
                ((rd8(edi_h) as u32) << 16) | (rd16(edi_h + 2) as u32);
            let mut gt = key_h > key_n;
            let take_body = if key_h == key_n {
                true
            } else {
                // Merge walk: advance while strictly below the node key.
                let mut body = false;
                loop {
                    if gt {
                        break;
                    }
                    walkcount += 1;
                    if (walkcount as i32) >= (f4 as i32) {
                        break;
                    }
                    ebx_g = frame24;
                    edi_h = rd32(ebx_g);
                    frame24 = ebx_g.wrapping_add(4);
                    black_box(rd32(frame24));
                    key_h = ((rd8(edi_h) as u32) << 16)
                        | (rd16(edi_h + 2) as u32);
                    gt = key_h > key_n;
                    if key_h == key_n {
                        body = true;
                        break;
                    }
                }
                body
            };
            if take_body {
                edi_stale = edi_h;
                let tag1 = rd8(node + 4) & 0xf;
                let j4 = rd32(edi_h + 4);
                if tag1 != 1 {
                    arm3_loop3(edi_h, node, j4, tag1, frac, esi_l, zero);
                } else {
                    let tag2 = rd8(edi_h + 1) & 0xf0;
                    if tag2 != 0x10 {
                        arm2_loop3(edi_h, node, j4, tag2, frac, esi_l);
                    } else {
                        arm1_loop3(edi_h, node, frac, esi_l, zero);
                    }
                }
                wr8(node + 4, rd8(node + 4) & 0xef);
                ebx_g += 4;
                joincount += 1;
                walkcount += 1;
            }
            mcur += 4;
            cnt -= 1;
            if cnt == 0 {
                break;
            }
        }
        tail(joincount, bound1, out)
    }
}

unsafe fn path_b(
    esi_l: u32,
    edx_t: u32,
    row_f: u32,
    table: u32,
    table_nonnull: bool,
    frac: f32,
    bound1: u32,
    f4: u32,
    out: [u32; 2],
) -> u8 {
    unsafe {
        if table_nonnull {
            path_b1(esi_l, edx_t, row_f, table, frac, bound1, out)
        } else {
            path_b2(esi_l, edx_t, row_f, frac, bound1, f4, out)
        }
    }
}
