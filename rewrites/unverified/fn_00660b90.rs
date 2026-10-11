// original: 0x00660B90 sn_key_pair_active_lookup (proposed)

/// Searches a list of key pairs for one matching an active entry.
///
/// `this` points to a record with a signed entry count at byte offset `0x540`,
/// key pairs at `0x2a0` with an eight-byte stride, and activity words at
/// `0x420` with the same stride. `pairs` points to eight-byte `(first, second)`
/// records and `pair_count` is a signed count. A match requires both keys to
/// compare equal and the corresponding activity word to equal exactly one.
/// Either non-positive signed count produces false. The routine is a thiscall
/// method with two stack arguments and returns its boolean result in AL; it
/// does not write memory or call another function.
lf_checker_rt::export!(thiscall, rw_00660b90(this: u32, pairs: u32, pair_count: i32) -> u32 {
    unsafe {
        const PAIR_TABLE: u32 = 0x2a0;
        const ACTIVE_TABLE: u32 = 0x420;
        const TABLE_COUNT: u32 = 0x540;
        const ENTRY_STRIDE: u32 = 8;
        const PAIR_STRIDE: u32 = 8;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        if pair_count <= 0 {
            return 0;
        }
        let table_count = unsafe { read_word(this.wrapping_add(TABLE_COUNT)) as i32 };
        if table_count <= 0 {
            return 0;
        }

        let mut pair_index = 0i32;
        while pair_index < pair_count {
            let pair_address = pairs.wrapping_add((pair_index as u32).wrapping_mul(PAIR_STRIDE));
            let first_key = unsafe { read_word(pair_address) };
            let second_key = unsafe { read_word(pair_address.wrapping_add(4)) };

            let mut entry_index = 0i32;
            while entry_index < table_count {
                let entry_offset = (entry_index as u32).wrapping_mul(ENTRY_STRIDE);
                let key_address = this.wrapping_add(PAIR_TABLE).wrapping_add(entry_offset);
                let active_address = this.wrapping_add(ACTIVE_TABLE).wrapping_add(entry_offset);
                if unsafe { read_word(key_address) } == first_key
                    && unsafe { read_word(key_address.wrapping_add(4)) } == second_key
                    && unsafe { read_word(active_address) } == 1
                {
                    return 1;
                }
                entry_index = entry_index.wrapping_add(1);
            }
            pair_index = pair_index.wrapping_add(1);
        }
        0
    }
});
