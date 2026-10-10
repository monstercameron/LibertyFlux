// original: 0x00897EF0 input_slot_store_byte_0x5d

/// Stores the low byte of the argument at the slot record of the input object `this`.
///
/// `this` is the input object. Its byte at +0xE7 (low three bits) selects one of
/// eight table words (stride 8, starting at 0x115F808); the table word is shifted
/// left by five and added to `this` to give the record. The argument is a stack
/// word, the value stored is its low 8 bits, written at record + 0x5d.
/// Thiscall: `this` in ECX, the one argument on the stack, callee cleans up.
/// The argument is compared as raw bits; no float arithmetic is done.
lf_checker_rt::export!(thiscall, rw_00897ef0(this: u32, value: u32) -> () {
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
        (record.wrapping_add(FIELD) as *mut u8).write(value as u8);
    }
});
