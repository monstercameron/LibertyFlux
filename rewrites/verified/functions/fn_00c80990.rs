// original: 0x00c80990 conv_table_entry (proposed)

/// Address row `idx` of the conversation table, or null when out of range.
///
/// Rows are `0xb0` bytes starting at `0x104b990`; indexes above `0x5e` (94,
/// 95 rows) yield null. Used by callers that then read the row's fields.
///
/// Original: cdecl, one stack word (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c80990(idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x104b990;
        const STRIDE: u32 = 0xb0;
        const MAX_IDX: u32 = 0x5e;
        if idx > MAX_IDX {
            return 0;
        }
        lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(STRIDE))
    }
});
