// original: 0x00C6F810 anim_group_get_index

/// Return the group index field at byte offset 0x48 in the selected 0x58-byte animation-group record. The shared global points to the first record; index arithmetic wraps at 32 bits.
lf_checker_rt::export!(stdcall, rw_0x00c6f810(group_index: u32) -> u32 {
    const GROUP_TABLE_GLOBAL:u32=0x016D_D640;
    const GROUP_STRIDE:u32=0x58;
    const INDEX_FIELD:u32=0x48;
    unsafe {
        let table=lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned();
        (table.wrapping_add(group_index.wrapping_mul(GROUP_STRIDE)).wrapping_add(INDEX_FIELD) as *const u32).read_unaligned()
    }
});
