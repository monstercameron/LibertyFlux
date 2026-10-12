// original: 0x005E2C80 apply_color_attributes

/// Apply each entry in an attribute table to the caller's color output.
///
/// The thiscall receiver contains a table pointer at offset `0x00` and an
/// unsigned 16-bit entry count at `0x04`; each table entry occupies `0x10`
/// bytes. Entry kind 7 copies its word at `0x04` to output offset `0xBC`.
/// Kind 9 decrements its word at `0x0C` only when it is exactly one, then
/// converts the float word at `0x04` to a signed integer and stores that result
/// back. The kind 9 value is copied to output offset `0xC0`. Other kinds do
/// nothing. The one stack argument is the output pointer, and EAX returns the
/// original entry count. The float conversion truncates toward zero and returns
/// `i32::MIN` for NaN or values outside the signed 32-bit range, as x86
/// `CVTTSS2SI` does.
lf_checker_rt::export!(thiscall, rw_005e2c80(this: *mut u8, output: *mut u8) -> u32 {
    const TABLE_POINTER_OFFSET: usize = 0x00;
    const ENTRY_COUNT_OFFSET: usize = 0x04;
    const ENTRY_STRIDE: usize = 0x10;
    const ENTRY_VALUE_OFFSET: usize = 0x04;
    const ENTRY_REMAINING_OFFSET: usize = 0x0c;
    const COLOR_OUTPUT_OFFSET: usize = 0xbc;
    const ALPHA_OUTPUT_OFFSET: usize = 0xc0;
    const KIND_COLOR: u32 = 7;
    const KIND_ALPHA: u32 = 9;
    const MAX_SIGNED_FLOAT: f32 = 2_147_483_648.0;
    const MIN_SIGNED_FLOAT: f32 = -2_147_483_648.0;

    fn cvttss2si(value: f32) -> i32 {
        if value.is_nan() || value >= MAX_SIGNED_FLOAT || value < MIN_SIGNED_FLOAT {
            i32::MIN
        } else {
            value.trunc() as i32
        }
    }

    unsafe {
        let entries = this
            .add(TABLE_POINTER_OFFSET)
            .cast::<*mut u8>()
            .read_unaligned();
        let entry_count = this
            .add(ENTRY_COUNT_OFFSET)
            .cast::<u16>()
            .read_unaligned() as u32;

        for entry_index in 0..entry_count {
            let entry = entries.add(entry_index as usize * ENTRY_STRIDE);
            let kind = entry.cast::<u32>().read_unaligned();
            if kind == KIND_COLOR {
                let color = entry.add(ENTRY_VALUE_OFFSET).cast::<u32>().read_unaligned();
                output
                    .add(COLOR_OUTPUT_OFFSET)
                    .cast::<u32>()
                    .write_unaligned(color);
            } else if kind == KIND_ALPHA {
                let remaining = entry
                    .add(ENTRY_REMAINING_OFFSET)
                    .cast::<u32>()
                    .read_unaligned();
                if remaining == 1 {
                    entry
                        .add(ENTRY_REMAINING_OFFSET)
                        .cast::<u32>()
                        .write_unaligned(0);
                    let float_bits = entry.add(ENTRY_VALUE_OFFSET).cast::<u32>().read_unaligned();
                    let converted = cvttss2si(f32::from_bits(float_bits));
                    entry
                        .add(ENTRY_VALUE_OFFSET)
                        .cast::<u32>()
                        .write_unaligned(converted as u32);
                }
                let alpha = entry.add(ENTRY_VALUE_OFFSET).cast::<u32>().read_unaligned();
                output
                    .add(ALPHA_OUTPUT_OFFSET)
                    .cast::<u32>()
                    .write_unaligned(alpha);
            }
        }

        entry_count
    }
});
