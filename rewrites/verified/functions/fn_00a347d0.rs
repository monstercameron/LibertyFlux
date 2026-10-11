// original: 0x00A347D0 reset_world_entry_tables

/// Calls the entry-reset helper for each of 90 records in the first world
/// table, then clears the active flag and timestamp words in each of 90
/// records in the second table. The first table's stride is 0x2c bytes and
/// the helper receives each record pointer in ECX. The second table's stride
/// is 0x24 bytes, with the two cleared fields at offsets zero and +0x1c. This
/// is a no-argument cdecl routine; EAX returns the relocated address just past
/// the second table.
lf_checker_rt::export!(cdecl, rw_00a347d0() -> u32 {
    const FIRST_TABLE_VA: u32 = 0x012d_e150;
    const SECOND_TABLE_VA: u32 = 0x012d_f0c8;
    const SECOND_TABLE_END_VA: u32 = 0x012d_fd70;
    const FIRST_STRIDE: u32 = 0x2c;
    const SECOND_STRIDE: u32 = 0x24;
    const RECORD_COUNT: u32 = 90;
    const TIMESTAMP_OFFSET: u32 = 0x1c;

    unsafe {
        let first_table = lf_checker_rt::global::<u8>(FIRST_TABLE_VA) as u32;
        let mut index = 0u32;
        while index < RECORD_COUNT {
            let entry = first_table.wrapping_add(index.wrapping_mul(FIRST_STRIDE));
            lf_checker_rt::callee_thiscall!(1, u32, entry);
            index += 1;
        }

        let second_table = lf_checker_rt::global::<u8>(SECOND_TABLE_VA) as u32;
        index = 0;
        while index < RECORD_COUNT {
            let entry = second_table.wrapping_add(index.wrapping_mul(SECOND_STRIDE));
            (entry as *mut u32).write_unaligned(0);
            (entry.wrapping_add(TIMESTAMP_OFFSET) as *mut u32).write_unaligned(0);
            index += 1;
        }

        lf_checker_rt::global::<u8>(SECOND_TABLE_END_VA) as u32
    }
});
