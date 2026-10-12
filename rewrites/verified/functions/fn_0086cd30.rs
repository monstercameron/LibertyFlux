// original: 0x0086CD30 Script_ProgramInsert

/// Insert or reject a program key in the engine's open-addressed table.
///
/// The stdcall parameters are a 32-bit key and its associated value. The
/// function uses unsigned `key % capacity` hashing, treats key words zero
/// and one as available slots, rejects an already-present key and a full
/// table, and updates the value beside a successful insertion. After each
/// occupied nonmatching slot it updates the probe step as `step / 2 + 1`.
/// If capacity is zero it calls the intercepted table allocator, reloads the
/// capacity, and continues with the refreshed globals. The result is a bool
/// in AL and the callee removes both stack arguments.
lf_checker_rt::export!(stdcall, rw_0086CD30(key: u32, value: u32) -> u8 {
    const TABLE_BASE_VA: u32 = 0x0111_1030;
    const TABLE_COUNT_VA: u32 = 0x0111_1034;
    const ALLOC_CAPACITY_VA: u32 = 0x0111_1038;
    const USED_COUNT_VA: u32 = 0x0111_103C;
    const TABLE_ALLOCATOR: u32 = 1;
    const SLOT_WORDS: usize = 2;

    unsafe {
        let mut slot_count = lf_checker_rt::global::<u32>(TABLE_COUNT_VA).read_unaligned();
        if slot_count == 0 {
            let requested_capacity =
                lf_checker_rt::global::<u32>(ALLOC_CAPACITY_VA).read_unaligned();
            let _ = lf_checker_rt::callee_stdcall!(TABLE_ALLOCATOR, u32, requested_capacity);
            slot_count = lf_checker_rt::global::<u32>(TABLE_COUNT_VA).read_unaligned();
        }

        let used_counter = lf_checker_rt::global::<u32>(USED_COUNT_VA);
        let used_count = used_counter.read_unaligned();
        if used_count == slot_count {
            return 0;
        }

        let table_address = lf_checker_rt::global::<u32>(TABLE_BASE_VA).read_unaligned();
        let table = table_address as *mut u32;
        let mut slot_index = key % slot_count;
        let mut probe_key = key;

        loop {
            let slot_word = slot_index.wrapping_mul(SLOT_WORDS as u32) as usize;
            let stored_key = table.add(slot_word).read_unaligned();
            if stored_key <= 1 {
                table.add(slot_word).write_unaligned(key);
                table.add(slot_word + 1).write_unaligned(value);
                used_counter.write_unaligned(used_count.wrapping_add(1));
                return 1;
            }
            if stored_key == key {
                return 0;
            }

            probe_key = (probe_key >> 1).wrapping_add(1);
            slot_index = probe_key.wrapping_add(slot_index) % slot_count;
        }
    }
});
