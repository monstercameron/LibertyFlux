// original: 0x0069b120 rage::crAnimChannelRawBool::vf6
/// Sample the packed-bool channel at frame `f`: round half up, take byte
/// `index >> 3` (pinned to the count at `this+0xC`), AND it with the low
/// index byte, and store whether any of the low three bits survive.
/// Returns `out`.
export!(thiscall, rw_0069b120(this: u32, f: f32, out: u32) -> u32 {
    unsafe {
        use core::arch::x86::_mm_cvttss_si32;
        use core::arch::x86::_mm_set_ss;
        let half = *global::<f32>(0xFE8830);
        let raw = _mm_cvttss_si32(_mm_set_ss(f + half));
        let index = if raw < 0 { 0u32 } else { raw as u32 };
        let count = *((this + 12) as *const u16) as u32;
        let byte = index >> 3;
        let at = if byte < count { byte } else { count };
        let keys = *((this + 8) as *const u32);
        let masked = *((keys + at) as *const u8) & (index as u8);
        *(out as *mut u8) = if (masked & 7) != 0 { 1 } else { 0 };
        out
    }
});
