// original: 0x00A34830 clear_world_entry_by_index_fields

/// Clears the active dword and timestamp word of one world-table record. The
/// index is multiplied by nine dwords, yielding the table's 0x24-byte record
/// stride; the timestamp is at +0x1c. The function returns the intermediate
/// nine-dword index in EAX. Its one index argument is a cdecl stack word.
lf_checker_rt::export!(cdecl, rw_00a34830(record_index: u32) -> u32 {
    const TABLE_VA: u32 = 0x012d_f0c8;
    const TIMESTAMP_TABLE_VA: u32 = 0x012d_f0e4;
    const RECORD_WORDS: u32 = 9;

    let scaled_index = record_index.wrapping_mul(RECORD_WORDS);
    let active_table = lf_checker_rt::global::<u8>(TABLE_VA) as u32;
    let timestamp_table = lf_checker_rt::global::<u8>(TIMESTAMP_TABLE_VA) as u32;

    unsafe {
        let active_slot = active_table.wrapping_add(scaled_index.wrapping_mul(4));
        let timestamp_slot = timestamp_table.wrapping_add(scaled_index.wrapping_mul(4));
        (active_slot as *mut u32).write_unaligned(0);
        (timestamp_slot as *mut u32).write_unaligned(0);
    }

    scaled_index
});
