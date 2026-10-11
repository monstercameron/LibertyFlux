// original: 0x008606D0 physics_set_scaled_response

/// Updates the response vector fields of the object in ECX. The scale is
/// `1.0 / this[0x150]`; each output is formed in the original single
/// precision order from `this[0x74]`, first multiplied by 14 or 20 and then
/// by that scale. The corresponding companion fields are set to one. A
/// separate word is cleared, and no helper is called.
lf_checker_rt::export!(thiscall, rw_008606d0(this: u32) -> u32 {
    unsafe {
        const SOURCE_MAGNITUDE: u32 = 0x0074;
        const SOURCE_DIVISOR: u32 = 0x0150;
        const RESPONSE_A: u32 = 0x0154;
        const RESPONSE_A_UNIT: u32 = 0x0158;
        const RESPONSE_B: u32 = 0x015c;
        const RESPONSE_B_UNIT: u32 = 0x0160;
        const RESPONSE_C: u32 = 0x0164;
        const RESPONSE_D: u32 = 0x016c;
        const RESET_WORD: u32 = 0x01b0;
        const RECIPROCAL_NUMERATOR_BITS: u32 = 0x3f80_0000;
        const RESPONSE_A_FACTOR_BITS: u32 = 0x4160_0000;
        const RESPONSE_B_FACTOR_BITS: u32 = 0x41a0_0000;
        const UNIT_BITS: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn read_bits(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_bits(base: u32, offset: u32, bits: u32) {
            unsafe { ((base.wrapping_add(offset)) as *mut u32).write_unaligned(bits) }
        }
        #[inline(always)]
        fn multiply(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn divide(numerator: f32, denominator: f32) -> f32 {
            core::hint::black_box(numerator) / core::hint::black_box(denominator)
        }

        let source = f32::from_bits(read_bits(this, SOURCE_MAGNITUDE));
        let divisor = f32::from_bits(read_bits(this, SOURCE_DIVISOR));
        let scale = divide(f32::from_bits(RECIPROCAL_NUMERATOR_BITS), divisor);
        let response_a = multiply(
            multiply(source, f32::from_bits(RESPONSE_A_FACTOR_BITS)),
            scale,
        );
        let response_b = multiply(
            multiply(source, f32::from_bits(RESPONSE_B_FACTOR_BITS)),
            scale,
        );

        write_bits(this, RESET_WORD, 0);
        write_bits(this, RESPONSE_A, response_a.to_bits());
        write_bits(this, RESPONSE_A_UNIT, UNIT_BITS);
        write_bits(this, RESPONSE_B, response_a.to_bits());
        write_bits(this, RESPONSE_B_UNIT, UNIT_BITS);
        write_bits(this, RESPONSE_C, response_b.to_bits());
        write_bits(this, RESPONSE_D, response_b.to_bits());
        0
    }
});
