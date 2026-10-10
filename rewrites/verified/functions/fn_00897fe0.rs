// original: 0x00897FE0 input_slot_toggle_bit40_0x5f

/// Toggles bit 0x40 of the flag byte at +0x5F of the slot record of the input object `this`
/// when bit 0 of the argument is set; the byte is otherwise unchanged.
///
/// The argument's low byte is shifted left by six and xored with the old flag byte; the bit 0x40
/// of that result, when set, is xored into the flag byte. Thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00897fe0(this: u32, value: u32) -> () {
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
        const FLAG_FIELD: u32 = 0x5F;
        const TOGGLE_MASK: u8 = 0x40;
        const TOGGLE_SHIFT: u32 = 6;
        // Bit 'mask' of the flag byte is xored with the low bit of the argument
        // shifted into that position: the xor of the shifted argument and the old
        // flag byte, masked to the bit, is xored into the byte.
        let flag = record.wrapping_add(FLAG_FIELD) as *mut u8;
        let old = flag.read();
        let argument_bits = (value as u8).wrapping_shl(TOGGLE_SHIFT);
        let change = (argument_bits ^ old) & TOGGLE_MASK;
        flag.write(old ^ change);
    }
});
