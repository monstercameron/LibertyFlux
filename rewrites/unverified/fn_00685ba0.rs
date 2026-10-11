// original: 0x00685BA0 find_frame_record_by_key

/// Search the sorted frame-record pointer array stored in `this`. Its count
/// is the unsigned 16-bit value at byte 0x10 and the array pointer is at byte
/// 0x0c. Each record's unsigned 24-bit key combines byte 5 with the word at
/// byte 6. The requested key is formed from the low byte of `key_high` and
/// low word of `key_low`; an unsigned binary search returns the matching
/// record pointer or zero. This thiscall method pops two stack words.
lf_checker_rt::export!(thiscall, rw_00685ba0(this: u32, key_high: u32, key_low: u32) -> u32 {
    unsafe {
        const ARRAY_POINTER_OFFSET: u32 = 0x0C;
        const COUNT_OFFSET: u32 = 0x10;
        const RECORD_HIGH_OFFSET: u32 = 5;
        const RECORD_LOW_OFFSET: u32 = 6;

        let key = ((key_high & 0xFF) << 16) | (key_low & 0xFFFF);
        let records = crate::read_word(this.wrapping_add(ARRAY_POINTER_OFFSET));
        let count = crate::read_halfword(this.wrapping_add(COUNT_OFFSET)) as i32;
        let mut low = 0i32;
        let mut high = count.wrapping_sub(1);

        while low <= high {
            let middle = low.wrapping_add(high) >> 1;
            let record = crate::read_word(records.wrapping_add((middle as u32).wrapping_mul(4)));
            let record_key = ((crate::read_byte(record.wrapping_add(RECORD_HIGH_OFFSET)) as u32) << 16)
                | crate::read_halfword(record.wrapping_add(RECORD_LOW_OFFSET)) as u32;
            if record_key == key {
                return record;
            }
            if record_key <= key {
                low = middle.wrapping_add(1);
            } else {
                high = middle.wrapping_sub(1);
            }
        }
        0
    }
});
