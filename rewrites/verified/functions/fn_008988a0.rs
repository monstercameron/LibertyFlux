// original: 0x008988a0 audio_curve_filter
//! Scales record fields through a double-precision curve helper, blends
//! the results, and optionally recurses over a 13-entry table.
//!
//! Calling convention is thiscall with two stack words, both floats:
//! `arg0` is a blend factor and `arg1` a mix factor that also gates the
//! recursion on its low byte. `this` points at an object whose first
//! word points at a record of half words.
//!
//! Two signed record half words are each scaled by a first file constant.
//! Each scaled value minus a floor constant is compared against zero; a
//! negative difference yields a zero result, otherwise the scaled value
//! times a second constant is widened to a double and passed, together
//! with a file double constant, to the curve helper (callees 1 and 2),
//! whose double answer is narrowed back to a float. The two results blend
//! with the first argument as `(s1 - s0) * arg0 + s0` and drive the float
//! helper (callee 3), whose answer is stored at `this+4`.
//!
//! Two unsigned record half words are scaled by the first constant and
//! mixed by the first argument as `(f15 - f17) * arg0 + f17`, stored at
//! `this+8`. When the second argument's low byte is zero the function
//! returns the float helper's answer word (still in the return register).
//! Otherwise it walks thirteen half words at `this+0x54`: every word
//! other than all-ones selects a record at the global base plus the word
//! times 112 and recurses through the stub (callee 4) with the first and
//! second arguments, and returns the last recursive answer when the last
//! slot fires, else all-ones.
//!
//! Floating-point order is pinned to the original's: every arithmetic
//! operand passes through a compiler barrier and the widening and
//! narrowing conversions are done on bits (round-to-nearest-even, NaN
//! payloads kept as the original's conversions keep them).

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

const SCALE: u32 = 0xFE870C;
const CURVE_SCALE: u32 = 0xFE876C;
const FLOOR: u32 = 0xFE8DF8;
const CURVE_BASE: u32 = 0xFE8A78;
const RECORD_BASE: u32 = 0x115F810;
const RECORD_STRIDE: u32 = 0x70;
const TABLE_OFF: u32 = 0x54;
const TABLE_LEN: u32 = 13;

export!(thiscall, rw_008988a0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let inner = word_at(this);
        let c1 = f32::from_bits(black_box(word_at(relocated(SCALE))));
        let c2 = f32::from_bits(black_box(word_at(relocated(CURVE_SCALE))));
        let c3 = f32::from_bits(black_box(word_at(relocated(FLOOR))));
        let d0 = (relocated(CURVE_BASE) as *const u64).read_unaligned();
        let v1 = black_box(half_at(inner.wrapping_add(0xD)) as i16 as f32) * black_box(c1);
        let s0 = if black_box(black_box(v1) - black_box(c3)) < 0.0 {
            0.0f32
        } else {
            let x = black_box(v1) * black_box(c2);
            let (a0, a1) = lo_hi(d0);
            let (b0, b1) = lo_hi(widen_f32_bits(x.to_bits()));
            let ans: u64 = callee_cdecl!(1, u64, a0, a1, b0, b1);
            f32::from_bits(narrow_f64_bits(ans))
        };
        let v2 = black_box(half_at(inner.wrapping_add(0xB)) as i16 as f32) * black_box(c1);
        let s1 = if black_box(black_box(v2) - black_box(c3)) < 0.0 {
            0.0f32
        } else {
            let x = black_box(v2) * black_box(c2);
            let (a0, a1) = lo_hi(d0);
            let (b0, b1) = lo_hi(widen_f32_bits(x.to_bits()));
            let ans: u64 = callee_cdecl!(2, u64, a0, a1, b0, b1);
            f32::from_bits(narrow_f64_bits(ans))
        };
        let base = black_box(s1) - black_box(s0);
        let t0 = f32::from_bits(arg0);
        let lerp = black_box(base) * black_box(t0) + black_box(s0);
        let f17 = black_box(half_at(inner.wrapping_add(0x17)) as u16 as f32) * black_box(c1);
        let f15 = black_box(half_at(inner.wrapping_add(0x15)) as u16 as f32) * black_box(c1);
        let curve: f32 = callee_cdecl!(3, f32, black_box(lerp).to_bits());
        (this.wrapping_add(4) as *mut u32).write(curve.to_bits());
        let mix = black_box(black_box(f15) - black_box(f17)) * black_box(t0) + black_box(f17);
        (this.wrapping_add(8) as *mut u32).write(mix.to_bits());
        if arg1 & 0xFF == 0 {
            return curve.to_bits();
        }
        let gbase = word_at(relocated(RECORD_BASE));
        let mut ans = 0u32;
        let mut fired12 = false;
        let mut i = 0u32;
        while i < TABLE_LEN {
            let ax = half_at(this.wrapping_add(TABLE_OFF).wrapping_add(i.wrapping_mul(2))) as u32;
            if ax != 0xFFFF {
                let rec = gbase.wrapping_add(ax.wrapping_mul(RECORD_STRIDE));
                ans = callee_thiscall!(4, u32, rec, arg0, arg1);
                if i == TABLE_LEN - 1 {
                    fired12 = true;
                }
            }
            i += 1;
        }
        if fired12 {
            ans
        } else {
            0xFFFF
        }
    }
});
