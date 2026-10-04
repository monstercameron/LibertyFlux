// original: 0x009f0e20 CPlayerPed::vf45
/// Player-ped virtual slot 45: range-gated relink plus state clearing.
///
/// Forwards the flag byte to the base entry first. Unless the link at
/// `0x38` already equals the target at `0x7b4` or the target is null, it
/// measures the squared distance between the point at
/// `[0x20]+0x30` and the target's point at `+0x40` with SSE scalar ops in
/// the original's order; a set flag byte or a distance above the constant
/// threshold calls the relink entry with 1; a below-threshold distance
/// returns the source point immediately. A set flag byte then clears
/// the nine words at `0xad0`..`0xaf8` and, when `0x219` is set, marks
/// `0xbe0`. Returns the relink answer when it ran, the source point on a
/// short miss, else the link word. NaN distances keep the old link,
/// matching the original's `jbe`.
export!(thiscall, rw_009f0e20(this_ptr: u32, flag: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_load_ss};
        let _: u32 = callee_thiscall!(2, u32, this_ptr, flag);
        let link = *((this_ptr + 0x38) as *const u32);
        let mut answer = link;
        let target = *((this_ptr + 0x7b4) as *const u32);
        if link != target && target != 0 {
            let src = *((this_ptr + 0x20) as *const u32);
            let dx = _mm_cvtss_f32(_mm_load_ss((src + 0x30) as *const f32))
                - _mm_cvtss_f32(_mm_load_ss((target + 0x40) as *const f32));
            let dy = _mm_cvtss_f32(_mm_load_ss((src + 0x34) as *const f32))
                - _mm_cvtss_f32(_mm_load_ss((target + 0x44) as *const f32));
            let dz = _mm_cvtss_f32(_mm_load_ss((src + 0x38) as *const f32))
                - _mm_cvtss_f32(_mm_load_ss((target + 0x48) as *const f32));
            let dist_sq = dx * dx + dy * dy + dz * dz;
            let threshold = f32::from_bits(*global::<u32>(0xfe8b40));
            if (flag & 0xff) != 0 || dist_sq > threshold {
                answer = callee_thiscall!(3, u32, this_ptr, 1);
            } else {
                return src;
            }
        }
        if (flag & 0xff) == 0 {
            return answer;
        }
        for off in [0xad8u32, 0xad4, 0xad0, 0xae8, 0xae4, 0xae0, 0xaf8, 0xaf4, 0xaf0] {
            *((this_ptr + off) as *mut u32) = 0;
        }
        if *((this_ptr + 0x219) as *const u8) != 0 {
            *((this_ptr + 0xbe0) as *mut u32) |= 0xc;
        }
        answer
    }
});
