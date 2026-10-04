// original: 0x00b78b90 row_index_of (proposed)

/// Find the index of a row pointer in the row table.
///
/// Compares `ptr` against the 64 row addresses (`ROW_TABLE + i * 0x50`)
/// and returns the first matching index, or `0xFFFF_FFFF` when the pointer
/// is not one of them. Reads no memory.
///
/// Original: cdecl with one stack word, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b78b90(ptr: u32) -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x0167CEA0;
        const ROW_STRIDE: u32 = 0x50;
        const ROWS: u32 = 64;
        const NONE: u32 = 0xFFFF_FFFF;
        let base = lf_checker_rt::relocated(ROW_TABLE);
        let mut i = 0u32;
        while i < ROWS {
            if ptr == base.wrapping_add(i * ROW_STRIDE) {
                return i;
            }
            i += 1;
        }
        NONE
    }
});
