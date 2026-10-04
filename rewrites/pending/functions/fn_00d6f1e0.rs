// original: 0x00d6f1e0 replay_bar_measure_slots
/// Publish the slot ratio and the scores of the two marker slots.
///
/// Stores `n / (hi - lo)` to `out_ratio` (`n` unsigned from +0x100,
/// `hi`/`lo` from +0xD0/+0xEC), then the callee-1 scores of the markers at
/// +0xB8/+0xCC to `out_first`/`out_second`. Returns the second score.
lf_checker_rt::export!(thiscall, rw_00d6f1e0(
    this_ptr: u32,
    out_ratio: u32,
    out_first: u32,
    out_second: u32,
) -> u32 {
    unsafe {
        use core::arch::x86::{
            _mm_cvtss_f32, _mm_div_ss, _mm_set_ss, _mm_sub_ss,
        };
        let b = this_ptr as *const u8;
        let n = *((b.add(0x100)) as *const u32);
        let hi = f32::from_bits(*((b.add(0xd0)) as *const u32));
        let lo = f32::from_bits(*((b.add(0xec)) as *const u32));
        let span = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(hi), _mm_set_ss(lo)));
        let ratio = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(n as f32), _mm_set_ss(span)));
        *(out_ratio as *mut u32) = ratio.to_bits();
        let m1 = *((b.add(0xb8)) as *const u32);
        let v1: u32 = lf_checker_rt::callee_thiscall!(1, u32, this_ptr, m1, 0);
        *(out_first as *mut u32) = v1;
        let m2 = *((b.add(0xcc)) as *const u32);
        let v2: u32 = lf_checker_rt::callee_thiscall!(1, u32, this_ptr, m2, 0);
        *(out_second as *mut u32) = v2;
        v2
    }
});
