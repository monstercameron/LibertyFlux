// original: 0x00C6FA10 anim_group_has_entry

/// Return AL=1 when the selected animation group's 12-byte entry array contains a matching first dword. The function rejects either 0xffffffff argument, treats a non-positive signed entry count as empty, and uses the group's entry-array pointer at +0x44.
lf_checker_rt::export!(stdcall, rw_0x00c6fa10(group_index: u32, entry_key: u32) -> u32 {
    const GROUP_TABLE_GLOBAL: u32 = 0x016D_D640;
    const GROUP_STRIDE: u32 = 0x58;
    const ENTRY_COUNT: usize = 0x38;
    const ENTRY_LIST: usize = 0x44;
    const ENTRY_STRIDE: usize = 12;

    if group_index == u32::MAX || entry_key == u32::MAX { return 0; }
    unsafe {
        let table = lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned();
        let group = (table as *const u8).wrapping_add(group_index.wrapping_mul(GROUP_STRIDE) as usize);
        let count = core::ptr::read_unaligned(group.add(ENTRY_COUNT) as *const i32);
        if count <= 0 { return 0; }
        let entries = core::ptr::read_unaligned(group.add(ENTRY_LIST) as *const u32) as *const u8;
        let mut index = 0i32;
        while index < count {
            let row = entries.wrapping_add((index as usize).wrapping_mul(ENTRY_STRIDE));
            if core::ptr::read_unaligned(row as *const u32) == entry_key { return 1; }
            index += 1;
        }
        0
    }
});
