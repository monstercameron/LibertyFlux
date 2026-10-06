// original: 0x009523A0 clamp_rows_to_mean (proposed)

/// Clamp every below-mean row value up to the mean of the row set.
///
/// Reads the row count `N` from `COUNT`. Sums the float at +0x18 of each
/// row callee 1 returns for indices `0..N` (unsigned bound), adding each
/// cell BEFORE the running sum exactly as the original's `addss` does, then
/// divides once by `N` converted exactly to float (the original goes
/// through doubles; both roundings are correct so the bits agree). When `N`
/// is non-zero, each row is resolved again (signed bound, same `N`) and a
/// row whose value is strictly below the mean — `mean > cell`, which is
/// exactly what the original's `comiss` + `jbe` skips on, NaN included — is
/// raised to the mean. No return channel.
///
/// Original: 0x009523A0 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_009523A0() -> u32 {
    unsafe {
        const COUNT: u32 = 0x11F707C;
        const VAL_OFF: u32 = 0x18;
        const ROW: u32 = 1;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let n = (lf_checker_rt::global::<u32>(COUNT) as *const u32).read();
        let mut sum = 0.0f32;
        if n != 0 {
            let mut i = 0u32;
            while i < n {
                let p = lf_checker_rt::callee_cdecl!(ROW, u32, i);
                let cell = f32::from_bits(
                    (p.wrapping_add(VAL_OFF) as *const u32).read_unaligned(),
                );
                sum = add(cell, sum);
                i += 1;
            }
        }
        let mean = div(sum, n as f32);
        if n != 0 {
            let nn = (lf_checker_rt::global::<u32>(COUNT) as *const u32).read();
            let mut j = 0u32;
            while (j as i32) < (nn as i32) {
                let p = lf_checker_rt::callee_cdecl!(ROW, u32, j);
                let cell = f32::from_bits(
                    (p.wrapping_add(VAL_OFF) as *const u32).read_unaligned(),
                );
                if mean > cell {
                    (p.wrapping_add(VAL_OFF) as *mut u32).write(mean.to_bits());
                }
                j += 1;
            }
        }
        0
    }
});
