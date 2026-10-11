// original: 0x00DC0920 task_record_insert

/// Insert a 16-byte record slot at `index` in a task record array.
///
/// The receiver stores the array pointer at byte offset 0 and a 16-bit
/// element count at byte offset 4. Existing records are 16 bytes each. If
/// `index` is below the current count, records from the end down through
/// `index` move one slot higher. The count then wraps as a 16-bit value and
/// the function returns the address of the requested slot. The checked
/// input range uses nonnegative indices that fit the allocated array.
lf_checker_rt::export!(thiscall, rw_00DC0920(this: u32, index: u32) -> u32 {
    unsafe {
        const ARRAY_POINTER: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const RECORD_BYTES: u32 = 16;
        const WORDS_PER_RECORD: u32 = RECORD_BYTES / 4;

        let array = (this.wrapping_add(ARRAY_POINTER) as *const u32).read_unaligned();
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;

        if (count as i32) > (index as i32) {
            let mut source_index = count;
            let copy_count = count.wrapping_sub(index);
            let mut remaining = copy_count;
            while remaining != 0 {
                source_index = source_index.wrapping_sub(1);
                let source = array.wrapping_add(source_index.wrapping_mul(RECORD_BYTES));
                let destination = source.wrapping_add(RECORD_BYTES);
                let mut word = 0;
                while word < WORDS_PER_RECORD {
                    let value = (source.wrapping_add(word * 4) as *const u32).read_unaligned();
                    (destination.wrapping_add(word * 4) as *mut u32).write_unaligned(value);
                    word += 1;
                }
                remaining -= 1;
            }
        }

        (this.wrapping_add(COUNT) as *mut u16)
            .write_unaligned((count as u16).wrapping_add(1));
        array.wrapping_add(index.wrapping_mul(RECORD_BYTES))
    }
});
