// original: 0x0069b070 rage::crAnimChannelRawInt::vf5
/// Sample the raw int channel at frame `f`: round half up (`trunc(f + 0.5)`
/// with the shared half constant), clamp to the key range at `this+0xC`, copy
/// that key to `out`. Returns `out`.
export!(thiscall, rw_0069b070(this: u32, f: f32, out: u32) -> u32 {
    unsafe {
        use core::arch::x86::_mm_cvttss_si32;
        use core::arch::x86::_mm_set_ss;
        let half = *global::<f32>(0xFE8830);
        let index = _mm_cvttss_si32(_mm_set_ss(f + half));
        let keys = *((this + 8) as *const u32);
        let n = (*((this + 12) as *const u16) as i32).wrapping_sub(1);
        let i = if index < 0 {
            0
        } else if index > n {
            n
        } else {
            index
        };
        *(out as *mut u32) = *((keys + (i as u32).wrapping_mul(4)) as *const u32);
        out
    }
});
