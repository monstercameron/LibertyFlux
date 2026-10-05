// original: 0x00E67580 task_entries_init_00E67580
/// Initialise the task entries, then forward a code reference.
///
/// Calls callee 1 (thiscall, no stack arguments) with ECX stepping
/// from `TABLE` by `STRIDE` for `ROWS` rows (16: the original
/// counts edi down from 0xF while non-negative), then calls
/// callee 2 (cdecl, one argument: the file address `TAIL`,
/// relocated).
///
/// Original: 0x00E67580 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E67580() -> u32 {
    unsafe {
        const TABLE: u32 = 0x012F9338;
        const STRIDE: u32 = 0x20;
        const ROWS: u32 = 16;
        const TAIL: u32 = 0x00E72250;
        let mut row = 0u32;
        while row < ROWS {
            lf_checker_rt::callee_thiscall!(1, u32,
                lf_checker_rt::relocated(TABLE.wrapping_add(row.wrapping_mul(STRIDE))));
            row += 1;
        }
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(TAIL));
        0
    }
});
