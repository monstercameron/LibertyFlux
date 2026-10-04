// original: 0x0069ada0 rage::crAnimChannelRawQuaternion::vf3
/// Sample the raw quaternion channel at frame `f` into the 16-byte `out`.
/// Key selection mirrors the float sampler (round, clamp, snap within 0.001).
/// The snap path copies the key; the lerp path interpolates all four
/// components and then runs the normalize callee on `out`, returning its
/// answer. The snap path returns `out`. Matches the original's EAX on both.
export!(thiscall, rw_0069ada0(this: u32, f: f32, out: u32) -> u32 {
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
                let src = keys + (i as u32).wrapping_mul(16);
                core::ptr::copy_nonoverlapping(src as *const u8, out as *mut u8, 16);
                out
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
                let a = keys + (i as u32).wrapping_mul(16);
                let b = keys + (j as u32).wrapping_mul(16);
                let aw = *((a + 12) as *const f32);
                let bw = *((b + 12) as *const f32);
                *((out + 12) as *mut f32) = (bw - aw) * frac + aw;
                let ax = *(a as *const f32);
                let bx = *(b as *const f32);
                *(out as *mut f32) = (bx - ax) * frac + ax;
                let ay = *((a + 4) as *const f32);
                let by = *((b + 4) as *const f32);
                *((out + 4) as *mut f32) = (by - ay) * frac + ay;
                let az = *((a + 8) as *const f32);
                let bz = *((b + 8) as *const f32);
                *((out + 8) as *mut f32) = (bz - az) * frac + az;
                callee_thiscall!(1, u32, out)
            }
        } else {
            let k = index.wrapping_add(1);
            let k = if k < 0 { 0 } else if k > n { n } else { k };
            let src = keys + (k as u32).wrapping_mul(16);
            core::ptr::copy_nonoverlapping(src as *const u8, out as *mut u8, 16);
            out
        }
    }
});
