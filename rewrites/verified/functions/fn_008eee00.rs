// original: 0x008eee00 query_slot_records_box
use lf_checker_rt::{callee_thiscall, export};

/// Query one slot's records against a coordinate box (original 0x008EEE00).
///
/// Walks the record list of slot `a7`, scaling each record's three fixed
/// point coordinates and keeping only records inside the six-bound box.
/// Survivors pass a flag filter, then take one of two tag tests chosen by
/// the mode byte: a bit-pair comparison when it is non-zero, a top-bit
/// comparison against the tag bound when it is zero. Matches are
/// reported through the match callee with two out-slots the callee clears;
/// the retry loops behind non-zero out-slots are kept exact. Returns
/// nothing meaningful.
export!(thiscall, rw_008eee00(
    this: u32,
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32,
    a6: u32, a7: u32, a8: u32,
) -> u32 {
    const BASES_OFF: usize = 0x804;
    const COUNTS_OFF: usize = 0xB04;
    const REC_LEN: usize = 0x20;
    const XY_SCALE: f32 = 0.125;
    const Z_SCALE: f32 = 0.015625;
    // `!(x >= b)` reproduces comiss+jb exactly, including NaN (unordered
    // also skips); a plain `<` would wrongly proceed past NaN bounds.
    #[inline(always)]
    fn below(x: f32, b: f32) -> bool {
        !(x >= b)
    }
    unsafe {
        let base = this as *mut u8;
        let slot = (a7 as usize).wrapping_mul(4);
        let recbase = (base.add(BASES_OFF + slot) as *const u32).read();
        if recbase == 0 {
            return 0;
        }
        let count = (base.add(COUNTS_OFF + slot) as *const u32).read() as i32;
        if count <= 0 {
            return 0;
        }
        let lo0 = f32::from_bits(a0);
        let hi0 = f32::from_bits(a1);
        let lo1 = f32::from_bits(a2);
        let hi1 = f32::from_bits(a3);
        let lo2 = f32::from_bits(a4);
        let hi2 = f32::from_bits(a5);
        let mut out1: u32 = 0;
        let mut out2: u32 = 0;
        let out1p = (&mut out1 as *mut u32) as u32;
        let out2p = (&mut out2 as *mut u32) as u32;
        let mut off: usize = 0;
        let mut left = count;
        while left != 0 {
            let rec = (recbase as *mut u8).add(off);
            let sx = (rec.add(0x14) as *const i16).read();
            let sy = (rec.add(0x16) as *const i16).read();
            let sz = (rec.add(0x18) as *const i16).read();
            let x = (sx as f32) * XY_SCALE;
            let y = (sy as f32) * XY_SCALE;
            let z = (sz as f32) * Z_SCALE;
            let mut fire = true;
            if below(x, lo0) {
                fire = false;
            } else if below(hi0, x) {
                fire = false;
            } else if below(y, lo1) {
                fire = false;
            } else if below(hi1, y) {
                fire = false;
            } else if below(z, lo2) {
                fire = false;
            } else if below(hi2, z) {
                fire = false;
            }
            if fire {
                let ok = callee_thiscall!(1, u32, this, rec as u32);
                if (ok as u8) != 0 {
                    let dl = a8 as u8;
                    let go = if dl != 0 {
                        let b1e = rec.add(0x1E).read();
                        let b1f = rec.add(0x1F).read();
                        ((b1e >> 5) & 4) != (b1f & 4)
                    } else {
                        let b1e = rec.add(0x1E).read();
                        (b1e >> 7) != (a6 as u8)
                    };
                    if go {
                        callee_thiscall!(2, u32, this, rec as u32, out1p, out2p, a6, a8);
                        while (out1p as *const u32).read() != 0 {
                            let cur = (out1p as *const u32).read();
                            callee_thiscall!(2, u32, this, cur, out1p, 0, a6, a8);
                        }
                        while (out2p as *const u32).read() != 0 {
                            let cur = (out2p as *const u32).read();
                            callee_thiscall!(2, u32, this, cur, out2p, 0, a6, a8);
                        }
                    }
                }
            }
            off = off.wrapping_add(REC_LEN);
            left -= 1;
        }
    }
    0
});
