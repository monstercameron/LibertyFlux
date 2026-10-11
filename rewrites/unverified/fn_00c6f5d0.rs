// original: 0x00C6F5D0 anim_hash_lookup

/// Find an exact key in an animation group's sorted 8-byte key/value records. The lower-bound search compares keys as signed 32-bit integers, then returns the associated value or zero. The group count is a signed dword at +0x38 and the pair table pointer is at +0x40; the table rows are 8 bytes.
lf_checker_rt::export!(stdcall, rw_0x00c6f5d0(group_index: u32, key: u32) -> u32 {
    const GROUP_TABLE_GLOBAL: u32 = 0x016D_D640;
    const GROUP_STRIDE: u32 = 0x58;
    const ENTRY_COUNT: usize = 0x38;
    const PAIR_TABLE: usize = 0x40;

    unsafe {
        let table = lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned();
        let group = (table as *const u8).wrapping_add(group_index.wrapping_mul(GROUP_STRIDE) as usize);
        let mut remaining = core::ptr::read_unaligned(group.add(ENTRY_COUNT) as *const i32);
        let mut candidate = core::ptr::read_unaligned(group.add(PAIR_TABLE) as *const u32) as *const u8;
        while remaining != 0 {
            let half = remaining / 2;
            let row = candidate.wrapping_add((half as usize).wrapping_mul(8));
            let row_key = core::ptr::read_unaligned(row as *const u32);
            if (row_key as i32) < (key as i32) {
                candidate = row.wrapping_add(8);
                remaining = remaining - half - 1;
            } else {
                remaining = half;
            }
        }
        if core::ptr::read_unaligned(candidate as *const u32) == key {
            core::ptr::read_unaligned(candidate.add(4) as *const u32)
        } else {
            0
        }
    }
});
