// original: 0x0069a7f0 rage::crAnimChannelRawFloat::vf4
/// Sample the raw float channel at frame `f`, storing the result at `out`.
///
/// The frame number is rounded to an integer key index with the original's
/// branch-free sequence (round to nearest via a 2^23 bias taken from the
/// shared constants, minus one when the rounded value overshoots `f`, so the
/// index is the key at or below `f` and the fraction is in [0, 1]); the index
/// is clamped to the key count at `this+0xC`. Fractions within 0.001 of a key
/// snap to that key, otherwise the two surrounding keys are interpolated.
/// Returns `out`.
export!(thiscall, rw_0069a7f0(this: u32, f: f32, out: u32) -> u32 {
    unsafe {
        use core::arch::x86::_mm_cvttss_si32;
        use core::arch::x86::_mm_set_ss;
        let sign_bits = f.to_bits() & 0x8000_0000;
        let abs_f = f32::from_bits(f.to_bits() ^ sign_bits);
        let magic = *global::<f32>(0xFE8CF8);
        let bias = f32::from_bits(
            (if abs_f < magic { magic.to_bits() } else { 0 }) | sign_bits,
        );
        let rounded = (f + bias) - bias;
        let over = rounded - f;
        let sign_f = f32::from_bits(sign_bits);
        let one = *global::<f32>(0xFE88E8);
        let adjust = if over < sign_f { 0.0f32 } else { one };
        let index = _mm_cvttss_si32(_mm_set_ss(rounded - adjust));
        let frac = f - index as f32;
        let hi = *global::<f32>(0xFE88DC);
        let lo = *global::<f32>(0xFE86B4);
        let keys = *((this + 8) as *const u32);
        let n = (*((this + 12) as *const u16) as i32).wrapping_sub(1);
        if !(frac > hi) {
            if !(frac > lo) {
                let i = if index < 0 {
                    0
                } else if index > n {
                    n
                } else {
                    index
                };
                *(out as *mut u32) =
                    *((keys + (i as u32).wrapping_mul(4)) as *const u32);
            } else {
                let j = index.wrapping_add(1);
                let j = if j < 0 { 0 } else if j > n { n } else { j };
                let i = if index < 0 {
                    0
                } else if index > n {
                    n
                } else {
                    index
                };
                let a = *((keys + (i as u32).wrapping_mul(4)) as *const f32);
                let b = *((keys + (j as u32).wrapping_mul(4)) as *const f32);
                *(out as *mut f32) = (b - a) * frac + a;
            }
        } else {
            let k = index.wrapping_add(1);
            let k = if k < 0 { 0 } else if k > n { n } else { k };
            *(out as *mut u32) = *((keys + (k as u32).wrapping_mul(4)) as *const u32);
        }
        out
    }
});
