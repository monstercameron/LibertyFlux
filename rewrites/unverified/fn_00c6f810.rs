// original: 0x00C6F810 anim_group_get_index

/// Return the dword field at +0x48 of the animation-group record selected by the input index. Records are 0x58 bytes apart. The original performs no bounds check and removes one stack argument.
lf_checker_rt::export!(stdcall, rw_0x00c6f810(group_index: u32) -> u32 {
    const GROUP_TABLE_GLOBAL: u32 = 0x016D_D640;
    const GROUP_STRIDE: u32 = 0x58;
    const GROUP_INDEX_FIELD: usize = 0x48;

    unsafe {
        let table = lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned();
        let row = (table as *const u8).wrapping_add(group_index.wrapping_mul(GROUP_STRIDE) as usize);
        core::ptr::read_unaligned(row.add(GROUP_INDEX_FIELD) as *const u32)
    }
});
