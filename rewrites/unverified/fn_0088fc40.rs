// original: 0x0088FC40 audio_compute_voice_mix
/// Rebuilds the mix values from two selected voice rows and optional scalar
/// inputs. Sentinel selectors skip their row; flag bits gate the first-row
/// float and integer contributions. It stores the mixed floats, a signed
/// angle accumulated with the original -1 sentinel and reduced by signed
/// remainder, a wrapping 16-bit sum, and unsigned minimum and maximum values.
/// Image-backed strides, row-table entries, and the scaling constant are read
/// from the mapped image for each call.
lf_checker_rt::export!(thiscall, rw_0088fc40(this: u32) -> u32 {
    const GROUP_STRIDE: u32 = 0x6f40;
    const PRIMARY_ROW_OFFSET: u32 = 0x6f10;
    const SECONDARY_ROW_OFFSET: u32 = 0x6f14;
    const PRIMARY_STRIDE_VA: u32 = 0x0115d964;
    const SECONDARY_STRIDE_VA: u32 = 0x0115d968;
    const ROW_BASE_VA: u32 = 0x0115d988;
    const ROW_TABLE_VA: u32 = 0x0115f80c;
    const SCALING_CONSTANT_VA: u32 = 0x00fe8bb0;
    const MODULUS: i32 = 360;
    const DEFAULT_MINIMUM: u32 = 0x5d5c;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_i16(address: u32) -> i16 {
        unsafe { (address as *const i16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    #[inline(always)]
    unsafe fn image_u32(file_va: u32) -> u32 {
        unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn row_address(
        table_base: u32,
        group_index: u8,
        selector: u8,
        stride: u32,
        field_offset: u32,
    ) -> u32 {
        table_base
            .wrapping_add((group_index as u32).wrapping_mul(GROUP_STRIDE))
            .wrapping_add((selector as u32).wrapping_mul(stride))
            .wrapping_add(field_offset)
    }

    #[inline(always)]
    unsafe fn row_table_word(row: u32) -> u32 {
        let row_index = unsafe { read_u8(row.wrapping_add(0xe7)) } & 7;
        unsafe { image_u32(ROW_TABLE_VA.wrapping_add((row_index as u32).wrapping_mul(8))) }
    }

    #[inline(always)]
    fn ordered_add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    #[inline(always)]
    fn ordered_mul(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    #[inline(always)]
    fn add_unless_sentinel(total: i32, value: i32) -> i32 {
        if total == -1 {
            value
        } else if value == -1 {
            total
        } else {
            total.wrapping_add(value)
        }
    }

    #[inline(always)]
    fn cvtt_f32_to_i32(value: f32) -> i32 {
        if !value.is_finite() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
            i32::MIN
        } else {
            // The preceding checks bound the value to the instruction's valid
            // signed 32-bit truncation range before Rust's cast is used.
            value as i32
        }
    }

    #[inline(always)]
    fn fistp_qword_low(value: f32) -> u32 {
        let wide_value = value as f64;
        if !value.is_finite()
            || wide_value >= 9_223_372_036_854_775_808.0
            || wide_value <= -9_223_372_036_854_775_808.0
        {
            0
        } else {
            // The value is strictly inside the signed 64-bit range. Truncating
            // first, then retaining the low word, matches the x87 store.
            (value as i64) as u32
        }
    }

    unsafe {
        let flags = unsafe { read_u8(this.wrapping_add(8)) };
        let group_index = unsafe { read_u8(this.wrapping_add(0x40)) };
        let primary_selector = unsafe { read_u8(this.wrapping_add(5)) };
        let secondary_selector = unsafe { read_u8(this.wrapping_add(4)) };
        let table_base = unsafe { image_u32(ROW_BASE_VA) };

        let primary_row = if primary_selector == 0xff {
            None
        } else {
            let primary_stride = unsafe { image_u32(PRIMARY_STRIDE_VA) };
            let slot = unsafe {
                row_address(
                    table_base,
                    group_index,
                    primary_selector,
                    primary_stride,
                    PRIMARY_ROW_OFFSET,
                )
            };
            Some(unsafe { read_u32(slot) })
        };

        let (primary_float_a, primary_float_b, primary_angle, primary_sum, minimum, maximum) =
            if let Some(row) = primary_row {
                (
                    unsafe { read_f32(row.wrapping_add(0x28)) },
                    unsafe { read_f32(row.wrapping_add(0x2c)) },
                    unsafe { read_i16(row.wrapping_add(0x34)) as i32 },
                    unsafe { read_i16(row.wrapping_add(0x36)) as i32 },
                    unsafe { read_u16(row.wrapping_add(0x30)) as u32 },
                    unsafe { read_u16(row.wrapping_add(0x32)) as u32 },
                )
            } else {
                (0.0, 0.0, -1, 0, DEFAULT_MINIMUM, 0)
            };

        let mut first_float = unsafe { read_f32(this.wrapping_add(0x24)) };
        if flags & 1 == 0 {
            first_float = ordered_add(first_float, primary_float_a);
        }

        let secondary_row = if secondary_selector == 0xff {
            None
        } else {
            let secondary_stride = unsafe { image_u32(SECONDARY_STRIDE_VA) };
            let slot = unsafe {
                row_address(
                    table_base,
                    group_index,
                    secondary_selector,
                    secondary_stride,
                    SECONDARY_ROW_OFFSET,
                )
            };
            Some(unsafe { read_u32(slot) })
        };

        let secondary_table_word = secondary_row.map(|row| unsafe { row_table_word(row) });
        if let (Some(row), Some(table_word)) = (secondary_row, secondary_table_word) {
            let first_offset = table_word.wrapping_add(2).wrapping_shl(5);
            let second_offset = table_word.wrapping_shl(5);
            let second_row_float = unsafe { read_f32(row.wrapping_add(first_offset).wrapping_add(0x40)) };
            first_float = ordered_add(second_row_float, first_float);
            let secondary_float = unsafe { read_f32(row.wrapping_add(second_offset).wrapping_add(0x44)) };
            let output_second = ordered_add(primary_float_b, secondary_float);
            unsafe {
                (this.wrapping_add(0x2c) as *mut u32).write_unaligned(output_second.to_bits());
            }
        }

        if secondary_row.is_none() {
            unsafe {
                (this.wrapping_add(0x2c) as *mut u32)
                    .write_unaligned(primary_float_b.to_bits());
            }
        }

        let first_pointer = unsafe { read_u32(this.wrapping_add(0x0c)) };
        if first_pointer != 0 {
            first_float = ordered_add(first_float, unsafe { read_f32(first_pointer) });
        }
        let output_first = ordered_add(
            unsafe { read_f32(this.wrapping_add(0x1c)) },
            first_float,
        );
        unsafe {
            (this.wrapping_add(0x28) as *mut u32).write_unaligned(output_first.to_bits());
        }

        let mut angle = if flags & 4 == 0 { primary_angle } else { -1 };
        let angle_pointer = unsafe { read_u32(this.wrapping_add(0x14)) };
        if angle_pointer != 0 {
            angle = add_unless_sentinel(angle, cvtt_f32_to_i32(unsafe { read_f32(angle_pointer) }));
        }
        angle = add_unless_sentinel(angle, unsafe { read_i16(this.wrapping_add(0x0a)) as i32 });
        let secondary_angle = if let (Some(row), Some(table_word)) = (secondary_row, secondary_table_word) {
            let offset = table_word.wrapping_shl(5);
            unsafe { read_i16(row.wrapping_add(offset).wrapping_add(0x52)) as i32 }
        } else {
            -1
        };
        angle = add_unless_sentinel(angle, secondary_angle);
        angle = add_unless_sentinel(angle, unsafe { read_i16(this.wrapping_add(0x7c)) as i32 });
        if angle != -1 {
            angle %= MODULUS;
        }
        unsafe { write_u16(this.wrapping_add(0x34), angle as u16) };

        let mut integer_sum = unsafe { read_i16(this.wrapping_add(0x22)) as i32 };
        if flags & 2 == 0 {
            integer_sum = integer_sum.wrapping_add(primary_sum);
        }
        if let (Some(row), Some(table_word)) = (secondary_row, secondary_table_word) {
            let offset = table_word.wrapping_shl(5);
            integer_sum = integer_sum.wrapping_add(unsafe {
                read_i16(row.wrapping_add(offset).wrapping_add(0x50)) as i32
            });
        }
        let scaled_pointer = unsafe { read_u32(this.wrapping_add(0x10)) };
        if scaled_pointer != 0 {
            let source = unsafe { read_f32(scaled_pointer) };
            let scale = unsafe { lf_checker_rt::global::<f32>(SCALING_CONSTANT_VA).read_unaligned() };
            integer_sum = integer_sum.wrapping_add(cvtt_f32_to_i32(ordered_mul(source, scale)));
        }
        integer_sum = integer_sum.wrapping_add(unsafe { read_u16(this.wrapping_add(0x20)) as i32 });
        unsafe { write_u16(this.wrapping_add(0x36), integer_sum as u16) };

        let mut minimum_value = minimum;
        let mut maximum_value = maximum;
        if let (Some(row), Some(table_word)) = (secondary_row, secondary_table_word) {
            let offset = table_word.wrapping_shl(5);
            let secondary_minimum = unsafe {
                read_u16(row.wrapping_add(offset).wrapping_add(0x58)) as u32
            };
            minimum_value = minimum_value.min(secondary_minimum);
            let secondary_maximum = unsafe {
                read_u16(row.wrapping_add(offset).wrapping_add(0x5a)) as u32
            };
            maximum_value = maximum_value.max(secondary_maximum);
        }
        let minimum_pointer = unsafe { read_u32(this.wrapping_add(0x18)) };
        if minimum_pointer != 0 {
            let converted = fistp_qword_low(unsafe { read_f32(minimum_pointer) });
            minimum_value = minimum_value.min(converted);
        }
        unsafe {
            write_u16(this.wrapping_add(0x30), minimum_value as u16);
            write_u16(this.wrapping_add(0x32), maximum_value as u16);
        }
    }
    0
});
