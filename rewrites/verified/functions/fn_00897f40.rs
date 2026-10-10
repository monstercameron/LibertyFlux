// original: 0x00897F40 input_slot_store_truncated_word_0x58

/// Stores a float argument, truncated toward zero to a 64-bit integer, as its low 16 bits at
/// +0x58 of the slot record of the input object `this`.
///
/// The record is chosen as for the other slot accessors. The conversion is the 64-bit
/// truncating store of the floating-point unit: NaN and values outside the signed 64-bit range
/// give the indefinite integer 0x8000000000000000, whose low word is zero. The control word of
/// the floating-point unit is saved and restored by the original; the rewrite does not change it.
lf_checker_rt::export!(thiscall, rw_00897f40(this: u32, value: u32) -> () {
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
        const FIELD: u32 = 0x58;
        // 64-bit truncating conversion: NaN and out-of-range values give the
        // 64-bit indefinite integer, whose low word is zero.
        let v = f64::from(f32::from_bits(value));
        let truncated: i64 = if v.is_nan() || v >= 9.223372036854775808e18 || v < -9.223372036854775808e18 {
            i64::MIN
        } else {
            v as i64
        };
        (record.wrapping_add(FIELD) as *mut u16).write_unaligned(truncated as u16);
    }
});
