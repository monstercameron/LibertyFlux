// original: 0x00A223D0 cam_follow_ped_blur_cap_store

/// Copy four four-float input records into output globals and publish rates.
///
/// The cdecl routine accepts four pointers to four-word float records, then a
/// float converted by x87 `FISTP` to a signed 64-bit integer with truncation
/// toward zero. Only the low word is published at global `+0x103c00c`; invalid
/// or out-of-range conversions publish zero, matching the indefinite 64-bit
/// result's low word. The four record pointers are copied in the order of the
/// output blocks beginning at `+0x103c010`, `+0x103bffc`, `+0x103c020` and
/// `+0x103c030`. Two additional float arguments are copied to `+0x103c040`
/// and `+0x103c044`. The final float is clamped to the minimum `0.15` only
/// when the ordered comparison finds it below that threshold; unordered NaN
/// values retain their input bits. The function returns the converted low word.
lf_checker_rt::export!(cdecl, rw_00A223D0(
    first_record: u32,
    second_record: u32,
    third_record: u32,
    fourth_record: u32,
    conversion_input: f32,
    second_rate: f32,
    third_rate: f32,
    clamp_rate: f32
) -> u32 {
    unsafe {
        let record_outputs: [(u32, u32); 4] = [
            (second_record, 0x0103_C010),
            (first_record, 0x0103_BFFC),
            (third_record, 0x0103_C020),
            (fourth_record, 0x0103_C030),
        ];
        const CONVERSION_OUTPUT: u32 = 0x0103_C00C;
        const SECOND_RATE_OUTPUT: u32 = 0x0103_C040;
        const THIRD_RATE_OUTPUT: u32 = 0x0103_C044;
        const CLAMP_RATE_OUTPUT: u32 = 0x0103_C048;
        const MINIMUM_RATE_BITS: u32 = 0x3E19_999A;
        const MINIMUM_RATE: f32 = f32::from_bits(MINIMUM_RATE_BITS);

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_global(file_va: u32, value: u32) {
            unsafe { lf_checker_rt::global::<u32>(file_va).write_unaligned(value) }
        }
        #[inline(always)]
        fn fistp_qword_low_word(value: f32) -> u32 {
            const I64_MIN: f64 = -9_223_372_036_854_775_808.0;
            const I64_LIMIT: f64 = 9_223_372_036_854_775_808.0;

            let extended = f64::from(value);
            // This bound matches signed 64-bit FISTP. Invalid values, values
            // below -2^63, and values at or above 2^63 store the indefinite
            // qword, whose low dword is zero. The remaining cast is bounded.
            if !extended.is_finite() || extended < I64_MIN || extended >= I64_LIMIT {
                0
            } else {
                (extended as i64) as u32
            }
        }

        for (source, destination) in record_outputs {
            let mut word_index = 0u32;
            while word_index < 4 {
                let value = read_word(source.wrapping_add(word_index * 4));
                write_global(destination + word_index * 4, value);
                word_index += 1;
            }
        }

        let converted = fistp_qword_low_word(conversion_input);
        write_global(CONVERSION_OUTPUT, converted);
        write_global(SECOND_RATE_OUTPUT, second_rate.to_bits());
        write_global(THIRD_RATE_OUTPUT, third_rate.to_bits());
        let clamped_rate = if core::hint::black_box(MINIMUM_RATE)
            > core::hint::black_box(clamp_rate)
        {
            MINIMUM_RATE_BITS
        } else {
            clamp_rate.to_bits()
        };
        write_global(CLAMP_RATE_OUTPUT, clamped_rate);
        converted
    }
});
