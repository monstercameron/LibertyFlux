// original: 0x00897F10 input_slot_store_scaled_byte_0x5d

/// Stores a float argument, scaled by 255.0, truncated toward zero, as one byte at
/// +0x5D of the slot record of the input object `this`.
///
/// The record is chosen as for the other slot accessors (index byte at +0xE7, table at
/// 0x115F808, shifted left by five). The argument is a float passed on the stack and is
/// multiplied by 255.0 in single precision. Truncation toward zero follows the 32-bit conversion of the processor:
/// NaN and values outside the signed 32-bit range give 0x80000000, whose low byte is zero.
lf_checker_rt::export!(thiscall, rw_00897f10(this: u32, value: u32) -> () {
    unsafe {
        const INDEX_BYTE: u32 = 0xE7;
        const INDEX_MASK: u8 = 0x07;
        const SLOT_TABLE: u32 = 0x0115_F808;
        const SLOT_STRIDE: u32 = 8;
        const RECORD_SHIFT: u32 = 5;
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        // The record is chosen by the low three bits of the index byte, and its
        // offset is the table word for that index shifted left by five.
        let index = u32::from(read_u8(this.wrapping_add(INDEX_BYTE)) & INDEX_MASK);
        let table_word = lf_checker_rt::global::<u32>(SLOT_TABLE.wrapping_add(index.wrapping_mul(SLOT_STRIDE))).read();
        let record = this.wrapping_add(table_word.wrapping_shl(RECORD_SHIFT));
        const FIELD: u32 = 0x5D;
        const SCALE_BITS: u32 = 0x437F_0000; // 255.0
        // 32-bit truncating conversion: NaN and out-of-range values give the 32-bit
        // indefinite integer, whose low byte is zero.
        fn truncate_to_i32(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        let scaled = f32::from_bits(value) * f32::from_bits(SCALE_BITS);
        (record.wrapping_add(FIELD) as *mut u8).write(truncate_to_i32(scaled) as u8);
    }
});
