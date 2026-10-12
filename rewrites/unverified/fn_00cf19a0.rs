// original: 0x00CF19A0 StatGroup_GetCount

/// Return the number of entries belonging to one statistics group.
///
/// The cdecl argument is an unsigned group index. Indices zero through seven
/// map to fixed counts; every larger index returns zero. The function only
/// reads its argument and returns a 32-bit count.
lf_checker_rt::export!(cdecl, rw_fn_00cf19a0(group_index: u32) -> u32 {
    const STAT_GROUP_COUNTS: [u32; 8] = [0x62, 0x13, 0x24, 0x1e, 0xe3, 0x45, 0x78, 0x25];
    STAT_GROUP_COUNTS.get(group_index as usize).copied().unwrap_or(0)
});
