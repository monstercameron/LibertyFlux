// original: 0x00bf7a30 task_vec3_scale_to_bytes_1e

/// Scale a 3-float vector by a shared constant and store the truncated
/// values as bytes: `dst[i] = cvttss2si(src[i] * k) as u8`.
///
/// `this` is the task object (bytes written at `+DST`); `src` points at
/// three floats. The scale `k` is the shared float constant. Truncation
/// is toward zero with x86 out-of-range semantics (`i32::MIN`).
///
/// Original: 0x00bf7a30 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bf7a30(this: u32, src: u32) -> u32 {
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
        const K: u32 = 0x00FE8BC4;
        const DST: u32 = 0x1e;
        let k = (lf_checker_rt::relocated(K) as *const f32).read_unaligned();
        let mut i = 0u32;
        while i < 3 {
            let x = rd_f32(src.wrapping_add(i * 4));
            let v = cvtt(fmul(x, k));
            ((this + DST + i) as *mut u8).write(v as u8);
            i += 1;
        }
        0
    }
});
