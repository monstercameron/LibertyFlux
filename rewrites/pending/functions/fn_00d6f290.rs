// original: 0x00d6f290 replay_bar_blend_factors
/// Blend the two clock readings with the selected time bases.
///
/// Reads two clock values through virtual slots +0x20/+0x24 of the object at
/// `src` (callees 1 and 2) into `out_first`/`out_second` as floats, then
/// writes `(first / second) * scale` to `out_blend`, where the scale
/// comes from `scale`. When the base ratio `r` is below 1 the blend is further
/// scaled by `r`; otherwise the scale itself is divided by `r`. Returns
/// the scale pointer. Each base is picked by one sample of callee 3.
lf_checker_rt::export!(stdcall, rw_00d6f290(
    src: u32,
    out_first: u32,
    out_second: u32,
    out_blend: u32,
    scale: u32,
) -> u32 {
    unsafe {
        use core::arch::x86::{
            _mm_cvtss_f32, _mm_div_ss, _mm_mul_ss, _mm_set_ss,
        };
        let obj = *(src as *const u32);
        let vtab = *(obj as *const u32);
        let slot1 = *(((vtab as *const u8).add(0x20)) as *const u32);
        let read1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot1 as usize);
        let v1 = read1(obj);
        *(out_first as *mut u32) = ((v1 as i32) as f32).to_bits();
        let slot2 = *(((vtab as *const u8).add(0x24)) as *const u32);
        let read2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot2 as usize);
        let v2 = read2(obj);
        *(out_second as *mut u32) = ((v2 as i32) as f32).to_bits();
        let w1: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        let mut s1 = *lf_checker_rt::global::<u32>(0x105c880);
        if (w1 & 0xFF) != 0 {
            s1 = *lf_checker_rt::global::<u32>(0x105c87c);
        }
        let w2: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        let mut s2 = *lf_checker_rt::global::<u32>(0x105c884);
        if (w2 & 0xFF) != 0 {
            s2 = *lf_checker_rt::global::<u32>(0x105c888);
        }
        let f1 = f32::from_bits(*(out_first as *const u32));
        let f2 = f32::from_bits(*(out_second as *const u32));
        let mut x = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(f1), _mm_set_ss(f2)));
        let r = _mm_cvtss_f32(_mm_div_ss(
            _mm_set_ss((s1 as i32) as f32),
            _mm_set_ss((s2 as i32) as f32),
        ));
        let sc = f32::from_bits(*(scale as *const u32));
        x = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(x), _mm_set_ss(sc)));
        *(out_blend as *mut u32) = x.to_bits();
        if 1.0f32 > r {
            x = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(x), _mm_set_ss(r)));
            *(out_blend as *mut u32) = x.to_bits();
        } else {
            let q = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(sc), _mm_set_ss(r)));
            *(scale as *mut u32) = q.to_bits();
        }
        scale
    }
});
