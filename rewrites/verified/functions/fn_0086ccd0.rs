// original: 0x0086CCD0 Script_ProgramLookup

/// Find a nonzero program key in the engine's open-addressed program table.
///
/// The stdcall argument is an unsigned 32-bit key. The function reads a
/// count and a pointer from two engine globals, starts at `key % count`, and
/// probes using a step that is repeatedly updated as `step / 2 + 1` after
/// each occupied nonmatching slot. An empty slot or key zero returns zero;
/// a match returns the value word beside the key. A zero count returns
/// immediately, and the stdcall callee removes the one stack argument.
lf_checker_rt::export!(stdcall, rw_0086CCD0(key: u32) -> u32 {
    const TABLE_BASE_VA: u32 = 0x0111_1030;
    const TABLE_COUNT_VA: u32 = 0x0111_1034;
    const SLOT_WORDS: usize = 2;

    unsafe {
        let slot_count = lf_checker_rt::global::<u32>(TABLE_COUNT_VA).read_unaligned();
        if slot_count == 0 {
            return 0;
        }

        let table_address = lf_checker_rt::global::<u32>(TABLE_BASE_VA).read_unaligned();
        let table = table_address as *const u32;
        let mut slot_index = key % slot_count;
        let mut probe_key = key;

        loop {
            let slot_word = slot_index.wrapping_mul(SLOT_WORDS as u32) as usize;
            let stored_key = table.add(slot_word).read_unaligned();
            if stored_key == key {
                if stored_key == 0 {
                    return 0;
                }
                return table.add(slot_word + 1).read_unaligned();
            }
            if stored_key == 0 {
                return 0;
            }

            probe_key = (probe_key >> 1).wrapping_add(1);
            slot_index = probe_key.wrapping_add(slot_index) % slot_count;
        }
    }
});
