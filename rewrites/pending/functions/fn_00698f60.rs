// original: 0x00698f60 rage::crAnimChannelDeltaFloat::vf4
/// Delta-float sampler: rounds the input down to a segment index with the
/// classic add-and-subtract-2^23 trick (globals hold the magic constants),
/// takes the fractional remainder, and either snaps to one neighbor sample
/// (fraction outside the (0.001, 0.999] band) or linearly interpolates
/// between the two bracketing samples. Returns the output pointer.
lf_k2_rt::export!(thiscall, rw_00698f60(this: *mut u8, x_bits: u32, out: *mut u32) -> u32 {
    unsafe {
        let two23 = *lf_k2_rt::global::<f32>(0x00FE8CF8);
        let one = *lf_k2_rt::global::<f32>(0x00FE88E8);
        let thresh_hi = *lf_k2_rt::global::<f32>(0x00FE88DC);
        let thresh_lo = *lf_k2_rt::global::<f32>(0x00FE86B4);
        let x = f32::from_bits(x_bits);
        let sign = x_bits & 0x8000_0000;
        // Round-to-nearest-even as a float; the magic is neutralized for
        // huge inputs exactly like the original's compare mask does.
        let absx = f32::from_bits(x_bits & 0x7FFF_FFFF);
        let magic = if absx < two23 {
            f32::from_bits(0x4B00_0000 | sign)
        } else {
            f32::from_bits(sign)
        };
        let rounded = (x + magic) - magic;
        // Step down when the rounded value overshoots (or is unordered).
        let s = f32::from_bits(sign);
        let down = rounded - x;
        let n = if !(down < s) { rounded - one } else { rounded };
        // Truncate toward zero with x86 convert semantics (invalid -> MIN).
        let esi: i32 = if n.is_nan() || n >= 2147483648.0 || n < -2147483648.0 {
            i32::MIN
        } else {
            n as i32
        };
        let frac = x - (esi as f32);
        // A fraction outside (0.001, 0.999] snaps to the neighbor sample;
        // inside the band the two bracketing samples are interpolated.
        if frac > thresh_hi {
            let v: u32 = lf_k2_rt::callee_thiscall!(
                1,
                u32,
                this as u32,
                esi.wrapping_add(1) as u32
            );
            *out = v;
        } else if !(frac > thresh_lo) {
            // Note: this path re-enters past the increment, so it samples
            // segment esi itself, not esi+1.
            let v: u32 = lf_k2_rt::callee_thiscall!(1, u32, this as u32, esi as u32);
            *out = v;
        } else {
            let v0 = f32::from_bits(lf_k2_rt::callee_thiscall!(
                2,
                u32,
                this as u32,
                esi as u32
            ));
            let v1 = f32::from_bits(lf_k2_rt::callee_thiscall!(
                3,
                u32,
                this as u32,
                esi.wrapping_add(1) as u32
            ));
            *out = ((v1 - v0) * frac + v0).to_bits();
        }
        out as u32
    }
});
