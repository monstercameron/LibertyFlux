// original: 0x00C6F560 NativeImpl_GET_ANIM_GROUP_FROM_CHAR_2

/// Return the animation-group record at the supplied index. The index is multiplied by the 0x58-byte record stride and added to the group-table pointer stored in the shared animation table global. There is no bounds check or memory write; arithmetic wraps at 32 bits.
lf_checker_rt::export!(stdcall, rw_0x00c6f560(group_index: u32) -> u32 {
    const GROUP_TABLE_GLOBAL: u32 = 0x016D_D640;
    const GROUP_STRIDE: u32 = 0x58;

    unsafe {
        let group_table = lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned();
        group_table.wrapping_add(group_index.wrapping_mul(GROUP_STRIDE))
    }
});
