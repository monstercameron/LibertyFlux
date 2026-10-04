// original: 0x00d6f490 replay_bar_slot_ratio
/// Publish the callee-1 reading over the slot span plus the two markers.
///
/// Stores `v / (hi - lo)` to `out_ratio` (`v` unsigned from callee 1,
/// `hi`/`lo` from +0xD0/+0xEC), then the markers at +0xB8/+0xCC to
/// `out_first`/`out_second`. Returns the third output pointer.
lf_checker_rt::export!(thiscall, rw_00d6f490(
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
        let hi = f32::from_bits(*((b.add(0xd0)) as *const u32));
        let lo = f32::from_bits(*((b.add(0xec)) as *const u32));
        let v: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let span = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(hi), _mm_set_ss(lo)));
        let q = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(v as f32), _mm_set_ss(span)));
        *(out_ratio as *mut u32) = q.to_bits();
        *(out_first as *mut u32) = *((b.add(0xb8)) as *const u32);
        *(out_second as *mut u32) = *((b.add(0xcc)) as *const u32);
        out_second
    }
});
