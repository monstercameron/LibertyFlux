// original: 0x00bf9de0 task_words_1c_to_scaled_floats

/// Expand three signed words into scaled floats:
/// `out[i] = (this[SRC + 2*i] as i16 as f32) * k`.
///
/// The scale `k` is the shared float constant, multiplied in the
/// same order as the original (value times constant).
///
/// Original: 0x00bf9de0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bf9de0(this: u32, out: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const K: u32 = 0x00EAC688;
        const SRC: u32 = 0x1c;
        let k = (lf_checker_rt::relocated(K) as *const f32).read_unaligned();
        let mut i = 0u32;
        while i < 3 {
            let w = ((this + SRC + i * 2) as *const u16).read_unaligned();
            let v = fmul((w as i16) as f32, k);
            (out.wrapping_add(i * 4) as *mut f32).write_unaligned(v);
            i += 1;
        }
        0
    }
});
