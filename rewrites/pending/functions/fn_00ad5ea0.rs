// original: 0x00AD5EA0 audio_grid_probe
/// Grid-cell query with a tagged dispatch table.
///
/// Scales both coordinates by 0.002, shifts by 6 and floors to integers with
/// the original's exact SSE sequence (2^23 magic rounding, subtract-one
/// adjust, truncate with `MIN` on overflow/NaN); cells outside the 12x12 grid
/// take the fallback exit, which publishes a global sample through `a3`,
/// flags through `a6` (unless null) and returns 1.
///
/// Inside the grid, one tagged word per cell dispatches: tag 0 returns the
/// word with its low byte cleared, tags 1 and 2 call out through per-tag
/// tables forwarding the arguments, and tag 3 walks a second tagged array
/// whose entries call out (tags 1/2), skip (tag 3) or stop with failure
/// (tag 0) until a call reports nonzero. Callee answers count only by their
/// low byte, but the full answer word is preserved into the return value.
export!(cdecl, rw_ad5ea0(a0: f32, a1: f32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    /// The original's exact SSE float-to-int lowering: scale and bias, round
    /// to nearest-even through the 2^23 magic, subtract one when the rounded
    /// value overshoots (a not-less-or-equal test, true on NaN), then
    /// truncate with `i32::MIN` on overflow or NaN like `cvttss2si`.
    fn cvt(x: f32) -> i32 {
        const SCALE: f32 = f32::from_bits(0x3B03126F); // 0.002
        const BIAS: f32 = 6.0;
        const MAGIC: f32 = 8388608.0;
        let scaled = x * SCALE + BIAS;
        let sign = scaled.to_bits() & 0x80000000;
        let ax = f32::from_bits(scaled.to_bits() & 0x7FFFFFFF);
        let magic = f32::from_bits((if ax < MAGIC { MAGIC.to_bits() } else { 0 }) | sign);
        let rounded = scaled + magic - magic;
        let frac = rounded - scaled;
        let signf = f32::from_bits(sign);
        let adj: f32 = if frac.is_nan() || frac > signf { 1.0 } else { 0.0 };
        let r = rounded - adj;
        if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
            i32::MIN
        } else {
            r as i32
        }
    }
    fn lowbit(ans: u32) -> u32 {
        ((ans & 0xFF) != 0) as u32
    }
    fn walk(w: u32, a0b: u32, a1b: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
        // Tagged walk: entries call (tags 1/2), skip (tag 3) or stop
        // (tag 0); the first nonzero call answer wins.
        let mut s = w & 0x3FFF;
        let mut w2 =
            unsafe { *lf_rb34_rt::global::<u16>(0x154E478).offset(s as isize) as u32 };
        if (w2 >> 14) == 0 {
            return w2 & 0xFF00;
        }
        loop {
            let top = w2 >> 14;
            if top == 1 {
                let ptr = lf_rb34_rt::relocated(0x1550EB0)
                    .wrapping_add((w2 & 0x3FFF).wrapping_mul(16));
                let ans = callee_cdecl!(1, u32, ptr, a0b, a1b, a2, a3, a4, a5, a6);
                if (ans & 0xFF) != 0 {
                    return (ans & 0xFFFFFF00) | 1;
                }
            } else if top == 2 {
                let ptr = lf_rb34_rt::relocated(0x154E308)
                    .wrapping_add((w2 & 0x3FFF).wrapping_mul(8));
                let ans = callee_cdecl!(2, u32, ptr, a0b, a1b, a2, a3, a4, a5);
                if (ans & 0xFF) != 0 {
                    return (ans & 0xFFFFFF00) | 1;
                }
            }
            s = s.wrapping_add(1);
            w2 =
                unsafe { *lf_rb34_rt::global::<u16>(0x154E478).offset(s as isize) as u32 };
            if (w2 >> 14) == 0 {
                return w2 & 0xFF00;
            }
        }
    }
    let gx = cvt(a0);
    let gy = cvt(a1);
    if (gx as u32) > 0xB || (gy as u32) > 0xB {
        unsafe {
            // Fallback: publish the sample, flag the status byte, keep the
            // pointer's upper bytes in the return value.
            let sample = *global::<u32>(0x154EC4C);
            *(a3 as *mut u32) = sample;
            if a6 != 0 {
                *(a6 as *mut u8) = 1;
            }
            return (a6 & 0xFFFFFF00) | 1;
        }
    }
    unsafe {
        let a0b = a0.to_bits();
        let a1b = a1.to_bits();
        let idx = (gy as u32).wrapping_add((gx as u32).wrapping_mul(12));
        let w = *global::<u16>(0x154E358).offset(idx as isize) as u32;
        match w >> 14 {
            0 => w & 0xFF00,
            1 => {
                let ptr = relocated(0x1550EB0).wrapping_add((w & 0x3FFF).wrapping_mul(16));
                let ans = callee_cdecl!(1, u32, ptr, a0b, a1b, a2, a3, a4, a5, a6);
                (ans & 0xFFFFFF00) | lowbit(ans)
            }
            2 => {
                let ptr = relocated(0x154E308).wrapping_add((w & 0x3FFF).wrapping_mul(8));
                let ans = callee_cdecl!(2, u32, ptr, a0b, a1b, a2, a3, a4, a5);
                (ans & 0xFFFFFF00) | lowbit(ans)
            }
            3 => walk(w, a0b, a1b, a2, a3, a4, a5, a6),
            // Unreachable (two tag bits cannot exceed 3): the original joins
            // this path to the same walk with index 0, so do that.
            _ => walk(0, a0b, a1b, a2, a3, a4, a5, a6),
        }
    }
});
