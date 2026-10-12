// original: 0x0086CDC0 Script_ProgramRemove

/// Remove a program key from the open-addressed table and leave a tombstone.
///
/// The stdcall argument is an unsigned 32-bit key. The routine starts at
/// `key % capacity`, stops at an empty zero key, and probes occupied slots
/// with a step repeatedly updated as `step / 2 + 1`. A matching key is
/// replaced with tombstone value one, its associated value word is cleared,
/// and the shared used count is decremented with wrapping arithmetic. A
/// match returns true in AL; an empty slot returns false. The caller's one
/// stack argument is removed by stdcall.
lf_checker_rt::export!(stdcall, rw_0086CDC0(key: u32) -> u8 {
    const TABLE_BASE_VA: u32 = 0x0111_1030;
    const TABLE_COUNT_VA: u32 = 0x0111_1034;
    const USED_COUNT_VA: u32 = 0x0111_103C;
    const TOMBSTONE_KEY: u32 = 1;
    const SLOT_WORDS: usize = 2;

    unsafe {
        let slot_count = lf_checker_rt::global::<u32>(TABLE_COUNT_VA).read_unaligned();
        let table_address = lf_checker_rt::global::<u32>(TABLE_BASE_VA).read_unaligned();
        let table = table_address as *mut u32;
        let used_counter = lf_checker_rt::global::<u32>(USED_COUNT_VA);
        let mut slot_index = key % slot_count;
        let mut probe_key = key;

        loop {
            let slot_word = slot_index.wrapping_mul(SLOT_WORDS as u32) as usize;
            let stored_key = table.add(slot_word).read_unaligned();
            if stored_key == 0 {
                return 0;
            }
            if stored_key == key {
                table.add(slot_word).write_unaligned(TOMBSTONE_KEY);
                table.add(slot_word + 1).write_unaligned(0);
                used_counter.write_unaligned(
                    used_counter.read_unaligned().wrapping_sub(1),
                );
                return 1;
            }

            probe_key = (probe_key >> 1).wrapping_add(1);
            slot_index = probe_key.wrapping_add(slot_index) % slot_count;
        }
    }
});
