// original: 0x00685C10 find_frame_record_and_insertion_index

/// Search the sorted frame-record pointer array stored in `this` using its
/// unsigned 16-bit count at byte 0x10 and array pointer at byte 0x0c. The
/// unsigned record key combines byte 5 with the word at byte 6; the requested
/// key combines the low byte of `key_high` and low word of `key_low`. A match
/// returns its record pointer and writes the matching index. A miss returns
/// zero and writes the lower-bound insertion index through `insertion_index`.
/// This thiscall method pops three stack words.
lf_checker_rt::export!(thiscall, rw_00685c10(this: u32, key_high: u32, key_low: u32, insertion_index: u32) -> u32 {
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
            crate::write_word(insertion_index, middle as u32);
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
        crate::write_word(insertion_index, low as u32);
        0
    }
});
