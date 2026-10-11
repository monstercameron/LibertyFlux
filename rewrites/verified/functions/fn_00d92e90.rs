// original: 0x00D92E90 mainloop_timing_clear_record_coordinates
lf_checker_rt::export!(thiscall, rw_00d92e90(array_owner: u32, count: u32) -> u32 {
    const ELEMENT_STRIDE: u32 = 0x28;
    const FIRST_FIELD: u32 = 0x0c;
    const SECOND_FIELD: u32 = 0x24;

    let records = unsafe { request_timing_array_item(array_owner, count, ELEMENT_STRIDE) };
    if records == 0 {
        return 0;
    }
    let last_index = count.wrapping_sub(1) as i32;
    if last_index >= 0 {
        let mut index = 0u32;
        loop {
            let record = records.wrapping_add(index.wrapping_mul(ELEMENT_STRIDE));
            unsafe {
                (record.wrapping_add(FIRST_FIELD) as *mut u32).write_unaligned(0);
                (record.wrapping_add(SECOND_FIELD) as *mut u32).write_unaligned(0);
            }
            if index == last_index as u32 {
                break;
            }
            index = index.wrapping_add(1);
        }
    }
    records
});
