// original: 0x009cb3d0 select_timing_table_entry

/// Select one of two seven-entry word tables using a mode byte, then return the entry at the supplied index. The function performs no bounds check; the proof keeps the index within both tables.
lf_checker_rt::export!(cdecl, rw_009cb3d0(index: u32) -> u32 {
    const MODE_VA: u32 = 0x011609F6;
    const PRIMARY_TABLE_VA: u32 = 0x0103ACB4;
    const ALTERNATE_TABLE_VA: u32 = 0x0103ACD0;
    unsafe {
        let mode = lf_checker_rt::global::<u8>(MODE_VA).read();
        let table_va = if mode == 0 { PRIMARY_TABLE_VA } else { ALTERNATE_TABLE_VA };
        lf_checker_rt::global::<u32>(table_va).add(index as usize).read_unaligned()
    }
});
