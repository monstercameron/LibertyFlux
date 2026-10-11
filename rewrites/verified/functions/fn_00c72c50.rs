// original: 0x00C72C50 scale_animation_timing_fields

/// Scale the four timing fields at `+0x10`, `+0x14`, `+0x2c` and `+0x30` by
/// the fixed rate factor. The reciprocal field at `+0x18` becomes one divided
/// by the newly scaled `+0x10` value. Each single-precision operation keeps
/// the source operand order and exact IEEE-754 bits.
lf_checker_rt::export!(thiscall, rw_00c72c50(this: u32) -> u32 {
    unsafe {
        const RATE_FACTOR_BITS: u32 = 0x3c8e_fa35;
        const ONE_BITS: u32 = 0x3f80_0000;
        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { f32::from_bits((address as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { (address as *mut u32).write_unaligned(value.to_bits()) }
        }
        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn div(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) / core::hint::black_box(right)
        }

        let rate_factor = f32::from_bits(RATE_FACTOR_BITS);
        let scaled_primary = mul(read_f32(this + 0x10), rate_factor);
        write_f32(this + 0x10, scaled_primary);
        write_f32(this + 0x18, div(f32::from_bits(ONE_BITS), scaled_primary));
        for offset in [0x14u32, 0x2c, 0x30] {
            write_f32(this + offset, mul(read_f32(this + offset), rate_factor));
        }
    }
    0
});
