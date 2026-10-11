// original: 0x00C49950 camera_spline_sample_global_curve

/// Interpolate one scalar value from adjacent entries of the global curve table.
///
/// The single stack argument is a float phase. A global signed count scales it;
/// the routine rounds through the x86 bias-and-subtract sequence, applies the
/// matching signed correction, and converts the result with the truncating
/// x86 float-to-integer instruction. That conversion yields `i32::MIN` for a
/// NaN, either infinity, a value at or above 2,147,483,648, or a value below
/// -2,147,483,648. The 32-bit table offset wraps before the two neighboring
/// float words are loaded. The scaled fractional remainder is divided by the
/// count and used to interpolate those words in the original operand order.
/// The output is returned in the x87 floating-point register; the callee pops
/// its one stack argument. The original also writes the final output into that
/// argument slot before returning.
lf_checker_rt::export!(stdcall, rw_c49950(input_bits: u32) -> f32 {
    unsafe {
        const TABLE_COUNT_VA: u32 = 0x016d_8db0;
        const SIGN_MASK_VA: u32 = 0x00fe_8d1c;
        const ROUNDING_LIMIT_VA: u32 = 0x00fe_8cf8;
        const ONE_VA: u32 = 0x00fe_88e8;
        const TABLE_VA: u32 = 0x016d_8d60;
        const I32_LOWER: f32 = -2_147_483_648.0;
        const I32_UPPER: f32 = 2_147_483_648.0;

        #[inline(always)]
        unsafe fn read_image_word(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        fn add(lhs: f32, rhs: f32) -> f32 {
            core::hint::black_box(lhs) + core::hint::black_box(rhs)
        }

        #[inline(always)]
        fn subtract(lhs: f32, rhs: f32) -> f32 {
            core::hint::black_box(lhs) - core::hint::black_box(rhs)
        }

        #[inline(always)]
        fn multiply(lhs: f32, rhs: f32) -> f32 {
            core::hint::black_box(lhs) * core::hint::black_box(rhs)
        }

        #[inline(always)]
        fn divide(lhs: f32, rhs: f32) -> f32 {
            core::hint::black_box(lhs) / core::hint::black_box(rhs)
        }

        #[inline(always)]
        fn cvttss2si(value: f32) -> i32 {
            if !value.is_finite() || value >= I32_UPPER || value < I32_LOWER {
                i32::MIN
            } else {
                // The explicit checks above bound this value to the x86
                // cvttss2si range before Rust performs the truncating cast.
                value as i32
            }
        }

        let input = f32::from_bits(input_bits);
        let integer_count = read_image_word(TABLE_COUNT_VA) as i32;
        let count = integer_count as f32;
        let one = f32::from_bits(read_image_word(ONE_VA));
        let sign_mask = read_image_word(SIGN_MASK_VA);
        let rounding_limit = f32::from_bits(read_image_word(ROUNDING_LIMIT_VA));

        // Preserve the source order of the count-times-phase multiplication.
        let scaled = multiply(count, input);
        let sign_bits = scaled.to_bits() & sign_mask;
        let magnitude = f32::from_bits(scaled.to_bits() ^ sign_bits);
        let rounding_bias_bits =
            (if magnitude < rounding_limit { rounding_limit.to_bits() } else { 0 }) | sign_bits;
        let rounding_bias = f32::from_bits(rounding_bias_bits);
        let shifted = add(scaled, rounding_bias);
        let rounded = subtract(shifted, rounding_bias);

        let rounding_error = subtract(rounded, scaled);
        let signed_zero = f32::from_bits(sign_bits);
        // CMPNLESS is true when the error is not below signed zero, including
        // unordered NaN comparisons; the original then subtracts exactly one.
        let correction = if rounding_error < signed_zero { 0.0 } else { one };
        let table_index_float = subtract(rounded, correction);
        let table_index = cvttss2si(table_index_float);

        let table_base = lf_checker_rt::relocated(TABLE_VA);
        let table_displacement = (table_index as u32).wrapping_mul(4);
        let table_slot = table_base.wrapping_add(table_displacement);
        let first_value = f32::from_bits(read_word(table_slot));
        let next_value = f32::from_bits(read_word(table_slot.wrapping_add(4)));

        let index_as_float = table_index as f32;
        let fraction = subtract(scaled, index_as_float);
        let inverse_count = divide(one, count);
        let interval = subtract(next_value, first_value);
        let normalized_fraction = multiply(fraction, inverse_count);
        let value_delta = multiply(normalized_fraction, interval);
        add(value_delta, first_value)
    }
});
