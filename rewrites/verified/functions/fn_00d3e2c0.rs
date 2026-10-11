// original: 0x00D3E2C0 task_registry_lower_reference_count (proposed)
/// Finds the task record whose identifier matches `record_id` and subtracts
/// one from its 32-bit reference count. The method scans ten 12-byte records
/// beginning at object offset 0x180; the decrement wraps as an unsigned
/// dword. It uses thiscall with one 32-bit stack argument.
lf_checker_rt::export!(thiscall, rw_d3e2c0(this: u32, record_id: u32) -> () {
    const RECORDS_OFFSET: usize = 0x180;
    const RECORD_STRIDE: usize = 12;
    const RECORD_COUNT: usize = 10;
    const REFCOUNT_OFFSET: usize = 4;

    unsafe {
        let object = this as *mut u8;
        for index in 0..RECORD_COUNT {
            let record = object.add(RECORDS_OFFSET + index * RECORD_STRIDE);
            if (record as *const u32).read_unaligned() == record_id {
                let refcount = record.add(REFCOUNT_OFFSET) as *mut u32;
                refcount.write_unaligned(refcount.read_unaligned().wrapping_sub(1));
                break;
            }
        }
    }
});
