// original: 0x00D3E160 peds_task_initialize_record_tables

/// Initialize two contiguous tables of three-word records on a task object.
///
/// The `thiscall` receiver points to the first table. Thirty-two records are
/// initialized at byte offsets `0` through `0x174`, followed by ten records
/// at offsets `0x180` through `0x1f4`. Each record receives the invalid-index
/// marker `u32::MAX` and two zero words. The routine reads no object fields,
/// calls no helpers, and returns the receiver plus `0x188` in EAX after the
/// first table loop.
lf_checker_rt::export!(thiscall, rw_00d3e160(this: u32) -> u32 {
    const FIRST_TABLE_RECORDS: u32 = 32;
    const SECOND_TABLE_RECORDS: u32 = 10;
    const RECORD_STRIDE: u32 = 12;
    const SECOND_TABLE_OFFSET: u32 = 0x180;
    const RETURN_CURSOR_OFFSET: u32 = 0x188;
    const INVALID_INDEX: u32 = u32::MAX;

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        for record_index in 0..FIRST_TABLE_RECORDS {
            let record = this.wrapping_add(record_index * RECORD_STRIDE);
            write_word(record, INVALID_INDEX);
            write_word(record.wrapping_add(4), 0);
            write_word(record.wrapping_add(8), 0);
        }

        for record_index in 0..SECOND_TABLE_RECORDS {
            let record = this
                .wrapping_add(SECOND_TABLE_OFFSET)
                .wrapping_add(record_index * RECORD_STRIDE);
            write_word(record, INVALID_INDEX);
            write_word(record.wrapping_add(4), 0);
            write_word(record.wrapping_add(8), 0);
        }

        this.wrapping_add(RETURN_CURSOR_OFFSET)
    }
});
