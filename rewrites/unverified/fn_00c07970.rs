// original: 0x00c07970 stream_column_sum (proposed)

/// Sum one float column across the leading records.
///
/// `this` points to the set whose record array is the dword at `RECORDS`;
/// each record is `STRIDE` bytes and contributes the float at `FIELD`. The
/// first multiple of four records are totalled in four lanes (the original's
/// vector loop, one lane per record of each group of four) and folded as the
/// original folds them, then the leftover records are added in order. A
/// non-positive `count` yields zero. The addition order is the original's.
///
/// Original: 0x00c07970 (thiscall, one stack word, float result).
lf_checker_rt::export!(thiscall, rw_00c07970(this: u32, count: u32) -> f32 {
    unsafe {
        const RECORDS: u32 = 0x00;
        const STRIDE: u32 = 0x50;
        const FIELD: u32 = 0x44;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        unsafe fn at(base: u32, rec: u32) -> f32 {
            unsafe { f32::from_bits((base.wrapping_add(rec.wrapping_mul(STRIDE)).wrapping_add(FIELD) as *const u32).read_unaligned()) }
        }
        if (count as i32) <= 0 {
            return 0.0;
        }
        let base = (this.wrapping_add(RECORDS) as *const u32).read_unaligned();
        let main = count & !3;
        let mut lanes = [0.0f32; 4];
        let mut g = 0u32;
        while g < main {
            lanes[0] = add(lanes[0], at(base, g));
            lanes[1] = add(lanes[1], at(base, g.wrapping_add(1)));
            lanes[2] = add(lanes[2], at(base, g.wrapping_add(2)));
            lanes[3] = add(lanes[3], at(base, g.wrapping_add(3)));
            g = g.wrapping_add(4);
        }
        let mut sum = if main == 0 {
            0.0f32
        } else {
            // Horizontal fold in the original's order.
            add(add(lanes[0], lanes[2]), add(lanes[1], lanes[3]))
        };
        let mut r = main;
        while r < count {
            sum = add(sum, at(base, r));
            r = r.wrapping_add(1);
        }
        sum
    }
});
