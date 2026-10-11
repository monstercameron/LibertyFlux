// original: 0x00C6F740 anim_group_find_by_index

/// Search the animation-group array's dword field at +0x48 using a signed positive count. Return the zero-based row index for an exact match, or 0xffffffff when the count is non-positive or no row matches. Rows have a 0x58-byte stride.
lf_checker_rt::export!(stdcall, rw_0x00c6f740(group_key: u32) -> u32 {
    const GROUP_COUNT_GLOBAL: u32 = 0x016D_D638;
    const GROUP_TABLE_GLOBAL: u32 = 0x016D_D640;
    const GROUP_STRIDE: usize = 0x58;
    const GROUP_KEY: usize = 0x48;

    unsafe {
        let count = lf_checker_rt::global::<i32>(GROUP_COUNT_GLOBAL).read_unaligned();
        if count <= 0 { return u32::MAX; }
        let base = lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned() as *const u8;
        let mut index = 0u32;
        while (index as i32) < count {
            let row = base.wrapping_add(index as usize * GROUP_STRIDE);
            if core::ptr::read_unaligned(row.add(GROUP_KEY) as *const u32) == group_key {
                return index;
            }
            index = index.wrapping_add(1);
        }
        u32::MAX
    }
});
