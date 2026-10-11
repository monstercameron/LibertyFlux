// original: 0x00D3E320 task_registry_update_or_insert (proposed)
/// Returns one when `record_id` is already present in the ten-entry recent
/// table at offset 0x180. Otherwise it scans the 32-entry primary table at
/// offset 0, stopping at the first empty identifier (-1) or matching record.
/// A match increments the unsigned reference count and compares `new_limit`
/// against the signed value in the parallel record at offset 0x188 plus the
/// table index times 12; the selected value is stored in the primary record. An
/// empty slot receives the identifier, limit, and count one. Records are
/// 12-byte triples; thiscall takes two stack words and returns a byte result.
lf_checker_rt::export!(thiscall, rw_d3e320(this: u32, record_id: u32, new_limit: i32) -> u8 {
    const RECENT_OFFSET: usize = 0x180;
    const RECENT_COUNT: usize = 10;
    const TABLE_OFFSET: usize = 0;
    const TABLE_COUNT: usize = 32;
    const RECORD_STRIDE: usize = 12;
    const ID_OFFSET: usize = 0;
    const REFCOUNT_OFFSET: usize = 4;
    const HIGH_WATER_OFFSET: usize = 8;
    const PARALLEL_LIMIT_OFFSET: usize = 0x188;
    const EMPTY_ID: u32 = u32::MAX;

    unsafe {
        let object = this as *mut u8;
        for index in 0..RECENT_COUNT {
            let entry = object.add(RECENT_OFFSET + index * RECORD_STRIDE);
            if (entry.add(ID_OFFSET) as *const u32).read_unaligned() == record_id {
                return 1;
            }
        }

        for index in 0..TABLE_COUNT {
            let entry = object.add(TABLE_OFFSET + index * RECORD_STRIDE);
            let stored_id = (entry.add(ID_OFFSET) as *const u32).read_unaligned();
            if stored_id == EMPTY_ID {
                (entry.add(ID_OFFSET) as *mut u32).write_unaligned(record_id);
                (entry.add(HIGH_WATER_OFFSET) as *mut i32).write_unaligned(new_limit);
                (entry.add(REFCOUNT_OFFSET) as *mut u32).write_unaligned(1);
                return 0;
            }
            if stored_id == record_id {
                let refcount = entry.add(REFCOUNT_OFFSET) as *mut u32;
                refcount.write_unaligned(refcount.read_unaligned().wrapping_add(1));
                let comparison_limit = object.add(PARALLEL_LIMIT_OFFSET + index * RECORD_STRIDE) as *const i32;
                let previous = comparison_limit.read_unaligned();
                let stored_limit = if new_limit > previous { new_limit } else { previous };
                (entry.add(HIGH_WATER_OFFSET) as *mut i32).write_unaligned(stored_limit);
                return 0;
            }
        }
    }
    0
});
