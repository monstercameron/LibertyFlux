// original: 0x00b4c160 group_index_from_entry

/// Find a group record address in the 64-entry table. The 32-bit pointer
/// arrives as one cdecl stack argument; entries are spaced by 0x5C bytes.
/// Return the zero-based entry index or u32::MAX when the pointer is absent.
lf_checker_rt::export!(cdecl, rw_00B4C160(entry: u32) -> u32 {
    const GROUP_TABLE_VA: u32 = 0x0166_6C88;
    const GROUP_STRIDE: u32 = 0x5C;
    const GROUP_COUNT: u32 = 64;

    let first = lf_checker_rt::global::<u8>(GROUP_TABLE_VA) as u32;
    let offset = entry.wrapping_sub(first);
    if offset % GROUP_STRIDE == 0 {
        let index = offset / GROUP_STRIDE;
        if index < GROUP_COUNT { return index; }
    }
    u32::MAX
});
