// original: 0x00DB6470 input_get_action_mapping_flags

/// Read the active mapping page and return the union of the flags for two
/// action indices. The mode word selects a 256-byte page in the mapping table;
/// each index selects one byte on that page. The function uses cdecl with two
/// stack arguments and returns their bytewise OR in EAX.
lf_checker_rt::export!(cdecl, rw_00db6470(first_action: u32, second_action: u32) -> u32 {
    unsafe {
        const MODE_WORD: u32 = 0x018b7da0;
        const MAPPING_TABLE: u32 = 0x018b7a90;
        let mode = lf_checker_rt::global::<u32>(MODE_WORD).read_unaligned();
        let page_offset = mode.wrapping_shl(8);
        let table = lf_checker_rt::global::<u8>(MAPPING_TABLE);
        let first_offset = page_offset.wrapping_add(first_action);
        let second_offset = page_offset.wrapping_add(second_action);
        let first_flags = table.wrapping_add(first_offset as usize).read();
        let second_flags = table.wrapping_add(second_offset as usize).read();
        u32::from(first_flags | second_flags)
    }
});
