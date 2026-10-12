// original: 0x00DC4BE0 task_slot_occupancy_count

/// Count the occupied task slots in a fixed-width table.
///
/// The receiver has 64 four-byte entries beginning at offset 4. The count
/// is the number of entries whose full 32-bit value is nonzero, including
/// entries in all 16 groups of four.
lf_checker_rt::export!(thiscall, rw_00DC4BE0(table: u32) -> u32 {
    unsafe {
        const FIRST_SLOT: u32 = 0x04;
        const GROUP_COUNT: u32 = 16;
        const SLOTS_PER_GROUP: u32 = 4;
        const GROUP_BYTES: u32 = 0x10;

        let mut occupied = 0u32;
        let mut group = 0u32;
        while group < GROUP_COUNT {
            let group_start = table
                .wrapping_add(FIRST_SLOT)
                .wrapping_add(group.wrapping_mul(GROUP_BYTES));
            let mut slot = 0u32;
            while slot < SLOTS_PER_GROUP {
                let address = group_start.wrapping_add(slot * 4);
                if (address as *const u32).read_unaligned() != 0 {
                    occupied += 1;
                }
                slot += 1;
            }
            group += 1;
        }
        occupied
    }
});
