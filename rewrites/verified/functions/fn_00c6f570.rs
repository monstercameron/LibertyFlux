// original: 0x00C6F570 anim_copy_entry

/// Search the selected animation group's 12-byte entry records by their first dword. A non-positive signed count yields false. On a match, copy all three dwords to the caller's output buffer and return one in AL; otherwise return zero. The group pointer, count, and entry pointer are read at offsets 0, 0x38, and 0x44, with group records spaced 0x58 bytes apart.
lf_checker_rt::export!(stdcall, rw_0x00c6f570(group_index: u32, key: u32, output: u32) -> u32 {
    const GROUP_TABLE_GLOBAL: u32 = 0x016D_D640;
    const GROUP_STRIDE: u32 = 0x58;
    const ENTRY_COUNT: usize = 0x38;
    const ENTRY_LIST: usize = 0x44;
    const ENTRY_STRIDE: usize = 12;

    unsafe {
        let table = lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned();
        let group = (table as *const u8).wrapping_add(group_index.wrapping_mul(GROUP_STRIDE) as usize);
        let count = core::ptr::read_unaligned(group.add(ENTRY_COUNT) as *const i32);
        if count <= 0 { return 0; }
        let entries = core::ptr::read_unaligned(group.add(ENTRY_LIST) as *const u32) as *const u8;
        let destination = output as *mut u8;
        let mut index = 0i32;
        while index < count {
            let entry = entries.wrapping_add((index as usize).wrapping_mul(ENTRY_STRIDE));
            if core::ptr::read_unaligned(entry as *const u32) == key {
                for word in 0..3usize {
                    let value = core::ptr::read_unaligned(entry.add(word * 4) as *const u32);
                    core::ptr::write_unaligned(destination.add(word * 4) as *mut u32, value);
                }
                return 1;
            }
            index += 1;
        }
        0
    }
});
