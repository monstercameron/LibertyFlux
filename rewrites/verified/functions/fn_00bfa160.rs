// original: 0x00bfa160 task_vec3_scale_to_words_28

/// Scale a 3-float vector by a shared constant and store the truncated
/// values as 16-bit words: `dst[i] = cvttss2si(src[i] * k) as u16`.
///
/// `this` is the task object (words written at `+DST`); `src` points at
/// three floats. The scale `k` is the shared float constant.
///
/// Original: 0x00bfa160 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bfa160(this: u32, src: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x <= -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        unsafe fn rd_f32(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        const K: u32 = 0x00EBA958;
        const DST: u32 = 0x28;
        let k = (lf_checker_rt::relocated(K) as *const f32).read_unaligned();
        let mut i = 0u32;
        while i < 3 {
            let x = rd_f32(src.wrapping_add(i * 4));
            let v = cvtt(fmul(x, k));
            ((this + DST + i * 2) as *mut u16).write_unaligned(v as u16);
            i += 1;
        }
        0
    }
});
