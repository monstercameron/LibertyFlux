// original: 0x00897E80 input_slot_read_float_0x40

/// Reads a float from the slot record of the input object `this`, at a fixed
/// field 64 bytes past the record.
///
/// The record is chosen by the byte at +0xE7 (low three bits) through the slot table
/// (words at 0x115F808 with stride 8): the table word for that index is taken, two is
/// added to it, and the sum is shifted left by five to give the offset of the field.
/// Thiscall with no stack arguments. The float is returned in ST0 as the original does;
/// the value is read bit for bit with no arithmetic.
lf_checker_rt::export!(thiscall, rw_00897e80(this: u32) -> f32 {
    unsafe {
        const INDEX_BYTE: u32 = 0xE7;
        const INDEX_MASK: u8 = 0x07;
        const SLOT_TABLE: u32 = 0x0115_F808;
        const SLOT_STRIDE: u32 = 8;
        const RECORD_SHIFT: u32 = 5;
        let index = u32::from(((this.wrapping_add(INDEX_BYTE)) as *const u8).read() & INDEX_MASK);
        let table_word = lf_checker_rt::global::<u32>(SLOT_TABLE.wrapping_add(index.wrapping_mul(SLOT_STRIDE))).read();
        // The field sits two table steps further on, which is 64 bytes past the record.
        let offset = table_word.wrapping_add(2).wrapping_shl(RECORD_SHIFT);
        f32::from_bits((this.wrapping_add(offset) as *const u32).read_unaligned())
    }
});
