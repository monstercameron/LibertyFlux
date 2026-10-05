// original: 0x00bf9d20 task_bytes_10_to_scaled_floats

/// Expand three signed bytes into scaled floats:
/// `out[i] = (this[SRC + i] as i8 as f32) * k`.
///
/// The scale `k` is the shared float constant, multiplied in the
/// same order as the original (value times constant).
///
/// Original: 0x00bf9d20 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bf9d20(this: u32, out: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const K: u32 = 0x00FE8700;
        const SRC: u32 = 0x10;
        let k = (lf_checker_rt::relocated(K) as *const f32).read_unaligned();
        let mut i = 0u32;
        while i < 3 {
            let b = ((this + SRC + i) as *const i8).read();
            let v = fmul(b as f32, k);
            (out.wrapping_add(i * 4) as *mut f32).write_unaligned(v);
            i += 1;
        }
        0
    }
});
