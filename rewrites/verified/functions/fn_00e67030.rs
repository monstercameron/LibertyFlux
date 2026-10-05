// original: 0x00E67030 task_rows_init_00E67030
/// Initialise every row of the task table through the row callee.
///
/// Calls callee 1 (thiscall, no stack arguments) with ECX stepping
/// from `TABLE` by `STRIDE` for `ROWS` rows (205: the original
/// counts edi down from 0xCC while non-negative).
///
/// Original: 0x00E67030 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E67030() -> u32 {
    unsafe {
        const TABLE: u32 = 0x012BD1E0;
        const STRIDE: u32 = 0x280;
        const ROWS: u32 = 205;
        let mut row = 0u32;
        while row < ROWS {
            lf_checker_rt::callee_thiscall!(1, u32,
                lf_checker_rt::relocated(TABLE.wrapping_add(row.wrapping_mul(STRIDE))));
            row += 1;
        }
        0
    }
});
