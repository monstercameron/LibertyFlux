// original: 0x008e8e10 pairwise_angle_position
//! Validates two packed table references, derives an angle over a scaled
//! record pair, and publishes either a copied or a blended triple.
//!
//! Calling convention is thiscall with five stack words: `out` receives
//! three floats, `arg1`/`arg2` each pack a table index in the low half
//! and a record selector in the high half, `arg3` receives one float,
//! and `arg4` is an optional flag byte. The table lives at `this+0x804`.
//!
//! Validation fails when either index is all-ones or its table slot is
//! null: the flag byte is cleared when present, three zero words are
//! stored through `out`, and `out` is returned.
//!
//! Otherwise the flag byte is set when present and one record is located
//! per packed argument (table slot plus selector times 32). Three signed
//! half words from the first record are scaled by two file constants,
//! two signed half words from the second record are scaled by the first
//! constant, and the two scaled differences form a pair. The angle helper
//! (callee 1) is called with the negated first difference and the second
//! difference, each widened to a double; its double answer is narrowed,
//! scaled by a third constant, and stored through `arg3`.
//!
//! When the flag byte stored in the first record is zero the three scaled
//! values are copied through `out`. Otherwise the difference pair is run
//! through the frame normaliser (callee 2, which rewrites the pair in
//! place) and blended with the record flag value: the flag as a float
//! times a fourth constant plus a fifth gives a factor `t`, and `out`
//! receives `(t*ny + r1a, -t*nx + r1b, t*0 + r1c)`. Returns `out`.
//!
//! Floating-point order is pinned to the original's: every arithmetic
//! operand passes through a compiler barrier, the widening and narrowing
//! conversions are done on bits (round-to-nearest-even, NaN payloads
//! kept as the original's conversions keep them), and the `t * +0.0`
//! term is computed, not folded, so a negative-zero third value rounds
//! the way the original's addition rounds it.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// Read one little-endian word from an absolute address.
#[inline(always)]
unsafe fn word_at(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

/// Read one little-endian half word from an absolute address.
#[inline(always)]
unsafe fn half_at(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read_unaligned() }
}

/// Bit-exact f32-to-f64 widening, as the original's widening conversion
/// does it: exact values, infinities kept, NaNs quieted with the payload
/// kept, denormals normalised, signed zeros kept.
fn widen_f32_bits(a: u32) -> u64 {
    let sign = u64::from(a >> 31) << 63;
    let e = (a >> 23) & 0xFF;
    let f = u64::from(a & 0x7F_FFFF);
    if e == 0xFF {
        if f == 0 {
            return sign | 0x7FF0_0000_0000_0000;
        }
        return sign | 0x7FF8_0000_0000_0000 | (f << 29);
    }
    if e == 0 {
        if f == 0 {
            return sign;
        }
        let lz = (f as u32).leading_zeros() - 9;
        let exp = 896 - lz;
        let frac = ((f << (lz + 1)) & 0x7F_FFFF) << 29;
        return sign | (u64::from(exp) << 52) | frac;
    }
    sign | (u64::from(e + 896) << 52) | (f << 29)
}

/// Bit-exact f64-to-f32 narrowing with round-to-nearest-even, as the
/// original's narrowing conversion does it: overflow to infinity, tiny
/// values to signed zero or a denormal, NaNs quieted keeping the upper
/// payload bits, infinities and signed zeros kept.
fn narrow_f64_bits(d: u64) -> u32 {
    let sign = ((d >> 63) as u32) << 31;
    let e = ((d >> 52) & 0x7FF) as i32;
    let f = d & 0xF_FFFF_FFFF_FFFF;
    if e == 0x7FF {
        if f == 0 {
            return sign | 0x7F80_0000;
        }
        return sign | 0x7FC0_0000 | ((f >> 29) as u32 & 0x3F_FFFF);
    }
    if e == 0 {
        return sign;
    }
    let sig = (1u64 << 52) | f;
    let re = e - 1023 + 127;
    if re >= 255 {
        return sign | 0x7F80_0000;
    }
    if re >= 1 {
        let mut q = (sig >> 29) as u32;
        let rem = sig & 0x1FFF_FFFF;
        let half = 1u64 << 28;
        if rem > half || (rem == half && q & 1 == 1) {
            q += 1;
            if q == 1 << 24 {
                let re2 = re as u32 + 1;
                if re2 == 255 {
                    return sign | 0x7F80_0000;
                }
                return sign | (re2 << 23);
            }
        }
        return sign | ((re as u32) << 23) | (q & 0x7F_FFFF);
    }
    let shift = (30 - re) as u32;
    if shift >= 54 {
        return sign;
    }
    let mut q = (sig >> shift) as u32;
    let rem = sig & ((1u64 << shift) - 1);
    let half = 1u64 << (shift - 1);
    if rem > half || (rem == half && q & 1 == 1) {
        q += 1;
        if q == 0x80_0000 {
            return sign | (1 << 23);
        }
    }
    sign | q
}

/// Low and high words of a double, the order the 8-byte vector
/// transport takes them.
#[inline(always)]
fn lo_hi(d: u64) -> (u32, u32) {
    ((d & 0xFFFF_FFFF) as u32, ((d >> 32) & 0xFFFF_FFFF) as u32)
}

const SCALE_A: u32 = 0xFE87A4;
const SCALE_B: u32 = 0xFE8720;
const ANGLE_SCALE: u32 = 0xE7C2A8;
const BLEND_SCALE: u32 = 0xFE8778;
const BLEND_BIAS: u32 = 0xE833D0;
const FP_ZERO: u32 = 0xFE8628;
const TABLE_OFF: u32 = 0x804;
const RECORD_STRIDE: u32 = 32;

export!(thiscall, rw_008e8e10(this: u32, out: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> u32 {
    unsafe {
        let fail = |out: u32, arg4: u32| unsafe {
            if arg4 != 0 {
                (arg4 as *mut u8).write(0);
            }
            let o = out as *mut u32;
            o.write(0);
            o.add(1).write(0);
            o.add(2).write(0);
        };
        let idx1 = arg1 & 0xFFFF;
        let idx2 = arg2 & 0xFFFF;
        if idx1 == 0xFFFF || idx2 == 0xFFFF {
            fail(out, arg4);
            return out;
        }
        let tab1 = word_at(this.wrapping_add(idx1.wrapping_mul(4)).wrapping_add(TABLE_OFF));
        if tab1 == 0 {
            fail(out, arg4);
            return out;
        }
        let tab2 = word_at(this.wrapping_add(idx2.wrapping_mul(4)).wrapping_add(TABLE_OFF));
        if tab2 == 0 {
            fail(out, arg4);
            return out;
        }
        if arg4 != 0 {
            (arg4 as *mut u8).write(1);
        }
        let rec1 = tab1.wrapping_add((arg1 >> 16).wrapping_mul(RECORD_STRIDE));
        let rec2 = tab2.wrapping_add((arg2 >> 16).wrapping_mul(RECORD_STRIDE));
        let c1 = f32::from_bits(black_box(word_at(relocated(SCALE_A))));
        let c2 = f32::from_bits(black_box(word_at(relocated(SCALE_B))));
        let r1a = black_box(half_at(rec1.wrapping_add(0x14)) as i16 as f32) * black_box(c1);
        let r1b = black_box(half_at(rec1.wrapping_add(0x16)) as i16 as f32) * black_box(c1);
        let r1c = black_box(half_at(rec1.wrapping_add(0x18)) as i16 as f32) * black_box(c2);
        let r2a = black_box(half_at(rec2.wrapping_add(0x14)) as i16 as f32) * black_box(c1);
        let r2b = black_box(half_at(rec2.wrapping_add(0x16)) as i16 as f32) * black_box(c1);
        let dx = black_box(r2a) - black_box(r1a);
        let dy = black_box(r2b) - black_box(r1b);
        let dxb = dx.to_bits();
        let (a0, a1) = lo_hi(widen_f32_bits(dxb ^ 0x8000_0000));
        let (b0, b1) = lo_hi(widen_f32_bits(dy.to_bits()));
        let ans: u64 = callee_cdecl!(1, u64, a0, a1, b0, b1);
        let c3 = f32::from_bits(black_box(word_at(relocated(ANGLE_SCALE))));
        let ang = black_box(f32::from_bits(narrow_f64_bits(ans))) * black_box(c3);
        (arg3 as *mut u32).write(ang.to_bits());
        let flag = (rec1.wrapping_add(0x1A) as *const u8).read();
        let o = out as *mut u32;
        if flag == 0 {
            o.write(black_box(r1a).to_bits());
            o.add(1).write(black_box(r1b).to_bits());
            o.add(2).write(black_box(r1c).to_bits());
            return out;
        }
        let mut pair = [dxb, dy.to_bits()];
        let _: u32 = callee_thiscall!(2, u32, pair.as_mut_ptr() as u32);
        let nx = f32::from_bits(pair[0]);
        let ny = f32::from_bits(pair[1]);
        let c4 = f32::from_bits(black_box(word_at(relocated(BLEND_SCALE))));
        let c5 = f32::from_bits(black_box(word_at(relocated(BLEND_BIAS))));
        let zero = f32::from_bits(black_box(word_at(relocated(FP_ZERO))));
        let t = black_box(flag as f32) * black_box(c4) + black_box(c5);
        let o0 = black_box(t) * black_box(ny) + black_box(r1a);
        let o1 = black_box(-black_box(nx)) * black_box(t) + black_box(r1b);
        let o2 = black_box(t) * black_box(zero) + black_box(r1c);
        o.write(o0.to_bits());
        o.add(1).write(o1.to_bits());
        o.add(2).write(o2.to_bits());
        out
    }
});
