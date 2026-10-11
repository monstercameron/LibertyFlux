// original: 0X00B4C290 group_entry_from_ped

/// Map a ped's byte-sized group index at offset 0xD32 to its global group
/// record. Index 0xFF is the null sentinel; other indices select 0x5C-byte
/// records beginning at the original group table. The ped arrives as a
/// stack argument and the record pointer is returned in EAX.
lf_checker_rt::export!(cdecl, rw_00B4C290(ped: u32) -> u32 {
    const GROUP_INDEX: u32 = 0xD32;
    const GROUP_TABLE_VA: u32 = 0x0166_6C88;
    const GROUP_ENTRY_STRIDE: u32 = 0x5C;
    const NO_GROUP: u8 = 0xFF;
    // SAFETY: the input ped record includes its group index byte.
    let index = unsafe { (ped.wrapping_add(GROUP_INDEX) as *const u8).read() };
    if index == NO_GROUP { return 0; }
    let table = lf_checker_rt::global::<u8>(GROUP_TABLE_VA) as u32;
    table.wrapping_add(u32::from(index).wrapping_mul(GROUP_ENTRY_STRIDE))
});
