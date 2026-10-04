// original: 0x0069aaa0 rage::crAnimChannelRawVector3::vf2
/// Sample the raw vector3 channel at frame `f` into the 12-byte `out`
/// (x, y, z). Key selection mirrors the float sampler (round, clamp to the
/// count at `this+0xC`, snap within 0.001 of a key). The snap path copies all
/// 16 bytes of the key including w, while the lerp path writes only x, y, z.
/// Returns `out` on the lerp path and the key's w word on the snap path,
/// matching the original's EAX.
export!(thiscall, rw_0069aaa0(this: u32, f: f32, out: u32) -> u32 {
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
                *((src + 12) as *const u32)
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
                let ax = *(a as *const f32);
                let bx = *(b as *const f32);
                *(out as *mut f32) = (bx - ax) * frac + ax;
                let ay = *((a + 4) as *const f32);
                let by = *((b + 4) as *const f32);
                *((out + 4) as *mut f32) = (by - ay) * frac + ay;
                let az = *((a + 8) as *const f32);
                let bz = *((b + 8) as *const f32);
                *((out + 8) as *mut f32) = (bz - az) * frac + az;
                out
            }
        } else {
            let k = index.wrapping_add(1);
            let k = if k < 0 { 0 } else if k > n { n } else { k };
            let src = keys + (k as u32).wrapping_mul(16);
            core::ptr::copy_nonoverlapping(src as *const u8, out as *mut u8, 16);
            *((src + 12) as *const u32)
        }
    }
});
