// original: 0x00BB87B0 append_counted_record_with_two_strings

/// Append a record and its two null-terminated strings to a counted table.
/// This x86 thiscall routine receives the table in ECX, a four-word source
/// record in the first stack slot, and a 64-byte string pair in the second.
/// The signed count at table offset zero is compared with eight; counts below
/// eight, including negative values, are processed, while eight or more return
/// zero without writing. Four source words are copied verbatim to the row at
/// `16 * (count + 29)`, equivalently `0x1d0 + 16 * count`. The strings are copied byte-for-byte to
/// `56 * count + 4` and `56 * count + 0x24`, using the source's first and
/// second 32-byte halves. The count is incremented and AL returns one. The
/// 32-bit layout and wrapping offsets preserve the original table addressing.
lf_checker_rt::export!(thiscall, rw_00bb87b0(this: u32, source_record: u32, text_pair: u32) -> u8 {
    unsafe {
        const COUNT_OFFSET: u32 = 0;
        const MAX_RECORDS: i32 = 8;
        const ROW_INDEX_BIAS: i32 = 29;
        const ROW_STRIDE: i32 = 16;
        const TEXT_STRIDE: i32 = 56;
        const FIRST_TEXT_OFFSET: u32 = 4;
        const SECOND_TEXT_OFFSET: u32 = 0x24;
        const SOURCE_SECOND_TEXT_OFFSET: u32 = 0x20;

        let count = (this.wrapping_add(COUNT_OFFSET) as *const i32).read_unaligned();
        if count >= MAX_RECORDS {
            return 0;
        }

        let row_offset = count.wrapping_add(ROW_INDEX_BIAS).wrapping_mul(ROW_STRIDE) as u32;
        let row = this.wrapping_add(row_offset);
        let mut word_index = 0u32;
        while word_index < 4 {
            let word = (source_record.wrapping_add(word_index * 4) as *const u32).read_unaligned();
            (row.wrapping_add(word_index * 4) as *mut u32).write_unaligned(word);
            word_index += 1;
        }

        let text_offset = count.wrapping_mul(TEXT_STRIDE) as u32;
        let first_destination = this.wrapping_add(text_offset).wrapping_add(FIRST_TEXT_OFFSET);
        let second_destination = this.wrapping_add(text_offset).wrapping_add(SECOND_TEXT_OFFSET);
        let mut byte_index = 0u32;
        loop {
            let byte = (text_pair.wrapping_add(byte_index) as *const u8).read();
            (first_destination.wrapping_add(byte_index) as *mut u8).write(byte);
            byte_index += 1;
            if byte == 0 {
                break;
            }
        }

        byte_index = 0;
        loop {
            let byte = (text_pair.wrapping_add(SOURCE_SECOND_TEXT_OFFSET).wrapping_add(byte_index) as *const u8).read();
            (second_destination.wrapping_add(byte_index) as *mut u8).write(byte);
            byte_index += 1;
            if byte == 0 {
                break;
            }
        }

        (this as *mut i32).write_unaligned(count.wrapping_add(1));
        1
    }
});
