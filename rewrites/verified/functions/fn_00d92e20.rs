// original: 0x00D92E20 mainloop_timing_normalize_eight_byte_records
lf_checker_rt::export!(thiscall, rw_00d92e20(array_owner: u32, count: u32) -> u32 {
    const ELEMENT_STRIDE: u32 = 8;
    const FIRST_WORD: u32 = 0xffff_0fff;
    const SECOND_KEEP_MASK: u32 = 0xefff_ffff;
    const SECOND_LOW_BITS: u32 = 0x0fff_ffff;

    let records = unsafe { request_timing_array_item(array_owner, count, ELEMENT_STRIDE) };
    if records == 0 {
        return 0;
    }
    let last_index = count.wrapping_sub(1) as i32;
    if last_index >= 0 {
        let mut index = 0u32;
        loop {
            let record = records.wrapping_add(index.wrapping_mul(ELEMENT_STRIDE));
            let second = unsafe { read_timing_word(record.wrapping_add(4)) };
            unsafe {
                (record as *mut u32).write_unaligned(FIRST_WORD);
                (record.wrapping_add(4) as *mut u32).write_unaligned(
                    (second & SECOND_KEEP_MASK) | SECOND_LOW_BITS,
                );
            }
            if index == last_index as u32 {
                break;
            }
            index = index.wrapping_add(1);
        }
    }
    records
});
