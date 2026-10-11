// original: 0x00C1DA60 CPlane::vf85

/// Report whether the four stored plane measures add to a value different
/// from the global threshold at file VA `0x00fe8628`. The single precision
/// additions occur in field order `+0x1ebc`, `+0x1eb0`, `+0x1eb4`, `+0x1eb8`;
/// the final not-equal comparison is true for unordered NaN results and false
/// for equal signed zero. Returns the boolean in AL; takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c1da60(this: u32) -> u32 {
    unsafe {
        const MEASURE_0: u32 = 0x1ebc;
        const MEASURE_1: u32 = 0x1eb0;
        const MEASURE_2: u32 = 0x1eb4;
        const MEASURE_3: u32 = 0x1eb8;
        const THRESHOLD_VA: u32 = 0x00fe8628;

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            let bits = unsafe { (address as *const u32).read_unaligned() };
            f32::from_bits(bits)
        }

        #[inline(always)]
        fn add_in_order(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        let mut total = read_f32(this.wrapping_add(MEASURE_0));
        total = add_in_order(total, read_f32(this.wrapping_add(MEASURE_1)));
        total = add_in_order(total, read_f32(this.wrapping_add(MEASURE_2)));
        total = add_in_order(total, read_f32(this.wrapping_add(MEASURE_3)));
        let threshold = read_f32(lf_checker_rt::relocated(THRESHOLD_VA));
        u32::from(total != threshold)
    }
});
