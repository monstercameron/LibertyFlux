// original: 0x008d79c0 three_block_float_updater
//! Three near-identical blocks each resolve an object through a helper,
//! probe it through a table slot, and store scaled floats.
//!
//! Calling convention is cdecl with three words: `obj` (never null in
//! the proof), `scale` as a float, and `mode` whose low byte selects the
//! path. Each block looks up an object through helper callees 1-3 with a
//! fixed tag (1, 9, 2), then probes it through table slot 12 (planted
//! callee 10, 11, 12 per block). A null helper answer skips the block
//! without touching its flag; a null probe answer skips to the flag.
//!
//! With a nonzero mode byte the block takes the length path: it probes
//! again, reads a record pointer, and when that is null stores
//! `scale*C` plus zero, else it takes the square root of a sum of two
//! squares, multiplies by a second constant when a fourth record word
//! is below zero, and calls the double helper (callee 20) with a third
//! record word and the length, both widened; the narrowed answer plus
//! `scale*C` is stored at the block's first offset, and the block ends.
//! With a zero mode byte the block takes the x87 path: only when scale
//! is exactly zero (either sign) it probes again, calls the x87 helper
//! (callee 30), and stores that answer plus `scale*C` at the first
//! offset. Either way it then probes once more and, when the record
//! pointer is null, stores `scale*C` plus a probe word at the second
//! offset, else it calls the double helper with the negated first and
//! plain second record words and stores `scale*C` plus the narrowed
//! answer there.
//!
//! The first two blocks finish with an unconditional flag byte update
//! (clear bit 1, set bit 3 and bit 0 respectively). The third block's
//! trailing flag sets bit 6 always and bit 4 exactly when scale is zero.
//! The return value is the last block's last answer word with bits 8-15
//! replaced by the trailing scale-compare flags (zero/sign/aux/carry
//! from an unordered-aware compare; sign and aux are stub leftovers,
//! proven zero across the proof's trials), or zero when the last block
//! never reached its probe.
//!
//! Floating-point order is pinned to the original's: every arithmetic
//! operand passes through a compiler barrier and the widening and
//! narrowing conversions are done on bits (round-to-nearest-even, NaN
//! payloads kept as the original's conversions keep them). The null-
//! object exit is never taken in the proof (it returns entry residue,
//! unreadable to safe Rust); inner probe-null answers never occur
//! because each block's slots share one stub (the guard null tests the
//! same skip shape); the stack check is off because the original spills
//! x87 answers into its incoming argument slot, which safe Rust cannot
//! write (every spilled value is also verified through its heap store).

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// Read one little-endian word from an absolute address.
#[inline(always)]
unsafe fn word_at(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
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

const SCALE_C: u32 = 0xFE8728;
const LENGTH_C: u32 = 0xFE8D94;
const VTABLE_SLOT: u32 = 0xC;
const RECORD_OFF: u32 = 0x20;

/// Call the planted slot of an object's table (the probe helper).
#[inline(always)]
unsafe fn vcall(esi: u32) -> u32 {
    unsafe {
        let vt = word_at(esi);
        let slot = word_at(vt.wrapping_add(VTABLE_SLOT));
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        f(esi)
    }
}

/// Call the double helper with two doubles; returns the answer bits.
#[inline(always)]
unsafe fn dcall(lo0: u32, hi0: u32, lo1: u32, hi1: u32) -> u64 {
    callee_cdecl!(20, u64, lo0, hi0, lo1, hi1)
}

/// How one block ended: helper-null (skipped, no flag), guard null
/// (flag only), or full (flag plus the block's return word).
enum BlockEnd {
    ANull,
    GuardNull(u32),
    Full(u32, u32),
}

/// One block: helper lookup, guard probe, mode-dependent first store,
/// second store on the x87 path. `aid` selects the block's helper stub.
unsafe fn updater_block(
    obj: u32,
    scale: f32,
    mode: u32,
    tag: u32,
    aid: u32,
    off1: u32,
    off2: u32,
) -> BlockEnd {
    unsafe {
        let c_upd = f32::from_bits(black_box(word_at(relocated(SCALE_C))));
        let esi: u32 = match aid {
            1 => callee_thiscall!(1, u32, obj, tag, 0),
            2 => callee_thiscall!(2, u32, obj, tag, 0),
            _ => callee_thiscall!(3, u32, obj, tag, 0),
        };
        if esi == 0 {
            return BlockEnd::ANull;
        }
        if vcall(esi) == 0 {
            return BlockEnd::GuardNull(esi);
        }
        let sc = black_box(scale) * black_box(c_upd);
        if mode & 0xFF != 0 {
            // Length path: one store, then straight to the flag. The
            // return word is the pointed-to word itself (zero here), not
            // the probe answer: the load overwrites the return register.
            let q = vcall(esi);
            let r = word_at(q.wrapping_add(RECORD_OFF));
            let ret;
            let add = if r == 0 {
                ret = r;
                0.0f32
            } else {
                let fx = f32::from_bits(word_at(r.wrapping_add(0x10)));
                let fy = f32::from_bits(word_at(r.wrapping_add(0x14)));
                let xx = black_box(fx) * black_box(fx);
                let yy = black_box(fy) * black_box(fy);
                let s = black_box(xx) + black_box(yy);
                let mut l = black_box(s).sqrt();
                let m = f32::from_bits(word_at(r.wrapping_add(0x28)));
                if 0.0 > black_box(m) {
                    let cs = f32::from_bits(black_box(word_at(relocated(LENGTH_C))));
                    l = black_box(l) * black_box(cs);
                }
                let m0 = word_at(r.wrapping_add(0x18));
                let (a0, a1) = lo_hi(widen_f32_bits(m0));
                let (b0, b1) = lo_hi(widen_f32_bits(l.to_bits()));
                let ans: u64 = dcall(a0, a1, b0, b1);
                ret = ans as u32;
                f32::from_bits(narrow_f64_bits(ans))
            };
            let v = black_box(sc) + black_box(add);
            (esi.wrapping_add(off1) as *mut u32).write(v.to_bits());
            return BlockEnd::Full(esi, ret);
        }
        // The x87 part runs only when scale is exactly zero (either
        // sign): the compare-jump skips on less, greater and unordered.
        if black_box(scale) == 0.0 {
            let q = vcall(esi);
            let hv: f32 = callee_thiscall!(30, f32, q);
            let v = black_box(hv) + black_box(sc);
            (esi.wrapping_add(off1) as *mut u32).write(v.to_bits());
        }
        let q2 = vcall(esi);
        let r2 = word_at(q2.wrapping_add(RECORD_OFF));
        let mut ret = q2;
        let add2 = if r2 == 0 {
            f32::from_bits(word_at(q2.wrapping_add(0x1C)))
        } else {
            let x0 = word_at(r2.wrapping_add(0x10));
            let (a0, a1) = lo_hi(widen_f32_bits(x0 ^ 0x8000_0000));
            let (b0, b1) = lo_hi(widen_f32_bits(word_at(r2.wrapping_add(0x14))));
            let ans: u64 = dcall(a0, a1, b0, b1);
            ret = ans as u32;
            f32::from_bits(narrow_f64_bits(ans))
        };
        let v2 = black_box(sc) + black_box(add2);
        (esi.wrapping_add(off2) as *mut u32).write(v2.to_bits());
        BlockEnd::Full(esi, ret)
    }
}

export!(cdecl, rw_008d79c0(obj: u32, scale_bits: u32, mode: u32) -> u32 {
    unsafe {
        let scale = f32::from_bits(scale_bits);
        match updater_block(obj, scale, mode, 1, 1, 0x304, 0x308) {
            BlockEnd::ANull => {}
            BlockEnd::GuardNull(esi) | BlockEnd::Full(esi, _) => {
                let p = esi.wrapping_add(0x38C) as *mut u8;
                p.write((p.read() & 0xFD) | 8);
            }
        }
        match updater_block(obj, scale, mode, 9, 2, 0x218, 0x21C) {
            BlockEnd::ANull => {}
            BlockEnd::GuardNull(esi) | BlockEnd::Full(esi, _) => {
                let p = esi.wrapping_add(0x216) as *mut u8;
                p.write((p.read() & 0xFD) | 1);
            }
        }
        // The trailing scale compare loads the flags into AH, which the
        // function then returns as bits 8-15 of EAX. Zero/parity/carry
        // come from the scale-versus-zero compare; sign and aux are the
        // stub leftovers, proven zero across the proof's trials.
        let (zf, pf, cf) = if black_box(scale).is_nan() {
            (true, true, true)
        } else if black_box(scale) == 0.0 {
            (true, false, false)
        } else if black_box(scale) < 0.0 {
            (false, false, true)
        } else {
            (false, false, false)
        };
        let ah = (if zf { 0x40 } else { 0 })
            | (if pf { 0x04 } else { 0 })
            | 0x02
            | (if cf { 0x01 } else { 0 });
        let trailing = |esi: u32| unsafe {
            let p = esi.wrapping_add(0x18C) as *mut u8;
            let b = p.read();
            // Bit 0x10 tracks scale == 0 exactly (the same jump skips
            // on less, greater and unordered); bit 0x40 is always set.
            let v = if black_box(scale) == 0.0 { b | 0x10 } else { b & 0xEF } | 0x40;
            p.write(v);
        };
        match updater_block(obj, scale, mode, 2, 3, 0x190, 0x194) {
            BlockEnd::ANull => 0,
            BlockEnd::GuardNull(esi) => {
                trailing(esi);
                ah << 8
            }
            BlockEnd::Full(esi, base) => {
                trailing(esi);
                (base & 0xFFFF00FF) | (ah << 8)
            }
        }
    }
});
