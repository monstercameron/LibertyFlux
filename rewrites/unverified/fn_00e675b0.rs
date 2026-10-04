// original: 0x00e675b0 clear_task_data_table
/// Clears the 2048-byte task data table to zero.
///
/// Writes 512 zero words over the table. Returns the address one past
/// the table, matching the pointer the original leaves in EAX.
export!(cdecl, rw_00e675b0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x012F9538;
        const WORDS: usize = 512;
        let base = global::<u32>(TABLE);
        let mut i = 0usize;
        while i < WORDS {
            base.add(i).write(0);
            i += 1;
        }
        relocated(TABLE).wrapping_add((WORDS * 4) as u32)
    }
});
