// original: 0x00D3E270 task_registry_raise_reference_limit (proposed)
/// Finds the task record whose identifier matches `record_id`, increases its
/// reference count, and raises its signed high-water value when `new_limit`
/// is greater. The object holds ten 12-byte records beginning at offset
/// 0x180; each record stores the identifier, reference count, and signed
/// high-water value. The thiscall method takes two 32-bit stack arguments.
lf_checker_rt::export!(thiscall, rw_d3e270(this: u32, record_id: u32, new_limit: i32) -> () {
    const RECORDS_OFFSET: usize = 0x180;
    const RECORD_STRIDE: usize = 12;
    const RECORD_COUNT: usize = 10;
    const REFCOUNT_OFFSET: usize = 4;
    const HIGH_WATER_OFFSET: usize = 8;

    unsafe {
        let object = this as *mut u8;
        for index in 0..RECORD_COUNT {
            let record = object.add(RECORDS_OFFSET + index * RECORD_STRIDE);
            if (record as *const u32).read_unaligned() == record_id {
                let refcount = record.add(REFCOUNT_OFFSET) as *mut u32;
                refcount.write_unaligned(refcount.read_unaligned().wrapping_add(1));

                let high_water = record.add(HIGH_WATER_OFFSET) as *mut i32;
                let previous = high_water.read_unaligned();
                if new_limit > previous {
                    high_water.write_unaligned(new_limit);
                }
                break;
            }
        }
    }
});
