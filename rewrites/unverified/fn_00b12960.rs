// original: 0x00B12960 hash_table_insert
/// Insert a keyed record into the table's bucket chain. The object arrives in
/// ECX; the stack parameters point to a hash word and a twelve-byte payload.
/// The count wraps as an unsigned 16-bit value. Empty-table allocation and
/// capacity rehash are delegated to their callees; the proof contract keeps
/// the table at nonzero capacity below its resize threshold.
lf_checker_rt::export!(thiscall, rw_00b12960(table: u32, hash_entry: u32, payload: u32) -> u32 {
    const BUCKETS_POINTER: u32 = 0;
    const CAPACITY_FIELD: u32 = 4;
    const COUNT_FIELD: u32 = 6;
    const BUCKET_STRIDE: u32 = 4;
    const RECORD_BYTES: u32 = 20;
    const PAYLOAD_TAIL_OFFSET: u32 = 8;

    unsafe {
        let mut capacity = (table.wrapping_add(CAPACITY_FIELD) as *const u16).read_volatile() as u32;
        if capacity == 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, table, 10u32, 1u32);
            capacity = (table.wrapping_add(CAPACITY_FIELD) as *const u16).read_volatile() as u32;
        }

        let count = (table.wrapping_add(COUNT_FIELD) as *mut u16).read_volatile();
        let next_count = count.wrapping_add(1);
        (table.wrapping_add(COUNT_FIELD) as *mut u16).write_volatile(next_count);
        if u32::from(next_count) == capacity {
            let resized_capacity = lf_checker_rt::callee_cdecl!(3, u32, capacity) as u16 as u32;
            let _ = lf_checker_rt::callee_thiscall!(4, u32, table, resized_capacity);
            capacity = (table.wrapping_add(CAPACITY_FIELD) as *const u16).read_volatile() as u32;
        }

        let hash = (hash_entry as *const u32).read_volatile();
        let bucket_index = hash % capacity;
        let buckets = (table.wrapping_add(BUCKETS_POINTER) as *const u32).read_volatile();
        let bucket_slot = buckets.wrapping_add(bucket_index.wrapping_mul(BUCKET_STRIDE));
        let previous_head = (bucket_slot as *const u32).read_volatile();
        let record = lf_checker_rt::callee_cdecl!(1, u32, RECORD_BYTES);
        if record != 0 {
            (record as *mut u32).write_volatile(hash);
            let payload_pair = (payload as *const u64).read_unaligned();
            (record.wrapping_add(4) as *mut u64).write_unaligned(payload_pair);
            let payload_tail = (payload.wrapping_add(PAYLOAD_TAIL_OFFSET) as *const u32).read_volatile();
            (record.wrapping_add(12) as *mut u32).write_volatile(payload_tail);
            (record.wrapping_add(16) as *mut u32).write_volatile(previous_head);
        }
        (bucket_slot as *mut u32).write_volatile(record);
        (bucket_slot as *const u32).read_volatile()
    }
});
