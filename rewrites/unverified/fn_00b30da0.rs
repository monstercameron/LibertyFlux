// original: 0x00B30DA0 ped_task_record_field_0

/// Read a word from the indexed task record table. Indices are one-based and
/// valid only in the inclusive range 1 through 30; invalid indices return zero.
/// Records use a 0x34-byte stride, and this accessor reads byte offset 0x00.
/// The table base is relocated data; the original uses a caller-cleaned cdecl
/// frame with one 32-bit index.
lf_checker_rt::export!(cdecl, rw_b30da0(record_index: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x0166_1424;
        const RECORD_STRIDE: u32 = 0x34;
        const FIELD_OFFSET: u32 = 0x00;
        if record_index.wrapping_sub(1) > 29 {
            return 0;
        }
        let address = lf_checker_rt::relocated(TABLE_VA)
            .wrapping_add(record_index * RECORD_STRIDE)
            .wrapping_add(FIELD_OFFSET);
        (address as *const u32).read_unaligned()
    }
});
