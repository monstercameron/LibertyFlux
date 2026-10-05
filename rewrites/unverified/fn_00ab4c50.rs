// original: 0x00ab4c50 stream_age_requests (proposed)

/// Age the delay of every live request, honouring `filter`.
///
/// Walks the bucket chain at `+0x143048`, skipping buckets whose tag at
/// `+8` differs from `filter` (any bucket when `filter` is zero). In each
/// kept bucket, every chained entry whose live byte at `+0x2c` is set gets
/// its delay at `+0x7c` recomputed: `k = trunc(delay * -255)`, `m =
/// max(-adjust - k, 0)`, `delay = m * (1/255)`, with the truncation
/// following x86 `cvttss2si` exactly (NaN and out-of-range give
/// `0x80000000`). No return value.
///
/// Original: 0x00ab4c50 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab4c50(this: u32, adjust: u32, filter: u32) -> u32 {
    unsafe {
        const CHAIN_OFF: u32 = 0x143048;
        const BUCKET_TAG: u32 = 8;
        const ENTRIES_OFF: u32 = 0xB8;
        const LIVE_OFF: u32 = 0x2C;
        const DELAY_OFF: u32 = 0x7C;
        const SCALE_DOWN: f32 = f32::from_bits(0xC37F_0000); // -255
        const SCALE_UP: f32 = f32::from_bits(0x3B80_8081); // 1/255
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate toward zero exactly as `cvttss2si` does.
        #[inline(always)]
        fn cvt_trunc(f: f32) -> i32 {
            if f.is_nan() {
                return i32::MIN;
            }
            let t = f.trunc();
            if t >= 2147483648.0 || t < -2147483648.0 {
                return i32::MIN;
            }
            t as i32
        }
        let mut bucket = ((this + CHAIN_OFF) as *const u32).read_unaligned();
        while bucket != 0 {
            if filter == 0 || ((bucket + BUCKET_TAG) as *const u32).read_unaligned() == filter {
                let mut entry = ((bucket + ENTRIES_OFF) as *const u32).read_unaligned();
                while entry != 0 {
                    let next = (entry as *const u32).read_unaligned();
                    if ((entry + LIVE_OFF) as *const u8).read() != 0 {
                        let delay =
                            f32::from_bits(((entry + DELAY_OFF) as *const u32).read_unaligned());
                        let k = cvt_trunc(mul(delay, SCALE_DOWN));
                        let d = (adjust as i32).wrapping_neg().wrapping_sub(k);
                        let m = if d > 0 { d } else { 0 };
                        let v = mul(m as f32, SCALE_UP);
                        ((entry + DELAY_OFF) as *mut u32).write_unaligned(v.to_bits());
                    }
                    entry = next;
                }
            }
            bucket = (bucket as *const u32).read_unaligned();
        }
        0
    }
});
